//! Build Event Protocol (BEP) parsing for dry-run analysis.

use crate::debug::debug_log;
use prost::Message;
use std::collections::HashMap;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::time::Instant;

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod build_event_stream {
    tonic::include_proto!("build_event_stream");
}

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod command_line {
    tonic::include_proto!("command_line");
}

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod blaze {
    pub mod invocation_policy {
        tonic::include_proto!("blaze.invocation_policy");
    }
    pub mod strategy_policy {
        tonic::include_proto!("blaze.strategy_policy");
    }
    tonic::include_proto!("blaze");
}

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod devtools {
    pub mod build {
        pub mod lib {
            pub mod packages {
                pub mod metrics {
                    tonic::include_proto!("devtools.build.lib.packages.metrics");
                }
            }
        }
    }
}

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod failure_details {
    tonic::include_proto!("failure_details");
}

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod tools {
    pub mod protos {
        tonic::include_proto!("tools.protos");
    }
}

#[allow(dead_code)]
#[allow(clippy::all)]
pub mod options {
    tonic::include_proto!("options");
}

use build_event_stream::{BuildEvent, build_event::Payload, build_event_id::Id};

#[derive(Debug, Default)]
pub struct DryRunResult {
    pub expanded_targets: Vec<String>,
    pub target_kinds: HashMap<String, String>,
    pub explicit_options: Vec<String>,
    pub command: Option<String>,
    pub exit_code: Option<i32>,
    pub success: bool,
    pub errors: Vec<String>,
    pub stderr: String,
}

fn parse_bep_file(path: &Path) -> io::Result<DryRunResult> {
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut result = DryRunResult {
        success: true,
        ..Default::default()
    };

    loop {
        let length = match read_varint(&mut reader) {
            Ok(len) => len,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };

        let mut buf = vec![0u8; length];
        reader.read_exact(&mut buf)?;

        match BuildEvent::decode(&*buf) {
            Ok(event) => process_event(&event, &mut result),
            Err(e) => {
                eprintln!("Warning: Failed to decode BEP event: {}", e);
            }
        }
    }

    Ok(result)
}

fn read_varint<R: Read>(reader: &mut R) -> io::Result<usize> {
    let mut result: usize = 0;
    let mut shift = 0;
    let mut buf = [0u8; 1];

    loop {
        reader.read_exact(&mut buf)?;
        let byte = buf[0];
        result |= ((byte & 0x7f) as usize) << shift;
        shift += 7;

        if (byte & 0x80) == 0 {
            break;
        }

        if shift >= 64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "varint too long",
            ));
        }
    }

    Ok(result)
}

fn process_event(event: &BuildEvent, result: &mut DryRunResult) {
    if let Some(id) = &event.id
        && let Some(Id::Pattern(_)) = &id.id
    {
        for child in &event.children {
            if let Some(child_id) = &child.id {
                match child_id {
                    Id::TargetConfigured(tc) => {
                        if !tc.label.is_empty() {
                            result.expanded_targets.push(tc.label.clone());
                        }
                    }
                    Id::TargetCompleted(tc) => {
                        if !tc.label.is_empty() && !result.expanded_targets.contains(&tc.label) {
                            result.expanded_targets.push(tc.label.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if let Some(payload) = &event.payload {
        match payload {
            Payload::Started(started) => {
                result.command = Some(started.command.clone());
            }
            Payload::OptionsParsed(options) => {
                result.explicit_options = options.explicit_cmd_line.clone();
            }
            Payload::Configured(configured) => {
                if let Some(id) = &event.id
                    && let Some(Id::TargetConfigured(tc)) = &id.id
                    && !tc.label.is_empty()
                    && !configured.target_kind.is_empty()
                {
                    result
                        .target_kinds
                        .insert(tc.label.clone(), configured.target_kind.clone());
                }
            }
            Payload::Aborted(aborted) => {
                use build_event_stream::aborted::AbortReason;
                let reason = AbortReason::try_from(aborted.reason).unwrap_or(AbortReason::Unknown);
                if reason != AbortReason::NoBuild && reason != AbortReason::NoAnalyze {
                    if !aborted.description.is_empty() {
                        result.errors.push(aborted.description.clone());
                    }
                    result.success = false;
                }
            }
            Payload::Finished(finished) => {
                if let Some(exit_code) = &finished.exit_code {
                    result.exit_code = Some(exit_code.code);
                    result.success = exit_code.code == 0;
                }
            }
            Payload::Progress(progress) => {
                if !progress.stderr.is_empty() {
                    result.stderr.push_str(&progress.stderr);
                }
            }
            _ => {}
        }
    }
}

pub fn dry_run(verb: &str, args: &[&str], workspace: &Path) -> Result<DryRunResult, String> {
    use std::process::Command;

    let bep_file = std::env::temp_dir().join(format!("bep_dry_run_{}.bin", std::process::id()));

    let mut cmd = Command::new("bazel");
    cmd.current_dir(workspace);
    cmd.arg(verb);

    let using_nobuild = matches!(verb, "build" | "test" | "run" | "coverage");
    if using_nobuild {
        cmd.arg("--nobuild");
    }

    cmd.args(args);
    cmd.arg(format!("--build_event_binary_file={}", bep_file.display()));

    debug_log!("bep", "Dry run: {:?}", cmd);
    let start = Instant::now();

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run bazel: {}", e))?;

    debug_log!("bep", "Dry run completed in {:.2?}", start.elapsed());

    let result = if bep_file.exists() {
        let result =
            parse_bep_file(&bep_file).map_err(|e| format!("Failed to parse BEP file: {}", e))?;
        let _ = std::fs::remove_file(&bep_file);
        result
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Dry run failed: {}", stderr));
    };

    if !result.errors.is_empty() {
        return Err(result.errors.join("\n"));
    }

    if result.stderr.contains("ERROR:") {
        let errors: Vec<&str> = result
            .stderr
            .lines()
            .filter(|line| {
                (line.contains("ERROR:") || line.contains("no such"))
                    && !line.contains("Unable to run tests")
                    && !line.contains("Couldn't start the build")
            })
            .collect();
        if !errors.is_empty() {
            return Err(errors.join("\n"));
        }
    }

    if let Some(exit_code) = result.exit_code
        && exit_code != 0
        && !using_nobuild
    {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Dry run failed with exit code {}: {}",
            exit_code, stderr
        ));
    }

    Ok(result)
}

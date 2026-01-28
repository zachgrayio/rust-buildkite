fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "bazel")]
    {
        use std::env;
        use std::path::PathBuf;

        let out_dir = PathBuf::from(env::var("OUT_DIR")?);

        tonic_build::configure()
            .build_server(false)
            .build_client(false)
            .file_descriptor_set_path(out_dir.join("proto_descriptor.bin"))
            .emit_rerun_if_changed(false)
            .compile_protos(
                &[
                    "proto/src/main/java/com/google/devtools/build/lib/buildeventstream/proto/build_event_stream.proto",
                    "proto/src/main/java/com/google/devtools/build/lib/packages/metrics/package_load_metrics.proto",
                    "proto/src/main/protobuf/action_cache.proto",
                    "proto/src/main/protobuf/command_line.proto",
                    "proto/src/main/protobuf/failure_details.proto",
                    "proto/src/main/protobuf/invocation_policy.proto",
                    "proto/src/main/protobuf/option_filters.proto",
                    "proto/src/main/protobuf/spawn.proto",
                    "proto/src/main/protobuf/strategy_policy.proto",
                ],
                &[
                    "proto",
                    "proto/src",
                    "proto/src/main/java",
                    "proto/src/main/protobuf",
                ],
            )?;

        println!("cargo:rerun-if-changed=proto");
    }

    Ok(())
}

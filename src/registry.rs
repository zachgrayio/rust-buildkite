#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchPattern {
    Exact(&'static str),
    Prefix(&'static str),
    AnyPrefix(&'static [&'static str]),
}

impl BranchPattern {
    #[must_use]
    pub fn matches(&self, branch: &str) -> bool {
        match self {
            BranchPattern::Exact(s) => branch == *s,
            BranchPattern::Prefix(p) => branch.starts_with(p),
            BranchPattern::AnyPrefix(prefixes) => prefixes.iter().any(|p| branch.starts_with(p)),
        }
    }
}

pub struct PipelineRegistration {
    pub id: &'static str,
    pub name: &'static str,
    pub generate: fn(),
    pub branch: Option<BranchPattern>,
    pub cron: Option<&'static str>,
}

/// Macro to ensure registered pipelines are linked into the binary.
///
/// Due to how Rust's linker works with `inventory`, pipeline modules that aren't
/// directly referenced by the binary may be stripped. This macro creates the
/// necessary references to force linking.
///
/// # Example
///
/// In your library's `lib.rs`:
/// ```ignore
/// rust_buildkite::link_pipelines!(
///     pipelines::premerge::premerge,
///     pipelines::postmerge::postmerge,
///     pipelines::release::release,
/// );
/// ```
///
/// Then in your binary:
/// ```ignore
/// fn main() {
///     mylib::link_pipelines();
///     for p in rust_buildkite::registered_pipelines() {
///         // ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! link_pipelines {
    ($($fn_path:path),* $(,)?) => {
        /// Forces the linker to include all registered pipeline modules.
        #[inline(never)]
        pub fn link_pipelines() {
            // reference each pipeline function to prevent linker from stripping the module
            let _: &[fn()] = &[$($fn_path),*];
        }
    };
}

inventory::collect!(PipelineRegistration);

pub fn registered_pipelines() -> impl Iterator<Item = &'static PipelineRegistration> {
    inventory::iter::<PipelineRegistration>()
}

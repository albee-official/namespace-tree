use std::path::PathBuf;

/// A single type declaration (class/interface/struct/enum/record) found in a file,
/// tagged with the fully-qualified namespace it lives in.
#[derive(Debug, Clone)]
pub struct TypeEntry {
    pub namespace: String,
    pub kind: String,
    pub name: String,
    pub file: PathBuf,
}

pub const GLOBAL_NAMESPACE: &str = "<global>";

use std::fs;
use std::path::Path;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::model::{TypeEntry, GLOBAL_NAMESPACE};

// Matches: namespace Foo.Bar.Baz { ... }  OR  namespace Foo.Bar.Baz;  (file-scoped)
static NAMESPACE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?m)^\s*namespace\s+([A-Za-z_][A-Za-z0-9_.]*)\s*[{;]").unwrap()
});

// Matches type declarations, capturing kind + name.
// Handles compound kinds like "record class" / "record struct" as a single unit,
// as well as plain "record Foo(...)" positional records.
static TYPE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?m)^\s*(?:public|internal|private|protected|static|sealed|abstract|partial|\s)*(record\s+class|record\s+struct|class|interface|struct|enum|record)\s+([A-Za-z_][A-Za-z0-9_]*)",
    )
    .unwrap()
});

static BLOCK_COMMENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?s)/\*.*?\*/").unwrap());
static LINE_COMMENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"//.*").unwrap());
static WHITESPACE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());

/// Remove // line comments and /* */ block comments (best-effort, not string-safe).
fn strip_comments(text: &str) -> String {
    let no_block = BLOCK_COMMENT_RE.replace_all(text, "");
    let no_line = LINE_COMMENT_RE.replace_all(&no_block, "");
    no_line.into_owned()
}

/// Parse a single .cs file, returning all (namespace, kind, name, file) entries found.
/// Handles both block-scoped and file-scoped namespaces, and multiple namespaces per file.
pub fn parse_file(path: &Path) -> Vec<TypeEntry> {
    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("  ! Could not read {}: {}", path.display(), e);
            return Vec::new();
        }
    };

    // Strip a UTF-8 BOM if present, mirroring Python's utf-8-sig handling.
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(&raw);
    let text = strip_comments(raw);

    let ns_matches: Vec<_> = NAMESPACE_RE.captures_iter(&text).collect();

    let mut results = Vec::new();

    if ns_matches.is_empty() {
        for tcap in TYPE_RE.captures_iter(&text) {
            let kind = normalize_kind(&tcap[1]);
            let name = tcap[2].to_string();
            results.push(TypeEntry {
                namespace: GLOBAL_NAMESPACE.to_string(),
                kind,
                name,
                file: path.to_path_buf(),
            });
        }
        return results;
    }

    for (i, ns_cap) in ns_matches.iter().enumerate() {
        let ns_name = ns_cap[1].to_string();
        let whole = ns_cap.get(0).unwrap();
        let start = whole.end();
        let end = ns_matches
            .get(i + 1)
            .map(|m| m.get(0).unwrap().start())
            .unwrap_or(text.len());
        let segment = &text[start..end];

        for tcap in TYPE_RE.captures_iter(segment) {
            let kind = normalize_kind(&tcap[1]);
            let name = tcap[2].to_string();
            results.push(TypeEntry {
                namespace: ns_name.clone(),
                kind,
                name,
                file: path.to_path_buf(),
            });
        }
    }

    results
}

fn normalize_kind(raw: &str) -> String {
    WHITESPACE_RE.replace_all(raw.trim(), " ").into_owned()
}

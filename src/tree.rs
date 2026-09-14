use std::collections::BTreeMap;

use serde::Serialize;

use crate::model::{TypeEntry, GLOBAL_NAMESPACE};

/// A node in the namespace tree. Children are kept in a BTreeMap so iteration
/// is always alphabetically sorted, matching the Python script's `sorted()` calls.
#[derive(Debug, Default)]
pub struct Node {
    pub children: BTreeMap<String, Node>,
    pub types: Vec<TypeEntry>,
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert one type entry, walking/creating namespace segments as needed.
    fn insert(&mut self, entry: TypeEntry) {
        let parts: Vec<String> = if entry.namespace == GLOBAL_NAMESPACE {
            vec![GLOBAL_NAMESPACE.to_string()]
        } else {
            entry.namespace.split('.').map(|s| s.to_string()).collect()
        };

        let mut node = self;
        for part in parts {
            node = node.children.entry(part).or_insert_with(Node::new);
        }
        node.types.push(entry);
    }
}

/// Build a nested namespace tree from a flat list of type entries.
pub fn build_tree(entries: Vec<TypeEntry>) -> Node {
    let mut root = Node::new();
    for entry in entries {
        root.insert(entry);
    }
    root
}

/// Render the tree as a `tree`-style ASCII listing, optionally including each
/// type's source file path.
pub fn render_text(root: &Node, show_files: bool) -> Vec<String> {
    let mut out = Vec::new();
    render_text_node(root, "", &mut out, show_files);
    out
}

fn render_text_node(node: &Node, prefix: &str, out: &mut Vec<String>, show_files: bool) {
    let keys: Vec<&String> = node.children.keys().collect();
    let has_types = !node.types.is_empty();

    for (idx, key) in keys.iter().enumerate() {
        let last = idx == keys.len() - 1 && !has_types;
        let connector = if last { "└── " } else { "├── " };
        out.push(format!("{prefix}{connector}{key}"));
        let extension = if last { "    " } else { "│   " };
        render_text_node(&node.children[*key], &format!("{prefix}{extension}"), out, show_files);
    }

    let mut sorted_types: Vec<&TypeEntry> = node.types.iter().collect();
    sorted_types.sort_by(|a, b| a.name.cmp(&b.name));

    for (idx, t) in sorted_types.iter().enumerate() {
        let last = idx == sorted_types.len() - 1;
        let connector = if last { "└── " } else { "├── " };
        let suffix = if show_files {
            format!("  ({})", t.file.display())
        } else {
            String::new()
        };
        out.push(format!("{prefix}{connector}[{}] {}{}", t.kind, t.name, suffix));
    }
}

/// JSON-serializable mirror of the tree, matching the shape produced by the
/// original Python `tree_to_dict`.
#[derive(Serialize)]
pub struct JsonNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<BTreeMap<String, JsonNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<JsonType>>,
}

#[derive(Serialize)]
pub struct JsonType {
    pub kind: String,
    pub name: String,
    pub file: String,
}

pub fn tree_to_json(node: &Node) -> JsonNode {
    let namespaces = if node.children.is_empty() {
        None
    } else {
        Some(
            node.children
                .iter()
                .map(|(k, v)| (k.clone(), tree_to_json(v)))
                .collect(),
        )
    };

    let types = if node.types.is_empty() {
        None
    } else {
        Some(
            node.types
                .iter()
                .map(|t| JsonType {
                    kind: t.kind.clone(),
                    name: t.name.clone(),
                    file: t.file.display().to_string(),
                })
                .collect(),
        )
    };

    JsonNode { namespaces, types }
}

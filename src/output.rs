use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::cli::MermaidStyle;
use crate::mermaid;
use crate::tree::Node;

pub fn write_markdown(path: &Path, tree_text: &str, ns_count: usize, type_count: usize) -> Result<()> {
    let content = format!(
        "# Namespace Tree\n\n```\n{tree_text}\n```\n\n_{ns_count} namespaces, {type_count} types_"
    );
    fs::write(path, content).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

pub fn write_json(path: &Path, json_tree: &crate::tree::JsonNode) -> Result<()> {
    let content = serde_json::to_string_pretty(json_tree)?;
    fs::write(path, content).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

pub fn write_mermaid(path: &Path, root: &Node, title: &str, style: MermaidStyle) -> Result<()> {
    let mermaid_src = match style {
        MermaidStyle::Mindmap => mermaid::to_mindmap(root, title),
        MermaidStyle::Flowchart => mermaid::to_flowchart(root, title),
    };

    let is_md = path
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase() == "md")
        .unwrap_or(false);

    let content = if is_md {
        // Wrap in a fenced mermaid code block so it renders in VS Code / GitHub previews
        format!("# Namespace Tree (Mermaid)\n\n```mermaid\n{mermaid_src}\n```\n")
    } else {
        format!("{mermaid_src}\n")
    };

    fs::write(path, content).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

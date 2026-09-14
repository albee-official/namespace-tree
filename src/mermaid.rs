use crate::tree::Node;

/// Build a Mermaid flowchart (left-to-right) from the namespace tree, with
/// color-coded styling per type kind. LR reads much better than TD for deep,
/// branchy trees since it grows down the page instead of sprawling sideways.
pub fn to_flowchart(root: &Node, title: &str) -> String {
    let mut lines = vec!["flowchart LR".to_string()];
    let mut counter = 0usize;

    let root_id = next_id(&mut counter);
    lines.push(format!("    {root_id}[\"{}\"]:::nsNode", title));

    walk_flowchart(root, &root_id, &mut counter, &mut lines);

    lines.push(String::new());
    lines.push("    classDef nsNode fill:#2b6cb0,color:#fff,stroke:#1a4971,font-weight:bold".into());
    lines.push("    classDef classNode fill:#38a169,color:#fff,stroke:#276749".into());
    lines.push("    classDef ifaceNode fill:#d69e2e,color:#fff,stroke:#975a16".into());
    lines.push("    classDef structNode fill:#805ad5,color:#fff,stroke:#553c9a".into());
    lines.push("    classDef enumNode fill:#dd6b20,color:#fff,stroke:#9c4221".into());
    lines.push("    classDef recordNode fill:#e53e3e,color:#fff,stroke:#9b2c2c".into());

    lines.join("\n")
}

fn next_id(counter: &mut usize) -> String {
    *counter += 1;
    format!("n{counter}")
}

fn kind_class(kind: &str) -> &'static str {
    match kind {
        "class" => "classNode",
        "interface" => "ifaceNode",
        "struct" => "structNode",
        "enum" => "enumNode",
        "record" | "record class" | "record struct" => "recordNode",
        _ => "classNode",
    }
}

fn walk_flowchart(node: &Node, parent_id: &str, counter: &mut usize, lines: &mut Vec<String>) {
    for (key, child) in &node.children {
        let node_id = next_id(counter);
        let safe_label = key.replace('"', "'");
        lines.push(format!("    {node_id}(\"{safe_label}\"):::nsNode"));
        lines.push(format!("    {parent_id} --> {node_id}"));
        walk_flowchart(child, &node_id, counter, lines);
    }

    let mut sorted_types = node.types.iter().collect::<Vec<_>>();
    sorted_types.sort_by(|a, b| a.name.cmp(&b.name));

    for t in sorted_types {
        let type_id = next_id(counter);
        let safe_name = t.name.replace('"', "'");
        let safe_kind = t.kind.replace('"', "'");
        let css_class = kind_class(&t.kind);
        lines.push(format!(
            "    {type_id}[\"{safe_kind}: {safe_name}\"]:::{css_class}"
        ));
        lines.push(format!("    {parent_id} --> {type_id}"));
    }
}

/// Build a Mermaid mindmap from the namespace tree. Mindmaps are purpose-built
/// for hierarchies like this and lay out far more compactly than a flowchart,
/// radiating out from the root instead of sprawling in one direction.
pub fn to_mindmap(root: &Node, title: &str) -> String {
    let mut lines = vec!["mindmap".to_string()];
    let safe_title = title.replace('"', "'");
    lines.push(format!("  root((\"{safe_title}\"))"));

    walk_mindmap(root, 2, &mut lines);

    lines.join("\n")
}

fn walk_mindmap(node: &Node, indent: usize, lines: &mut Vec<String>) {
    let pad = "  ".repeat(indent);

    for (key, child) in &node.children {
        let safe_label = key.replace('"', "'").replace('(', "").replace(')', "");
        lines.push(format!("{pad}{safe_label}"));
        walk_mindmap(child, indent + 1, lines);
    }

    let mut sorted_types = node.types.iter().collect::<Vec<_>>();
    sorted_types.sort_by(|a, b| a.name.cmp(&b.name));

    for t in sorted_types {
        let safe_name = t.name.replace('"', "'").replace('(', "").replace(')', "");
        let safe_kind = t.kind.replace('"', "'");
        lines.push(format!("{pad}\"{safe_kind}: {safe_name}\""));
    }
}

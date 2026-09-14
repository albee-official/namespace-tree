use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Scans a folder of C# (.cs) files, extracts namespace + type declarations
/// (class, interface, struct, enum, record), and prints/exports a tree
/// grouped PURELY by namespace (folder structure is ignored).
#[derive(Parser, Debug)]
#[command(name = "namespace-tree", version, about, long_about = None)]
pub struct Args {
    /// Root folder to scan for .cs files
    pub root: PathBuf,

    /// Write a Markdown file with the tree (e.g. tree.md)
    #[arg(long)]
    pub out: Option<PathBuf>,

    /// Write a JSON file with the tree (e.g. tree.json)
    #[arg(long)]
    pub json: Option<PathBuf>,

    /// Write a Mermaid diagram file (e.g. tree.mmd or tree.md)
    #[arg(long)]
    pub mermaid: Option<PathBuf>,

    /// Mermaid diagram style
    #[arg(long, value_enum, default_value_t = MermaidStyle::Mindmap)]
    pub mermaid_style: MermaidStyle,

    /// Show the source .cs file path next to each type in the printed/Markdown tree
    #[arg(long)]
    pub show_files: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum MermaidStyle {
    Flowchart,
    Mindmap,
}

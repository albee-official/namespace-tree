mod cli;
mod mermaid;
mod model;
mod output;
mod parser;
mod tree;

use std::collections::HashSet;
use std::process::ExitCode;

use clap::Parser;
use walkdir::WalkDir;

use cli::Args;
use model::TypeEntry;

fn main() -> ExitCode {
    let args = Args::parse();

    if !args.root.exists() {
        eprintln!("Error: path '{}' does not exist.", args.root.display());
        return ExitCode::from(1);
    }

    // Discover .cs files, skipping generated / obj / bin / .vs folders.
    let cs_files: Vec<_> = WalkDir::new(&args.root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext == "cs").unwrap_or(false))
        .filter(|e| {
            !e.path()
                .components()
                .any(|c| matches!(c.as_os_str().to_str(), Some("obj") | Some("bin") | Some(".vs")))
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if cs_files.is_empty() {
        println!("No .cs files found under '{}'.", args.root.display());
        return ExitCode::SUCCESS;
    }

    println!(
        "Scanning {} .cs file(s) under '{}'...\n",
        cs_files.len(),
        args.root.display()
    );

    let all_entries: Vec<TypeEntry> = cs_files.iter().flat_map(|f| parser::parse_file(f)).collect();

    if all_entries.is_empty() {
        println!("No namespace/type declarations found.");
        return ExitCode::SUCCESS;
    }

    let ns_count = all_entries
        .iter()
        .map(|e| e.namespace.as_str())
        .collect::<HashSet<_>>()
        .len();
    let type_count = all_entries.len();

    let root_node = tree::build_tree(all_entries);
    let lines = tree::render_text(&root_node, args.show_files);
    let text_output = lines.join("\n");

    println!("{text_output}");
    println!("\n{ns_count} namespace(s), {type_count} type(s) found.");

    let title = args
        .root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Namespace Tree".to_string());

    if let Some(out_path) = &args.out {
        if let Err(e) = output::write_markdown(out_path, &text_output, ns_count, type_count) {
            eprintln!("Error writing markdown: {e}");
            return ExitCode::from(1);
        }
        println!("\nMarkdown tree written to: {}", out_path.display());
    }

    if let Some(json_path) = &args.json {
        let json_tree = tree::tree_to_json(&root_node);
        if let Err(e) = output::write_json(json_path, &json_tree) {
            eprintln!("Error writing JSON: {e}");
            return ExitCode::from(1);
        }
        println!("JSON tree written to: {}", json_path.display());
    }

    if let Some(mermaid_path) = &args.mermaid {
        if let Err(e) = output::write_mermaid(mermaid_path, &root_node, &title, args.mermaid_style) {
            eprintln!("Error writing Mermaid diagram: {e}");
            return ExitCode::from(1);
        }
        println!("Mermaid diagram written to: {}", mermaid_path.display());
    }

    ExitCode::SUCCESS
}

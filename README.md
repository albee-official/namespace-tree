# namespace-tree

A Rust CLI that scans a folder of C# (`.cs`) files, extracts namespace + type
declarations (`class`, `interface`, `struct`, `enum`, `record`), and prints or
exports a tree grouped purely by namespace (folder structure is ignored).

## Layout

```
src/
  main.rs     - CLI entry point: file discovery, orchestration, console output
  cli.rs      - clap argument definitions
  model.rs    - shared data types (TypeEntry)
  parser.rs   - regex-based .cs parsing (namespace + type extraction, comment stripping)
  tree.rs     - namespace tree structure, text rendering, JSON conversion
  mermaid.rs  - Mermaid flowchart / mindmap diagram generation
  output.rs   - file-writing helpers (markdown / json / mermaid)
```

## Build

```bash
cargo build --release
```

## Usage

```bash
namespace-tree <ROOT_FOLDER> [OPTIONS]

Options:
      --out <OUT>                    Write a Markdown file with the tree (e.g. tree.md)
      --json <JSON>                  Write a JSON file with the tree (e.g. tree.json)
      --mermaid <MERMAID>            Write a Mermaid diagram file (e.g. tree.mmd or tree.md)
      --mermaid-style <STYLE>        Mermaid diagram style [default: mindmap] [possible values: flowchart, mindmap]
      --show-files                   Show the source .cs file path next to each type
  -h, --help                         Print help
  -V, --version                      Print version
```

### Examples

```bash
namespace-tree ./MySolution
namespace-tree ./MySolution --out namespaces.md --show-files
namespace-tree ./MySolution --json tree.json --mermaid tree.mmd --mermaid-style flowchart
```

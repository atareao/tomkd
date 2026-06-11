<div align="center">

# tomkd

**Convert HTML to Markdown from the command line**

[![Crates.io](https://img.shields.io/crates/v/tomkd?style=flat-square)](https://crates.io/crates/tomkd)
[![Rust](https://img.shields.io/badge/Rust-2024-dea584?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

[Features](#features) • [Installation](#installation) • [Usage](#usage)

</div>

`tomkd` is a fast, zero-config CLI tool that converts HTML documents to Markdown. It reads from a file or stdin and writes clean Markdown—perfect for scripting, content migration, or preprocessing HTML for documentation.

## Features

- **File-to-file conversion** – Pass an input and output path for batch processing.
- **Automatic output path** – Omit `-o` and the output file name is derived from the input (`.html` → `.md`).
- **Stdin support** – Pipe HTML directly into `tomkd`.
- **Lightweight** – Built on [`html-to-markdown-rs`](https://crates.io/crates/html-to-markdown-rs) for reliable, standards-compliant conversion.
- **Debug logging** – Trace input size, conversion steps, and output path with structured logs.

## Installation

```bash
cargo install tomkd
```

## Usage

```bash
tomkd -i article.html -o article.md    # Convert file to Markdown
tomkd -i article.html                    # Derive output as article.md
cat article.html | tomkd                 # Read from stdin, write to output.md
cat article.html | tomkd -o out.md       # Read from stdin, write to out.md
```

### Options

| Flag | Description |
|---|---|
| `-i, --input <FILE>` | Input HTML file (reads from stdin if omitted) |
| `-o, --output <FILE>` | Output Markdown file (derived from input or defaults to `output.md`) |
| `-V, --version` | Print version |
| `-h, --help` | Print help |

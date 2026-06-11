use clap::Parser;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use tracing::{Level, debug, info, instrument};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Parser)]
#[command(
    name = "tomkd",
    about = "Convert HTML to Markdown",
    long_about = "Converts HTML documents to Markdown format.\n\nReads HTML from a file or stdin and writes Markdown to a file or stdout.\nIf no output file is specified, the output path is derived from the input\npath by changing the extension to .md, or defaults to output.md when\nreading from stdin.",
    version,
    after_help = "EXAMPLES:\n  tomkd -i article.html -o article.md    Convert file to Markdown\n  tomkd -i article.html                    Derive output as article.md\n  cat article.html | tomkd                 Read from stdin, write to output.md\n  cat article.html | tomkd -o out.md       Read from stdin, write to out.md"
)]
struct Args {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Input HTML file (reads from stdin if omitted"
    )]
    input: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Output Markdown file (derived from input or defaults to output.md if omitted"
    )]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args = Args::parse();
    info!("tomkd started");

    let html = read_input(&args)?;
    let markdown = convert_html(&html)?;
    write_output(&args, &markdown)?;

    info!("conversion complete");
    Ok(())
}

#[instrument]
fn read_input(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    let html = match &args.input {
        Some(path) => {
            debug!(input = ?path, "reading file");
            fs::read_to_string(path)?
        }
        None => {
            debug!("reading stdin");
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };
    debug!(len = html.len(), "input read");
    Ok(html)
}

#[instrument]
fn convert_html(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    info!("converting html to markdown");
    let result = html_to_markdown_rs::convert(html, None)?;
    let markdown = result.content.unwrap_or_default();
    debug!(len = markdown.len(), "conversion done");
    Ok(markdown)
}

#[instrument]
fn write_output(args: &Args, markdown: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = match args.output {
        Some(ref path) => path.clone(),
        None => match &args.input {
            Some(path) => path.with_extension("md"),
            None => PathBuf::from("output.md"),
        },
    };
    debug!(output = ?output_path, "writing output");
    fs::write(&output_path, markdown)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_html_basic() {
        let html = "<p>Hello <strong>world</strong></p>";
        let result = convert_html(html).unwrap();
        assert_eq!(result, "Hello **world**\n");
    }

    #[test]
    fn test_convert_html_heading() {
        let html = "<h1>Title</h1><p>Body</p>";
        let result = convert_html(html).unwrap();
        assert_eq!(result, "# Title\n\nBody\n");
    }

    #[test]
    fn test_convert_html_link() {
        let html = r#"<a href="https://example.com">click here</a>"#;
        let result = convert_html(html).unwrap();
        assert_eq!(result, "[click here](https://example.com)\n");
    }

    #[test]
    fn test_convert_html_empty() {
        let result = convert_html("").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_convert_html_list() {
        let html = "<ul><li>one</li><li>two</li></ul>";
        let result = convert_html(html).unwrap();
        assert_eq!(result, "- one\n- two\n");
    }

    #[test]
    fn test_write_output_creates_file() {
        let dir = std::env::temp_dir().join("tomkd_test_output");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let out_path = dir.join("out.md");
        let args = Args {
            input: None,
            output: Some(out_path.clone()),
        };
        write_output(&args, "# Hello\n").unwrap();

        let content = std::fs::read_to_string(&out_path).unwrap();
        assert_eq!(content, "# Hello\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_write_output_derives_path_from_input() {
        let dir = std::env::temp_dir().join("tomkd_test_derive");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let input_path = dir.join("article.html");
        std::fs::write(&input_path, "<p>test</p>").unwrap();

        let args = Args {
            input: Some(input_path),
            output: None,
        };
        let html = read_input(&args).unwrap();
        let markdown = convert_html(&html).unwrap();
        write_output(&args, &markdown).unwrap();

        let derived = dir.join("article.md");
        let content = std::fs::read_to_string(&derived).unwrap();
        assert_eq!(content, "test\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_write_output_default_path() {
        let args = Args {
            input: None,
            output: None,
        };
        write_output(&args, "content").unwrap();
        let content = std::fs::read_to_string("output.md").unwrap();
        assert_eq!(content, "content");
        std::fs::remove_file("output.md").unwrap();
    }

    #[test]
    fn test_convert_html_multiline_paragraph() {
        let html = "<p>First line</p><p>Second line</p>";
        let result = convert_html(html).unwrap();
        assert_eq!(result, "First line\n\nSecond line\n");
    }
}

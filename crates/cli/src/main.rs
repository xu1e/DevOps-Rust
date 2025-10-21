use clap::Parser;
use std::path::PathBuf;

/// Simple CLI to demonstrate reading files and running utilities
#[derive(Parser)]
struct Args {
    /// Path to a JSON or YAML file to read
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// A regex pattern to test against sample text
    #[arg(short, long)]
    pattern: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(pat) = args.pattern {
        let text = "the quick brown fox jumps over 13 lazy dogs";
        match core::contains_pattern(text, &pat) {
            Ok(found) => println!("pattern found? {}", found),
            Err(e) => println!("invalid pattern: {}", e),
        }
    }

    if let Some(input) = args.input {
        if input.extension().and_then(|s| s.to_str()) == Some("json") {
            let v: serde_json::Value = files_utils::read_json_file(&input)?;
            println!("Read JSON: {}", v);
        } else if input.extension().and_then(|s| s.to_str()) == Some("yaml") ||
                  input.extension().and_then(|s| s.to_str()) == Some("yml") {
            let v: serde_yaml::Value = files_utils::read_yaml_file(&input)?;
            println!("Read YAML: {:#?}", v);
        } else {
            println!("Unsupported input type");
        }
    }

    Ok(())
}

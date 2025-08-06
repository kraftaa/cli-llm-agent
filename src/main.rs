mod llm;

use clap::{Parser, Subcommand};
use std::fs;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "sql_explainer_local")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Explain SQL from text (-t) or file (-f)
    Explain {
        #[arg(short, long)]
        t: Option<String>,
        #[arg(short, long)]
        f: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let input = match cli.command {
        Commands::Explain { t: Some(text), .. } => text,
        Commands::Explain { f: Some(path), .. } => fs::read_to_string(path)?,
        _ => {
            eprintln!("Error: Please provide either -t \"text\" or -f \"file.sql\"");
            return Ok(());
        }
    };

    // Prepare a clear prompt for the model
    let prompt = format!(
        "Explain this SQL query clearly:\n\n{}\n\nExplanation:",
        input
    );

    let explanation = llm::infer_local(&prompt)?;
    // Clean output a bit
    let explanation = explanation.trim_start_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace());

    println!("\n🧠 Explanation:\n{}", explanation);

    Ok(())
}

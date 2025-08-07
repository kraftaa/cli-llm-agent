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
use llama_cpp::{LlamaModel, LlamaParams, SessionParams};
use std::path::Path;
use std::io::{self, Write};

#[tokio::main]
// async fn main() -> Result<()> {
//     let cli = Cli::parse();
//
//     let input = match cli.command {
//         Commands::Explain { t: Some(text), .. } => text,
//         Commands::Explain { f: Some(path), .. } => fs::read_to_string(path)?,
//         _ => {
//             eprintln!("Error: Please provide either -t \"text\" or -f \"file.sql\"");
//             return Ok(());
//         }
//     };
//
//     // Prepare a clear prompt for the model
//     let prompt = format!(
//         "Explain this SQL query clearly:\n\n{}\n\nExplanation:",
//         input
//     );
//
//     let explanation = llm::infer_local(&prompt)?;
//     // Clean output a bit
//     let explanation = explanation.trim_start_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace());
//
//     println!("\n🧠 Explanation:\n{}", explanation);
//
//     Ok(())
// }

// trying not to load model each time
// async fn main() -> Result<()> {
//     let cli = Cli::parse();
//
//     let input = match cli.command {
//         Commands::Explain { t: Some(text), .. } => text,
//         Commands::Explain { f: Some(path), .. } => fs::read_to_string(path)?,
//         _ => {
//             eprintln!("Error: Please provide either -t \"text\" or -f \"file.sql\"");
//             return Ok(());
//         }
//     };
//
//     // Prepare a clear prompt for the model
//     let prompt = format!(
//         "Explain this SQL query clearly:\n\n{}\n\nExplanation:",
//         input
//     );
//
//     let explanation = llm::infer_local(&prompt)?;
//     // Clean output a bit
//     let explanation = explanation.trim_start_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace());
//
//     println!("\n🧠 Explanation:\n{}", explanation);
//
//     Ok(())
// }
// cargo run -- explain -t "SELECT * FROM orders WHERE amount > 1000 and id in (1,2)"

async fn main() -> Result<()> {
        // let model_path = Path::new("models/ggml-model-q4_0.gguf");
        let model_path = Path::new("models/llama-2-7b-chat.Q4_0.gguf");
        println!("Loading model from {:?}", model_path);

        let model = LlamaModel::load_from_file(model_path, LlamaParams::default())?;
        let mut session = model.create_session(SessionParams::default())?;
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

    // let explanation = llm::infer_local(&prompt)?;
    let explanation = llm::infer_local(&mut session, &prompt)?;

    // Clean output a bit
    let explanation = explanation.trim_start_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace());

    println!("\n🧠 Explanation:\n{}", explanation);

    Ok(())
}
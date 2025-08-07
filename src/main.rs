use cli_llm_agent::llm;
// use cli_llm_agent::llm::infer_local;

use clap::{Parser, Subcommand};
use std::fs;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "llm")]
#[command(about = "LLM CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    Chat,
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
async fn main() -> Result<()> {
    // let model_path = Path::new("models/ggml-model-q4_0.gguf");
    let model_path = Path::new("models/llama-2-7b-chat.Q4_0.gguf");
    println!("🔍 Loading model from {:?}", model_path);

    let model = LlamaModel::load_from_file(model_path, LlamaParams::default())?;

    let cli = Cli::parse();
    let session_params = SessionParams {
                    n_threads: 8,
                    n_ctx: 2048,
                    ..Default::default()
                };
    match cli.command {
        Commands::Chat => {
            let stdin = io::stdin();

            let mut session = model.create_session(session_params)?;
            // let mut session = model.create_session(SessionParams::default())?;

            loop {
                print!("\n> ");
                io::stdout().flush()?;
                let mut input = String::new();
                stdin.read_line(&mut input)?;
                if input.trim().eq_ignore_ascii_case("exit") {
                    break;
                }

                run_chat_prompt(&mut session, &input)?;
            }
        }

        Commands::Explain { t, f } => {
            // let mut session = model.create_session(SessionParams::default())?;
            let mut session = model.create_session(session_params)?;

            if let Some(text) = t {
                run_explain_prompt(&mut session, &text)?;
            } else if let Some(file_path) = f {
                let text = fs::read_to_string(file_path)?;
                run_explain_prompt(&mut session, &text)?;
            } else {
                eprintln!("❌ Please provide either --t <text> or --f <file>");
            }
        }
    }

    Ok(())
}

fn run_prompt(session: &mut llama_cpp::LlamaSession, input: &str) -> Result<()> {
    let prompt = format!(
        "### Question:\n{}\n\n### Answer:",
        // "Explain this SQL query clearly:\n\n{}\n\nExplanation:",
        input.trim()
    );

    let explanation = llm::infer_local(session, &prompt)?;

    let explanation = explanation
        .trim_start_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace());

    let clean = explanation
        .lines()
        .filter(|line| !line.contains("blockquote") && !line.contains("Comment:"))
        .collect::<Vec<_>>()
        .join("\n");
    println!("\n🧠 Explanation:\n{}", explanation);
    Ok(())
}

fn run_chat_prompt(session: &mut llama_cpp::LlamaSession, user_input: &str) -> Result<()> {
    let prompt = format!(
        "[INST] You are a helpful assistant. Answer the user politely. [/INST]\nUser: {}\nAssistant:",
        user_input.trim()
    );

    let response = llm::infer_local(session, &prompt)?;
    println!("\n🤖 {}\n", response.trim());
    Ok(())
}

fn run_explain_prompt(session: &mut llama_cpp::LlamaSession, sql: &str) -> Result<()> {
    let prompt = format!(
        "[INST] Explain this SQL query in simple English: [/INST]\n```sql\n{}\n```",
        sql.trim()
    );

    let response = llm::infer_local(session, &prompt)?;
    println!("\n🧠 {}\n", response.trim());
    Ok(())
}

fn run_inference(session: &mut llama_cpp::LlamaSession, prompt: &str) {
    println!("🔍 Running inference with prompt:\n{}\n", prompt);

    let result = llm::infer_local(session, &prompt);  // Your existing model call here
    println!("📤 Result:\n{:?}\n", result);
}
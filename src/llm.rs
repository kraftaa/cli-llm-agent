
// wget -O models/ggml-gpt4all-j-v1.3-groovy.bin https://huggingface.co/nomic-ai/gpt4all-j/resolve/main/ggml-gpt4all-j-v1.3-groovy.bin
// wget -O models/ggml-alpaca-7b-q4.bin https://huggingface.co/tloen/alpaca-lora-7b/resolve/main/ggml-alpaca-7b-q4.bin
// mkdir -p models
// wget -O models/ggml-model-q4_0.gguf \
// https://huggingface.co/TheBloke/LLaMA-2-7B-GGUF/resolve/main/llama-2-7b.Q4_0.gguf
use llama_cpp::{LlamaModel, LlamaParams, SessionParams};
use llama_cpp::standard_sampler::StandardSampler;
use anyhow::Result;
use std::path::Path;
use std::io::{self, Write};

pub fn infer_local(prompt: &str) -> Result<String> {
    let model_path = Path::new("models/ggml-model-q4_0.gguf");
    println!("Loading model from {:?}", model_path);

    let model = LlamaModel::load_from_file(model_path, LlamaParams::default())
        .expect("Failed to load model");

    println!("Creating session...");
    let mut session = model.create_session(SessionParams::default())
        .expect("Failed to create session");

    println!("Advancing context with prompt...");
    session.advance_context(prompt)?;

    println!("Starting completion...");
    let mut completions = session
        .start_completing_with(StandardSampler::default(), 200)
        .unwrap()
        .into_strings();

    let mut output = String::new();

    for token in completions {
        output.push_str(&token);
        print!("{token}");
        io::stdout().flush()?;
    }

    println!("\nGeneration complete.");

    Ok(output)
}

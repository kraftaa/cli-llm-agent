
// wget -O models/ggml-gpt4all-j-v1.3-groovy.bin https://huggingface.co/nomic-ai/gpt4all-j/resolve/main/ggml-gpt4all-j-v1.3-groovy.bin
// wget -O models/ggml-alpaca-7b-q4.bin https://huggingface.co/tloen/alpaca-lora-7b/resolve/main/ggml-alpaca-7b-q4.bin
// mkdir -p models
// wget -O models/ggml-model-q4_0.gguf \
// https://huggingface.co/TheBloke/LLaMA-2-7B-GGUF/resolve/main/llama-2-7b.Q4_0.gguf
// https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGUF/resolve/main/llama-2-7b-chat.gguf
// models/llama-2-7b-chat.gguf (Q4_K_M)
//Llama-2-7B-Chat-GGUF
// llama-2-7b-chat.Q4_0.gguf
//mistral-7b-instruct.Q4_0.gguf
// mistral-7b-instruct.Q4_0.gguf
//
// zephyr-7b-alpha.Q4_0.gguf
//
// phi-2-instruct.Q4_0.gguf

use llama_cpp::standard_sampler::StandardSampler;
use anyhow::Result;
use std::path::Path;
use std::io::{self, Write};

pub struct InferenceConfig {
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_k: i32,
    pub top_p: f32,
    pub stream: bool,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            max_tokens: 200,
            temperature: 0.7,
            top_k: 40,
            top_p: 0.9,
            stream: true,
        }
    }
}

pub fn infer_local(
    session: &mut llama_cpp::LlamaSession,
    prompt: &str,
    // cfg: &InferenceConfig,
) -> Result<String> {
    println!("Advancing context with prompt...");
    session.advance_context(prompt)?;

    println!("Starting completion...");

    let sampler = StandardSampler::default();
        // .with_temp(cfg.temperature)
        // .with_top_k(cfg.top_k)
        // .with_top_p(cfg.top_p);

    let mut completions = session
        // .start_completing_with(sampler, cfg.max_tokens)
        .start_completing_with(sampler, 200)
        .unwrap()
        .into_strings();

    let mut output = String::new();

    for token in completions {
        output.push_str(&token);

        // if cfg.stream {
            print!("{token}");
            io::stdout().flush()?;
        // }
    }

    // if cfg.stream {
    //     println!("\nGeneration complete.");
    // }

    Ok(output)
}

// pub fn infer_local(session: &mut llama_cpp::LlamaSession, prompt: &str) -> Result<String> {
//
//     // session.advance_context(prompt)?;
//     // session.reset();
//     println!("Advancing context with prompt...");
//     session.advance_context(prompt)?;
//
//     println!("Starting completion...");
//     let mut completions = session
//         .start_completing_with(StandardSampler::default(), 200)
//         .unwrap()
//         .into_strings();
//
//     let mut output = String::new();
//
//     for token in completions {
//         output.push_str(&token);
//         print!("{token}");
//         io::stdout().flush()?;
//     }
//
//     println!("\nGeneration complete.");
//
//     Ok(output)
// }
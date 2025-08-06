Download into models/

mkdir -p models
wget -O models/ggml-model-q4_0.gguf \
https://huggingface.co/TheBloke/LLaMA-2-7B-GGUF/resolve/main/llama-2-7b.Q4_0.gguf


How to use:
Run in command line: 
```
cargo run -- explain -t "SELECT * FROM orders WHERE amount > 1000"
```

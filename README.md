Download into models/

```shell
mkdir -p models
wget -O models/ggml-model-q4_0.gguf \
https://huggingface.co/TheBloke/LLaMA-2-7B-GGUF/resolve/main/llama-2-7b.Q4_0.gguf
```
For chat better to load chat model:
```shell
wget -O models/llama-2-7b-chat.gguf \
https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGUF/resolve/main/llama-2-7b-chat.gguf
```

How to use:
Run in command line for `main-cli-command.rs`: 
```
cargo run --bin main-cli-command -- explain -t "SELECT * FROM orders WHERE amount > 1000 and id in (1,2)"\n

```


if using chat model in `main.rs`
```
 cargo run --bin cli-llm-agent -- chat
```

1. single question
```shell
cargo run --bin cli-llm-agent -- explain -t "SELECT * FROM orders WHERE amount > 1000 and id in (1,2)"\n
```
2. from the file
```shell
cargo run --bin cli-llm-agent  -- explain -f example.sql
```

3. as a chat
```shell
 cargo run --bin cli-llm-agent -- chat
```

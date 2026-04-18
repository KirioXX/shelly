# 🐚 Shelly

A Rust-based terminal assistant that translates natural language prompts into shell commands and injects them directly into your shell's editing buffer.

## 🚀 Features

- **Natural Language to Command**: Describe what you want to do, and Shelly gives you the exact command.
- **Direct Buffer Injection**: Instead of just printing a command, Shelly can inject it directly into your shell's prompt (via `print -z` in Zsh, `bind` in Bash, or `commandline -r` in Fish), so you can review and edit it before hitting Enter.
- **Zero-Leak Output**: Uses a strict `stdout`/`stderr` split. AI reasoning and spinners go to `stderr`, while only the final executable command goes to `stdout`.
- **Interactive Setup**: A guided wizard to configure your AI provider and shell integration.

## 🛠️ Installation

### 1. Build the binary
```bash
cargo build --release
mv target/release/shelly /usr/local/bin/shelly
```

### 2. Run the setup wizard
```bash
shelly setup
```
The wizard will collect your API key, preferred model, and set up the necessary shell wrapper in your `.zshrc`, `.bashrc`, or `config.fish`.

### 3. Restart your shell
```bash
source ~/.zshrc  # for Zsh
source ~/.bashrc  # for Bash
source ~/.config/fish/config.fish  # for Fish
```

## 📖 Usage

Simply describe the command you need:

```bash
shelly "list all docker containers running on port 8080"
shelly "find all files larger than 100MB in the current directory"
shelly "git commit all changes with the message 'fix: resolve bug #123'"
```

## ⚙️ Architecture

Shelly leverages an async Rust core powered by:
- **`tokio`**: For asynchronous AI requests.
- **`async-openai`**: For OpenAI-compatible API communication.
- **`clap`**: For CLI argument parsing.
- **`indicatif`**: For real-time progress spinners.
- **`confy`**: For cross-platform configuration management.

## 📜 License
MIT

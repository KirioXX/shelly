# 🐚 Shelly

A Rust-based terminal assistant that translates natural language prompts into shell commands and injects them directly into your shell's editing buffer.

## 🚀 Features

- **Natural Language to Command**: Describe what you want to do, and Shelly gives you the exact command.
- **Smart Skill Matching**: Automatically detects when to use specialized skills (like generating cURL commands) and applies expert guidance.
- **Direct Buffer Injection**: Instead of just printing a command, Shelly can inject it directly into your shell's prompt (via `print -z` in Zsh, `bind` in Bash, or `commandline -r` in Fish), so you can review and edit it before hitting Enter.
- **Context-Aware**: Knows your OS and shell type to generate appropriate commands.
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

### Dry-Run Mode

Preview commands before executing them:

```bash
shelly --dry-run "delete all log files older than 30 days"
```

This shows the generated command without injecting it into your shell, allowing you to verify it's safe before running.

### Skills

Shelly automatically matches your prompt to specialized skills in `~/.config/shelly/skills/`:

```bash
# Automatically uses curl-command-generator skill
shelly "generate curl commands for my API endpoints"
```

When a skill is activated, you'll see `📚 Using skill: <skill-name>`.

#### Installing Skills

Skills are Markdown files with frontmatter:

```markdown
---
name: my-skill
description: Use when users want X, Y, or Z
---

# Instructions for the AI...
```

Install by copying to shelly's skills directory:

```bash
mkdir -p ~/.config/shelly/skills/my-skill
cp path/to/SKILL.md ~/.config/shelly/skills/my-skill/
```

## ⚙️ Architecture

Shelly leverages an async Rust core powered by:
- **`tokio`**: For asynchronous AI requests.
- **`async-openai`**: For OpenAI-compatible API communication.
- **`clap`**: For CLI argument parsing.
- **`indicatif`**: For real-time progress spinners.
- **`handlebars`**: For templating system prompts with OS/shell context.
- **`console`**: For styled terminal output.
- **`confy`**: For cross-platform configuration management.

## 📜 License
MIT

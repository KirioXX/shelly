# 🐚 Shelly

A Rust-based terminal assistant that translates natural language prompts into shell commands and injects them directly into your shell's editing buffer.

## 🚀 Features

- **Natural Language to Command**: Describe what you want, get the exact command
- **Direct Buffer Injection**: Commands appear in your shell for review before running
- **AI Tools**: Web search, file reading, automatic clarification for better context
- **Smart Skill Matching**: Expert guidance for specific tasks, multiple skills can be active at once
- **Shell Completions**: Tab completion for Bash, Zsh, Fish
- **Zero-Leak Output**: Only the command goes to stdout

## Quick Start

### Installation

```bash
# Clone and build
cargo build --release
just install  # Or: cp target/release/shelly ~/.cargo/bin/
```

### Setup

```bash
shelly setup
```

The wizard will:

1. **Configure your AI provider** — Model, API URL, and API key saved to `~/.config/shelly/config.toml`
2. **Install the shell wrapper** — Detects Bash/Zsh/Fish and appends a function to your shell config that captures Shelly's output and injects it directly into your terminal buffer

> ⚠️ The shell wrapper is essential for the "commands appear in your shell for review" experience. Without it, you'll need to copy-paste output manually.

### Tweak settings later

```bash
shelly config        # View or edit config interactively
shelly config --show # Display only, no prompts
```

### Usage

```bash
shelly generate "list all docker containers"
shelly generate --dry-run "delete old log files"
shelly generate --skills safe-bash,backup "archive important files"
```

## Development

```bash
just          # See all available commands
just build    # Build release binary
just test     # Run tests
```

## Documentation

All documentation lives in the Obsidian vault under `01 - Projects/Shelly/`:

- [[01 - Projects/Shelly/Usage Guide|📖 Usage Guide]] — Complete usage documentation
- [[01 - Projects/Shelly/0. Index|🏠 Project Hub]] — Quick facts, tech stack, entry points
- [[01 - Projects/Shelly/Kanban|📋 Kanban Board]] — Current status, backlog, done
- [[01 - Projects/Shelly/Architecture|🏗️ Architecture]] — Mermaid diagrams, module breakdown
- [[01 - Projects/Shelly/Changelog|📜 Changelog]] — Feature timeline with commits
- [[01 - Projects/Shelly/Skills Guide|🎯 Skills]] — Skills system documentation

## Architecture

Shelly is built with:

- **Rust** + **Tokio** - Async runtime
- **Clap** - CLI parsing with completions
- **Async-openai** - AI API communication
- **Dialoguer** - Interactive prompts

## License

MIT

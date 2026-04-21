# 🐚 Shelly

A Rust-based terminal assistant that translates natural language prompts into shell commands and injects them directly into your shell's editing buffer.

## 🚀 Features

- **Natural Language to Command**: Describe what you want, get the exact command
- **Direct Buffer Injection**: Commands appear in your shell for review before running
- **AI Tools**: Web search, file reading, automatic clarification for better context
- **Smart Skill Matching**: Expert guidance for specific tasks (curl, docker, etc.)
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
# Follow the wizard to configure your AI provider and shell
```

### Usage

```bash
shelly generate "list all docker containers"
shelly generate --dry-run "delete old log files"
```

## Development

```bash
just          # See all available commands
just build    # Build release binary  
just test     # Run tests
```

## Documentation

- [📖 Usage Guide](docs/usage.md) - Complete usage documentation
- [🎯 Skills](docs/skills.md) - Skills system documentation  
- [🔧 AI Tools](docs/usage.md#ai-tools) - Web search, file reading
- [❓ Interactive Clarification](docs/usage.md#interactive-clarification) - When AI needs help
- [⌨️ Shell Completions](docs/usage.md#shell-completions) - Tab completion setup

## Architecture

Shelly is built with:
- **Rust** + **Tokio** - Async runtime
- **Clap** - CLI parsing with completions
- **Async-openai** - AI API communication
- **Dialoguer** - Interactive prompts

## License

MIT

# 🐚 shelly

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

#### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/KirioXX/shelly/main/scripts/install.sh | sh
```

Or specify a custom install directory:

```bash
curl -fsSL https://raw.githubusercontent.com/KirioXX/shelly/main/scripts/install.sh | INSTALL_DIR=/usr/local/bin sh
```

The script downloads the latest release for your OS, extracts it, and places the
binary in `~/.local/bin` (or your chosen directory).

#### Windows (PowerShell)

```powershell
iwr -useb https://raw.githubusercontent.com/KirioXX/shelly/main/scripts/install.ps1 | iex
```

Or with a custom install directory:

```powershell
$env:INSTALL_DIR = "C:\Tools"; iwr -useb https://raw.githubusercontent.com/KirioXX/shelly/main/scripts/install.ps1 | iex
```

#### Manual build

If you prefer to build from source:

```bash
# Clone then run
just install

# or completly manual
cargo build --release
cp target/release/shelly ~/.local/bin/
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

### Self-update

Once installed via a release binary, you can update to the latest version with:

```bash
shelly update
```

This fetches the latest GitHub release, downloads the matching artifact for your
platform, and replaces the current binary after confirmation.

### Usage

```bash
# Without shell wrapper
shelly generate "list all docker containers"
shelly generate --dry-run "delete old log files"
shelly generate --skills safe-bash,backup "archive important files"

# With shell wrapper
shelly "list all docker containers"
shelly --dry-run "delete old log files"
shelly --skills safe-bash,backup "archive important files"
```

## Development

```bash
just          # See all available commands
just build    # Build release binary
just test     # Run tests
```

## Documentation

- [📖 Usage Guide](docs/USAGE.md) — Complete usage documentation
- [🏗️ Architecture](docs/ARCHITECTURE.md) — Mermaid diagrams, module breakdown
- [📜 Changelog](docs/CHANGELOG.md) — Feature timeline with commits
- [🎯 Skills Guide](docs/SKILLS.md) — Skills system documentation
- [🗂️ Plans](docs/plans/) — Archived implementation plans
- [🤖 AGENT.md](AGENT.md) — Guide for coding agents working on this project

## Architecture

Shelly is built with:

- **Rust** + **Tokio** - Async runtime
- **Clap** - CLI parsing with completions
- **Async-openai** - AI API communication
- **Dialoguer** - Interactive prompts

## License

MIT

# Shelly Usage Guide

Complete guide to using shelly effectively.

## Table of Contents
- [Basic Usage](#basic-usage)
- [Dry-Run Mode](#dry-run-mode)
- [List Available Commands](#list-available-commands)
- [Shell Completions](#shell-completions)
- [AI Tools](#ai-tools)
- [Interactive Clarification](#interactive-clarification)
- [Development Commands](#development-commands)

---

## Basic Usage

Simply describe the command you need:

```bash
shelly generate "list all docker containers running on port 8080"
shelly generate "find all files larger than 100MB in the current directory"
shelly generate "git commit all changes with the message 'fix: resolve bug #123'"
```

Shelly will generate the command and inject it into your shell prompt for review before execution.

---

## Dry-Run Mode

Preview commands before executing them:

```bash
shelly generate --dry-run "delete all log files older than 30 days"
```

This shows the generated command without injecting it into your shell, allowing you to verify it's safe before running.

---

## List Available Commands

See all available subcommands:

```bash
shelly cmds
```

This outputs:
- `setup` - Run the setup wizard
- `generate` - Generate shell commands from natural language
- `cmds` - List all available commands
- `completions` - Generate shell completion scripts

---

## Shell Completions

Generate shell completion scripts for tab completion support:

```bash
# Bash
shelly completions bash > ~/.bash_completion.d/shelly

# Zsh
shelly completions zsh > ~/.zsh/completions/_shelly

# Fish
shelly completions fish > ~/.config/fish/completions/shelly.fish
```

Then reload your shell configuration:

```bash
source ~/.bashrc  # for Bash
source ~/.zshrc   # for Zsh
```

Fish completions are loaded automatically.

---

## AI Tools

Shelly can automatically use tools to gather information when generating commands:

- **web_search** - Search the web for current information (versions, events, facts)
- **read_file** - Read file contents to understand project context

The AI decides when to use tools based on your prompt. You don't need to do anything special - just ask naturally!

Examples that trigger tools:
```bash
# Triggers web search for latest version
shelly "what's the latest version of Go"

# Triggers read_file to see your config
shelly "show me my Cargo.toml dependencies"

# May trigger web search for current best practices
shelly "how do I set up a Next.js project with the latest version"
```

Tools are executed transparently and their results are incorporated into the generated command.

---

## Interactive Clarification

When your request is ambiguous, shelly can ask for clarification using a 
selectable list interface:

```bash
$ shelly "delete logs"
🤔 The AI needs clarification:
Which logs would you like to delete?
Navigate with ↑↓ and press Enter to select:

> System logs older than 7 days
  Application logs in current project  
  All logs everywhere (dangerous!)
  Cancel / Don't delete anything

Your choice: Application logs in current project

✓ Command generated: rm -f logs/*.log
```

This uses arrow-key navigation and prevents destructive mistakes.

---

## Development Commands

We use [`just`](https://github.com/casey/just) for common development tasks:

```bash
# See all available commands
just

# Build release binary
just build

# Run tests
just test

# Run linting
just lint

# Build and install to cargo bin directory
just install
```

Available recipes:

| Command | Description |
|---------|-------------|
| `just build` | Build release binary |
| `just install` | Build and install to cargo's bin directory (`$CARGO_HOME/bin`) |
| `just check` | Quick cargo check |
| `just lint` | Run clippy |
| `just test` | Run tests |
| `just run -- <args>` | Dev build and run with arguments |

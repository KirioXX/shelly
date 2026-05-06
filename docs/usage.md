# Shelly Usage Guide

Complete guide to using shelly effectively.

## Table of Contents

- [Setup](#setup)
- [Basic Usage](#basic-usage)
- [Dry-Run Mode](#dry-run-mode)
- [List Available Commands](#list-available-commands)
- [Shell Completions](#shell-completions)
- [AI Tools](#ai-tools)
- [Interactive Clarification](#interactive-clarification)
- [Development Commands](#development-commands)

---

## Setup

Before using Shelly, run the setup wizard:

```bash
shelly setup
```

This does two things:

### 1. Configure your AI provider

The wizard asks for:

- **Model** (e.g. `gpt-4o`, `claude-3-5-sonnet`, or a local Ollama model)
- **API Base URL** (leave blank for OpenAI)
- **API Key**

These are saved to `~/.config/shelly/config.toml`.

### 2. Install the shell wrapper (essential!)

This is the magic that makes Shelly feel native. The wizard detects your shell and appends a wrapper function to your shell config file. This function calls `shelly generate`, captures its **stdout** (the raw command), and pastes it into your shell buffer for review before you hit Enter.

Supported shells: **Bash**, **Zsh**, **Fish**

**How it works:**

```
┌──────────┐      ┌──────────────┐      ┌──────────────┐
│  User    │─────▶│ shelly alias │─────▶│  AI API      │
│  types   │      │ (wrapper fn) │      │  reasoning   │
└──────────┘      └──────────────┘      └──────────────┘
                              │
                              ▼                    stdout
                     ┌─────────────┐           ┌────────────┐
                     │ paste into  │◀──────────│   command  │
                     │ shell buffer│           │   string   │
                     └─────────────┘           └────────────┘
```

### Manual setup (if you skipped the wizard)

If you already have a config file but never installed the shell wrapper, add this to your `.bashrc` or `.zshrc`:

**Bash** (appends to `.bashrc`):

```bash
shelly() {
    local cmd
    cmd=$(command shelly "$@")
    READLINE_LINE="${cmd}"
    READLINE_POINT=${#cmd}
}
```

**Zsh** (appends to `.zshrc`):

```bash
shelly() {
    local cmd
    cmd=$(command shelly "$@")
    BUFFER="${cmd}"
    CURSOR=${#cmd}
}
```

After editing, reload your shell:

```bash
source ~/.bashrc   # or ~/.zshrc
```

> ⚠️ **Without the wrapper, `shelly generate` still works, but you'll have to copy-paste the output manually.** The wrapper enables the "zero-click" buffer injection experience.

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

## Skills

Shelly supports extensible skills to enhance command generation for specific domains.

### View Installed Skills

```bash
shelly skills list
```

### Install Skills

Install skills from GitHub repositories:

```bash
# Full URL
shelly skills add https://github.com/username/my-skill

# Shorthand
shelly skills add username/my-skill

# Install specific skill from multi-skill repo
shelly skills add username/my-skills-repo --skill my-specific-skill
```

Skills are downloaded, extracted, and validated (requires `SKILL.md` file)
to `~/.config/shelly/skills/`.

### Using Skills

Skills are automatically loaded when your prompt matches their description:

```bash
# Automatically uses matching skills
shelly generate "create curl command for POST request"
```

Manually specify skills with `--skills`:

```bash
# Use a single skill
shelly generate --skills safe-bash "delete old files"

# Use multiple skills (comma-separated)
shelly generate --skills safe-bash,backup "archive and compress logs"
```

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

| Command              | Description                                                    |
| -------------------- | -------------------------------------------------------------- |
| `just build`         | Build release binary                                           |
| `just install`       | Build and install to cargo's bin directory (`$CARGO_HOME/bin`) |
| `just check`         | Quick cargo check                                              |
| `just lint`          | Run clippy                                                     |
| `just test`          | Run tests                                                      |
| `just run -- <args>` | Dev build and run with arguments                               |

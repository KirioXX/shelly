---
project: shelly
status: On Going
---

# 🐚 Shelly

**A Rust-based terminal assistant that translates natural language prompts into shell commands and injects them directly into your shell's editing buffer.**

## Quick Facts

| | |
|---|---|
| **Repository** | `~/Documents/Sandbox/shelly` |
| **Language** | Rust (Edition 2024) |
| **Version** | 0.1.0 |
| **Status** | MVP complete; iterating on skills & UX |
| **Tests** | 25+ passing |
| **Last activity** | 2025-05-03 |

## What It Does

Describe what you want in plain English, and Shelly generates the exact shell command — then injects it into your terminal buffer for review before execution.

```bash
shelly generate "list all docker containers running on port 8080"
```

## Tech Stack

- **Rust** + **Tokio** — Async runtime
- **Clap** — CLI parsing with derive macros & completions
- **async-openai** — OpenAI-compatible API integration
- **Dialoguer** — Interactive setup wizard & clarification prompts
- **Indicatif** — Terminal spinners
- **Handlebars** — System prompt templating
- **Serde** — Config & structured output serialization

## Quick Links

- [🏗️ Architecture](ARCHITECTURE.md) — How it works under the hood
- [📜 Changelog](CHANGELOG.md) — Feature timeline

## Entry Points

| Command | Purpose |
|---|---|
| `shelly setup` | Interactive onboarding + shell integration |
| `shelly generate "<prompt>"` | Generate a command from natural language |
| `shelly generate --dry-run "<prompt>"` | Preview without injecting |
| `shelly skills list` | See installed skills |
| `shelly skills add <url>` | Install a skill from GitHub |
| `shelly completions <shell>` | Generate shell completion scripts |

## Documentation in Repo

- `README.md` — Quick start & overview
- [`docs/USAGE.md`](USAGE.md) — Full usage guide
- [`docs/SKILLS.md`](SKILLS.md) — Skills system docs
- [`docs/ARCHITECTURE.md`](ARCHITECTURE.md) — Architecture & internals
- [`docs/CHANGELOG.md`](CHANGELOG.md) — Feature timeline
- `AGENT.md` — Guide for coding agents working on this project

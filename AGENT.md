# 🤖 Agent Guide — shelly

> This file is for **AI coding assistants** working on the shelly project.
> It supplements the user-facing docs (`README.md`, `docs/usage.md`, `docs/skills.md`).

---

## 📚 Where to Find Information

### In the Obsidian Vault (single source of truth)

All documentation, planning, and project knowledge lives in `01 - Projects/Shelly/`:

| Note              | What's in it                                              | Why you'd read it         |
| ----------------- | --------------------------------------------------------- | ------------------------- |
| `0. Index.md`     | Project hub, quick facts, tech stack, entry points        | Get oriented              |
| `Kanban.md`       | **Board** — backlog, in-progress, done, archive           | Know what's active        |
| `Architecture.md` | Mermaid diagrams, module breakdown, data flow             | Understand internals      |
| `Changelog.md`    | Feature timeline with commit hashes                       | Trace history             |
| `Usage Guide.md`  | User-facing CLI guide (setup, usage, skills, completions) | How the tool works        |
| `Skills Guide.md` | Skills system — install, format, matching, creation       | Skill system context      |
| `Plan - *.md`     | Implementation plans for new features                     | Before building a feature |
| `Plans/`          | Archived implementation plans (historical)                | Past decisions            |

### In the Repo (code only)

| File        | What's in it                                    |
| ----------- | ----------------------------------------------- |
| `README.md` | Elevator pitch, quick start, link to vault docs |
| `AGENT.md`  | This file — agent onboarding                    |
| `src/`      | Source code                                     |
| `tests/`    | Test files                                      |

**Rule:** Before writing code, read the relevant plan in the vault. After writing code, update the **Kanban** and **Changelog**.

---

## 📋 Planning & Progress

**All active planning lives in the Obsidian vault.**

| Note                               | Purpose                                       |
| ---------------------------------- | --------------------------------------------- |
| `01 - Projects/Shelly/Kanban.md`   | **Kanban board** — backlog, in-progress, done |
| `01 - Projects/Shelly/Plan - *.md` | Implementation plans for new features         |

**Rule:** Before starting work, check the Kanban. After completing work, move the task to ✅ Done.

---

## 🏗️ Architecture at a Glance

- **Language:** Rust (Edition 2024), async via Tokio
- **CLI:** Clap with derive macros + completion generation
- **AI:** OpenAI-compatible API via `async-openai`, structured JSON output
- **Config:** `confy` → `~/.config/shelly/config.toml`
- **Skills:** Markdown files in `~/.config/shelly/skills/`, progressive disclosure (metadata only in prompt)
- **Tools:** `web_search`, `read_file`, `ask_user` (enabled since `56a28fe`)

### Core Principle: Zero-Leak stdout

Only the final `command` string reaches stdout. Everything else (spinners, reasoning, warnings, errors) goes to stderr. The shell wrapper captures stdout and injects it into the editing buffer.

---

## 📁 Module Map

| Path                                       | Role                                            |
| ------------------------------------------ | ----------------------------------------------- |
| `src/main.rs`                              | CLI entry point, subcommand dispatch            |
| `src/lib.rs`                               | Module re-exports, constants                    |
| `src/config.rs`                            | `Config` struct + `Shell` enum                  |
| `src/commands/mod.rs`                      | `Commands` enum (clap subcommands)              |
| `src/commands/ai/mod.rs`                   | AI core — tool loop, JSON parsing, client setup |
| `src/commands/ai/system_prompt.rs`         | `build_system_prompt()` + skill matching        |
| `src/commands/ai/prompts/system-prompt.md` | Handlebars template for LLM system prompt       |
| `src/commands/setup/`                      | Interactive setup wizard                        |
| `src/commands/skills.rs`                   | `skills list`, `skills add` implementation      |
| `src/tools/mod.rs`                         | `Tool` trait + `ToolRegistry`                   |
| `src/tools/web_search.rs`                  | DuckDuckGo search tool                          |
| `src/tools/read_file.rs`                   | File read tool (security scoped)                |
| `src/tools/ask_user.rs`                    | Interactive clarification tool                  |
| `src/skills/mod.rs`                        | `Skill` struct, parsing, discovery, matching    |
| `src/skills/installer.rs`                  | GitHub download + validation                    |
| `tests/cli_smoke_test.rs`                  | CLI smoke tests                                 |
| `tests/mock_openai_tests.rs`               | Mock API tests                                  |

---

## ✅ Before & After Checklist

### Before starting a task

1. Read the **Kanban** (`01 - Projects/shelly/Kanban.md`) for current priorities
2. Read `Usage Guide.md` and `Skills Guide.md` in Obsidian for user-facing context
3. Read `Architecture.md` in Obsidian if the change touches core flows
4. Run `just test` to establish a green baseline

### When implementing

1. Follow existing patterns in the module you're editing
2. Add/update tests in the same module or `tests/` folder
3. Run `just check` and `just lint` frequently

### Before claiming done

1. `just test` — all tests must pass
2. `just lint` — clippy clean
3. Update the **Kanban** — move task to ✅ Done and mark `- [x]`
4. Update `Changelog.md` if it's a notable feature
5. Update relevant docs in Obsidian (Usage Guide, Skills Guide, Plans) if the change affects user-facing behavior

---

## 🧪 Common Commands

```bash
just           # List available recipes
just build     # Release build
just test      # Run full test suite
just lint      # Run clippy
just install   # Build & install to cargo bin
```

---

## 🔒 Security Boundaries

- `read_file` is restricted to the project directory and home directory
- `ask_user` always includes a "Cancel / abort" option
- Destructive commands trigger a `warning` field in the JSON response
- Skills must contain a `SKILL.md` file to be considered valid

---

_Generated: 2026-05-06_

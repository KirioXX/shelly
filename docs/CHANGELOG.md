---
title: "Shelly Changelog"
project: shelly
tags: [changelog, history, timeline]
---

# 📜 Shelly Changelog

## 2026-06

### 2026-06-15
- **CI** — Artifact naming: build artifacts include commit SHA, release artifacts include version tag (`9c0ce71`)
- **CI** — Fix artifact upload path on Unix: `tar -C target/release` so archives are created in the repo root (`22a3cd6`)
- **CI** — Add release workflow with binary artifacts: `build.yml` uploads artifacts, `release.yml` triggered by `v*` tags creates draft releases (`2e51244`)
- **Feature** — Install scripts for easy one-liner setup:
  - `scripts/install.sh` for macOS/Linux (`curl ... | sh`)
  - `scripts/install.ps1` for Windows (`iwr ... | iex`)
  - Default install dir: `~/.local/bin` / `%USERPROFILE%\.local\bin` (`9146166`)
- **Feature** — Self-update command: `shelly update` fetches latest release, confirms, downloads matching artifact, extracts, and replaces the running binary with rollback on failure (`e82efcb`)
- **Build** — Add `build.rs` to embed short Git SHA at compile time; add `json` feature to `reqwest`
- **Docs** — README: one-liner install instructions for both platforms; `shelly update` documentation

## 2026-05

### 2026-05-12
- **Docs** — Move all documentation to Obsidian vault as single source of truth: `docs/usage.md`, `docs/skills.md`, archived plans → vault; update `README.md` and `AGENT.md` to point to vault (`042502e`)

### 2026-05-06
- **Feature** — Add `shelly config` command: view/edit config without re-running setup wizard (`7b26269`)
- **Docs** — Document `shelly config` in `usage.md` and `README.md` (`021df69`)
- **Fix** — Re-enable `ask_user` tool and fix TTY conflict: `dialoguer::Select` now works by clearing the `indicatif` spinner before tool execution (`56a28fe`)
- **Fix** — Gracefully handle markdown-fenced JSON responses from LLM (`f3d1f7e`)
- **Docs** — Add detailed `setup` documentation explaining the shell wrapper function (`docs/usage.md`, `README.md`)
- **Docs** — Remove `TODO.md` in favor of Obsidian Kanban
- **Project** — Add `AGENT.md` for coding agent guidance

## 2025-05

### 2025-05-03
- **Chore** — Add project-level Ollama models config with `kimi-k2.6:cloud` (`19842a0`)

## 2025-04

### 2025-04-24
- **Feature** — Add structured output support: AI now returns JSON with `command` + optional `warning` fields (`2ca7651`)
- **Feature** — Add `-d` abbreviation alias for `--dry-run` (`748c438`)
- **Docs** — Mark progressive skill disclosure as complete in TODO (`591f198`)
- **Test** — Verify progressive disclosure passes full suite (25 tests) + e2e (`cbbdef5`)
- **Refactor** — Progressive skill disclosure with `build_system_prompt` + tests (`ed25bbb`)
- **Feature** — Add `path` field to `Skill` struct for progressive disclosure (`fb4b33d`)
- **Docs** — Progressive skill loading implementation plan saved (`e6ebbac`)

### 2025-04-21 — 2025-04-22
- **Feature** — Enable multiple skills in one session (`cac7381`)
- **Feature** — Match skills.sh format for adding (`293f6e4`)
- **Refactor** — Package setup in own module folder (`d24ff0e`)
- **Refactor** — Move command enums to modules (`cab630e`)
- **Feature** — Add `skills add` subcommand (`0e3107b`)
- **Chore** — Ignore `.pi-lens` folder (`9886bd8`)
- **Fix** — Better error debugging for API failures (`99d2f13`)
- **Temp** — Disable `AskUser` tool (requires API with OpenAI tool support) (`0561d76`)
- **Docs** — Reorganize README; create `usage.md` + `skills.md` (`5fa213b`)
- **Docs** — Mark documentation reorganization as completed (`e9a04cc`)
- **Docs** — Mark `ask_user` tool plan as completed (`e16908b`)
- **Docs** — Document `ask_user` Select interface (`896e4b7`)
- **Test** — Verify `ask_user` tool exists and compiles (`a9467cb`)
- **Feature** — Add `ask_user` tool with `dialoguer::Select` (`a244480`)
- **Docs** — Add `ask_user` guidance to system prompt (`5cae91a`)
- **Test** — Complete test suite (17 tests, all passing) (`71453f4`)
- **Docs** — Mark test suite as completed in TODO (`744c80c`)

### Earlier
- **Feature** — AI Tool Calling: `web_search` + `read_file` tools
- **Feature** — Shell completions (Bash/Zsh/Fish)
- **Feature** — Dry-run mode
- **Feature** — Setup wizard with shell integration
- **Feature** — Core `shelly generate` with stdout/stderr split

---

## Design Documents

All implementation plans are archived in `docs/plans/`:

| Plan | Status | File |
|---|---|---|
| Add Tests | ✅ Complete | [`plans/2025-04-21-add-tests.md`](plans/2025-04-21-add-tests.md) |
| AI Tool Calling | ✅ Complete | [`plans/2025-04-21-ai-tool-calling.md`](plans/2025-04-21-ai-tool-calling.md) |
| Migrate AI to Subcommand | ✅ Complete | [`plans/2025-04-21-migrate-ai-to-subcommand.md`](plans/2025-04-21-migrate-ai-to-subcommand.md) |
| Shell Completions | ✅ Complete | [`plans/2025-04-21-shell-completions.md`](plans/2025-04-21-shell-completions.md) |
| Ask User Tool | ✅ Complete (disabled) | [`plans/2025-04-22-ask-user-tool.md`](plans/2025-04-22-ask-user-tool.md) |
| Reorganize Docs | ✅ Complete | [`plans/2025-04-22-reorganize-docs.md`](plans/2025-04-22-reorganize-docs.md) |
| Progressive Skill Loading | ✅ Complete | [`plans/2026-04-24-progressive-skill-loading.md`](plans/2026-04-24-progressive-skill-loading.md) |

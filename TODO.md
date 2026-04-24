# Shelly TODO

## 🚧 In Progress / Ready for Development

## 📋 Backlog

### Skills System
- [x] Support multiple skills in one session
- [x] Manual skill selection (`--skills skill1,skill2` flag)
- [x] Skill install command: `shelly skills add <url>` (supports multiple skills per repo with `--skill <name>` for specific skill)
- [ ] Automatic skill installation (discover and install missing skills on demand)

### Shell & UX
- [ ] Command history
  - Store generated commands with timestamps
  - `shelly history` to list, `shelly undo` or replay
- [x] **Interactive clarification tool** (`ask_user`)
  - ~~When prompt is ambiguous, AI can request user choice~~
  - ~~Uses dialoguer::Select to present options interactively~~
  - ~~Arrow key navigation, Enter to select~~
  - ~~Returns selected value to AI for command generation~~

### Configuration
- [ ] Configuration editing (`shelly config`)
  - View/edit settings without re-running setup
  - Show current config when re-running setup

### Testing & Quality
- [x] **Add tests**
  - ~~Unit tests for shell detection~~
  - ~~Unit tests for tools (ToolRegistry, traits)~~
  - ~~Unit tests for skills (SkillManager, parsing)~~
  - ~~Smoke tests for CLI commands~~

## ✅ Completed

### Core Features
- [x] Fish shell support
- [x] System prompt with OS/shell context
- [x] Proper stdout/stderr split
- [x] Pixel spinner and styled output
- [x] Error handling for edge cases

### Architecture
- [x] Move shell scripts to external files with `include_str!`
- [x] Migrate AI call to dedicated subcommand (`shelly generate`)

### CLI Features
- [x] Dry-run mode (`--dry-run` flag)
- [x] Shell completion generation (`shelly completions <shell>`)

### AI Tools
- [x] **Setup tools** (web search, file reading)
  - Tool framework with trait and registry
  - Web search tool (DuckDuckGo)
  - Read file tool (with security restrictions)
  - AI decides when to use tools based on prompts

### Skills
- [x] Progressive skill disclosure (lazy loading)
  - System prompt only embeds `name` + `description` + `path` for matching skills
  - AI uses `read_file` tool to load full skill content on demand
  - Enables multi-file skills without context bloat, matching Anthropic Agent Skills pattern

---

*Last updated: 2025-04-21*

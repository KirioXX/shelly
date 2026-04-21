# Shelly TODO

## 🚧 In Progress / Ready for Development

## 📋 Backlog

### Skills System
- [ ] Support multiple skills in one session
- [ ] Manual skill selection (`--skill <name>` flag)
- [ ] Skill install command: `shelly skill add <url>`
- [ ] Automatic skill installation (discover and install missing skills on demand)

### Shell & UX
- [ ] Command history
  - Store generated commands with timestamps
  - `shelly history` to list, `shelly undo` or replay
- [ ] Multi-turn conversations (`shelly --chat`)
  - Back-and-forth refinement
  - Context from previous commands

### Configuration
- [ ] Configuration editing (`shelly config`)
  - View/edit settings without re-running setup
  - Show current config when re-running setup

### Testing & Quality
- [ ] Add tests
  - Unit tests for shell detection
  - Tests for AI response parsing
  - Mock API responses for testing

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
- [x] Enable skills (shelly-specific in `~/.config/shelly/skills/`)
  - Skill discovery and loading system
  - Match prompts to skills by keywords in description
  - Prepend skill instructions to system prompt
  - Installed: `curl-command-generator` (copied from Pi skills)

---

*Last updated: 2025-04-21*

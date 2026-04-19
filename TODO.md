# Shelly TODO

## 🎯 Ready for Implementation

- [x] **Dry-run mode** (`--dry-run` flag)
  - ~~Show command without injecting it~~
  - ~~Useful for testing AI responses before running~~
  
- [ ] **Shell completion generation** (`--completions <shell>`)
  - Generate completion scripts for Bash/Zsh/Fish
  - Users can pipe to their shell config

- [x] **Enable skills** (shelly-specific in `~/.config/shelly/skills/`)
  - ~~Create skill discovery and loading system~~
  - ~~Match prompts to skills by keywords in description~~
  - ~~Prepend skill instructions to system prompt~~
  - Installed: `curl-command-generator` (copied from Pi skills)
  
- [ ] **Setup tools** (e.g. web search, file reading)
  - Tool definitions for the AI (web search, read file, etc.)
  - Let AI decide when to use tools

## 🛠️ Future Enhancements (after testing)

- [ ] **Support multiple skills** in one session
- [ ] **Manual skill selection** (`--skill <name>` flag)
- [ ] **Skill install command**: `shelly skill add <url>`

## 🚀 Future Ideas

- [ ] **Command history**
  - Store generated commands with timestamps
  - `shelly history` to list, `shelly undo` or replay

- [ ] **Add tests**
  - Unit tests for shell detection
  - Tests for AI response parsing
  - Mock API responses for testing

- [ ] **Multi-turn conversations**
  - `shelly --chat` for back-and-forth refinement
  - Context from previous commands

- [ ] **Configuration editing**
  - `shelly config` to view/edit settings
  - Change model without re-running setup

## ✅ Completed

- [x] Fish shell support
- [x] Move shell scripts to external files with `include_str!`
- [x] System prompt with OS/shell context
- [x] Proper stdout/stderr split
- [x] Pixel spinner and styled output
- [x] Error handling for edge cases

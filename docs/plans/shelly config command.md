---
title: "Plan: shelly config command"
project: shelly
tags: [plan, implementation, kanban]
---

# 📋 Plan: `shelly config` Command

> Related ticket: [[01 - Projects/shelly/Kanban#🚧 In Progress|Configuration editing — View/edit without re-running setup]]

---

## Goal

Let users view and edit their Shelly configuration without re-running the full setup wizard (which also touches shell rc files).

---

## Files to Modify

| File | Change |
|---|---|
| `src/commands/mod.rs` | Add `Config { show: bool }` variant to `Commands` enum |
| `src/commands/config.rs` | **New** — Config view/edit implementation |
| `src/commands/mod.rs` | Add `pub mod config;` |
| `src/main.rs` | Add `Commands::Config` match arm |

---

## Subcommands

| Command | Behaviour |
|---|---|
| `shelly config` | Display config + prompt "Want to edit?" → if yes, interactive edit |
| `shelly config --show` | Display only, no prompts |

---

## Display Format

API key is **masked** for security:

```
Current configuration:

  Model:     claude-3-5-sonnet
  API URL:   https://api.openai.com/v1
  API Key:   sk-****abcd
  Shell:     Zsh
```

---

## Edit Flow

1. Show masked config
2. **"Want to edit these settings?"** Yes/No
3. For each field (model, api_url, api_key, shell):  
   - Show current value as dialoguer **default**
   - Press Enter → keep current value
4. **"Change API key?"** Yes/No — so we can skip re-entry on most runs
5. Save via `confy::store(APP_NAME, CONFIG_NAME, new_config)`
6. Print "✓ Config saved"

---

## Key Decisions

| Decision | Rationale |
|---|---|
| **API key masking** | `sk-****abcd` — first 5 chars + last 4, everything masked |
| **No shell edits** | Unlike `setup()`, `config` never touches `.bashrc`/`.zshrc` |
| **Skip empty fields** | Press Enter without typing = keep current value |
| **No `--set key=value` yet** | Interactive MVP first; CLI flags can be added later |
| **No shell re-installation** | If user changed shell type, just save the enum; do NOT rewrite `.bashrc` |

---

## Implementation Steps

### Task 1: Add CLI Variant

**File**: `src/commands/mod.rs`  
Add `Config` subcommand with a `--show` flag:

```rust
/// View or edit configuration
Config {
    /// Display config without editing
    #[arg(short, long)]
    show: bool,
},
```

---

### Task 2: Create `config.rs` Module

**File**: `src/commands/config.rs` (new)

```rust
pub fn config(show_only: bool) -> Result<(), Box<dyn Error>> {
    // 1. Load existing config via confy
    // 2. Format & display (with masked key)
    // 3. If --show, return early
    // 4. Ask "Edit?" → if no, return
    // 5. Prompt each field (with current value as default)
    // 6. Save via confy::store()
}
```

**Helper functions** (all testable):

| Function | Pure? | Tests |
|---|---|---|
| `mask_api_key(key: &str) -> String` | ✅ | Display masking edge cases |
| `format_config(cfg: &Config) -> String` | ✅ | Output format, empty fields, None shell |
| `prompt_edit(cfg: &Config) -> Result<Config, ...>` | ❌ (dialoguer) | Extracted thin wrapper, tested indirectly |

---

### Task 3: Wire up in `main.rs`

**File**: `src/main.rs`  
Add match arm:
```rust
Commands::Config { show } => {
    if let Err(err) = commands::config::config(show) {
        eprintln!("Failed to show/edit config: {:?}", err);
        std::process::exit(1);
    }
}
```

---

## Tests

### Unit Tests (pure functions)

```rust
fn test_mask_api_key_normal()  // "sk-abcdefghijklmnopqrstuvwxyz" → "sk-****wxyz"
fn test_mask_api_key_short()   // "abc" → "***"
fn test_mask_api_key_empty()   // "" → ""
fn test_mask_api_key_exact_9()  // "sk-ABCDEFGH" → "sk-****GH"
fn test_format_config_full()    // All fields present
fn test_format_config_no_shell() // shell: None shows "not set"
fn test_format_config_empty_key() // api_key: "" shows "(not set)"
```

### Strategy
- Interactive `dialoguer` loop stays thin and untested (same pattern as `setup()`)
- Pure display/masking functions have full unit-test coverage
- `cargo test` passes before claiming completion

---

## Risks & Mitigations

| Risk | Mitigation |
|---|---|
| **Testing interactive dialoguer** is hard | Extract pure display/masking functions; keep interactive loop thin |
| **Confy path varies by OS** | Only test display helpers, not confy load/save directly |
| **Masking might leak key length** | Fixed format: show prefix (first 2 chars) + asterisks + suffix (last 4 chars) regardless of length |

---

## Acceptance Criteria

- [ ] `shelly config` displays config and offers to edit
- [ ] `shelly config --show` displays silently and exits
- [ ] API key is masked in output
- [ ] Editing keeps values when Enter is pressed
- [ ] All new code has tests
- [ ] All tests pass (`cargo test`)
- [ ] Clippy clean (`just lint`)

---

*Plan written by agent — 2026-05-06*

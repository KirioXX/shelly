---
title: "Plan: Command History System"
project: shelly
tags: [plan, implementation, kanban]
---

# 📋 Plan: Command History System

> Related ticket: [[01 - Projects/shelly/Kanban#🚧 In Progress|Command history — `shelly history`, `shelly undo`, replay]]

---

## Goal

Record every generated command with metadata, allow browsing, searching, and replaying — all without leaving the terminal.

---

## Data Model

Each history entry is a JSON object stored as one line in a JSONL file:

```json
{
  "id": "uuid-v4",
  "timestamp": "2026-05-06T14:32:01Z",
  "prompt": "list all docker containers",
  "command": "docker ps -a",
  "shell": "zsh",
  "dry_run": false
}
```

| Field | Type | Description |
|---|---|---|
| `id` | UUID | Unique identifier per entry |
| `timestamp` | ISO 8601 | When the command was generated |
| `prompt` | String | The original natural language prompt |
| `command` | String | The generated shell command |
| `shell` | String | Shell type at generation time |
| `dry_run` | Bool | Whether `--dry-run` was used |

---

## Storage

**Format:** JSON Lines (`.jsonl`) — one JSON object per line  
**Location:** `~/.config/shelly/history.jsonl`

**Why JSONL over SQLite?**
- Zero new dependencies (just `serde_json`, already used)
- Append-only = simple, fast, no locking issues
- Human-readable with `jq`, `grep`, `tail`
- Easy to back up / version control (if user wants)
- No schema migrations ever needed

**Trade-off:** No SQL queries, but we only need list/filter/replay — easily done in memory.

---

## New CLI Subcommands

### `shelly history`

| Flag | Description |
|---|---|
| (no args) | Show last 20 entries |
| `--limit N` / `-n N` | Show last N entries |
| `--search PATTERN` / `-s PATTERN` | Filter by prompt or command (substring match) |
| `--clear` | Clear entire history (with `dialoguer::Confirm`) |
| `--raw` | Output raw JSONL (pipeable to `jq`) |

**Display format:**
```
  1. 2026-05-06 14:32  "list docker containers"     →  docker ps -a
  2. 2026-05-06 14:35  "git status"                 →  git status
  3. 2026-05-06 14:38  "find large files"           →  find . -size +100M
```

### `shelly undo`

| Flag | Description |
|---|---|
| (no args) | Replay the most recent command |
| `--index N` / `-i N` | Replay the Nth most recent command (from `shelly history`) |
| `--dry-run` / `-d` | Show what would be replayed, don't inject |

**Behavior:** Identical to `shelly generate` — prints command to stdout for shell buffer injection. Includes a `⚠️ Replaying: "original prompt"` message on stderr.

---

## Files to Create / Modify

| File | Action | Description |
|---|---|---|
| `src/history.rs` | **New** | Core history module: read, write, filter, replay |
| `src/commands/history.rs` | **New** | `shelly history` subcommand implementation |
| `src/commands/undo.rs` | **New** | `shelly undo` subcommand implementation |
| `src/commands/mod.rs` | **Modify** | Add `History` and `Undo` variants to `Commands` enum |
| `src/lib.rs` | **Modify** | Add `pub mod history;` |
| `src/main.rs` | **Modify** | Add match arms for `History` and `Undo` |
| `src/commands/ai/mod.rs` | **Modify** | After successful generation, append entry to history |

---

## Implementation Steps

### Task 1: Core History Module (`src/history.rs`)

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: String,
    pub prompt: String,
    pub command: String,
    pub shell: String,
    pub dry_run: bool,
}

pub fn history_path() -> PathBuf { ... }
pub fn append(entry: HistoryEntry) -> Result<(), ...> { ... }
pub fn load(limit: usize) -> Result<Vec<HistoryEntry>, ...> { ... }
pub fn search(pattern: &str, limit: usize) -> Result<Vec<HistoryEntry>, ...> { ... }
pub fn clear() -> Result<(), ...> { ... }
pub fn replay(index: usize) -> Result<HistoryEntry, ...> { ... }  // 0 = most recent
```

**Key decisions:**
- `append()` opens file in append mode, writes one JSON line, flushes
- `load()` reads file backwards (or reads all and reverses) to get newest first
- `search()` case-insensitive substring match on both `prompt` and `command`
- `replay()` returns the entry; the caller decides how to display/replay it

### Task 2: `shelly history` Subcommand (`src/commands/history.rs`)

```rust
pub fn history(limit: usize, search: Option<String>, clear: bool, raw: bool) -> Result<(), ...> {
    if clear {
        // dialoguer::Confirm, then truncate history file
    }
    // load or search entries
    // format as table or raw JSONL
}
```

### Task 3: `shelly undo` Subcommand (`src/commands/undo.rs`)

```rust
pub fn undo(index: usize, dry_run: bool) -> Result<(), ...> {
    let entry = history::replay(index)?;
    if dry_run {
        eprintln!("🔍 Would replay: {}", entry.command);
        Ok(())
    } else {
        eprintln!("↩️  Replaying: {}", entry.prompt);
        // Print command to stdout for shell injection
        println!("{}", entry.command);
        Ok(())
    }
}
```

### Task 4: Wire into `shelly generate`

After successful command generation in `src/commands/ai/mod.rs`, append:

```rust
let entry = HistoryEntry {
    id: Uuid::new_v4().to_string(),
    timestamp: Utc::now().to_rfc3339(),
    prompt: full_prompt,
    command: command.clone(),
    shell: cfg.shell.as_ref().map(|s| s.to_string()).unwrap_or_default(),
    dry_run,
};
history::append(entry)?;
```

**Note:** Only append if `!dry_run` (or append with `dry_run: true` flag — useful for auditing). Decision: **always append**, mark dry_run field.

### Task 5: CLI Enum Updates

```rust
/// Browse command history
History {
    #[arg(short, long, default_value = "20")]
    limit: usize,
    #[arg(short, long)]
    search: Option<String>,
    #[arg(long)]
    clear: bool,
    #[arg(long)]
    raw: bool,
},

/// Replay a previous command
Undo {
    #[arg(short, long, default_value = "0")]
    index: usize,
    #[arg(short = 'd', long)]
    dry_run: bool,
},
```

---

## Tests

### Unit Tests (`src/history.rs`)

```rust
fn test_append_and_load()          // Write 3 entries, load all, verify order
fn test_load_limit()               // Write 5, load 2, get last 2
fn test_search_prompt()            // Search by prompt substring
fn test_search_command()           // Search by command substring
fn test_search_case_insensitive() // "DOCKER" matches "docker"
fn test_replay_most_recent()       // replay(0) returns last entry
fn test_replay_index()             // replay(2) returns 3rd from end
fn test_replay_empty_history()     // Returns error "No commands in history"
fn test_clear()                    // Clear file, verify empty
fn test_history_entry_serialize()  // JSON round-trip
```

### Smoke Tests (`tests/cli_smoke_test.rs`)

```rust
fn test_history_help_shows_flags()   // --help lists --limit, --search, --clear, --raw
fn test_undo_help_shows_flags()      // --help lists --index, --dry-run
fn test_history_raw_empty()          // history --raw on empty file → no output, success
```

---

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `uuid` | `1.0` | Unique entry IDs |
| `chrono` | `0.4` | ISO 8601 timestamps |

Both are lightweight and commonly used. `chrono` may already be a transitive dependency.

---

## Risks & Mitigations

| Risk | Mitigation |
|---|---|
| **History file grows unbounded** | `--clear` flag; future: `--prune N` to keep only last N days |
| **Concurrent writes** | Append mode is atomic on POSIX; good enough for CLI tool |
| **UUID + chrono add deps** | Both are tiny; `uuid` has `v4` feature only |
| **Undo with wrong shell** | Store shell name in entry; replay warns if current shell differs |

---

## Acceptance Criteria

- [ ] `shelly generate "foo"` appends an entry to history
- [ ] `shelly history` shows last 20 entries in human-readable table
- [ ] `shelly history --search docker` filters entries
- [ ] `shelly history --clear` empties history (with confirmation)
- [ ] `shelly undo` replays the most recent command to stdout
- [ ] `shelly undo --index 2` replays the 3rd most recent
- [ ] Dry-run commands are marked in history (not skipped)
- [ ] All new code has tests
- [ ] `cargo test` passes (51+ tests)
- [ ] `just lint` clean

---

*Plan written by agent — 2026-05-06*

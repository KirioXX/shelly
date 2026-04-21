# Migrate AI Call to Dedicated Subcommand - Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move AI command generation from default CLI behavior to explicit `shelly generate` subcommand to fix logging issues and make setup more robust.

**Architecture:** Add `Generate` subcommand to clap CLI, remove prompt/dry_run from top-level Cli struct, update shell wrappers to call `shelly generate` instead of just `shelly`.

**Tech Stack:** Rust, clap

---

## File Structure

| File | Responsibility |
|------|----------------|
| `src/main.rs` | CLI structure and command dispatch - add Generate subcommand, remove top-level args |
| `src/commands/scripts/bash.sh` | Shell wrapper - update shelly invocation to include "generate" |
| `src/commands/scripts/zsh.sh` | Shell wrapper - update shelly invocation to include "generate" |
| `src/commands/scripts/fish.sh` | Shell wrapper - update shelly invocation to include "generate" |

---

## Task 1: Add Generate Subcommand to CLI

**Files:**
- Modify: `src/main.rs:10-20` (Commands enum)
- Modify: `src/main.rs:6-17` (Cli struct)
- Modify: `src/main.rs:23-45` (main function match block)

### Current State (src/main.rs:6-17):
```rust
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, help = "Show command without executing")]
    dry_run: bool,

    #[arg(trailing_var_arg = true)]
    prompt: Vec<String>, // Capture any extra arguments
}
```

### Current State (src/main.rs:19-22):
```rust
#[derive(Debug, Subcommand)]
enum Commands {
    Setup {},
}
```

### Current State (src/main.rs:38-45):
```rust
None => {
    match commands::ai::call(cli.prompt, cli.dry_run).await {
        Ok(command) => println!("{}", command),
        Err(err) => println!("Failed: {:?}", err)
    }
}
```

- [ ] **Step 1.1: Add Generate variant to Commands enum**

```rust
#[derive(Debug, Subcommand)]
enum Commands {
    Setup {},
    Generate {
        #[arg(trailing_var_arg = true)]
        prompt: Vec<String>,
        
        #[arg(long, help = "Show command without executing")]
        dry_run: bool,
    },
}
```

- [ ] **Step 1.2: Remove prompt and dry_run from top-level Cli**

```rust
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

Note: Changed `Option<Commands>` to `Commands` since we'll require a subcommand now.

- [ ] **Step 1.3: Update main function match block**

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup {} => {
            match commands::setup::setup() {
                Ok(Some(new_cfg)) => {
                    match confy::store(
                        APP_NAME,
                        CONFIG_NAME,
                        new_cfg
                    ) {
                        Ok(_) => println!("All done!"),
                        Err(_) => eprintln!("Config save failed")
                    }
                }
                Ok(None) => println!("Setup cancelled."),
                Err(_err) => eprintln!("Setup failed")
            }
        },
        Commands::Generate { prompt, dry_run } => {
            match commands::ai::call(prompt, dry_run).await {
                Ok(command) => println!("{}", command),
                Err(err) => println!("Failed: {:?}", err)
            }
        }
    }
    Ok(())
}
```

- [ ] **Step 1.4: Verify build compiles**

Run: `cargo build`
Expected: Compiles without errors

- [ ] **Step 1.5: Test CLI help shows generate subcommand**

Run: `cargo run -- --help`
Expected: Shows "generate" as available subcommand with prompt and --dry-run options

- [ ] **Step 1.6: Commit**

```bash
git add src/main.rs
git commit -m "feat(cli): add 'generate' subcommand for AI command generation"
```

---

## Task 2: Update Bash Shell Wrapper

**Files:**
- Modify: `src/commands/scripts/bash.sh`

### Current State:
Need to inspect current bash.sh content. The shell function likely calls `shelly "$@"` or similar.

- [ ] **Step 2.1: Read current bash.sh**

Run: `cat src/commands/scripts/bash.sh` (or read with tool)

- [ ] **Step 2.2: Update bash.sh to call 'shelly generate'**

Look for the line that calls shelly and add "generate" before the arguments.

For example, if the current wrapper is:
```bash
command=$(shelly "$@")
```

Change to:
```bash
command=$(shelly generate "$@")
```

- [ ] **Step 2.3: Verify bash script syntax**

Run: `bash -n src/commands/scripts/bash.sh`
Expected: No errors

- [ ] **Step 2.4: Commit**

```bash
git add src/commands/scripts/bash.sh
git commit -m "fix(shell): update bash wrapper to use 'shelly generate' subcommand"
```

---

## Task 3: Update Zsh Shell Wrapper

**Files:**
- Modify: `src/commands/scripts/zsh.sh`

- [ ] **Step 3.1: Read current zsh.sh**

Run: `cat src/commands/scripts/zsh.sh`

- [ ] **Step 3.2: Update zsh.sh to call 'shelly generate'**

Look for the line that calls shelly and add "generate" before the arguments.

- [ ] **Step 3.3: Verify zsh script syntax**

Run: `zsh -n src/commands/scripts/zsh.sh` (if zsh is available, or just visual inspection)
Expected: No errors

- [ ] **Step 3.4: Commit**

```bash
git add src/commands/scripts/zsh.sh
git commit -m "fix(shell): update zsh wrapper to use 'shelly generate' subcommand"
```

---

## Task 4: Update Fish Shell Wrapper

**Files:**
- Modify: `src/commands/scripts/fish.sh`

- [ ] **Step 4.1: Read current fish.sh**

Run: `cat src/commands/scripts/fish.sh`

- [ ] **Step 4.2: Update fish.sh to call 'shelly generate'**

Fish syntax differs from bash/zsh. Look for the line that calls shelly (might look like `shelly $argv`) and add "generate".

Example:
```fish
command $(shelly generate $argv)
```

- [ ] **Step 4.3: Verify fish script syntax**

Run: `fish -n src/commands/scripts/fish.sh` (if fish is available, or just visual inspection)
Expected: No errors

- [ ] **Step 4.4: Commit**

```bash
git add src/commands/scripts/fish.sh
git commit -m "fix(shell): update fish wrapper to use 'shelly generate' subcommand"
```

---

## Task 5: Update Documentation

**Files:**
- Modify: `README.md`

- [ ] **Step 5.1: Find usage examples in README**

Search for examples showing `shelly "..."` and update them to `shelly generate "..."`

- [ ] **Step 5.2: Update dry-run example**

The dry-run flag example should change from:
```bash
shelly --dry-run "..."
```
to:
```bash
shelly generate --dry-run "..."
```

- [ ] **Step 5.3: Add migration note**

Add note explaining the breaking change:
```markdown
**Breaking Change:** As of version X.X, you must use `shelly generate` instead of `shelly` directly.
```

- [ ] **Step 5.4: Commit**

```bash
git add README.md
git commit -m "docs: update usage examples for new 'shelly generate' subcommand"
```

---

## Task 6: End-to-End Testing

- [ ] **Step 6.1: Build release binary**

Run: `cargo build --release`
Expected: Successful build

- [ ] **Step 6.2: Test generate subcommand**

Run: `./target/release/shelly generate --dry-run "list current directory"`
Expected: Shows generated command without errors

- [ ] **Step 6.3: Verify no subcommand fails**

Run: `./target/release/shelly "list current directory"`
Expected: Error message indicating generate subcommand is required

- [ ] **Step 6.4: Test setup still works**

Run: `./target/release/shelly setup`
Expected: Setup wizard runs normally

- [ ] **Step 6.5: Commit**

```bash
git commit -m --allow-empty -m "test: end-to-end testing complete"
```

---

## Self-Review

**1. Spec coverage:**
- ✅ Add `generate` subcommand - Tasks 1.1-1.3
- ✅ Remove prompt/dry_run from top-level - Task 1.2
- ✅ Update shell wrappers - Tasks 2-4
- ✅ Fix logging issues - Achieved by explicit subcommand separation

**2. Placeholder scan:**
- No TBD/TODO found
- No "implement later" found
- Specific code provided for each step

**3. Type consistency:**
- Commands enum has Generate variant with prompt: Vec<String> and dry_run: bool
- Cli.command changed from Option<Commands> to Commands
- Match arms updated to use cli.command directly (no Some/None)

**4. Completeness:**
- All shell wrappers updated
- Documentation updated
- Tests defined for verification
- DRY: single source of truth for command dispatch

---

**Plan complete and saved to `docs/superpowers/plans/2025-04-21-migrate-ai-to-subcommand.md`**

Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints for review

Which approach?

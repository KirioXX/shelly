# Shell Completions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `shelly completions <shell>` subcommand to generate shell completion scripts for Bash, Zsh, and Fish.

**Architecture:** Add `Completions` variant to Commands enum that takes a shell argument, use `clap_complete` crate to generate completions from the CLI definition, output to stdout so users can redirect to their shell config.

**Tech Stack:** Rust, clap, clap_complete

---

## File Structure

| File | Responsibility |
|------|----------------|
| `Cargo.toml` | Add clap_complete dependency |
| `src/main.rs` | Add `Completions` subcommand, implement completion generation |
| `README.md` | Document how to use completions |

---

## Task 1: Add clap_complete Dependency

**Files:**
- Modify: `Cargo.toml:15-20` (dependencies section)

Current state (check with `cat Cargo.toml`):

- [ ] **Step 1.1: Add clap_complete to Cargo.toml**

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
clap_complete = "4.5"
# ... other deps
```

- [ ] **Step 1.2: Verify cargo fetches the dependency**

Run: `cargo check`
Expected: Compiles, fetches clap_complete

- [ ] **Step 1.3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "deps: add clap_complete for shell completion generation"
```

---

## Task 2: Add Completions Subcommand

**Files:**
- Modify: `src/main.rs:1-5` (imports)
- Modify: `src/main.rs:18-28` (Commands enum)
- Modify: `src/main.rs:55-70` (match block)

Current state of Commands enum (need to add Completions variant):

```rust
#[derive(Debug, Subcommand)]
enum Commands {
    Setup {},
    Generate { ... },
    Cmds {},
}
```

- [ ] **Step 2.1: Add required imports for clap_complete**

Add to top of src/main.rs (with other use statements):

```rust
use clap_complete::{generate, shells::Shell, Generator};
use std::io;
```

- [ ] **Step 2.2: Add Completions variant to Commands enum**

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
    Cmds {},
    /// Generate shell completion scripts
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}
```

- [ ] **Step 2.3: Add implementation for Completions in match block**

Add this arm to the match statement:

```rust
Commands::Completions { shell } => {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    match shell {
        Shell::Bash => generate(shells::Bash, &mut cmd, name, &mut io::stdout()),
        Shell::Zsh => generate(shells::Zsh, &mut cmd, name, &mut io::stdout()),
        Shell::Fish => generate(shells::Fish, &mut cmd, name, &mut io::stdout()),
        _ => {
            eprintln!("Shell not supported yet");
            return Ok(());
        }
    }
}
```

- [ ] **Step 2.4: Fix imports for shells**

Update the import from `use clap_complete::{generate, shells::Shell, Generator};` to:

```rust
use clap_complete::{generate, shells};
use clap::ValueEnum;
```

Also note: the Commands enum variant uses `clap::ValueEnum` for Shell. We need Shell to be available, which it should be from `clap_complete::shells::Shell` which implements ValueEnum.

Actually, the shell argument should be:

```rust
Completions {
    #[arg(value_enum)]
    shell: shells::Shell,
}
```

- [ ] **Step 2.5: Verify build compiles**

Run: `cargo build --release`
Expected: Compiles without errors

- [ ] **Step 2.6: Test completions generation**

Run: `shelly completions bash | head -20`
Expected: Shows bash completion script content

Run: `shelly completions zsh | head -20`
Expected: Shows zsh completion script content

Run: `shelly completions fish | head -20`
Expected: Shows fish completion script content

- [ ] **Step 2.7: Commit**

```bash
git add src/main.rs
git commit -m "feat: add 'completions' subcommand for shell completion generation"
```

---

## Task 3: Update Documentation

**Files:**
- Modify: `README.md` (add completions section)

- [ ] **Step 3.1: Add completions section after Usage**

Add a new section after the Usage section or at the end of the README:

```markdown
### Shell Completions

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
```

- [ ] **Step 3.2: Verify completions work end-to-end**

After installing completions:
- Type `shelly <TAB>` and see subcommand suggestions
- Type `shelly generate --<TAB>` and see flag suggestions

- [ ] **Step 3.3: Commit**

```bash
git add README.md
git commit -m "docs: add shell completions documentation"
```

---

## Self-Review

**1. Spec coverage:**
- ✅ Add completions subcommand - Task 2.2
- ✅ Support bash/zsh/fish - Tasks 2.6
- ✅ Output to stdout for redirection - Task 2.3
- ✅ Document usage - Task 3.1

**2. Placeholder scan:**
- No TBD/TODO found
- No "implement later" found
- Specific code provided for each step

**3. Type consistency:**
- Commands enum has new Completions variant with shell: shells::Shell
- Uses clap_complete::shells for Shell enum (implements ValueEnum)
- Generator type from clap_complete used correctly

**4. Completeness:**
- Dependency added
- Command implemented
- Documentation updated
- Tests defined for verification

---

**Plan complete and saved to `docs/superpowers/plans/2025-04-21-shell-completions.md`**

Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints for review

Which approach?

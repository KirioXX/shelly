# Shelly CLI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust CLI that generates shell commands from natural language, streaming reasoning to stderr and the final command to stdout for shell buffer injection.

**Architecture:** Async runtime using `tokio` and `async-openai`. Strict stream management ensures only the final command reaches stdout. A `setup` wizard handles config and shell integration.

**Tech Stack:** Rust, `clap`, `tokio`, `async-openai`, `confy`, `indicatif`, `dialoguer`, `serde`.

---

### Task 1: Dependency Update & Project Structure

**Files:**
- Modify: `Cargo.toml`

**Step 1: Add necessary dependencies**
Add the following to `Cargo.toml`:
```toml
tokio = { version = "1", features = ["full"] }
async-openai = "0.23.0" 
indicatif = "0.17"
# Keep existing clap, confy, dialoguer, serde
```

**Step 2: Verify dependencies**
Run: `cargo check`
Expected: Success.

**Step 3: Commit**
```bash
git add Cargo.toml
git commit -m "build: add tokio, async-openai, and indicatif"
```

---

### Task 2: CLI Command Refactoring

**Files:**
- Modify: `src/main.rs`

**Step 1: Update Commands Enum**
Change `Commands::Init {}` to `Commands::Setup {}`.

**Step 2: Update main match arm**
Replace `Commands::Init` match arm with `Commands::Setup`.

**Step 3: Run to verify it compiles**
Run: `cargo check`
Expected: Success.

**Step 4: Commit**
```bash
git add src/main.rs
git commit -m "refactor: rename init to setup"
```

---

### Task 3: Implement the Setup Wizard & Shell Integration

**Files:**
- Modify: `src/main.rs`
- Modify: `src/config.rs`

**Step 1: Update `Config` struct in `src/config.rs`**
Ensure `Config` has `base_url` and `api_key` mapping to the needs of `async-openai`.

**Step 2: Implement `setup()` function in `src/main.rs`**
Replace the old `init()` logic with a new `setup()` flow:
1. Collect API Key, Base URL, Model using `dialoguer`.
2. Collect Shell Type (Bash/Zsh).
3. Save to config via `confy`.
4. **Shell Integration**: Identify `~/.zshrc` or `~/.bashrc`, show the wrapper function, and append it if user confirms.

**Step 3: Implement the wrapper templates**
Bake the `.zsh` and `.bash` templates into the binary using `include_str!`.

**Step 4: Test the setup flow**
Run: `cargo run -- setup`
Expected: Wizard completes, config saved, and shell rc updated.

**Step 5: Commit**
```bash
git add src/main.rs src/config.rs
git commit -m "feat: implement comprehensive setup wizard with shell integration"
```

---

### Task 4: Implement AI Core Logic (The Stream Split)

**Files:**
- Modify: `src/main.rs`
- Create: `src/ai.rs` (or add to main)

**Step 1: Add `tokio::main` to `main()`**
Make `main` async to support `async-openai`.

**Step 2: Implement `execute_prompt(prompt: String)`**
1. Load config.
2. Init `async_openai::Client`.
3. Setup `indicatif` spinner on `stderr`.
4. Send prompt with system instructions: *"Reasoning on stderr, only the command in markdown blocks on stdout."*
5. **The Streaming Loop**: 
   - Stream chunks from LLM.
   - Read response; print all non-code-block content to `stderr`.
   - Capture the content inside the first ` ```bash ` or ` ``` ` block.

**Step 3: Implement Final Output**
Print the captured command string to `stdout` and exit.

**Step 4: Commit**
```bash
git add src/main.rs src/ai.rs
git commit -m "feat: implement AI core with stdout/stderr split and streaming"
```

---

### Task 5: Verification & TDD

**Files:**
- Create: `tests/stream_test.rs`

**Step 1: Write a test for stdout leak**
Create a test that mocks an LLM response (containing both reasoning and a command) and asserts that `stdout` contains ONLY the command.

**Step 2: Run the test**
Run: `cargo test`
Expected: PASS.

**Step 3: Manual End-to-End Test**
1. Run `cargo run -- setup`.
2. Source the shell rc.
3. Run `shelly "list files alphabetically"` (via the shell function).
4. Verify reasoning appears but the command is injected into the buffer.

**Step 4: Final Commit**
```bash
git add tests/stream_test.rs
git commit -m "test: verify stdout/stderr split"
```

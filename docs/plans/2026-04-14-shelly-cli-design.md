# Design Document: Shelly CLI
**Date:** 2026-04-14
**Status:** Approved

## 1. Overview
Shelly is a Rust-based CLI assistant that translates natural language prompts into shell commands and injects them directly into the user's shell editing buffer.

## 2. Core Architecture

### 2.1 Stream Management (The Stdout/Stderr Split)
To allow the shell to capture the command while the user sees the explanation, a strict output policy is implemented:
- **`stderr`**: All AI reasoning, `indicatif` spinners, progress updates, and error messages.
- **`stdout`**: Reserved **exclusively** for the final, sanitized executable command string.
- **Impact**: When the shell wrapper runs `cmd=$(command shelly "...")`, only the final command is captured in `$cmd`. Everything else is printed to the terminal.

### 2.2 Async Runtime & AI Integration
- **Runtime**: `tokio` for async execution.
- **Client**: `async-openai` library for type-safe OpenAI-compatible API calls.
- **Streaming**: Responses will be streamed from the LLM to provide real-time reasoning on `stderr`.

## 3. Command Specification

### 3.1 `shelly setup`
An interactive onboarding wizard using `dialoguer` that handles:
1. **Configuration**: Collects API Key, Base URL, and Model. Saves to `~/.config/shelly/config.toml` via `confy`.
2. **Shell Integration**: 
    - Identifies the current shell (Bash or Zsh).
    - Presents the specific shell wrapper function.
    - Offers to automatically append the function to the user's shell config file (`.zshrc` or `.bashrc`).

### 3.2 `shelly <prompt>`
The main logic flow:
1. **Init**: Load config from environment (`SHELLY_API_KEY`) or `config.toml`.
2. **Request**:
    - Start `indicatif` spinner on `stderr`.
    - Send prompt with a system instruction: *"Provide an explanation on stderr and ONLY the final executable command on stdout, wrapped in markdown code blocks."*
3. **Processing**:
    - Stream reasoning text to `stderr`.
    - Parse the final markdown code block to extract the raw command.
4. **Finalization**: Print the extracted command to `stdout` and exit.

## 4. Technical Stack
- **Language**: Rust (Latest Stable)
- **CLI Parsing**: `clap`
- **Configuration**: `confy` / `serde` / `toml`
- **UI/UX**: `dialoguer` (Setup), `indicatif` (Spinners)
- **AI**: `async-openai`, `tokio`, `reqwest`

## 5. UX Requirements
- **No Leaks**: Zero AI "chatter" on `stdout`.
- **Clean Exits**: `Ctrl+C` should exit without inserting fragments into the shell buffer.
- **Safe Defaults**: If no command is generated, `stdout` must remain empty.

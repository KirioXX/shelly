# Progressive Skill Loading (Lazy Skills) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the skills system to use progressive disclosure: only `name` + `description` + `path` are embedded in the system prompt. The AI loads full skill content on-demand via the existing `read_file` tool, matching Anthropic's documented Agent Skills pattern.

**Architecture:** Add a `path: PathBuf` field to the `Skill` struct so the AI knows where each skill lives. Extract a pure `build_system_prompt` function for testability. Replace eager skill-content embedding with a metadata-only skills section that instructs the AI to `read_file` when needed. The `ReadFile` tool already permits reads within `~/.config/shelly/skills/`, so no new tool is required.

**Tech Stack:** Rust, handlebars, async-openai, tempfile (for tests)

---

## File Structure

| File | Responsibility |
|------|---------------|
| `src/skills/mod.rs` | `Skill` struct, parsing, discovery. Gets new `path` field. |
| `src/commands/ai/system_prompt.rs` | Refactored into `get_system_prompt` (fetches skills) and `build_system_prompt` (pure assembly). Skills section now emits metadata + paths only. |
| `src/commands/ai/prompts/system-prompt.md` | No template changes needed — skills context is appended in Rust. |
| `tests/cli_smoke_test.rs` | No changes — unrelated smoke tests. |

---

## Task 1: Add `path` Field to `Skill` Struct

**Files:**
- Modify: `src/skills/mod.rs`

- [ ] **Step 1.1: Add `path` field to `Skill`**

In `src/skills/mod.rs`, change the `Skill` struct:

```rust
#[derive(Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub content: String,
    pub path: PathBuf,
}
```

- [ ] **Step 1.2: Populate `path` in `parse_skill`**

In the same file, in `parse_skill`, set the field when constructing `Skill`:

```rust
        Ok(Some(Skill {
            name,
            description,
            content: body_lines.join("\n"),
            path: path.clone(),
        }))
```

- [ ] **Step 1.3: Update `test_parse_valid_skill` to assert `path`**

Add an assertion that `skill.path.ends_with("SKILL.md")`:

```rust
        assert_eq!(skill.path, skill_file);
```

- [ ] **Step 1.4: Verify build compiles**

Run: `cargo check`
Expected: Success

- [ ] **Step 1.5: Run skill tests**

Run: `cargo test skills::tests`
Expected: All tests pass

- [ ] **Step 1.6: Commit**

```bash
git add src/skills/mod.rs
git commit -m "feat(skills): add path field to Skill struct for progressive disclosure"
```

---

## Task 2: Extract Pure `build_system_prompt` Function for Testability

**Files:**
- Modify: `src/commands/ai/system_prompt.rs`

- [ ] **Step 2.1: Extract `build_system_prompt`**

Add a new private function that assembles the prompt from template variables and a slice of `Skill`s. The existing `get_system_prompt` will call it after fetching skills.

Replace the entire contents of `src/commands/ai/system_prompt.rs` with:

```rust
use std::{collections::BTreeMap, error::Error};

use handlebars::Handlebars;

use crate::{config::Config, skills::Skill};

const SYSTEM_PROMPT_TEMPLATE: &str = include_str!("prompts/system-prompt.md");

fn get_matching_skills(
    full_prompt: &str,
    manual_skills: &Option<String>,
) -> Result<Vec<Skill>, Box<dyn Error>> {
    let skill_manager = crate::skills::SkillManager::new()?;

    // If manual skills specified, use those
    if let Some(skill_names) = manual_skills {
        let names: Vec<String> = skill_names
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        Ok(skill_manager.load_skills_by_name(&names)?)
    } else {
        // Otherwise auto-detect matching skills
        Ok(skill_manager.find_matching_skills(full_prompt)?)
    }
}

fn build_system_prompt(
    os: &str,
    shell: &str,
    skills: &[Skill],
) -> Result<String, Box<dyn Error>> {
    let handlebars = Handlebars::new();
    let mut data = BTreeMap::new();
    data.insert("os", os);
    data.insert("shell", shell);

    let mut system_prompt = handlebars.render_template(SYSTEM_PROMPT_TEMPLATE, &data)?;

    // Append skill metadata (progressive disclosure — AI reads SKILL.md via read_file)
    if !skills.is_empty() {
        system_prompt.push_str("\n\n# Available Skills\n\n");
        system_prompt.push_str(
            "The following skills may be relevant to your request. \
             Use the `read_file` tool to read the full instructions only when needed:\n\n",
        );

        for skill in skills {
            system_prompt.push_str(&format!("- **{}**: {}\n", skill.name, skill.description));
            system_prompt.push_str(&format!("  File: `{}`\n", skill.path.display()));
        }

        system_prompt.push_str("\nOnly read a skill if its description matches the current task.\n");
    }

    Ok(system_prompt)
}

pub fn get_system_prompt(
    full_prompt: &str,
    cfg: &Config,
    manual_skills: &Option<String>,
) -> Result<String, Box<dyn Error>> {
    let matching_skills = get_matching_skills(full_prompt, manual_skills)?;

    let os = std::env::consts::OS;
    let shell_str = cfg
        .shell
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    build_system_prompt(os, &shell_str, &matching_skills)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn mock_skill(name: &str, description: &str, content: &str, path: &str) -> Skill {
        Skill {
            name: name.to_string(),
            description: description.to_string(),
            content: content.to_string(),
            path: PathBuf::from(path),
        }
    }

    #[test]
    fn test_build_system_prompt_no_skills() {
        let result = build_system_prompt("linux", "bash", &[]).unwrap();
        assert!(result.contains("linux"));
        assert!(result.contains("bash"));
        assert!(!result.contains("Available Skills"));
    }

    #[test]
    fn test_build_system_prompt_with_skills_only_metadata() {
        let skills = vec![mock_skill(
            "curl-gen",
            "Generate curl commands",
            "very long content that should NOT appear in prompt",
            "/home/user/.config/shelly/skills/curl/SKILL.md",
        )];
        let result = build_system_prompt("macos", "zsh", &skills).unwrap();

        assert!(result.contains("macos"));
        assert!(result.contains("zsh"));
        assert!(result.contains("Available Skills"));
        assert!(result.contains("curl-gen"));
        assert!(result.contains("Generate curl commands"));
        assert!(result.contains("/home/user/.config/shelly/skills/curl/SKILL.md"));
        assert!(
            !result.contains("very long content that should NOT appear in prompt"),
            "Full skill content must NOT be embedded in system prompt"
        );
    }

    #[test]
    fn test_build_system_prompt_multiple_skills() {
        let skills = vec![
            mock_skill("a", "desc a", "content a", "/path/a/SKILL.md"),
            mock_skill("b", "desc b", "content b", "/path/b/SKILL.md"),
        ];
        let result = build_system_prompt("linux", "fish", &skills).unwrap();

        assert!(result.contains("a"));
        assert!(result.contains("b"));
        assert!(result.contains("desc a"));
        assert!(result.contains("desc b"));
        assert!(!result.contains("content a"));
        assert!(!result.contains("content b"));
    }

    #[test]
    fn test_build_system_prompt_includes_read_file_hint() {
        let skills = vec![mock_skill(
            "x",
            "desc",
            "content",
            "/path/SKILL.md",
        )];
        let result = build_system_prompt("linux", "bash", &skills).unwrap();
        assert!(result.contains("read_file"));
        assert!(result.contains("only when needed"));
    }
}
```

- [ ] **Step 2.2: Verify compilation**

Run: `cargo check`
Expected: Success

- [ ] **Step 2.3: Run new tests**

Run: `cargo test commands::ai::system_prompt::tests`
Expected: 4 tests pass

- [ ] **Step 2.4: Commit**

```bash
git add src/commands/ai/system_prompt.rs
git commit -m "refactor(ail): extract build_system_prompt, add progressive disclosure for skills"
```

---

## Task 3: Verify No Regressions in Existing Tests

**Files:**
- None (verification only)

- [ ] **Step 3.1: Run full test suite**

Run: `cargo test`
Expected: All tests pass (currently 17)

- [ ] **Step 3.2: Build release**

Run: `cargo build --release`
Expected: Success

- [ ] **Step 3.3: Commit verification**

```bash
git commit --allow-empty -m "test: verify progressive disclosure refactor passes full suite"
```

---

## Task 4: Manual End-to-End Verification

**Files:**
- None (manual verification only)

- [ ] **Step 4.1: Check that existing skills still get discovered**

Run (with a test prompt matching your installed skill):
```bash
shelly "use curl to fetch example.com"
```

Observe stderr output: you should see "📚 Auto-detected skill: find-skills" (or whatever your installed skill is). The system prompt now only carries metadata, but the skill should still be listed.

- [ ] **Step 4.2: Verify with `--dry-run`**

Run:
```bash
shelly --dry-run "use curl to fetch example.com"
```

Expected: AI generates a curl command. The skill content is NOT in the prompt; the AI may or may not choose to `read_file` the skill depending on whether it needs the extra instructions.

- [ ] **Step 4.3: Verify manual skills work**

Run:
```bash
shelly --skills find-skills --dry-run "use curl to fetch example.com"
```

Expected: Skill is listed as "Using skill: find-skills" in stderr, and the command is generated correctly.

- [ ] **Step 4.4: Commit**

```bash
git commit --allow-empty -m "refactor(skills): progressive disclosure implementation verified e2e"
```

---

## Self-Review

**1. Spec coverage:**
- ✅ `Skill.path` field added — Task 1
- ✅ Progressively load only metadata in system prompt — Task 2
- ✅ AI instructed to use `read_file` for full content — Task 2
- ✅ Multi-file skills supported (AI reads SKILL.md, which may reference other files) — Task 2
- ✅ Testability improved with pure `build_system_prompt` — Task 2
- ✅ No new tool needed (uses existing `ReadFile`) — verified

**2. Placeholder scan:**
- No TBD/TODO/fill-in-details in plan
- All code blocks contain complete, compilable code
- All file paths are exact

**3. Type consistency:**
- `Skill.path` is `PathBuf` in struct, populated from `path.clone()` in parser
- `build_system_prompt` accepts `&[Skill]` slice, matching `get_matching_skills` return
- `get_system_prompt` signature unchanged (no callers need updating)

**4. Completeness:**
- Tests cover: no skills, single skill (metadata only), multiple skills, presence of `read_file` hint, absence of content
- Full test run verified
- Manual e2e verification steps included

---

## Execution Handoff

**Plan complete and saved to `docs/superpowers/plans/2026-04-24-progressive-skill-loading.md`.**

Two execution options:

**1. Subagent-Driven (recommended)** — I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** — Execute tasks in this session using executing-plans, batch execution with checkpoints

Which approach?

# Reorganize README and Create Documentation Structure

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Split the lengthy README.md into focused documentation files while keeping a lean README with essential info and clear navigation.

**Architecture:** Create dedicated docs for usage details, skills, and architecture. Reduce README to quick-start overview with links to full docs. Update any internal references.

**Tech Stack:** Markdown, existing docs structure

---

## Current State Analysis

The README.md has grown to include:
- Features list
- Installation steps
- Just commands table
- Usage examples (basic, dry-run, completions, AI tools, interactive clarification)
- Skills documentation
- Architecture details
- License

This is information overload for new users.

---

## File Structure

| File | Responsibility |
|------|----------------|
| `README.md` | Quick overview, features, installation, links to docs |
| `docs/usage.md` | Detailed usage: basic, dry-run, completions, tools, clarification |
| `docs/skills.md` | Skills system documentation |
| `docs/architecture.md` | Technical details (optional - may skip) |

---

## Task 1: Create docs/usage.md

**Files:**
- Create: `docs/usage.md`

**Content to move from README:**
- ### Usage section with all examples
- ### Dry-Run Mode
- ### List Available Commands
- ### Shell Completions
- ### AI Tools
- ### Interactive Clarification

**Step 1.1: Create docs/usage.md with header**

```markdown
# Shelly Usage Guide

Complete guide to using shelly effectively.

## Table of Contents
- [Basic Usage](#basic-usage)
- [Dry-Run Mode](#dry-run-mode)
- [List Commands](#list-commands)
- [Shell Completions](#shell-completions)
- [AI Tools](#ai-tools)
- [Interactive Clarification](#interactive-clarification)

---
```

**Step 1.2: Copy Usage section from README**

Copy everything from "### Usage" through "### Interactive Clarification" sections.

**Step 1.3: Review and clean up formatting**

Ensure markdown is clean and well-formatted.

**Step 1.4: Test links work**

Verify internal TOC links work.

**Step 1.5: Commit**

```bash
git add docs/usage.md
git commit -m "docs: create dedicated usage guide"
```

---

## Task 2: Create docs/skills.md

**Files:**
- Create: `docs/skills.md`
- Modify: `README.md` (remove skills section content)

**Step 2.1: Create docs/skills.md with header**

```markdown
# Skills System

Shelly can automatically load specialized skills to provide expert guidance for specific tasks.

## What are Skills?

Skills are Markdown files that provide the AI with context-specific instructions...

[rest of skills content from README]
```

**Step 2.2: Copy Skills section from README**

Move the entire "### Skills" section and "#### Installing Skills" subsection.

**Step 2.3: Commit**

```bash
git add docs/skills.md
git commit -m "docs: create dedicated skills documentation"
```

---

## Task 3: Trim and Restructure README.md

**Files:**
- Modify: `README.md`

**Step 3.1: Read current README structure**

Note the sections and their order.

**Step 3.2: Create new lean README structure**

```markdown
# 🐚 Shelly

A Rust-based terminal assistant that translates natural language prompts into shell commands.

## Features

- **Natural Language to Command**: Describe what you want, get the exact command
- **Direct Buffer Injection**: Commands appear in your shell prompt for review
- **AI Tools**: Web search, file reading for better context
- **Interactive Clarification**: AI asks when unsure, prevents mistakes
- **Shell Completion**: Tab completion support
- **Skills System**: Expert guidance for specific tasks

## Quick Start

### Installation

```bash
# Clone and build
git clone https://github.com/yourusername/shelly
cd shelly
cargo build --release

# Or use just
just install
```

### Setup

```bash
shelly setup
# Follow the wizard to configure your AI provider
```

### Usage

```bash
shelly "list all docker containers"
shelly generate "find files larger than 100MB"
```

## Development

```bash
just          # See all available commands
just build    # Build release binary
just test     # Run tests
```

## Documentation

- [📖 Usage Guide](docs/usage.md) - Complete usage documentation
- [🎯 Skills](docs/skills.md) - Skills system documentation
- [📋 Shell Completions](docs/usage.md#shell-completions) - Tab completion setup
- [🔧 AI Tools](docs/usage.md#ai-tools) - Web search, file reading
- [❓ Interactive Clarification](docs/usage.md#interactive-clarification) - When AI needs help

## License

MIT
```

**Step 3.3: Implement the new README**

Replace README content with the lean structure above.

**Step 3.4: Verify links work**

Check that `docs/usage.md` and `docs/skills.md` links are correct.

**Step 3.5: Commit**

```bash
git add README.md
git commit -m "docs: trim README to essentials, link to detailed docs"
```

---

## Task 4: Verify Everything Works

**Step 4.1: Verify docs directory structure**

```bash
ls -la docs/
# Should show: usage.md, skills.md
```

**Step 4.2: Check README renders correctly**

Preview README in markdown viewer if possible.

**Step 4.3: Verify all links in README**

- [docs/usage.md](docs/usage.md)
- [docs/skills.md](docs/skills.md)
- Internal anchors to sections

**Step 4.4: Run tests to ensure nothing broke**

```bash
cargo test
```

**Step 4.5: Final commit**

```bash
git commit --allow-empty -m "docs: documentation reorganization complete"
```

---

## Self-Review

**1. Spec coverage:**
- ✅ Create docs/usage.md
- ✅ Create docs/skills.md
- ✅ Trim README to essentials
- ✅ All links working

**2. Content preserved:**
- All usage examples moved to usage.md
- Skills documentation preserved
- No information lost

**3. Navigation:**
- README provides clear overview
- Links to detailed docs
- TOC in usage.md for easy navigation

**4. Completeness:**
- Lean README for quick scanning
- Detailed docs for deep dives
- Skills documentation separate

---

**Plan complete.**

Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task

**2. Inline Execution** - Execute tasks in this session

Which approach?
\n\n---\n\n## ✅ Plan Execution Complete\n\n**Executed:** 2025-04-22 via inline execution\n\n**Status:** All tasks completed\n\n**Commits:**\n- 19146ce - docs: reorganize README and create dedicated usage/skills docs\n\n**Changes:**\n- README: ~175 lines → ~50 lines (lean overview)\n- Created docs/usage.md: Complete usage guide\n- Created docs/skills.md: Skills system documentation\n- All links verified\n- All 17 tests still passing

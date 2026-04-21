# Skills System

Shelly can automatically load specialized skills to provide expert guidance for specific tasks.

## What are Skills?

Skills are Markdown files that provide the AI with context-specific instructions. When your prompt matches a skill's description, shelly automatically loads that skill's guidance to generate better commands.

## How It Works

Shelly matches your prompt to skills based on keywords in the skill's `description` field:

```bash
# Automatically uses curl-command-generator skill
shelly generate "generate curl commands for my API endpoints"
```

When a skill is activated, you'll see:
```
📚 Using skill: curl-command-generator
```

## Installing Skills

Skills are Markdown files with YAML frontmatter stored in `~/.config/shelly/skills/<skill-name>/`.

### Skill File Format

Each skill is a Markdown file named `SKILL.md` with this structure:

```markdown
---
name: my-skill
description: Use when users want X, Y, or Z
---

# Instructions for the AI

Here you provide detailed instructions, examples, constraints, etc. 
for the AI to follow when this skill is active.

## Examples

- Example 1: ...
- Example 2: ...

## Constraints

- Only use ...
- Never ...
```

### Installation Steps

1. Create the skills directory:
```bash
mkdir -p ~/.config/shelly/skills/my-skill
```

2. Copy or create your SKILL.md:
```bash
cp path/to/SKILL.md ~/.config/shelly/skills/my-skill/
```

3. Test it:
```bash
shelly "test my skill description"
```

## Skill Matching

The skill system uses simple keyword matching on the `description` field:

- The description is split into words
- Each word (longer than 3 characters) is treated as a keyword
- If your prompt contains any keyword from a skill's description, that skill matches

For example, a skill with description `"Use when users want to generate curl commands"` would match prompts like:
- "create a curl command"
- "generate curl for my api"
- "curl example needed"

## Built-in Skills

Currently shelly ships with example skills that you can copy and modify:
- `curl-command-generator` - Expert at generating curl commands

## Creating Custom Skills

1. Think about a specific task you do often (e.g., "database backups", "docker compose", "kubernetes").

2. Write detailed instructions in your SKILL.md:
   - What the AI should do
   - Common patterns and examples
   - Options and flags to consider
   - Edge cases and safety checks

3. Test your skill with various prompts to ensure it matches correctly.

4. Refine the description to match the keywords you actually use.

## Tips

- **Keep descriptions focused**: A skill for "docker commands" shouldn't also try to handle "kubernetes"
- **Be specific in instructions**: The AI can only follow what you write
- **Include examples**: Real command examples help the AI understand the pattern
- **One skill per task**: Don't try to make mega-skills that handle everything

## Future Enhancements

Planned skills system improvements:
- Manual skill selection via `--skill <name>` flag
- `shelly skill add <url>` command for easy installation
- Automatic skill discovery and installation
- Multiple skills active in one session

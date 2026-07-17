# Skill Registry

Source of truth for how Prannay's skills are organized. Four categories.
Each SKILL.md carries a matching `category:` frontmatter key. Update this
file in the same turn a skill is added, renamed, or archived.

## Taste — what good looks like

| Skill | Purpose | Hardened |
|---|---|---|
| good-writing | Clear persuasive prose, Scott Adams method | 2026-06-09 |
| good-code | Minimal scannable code, no comments, vertical declarations | 2026-06-09 |
| emil-design-eng | UI polish, animation, design-engineering taste | 2026-06-10 |
| good-product | Product-level UX taste: performance-as-UX, minimal chrome, restrained visual system, fast paths, content-fidelity craft, plain copy | new 2026-07-17 |

Planned: blog-writing, twitter-writing (voice from ~/Agent/soul/SOUL.md).

## Context — compress and align understanding

| Skill | Purpose | Hardened |
|---|---|---|
| example | One concrete diagram-first thought experiment instead of abstract explanation | 2026-06-09 |
| plannotator | Browser UI for annotating docs, reviewing diffs, marking up last reply | new 2026-06-10 |
| handoff | Compact conversation into a doc for the next session | 2026-06-10 |
| teach | Verify deep understanding incrementally | 2026-06-10 |

Planned: learn, memory.

## Tools — invocable capabilities

| Skill | Purpose | Hardened |
|---|---|---|
| notify | Push notification to phone via notify CLI | 2026-06-10 |
| web | Drive a real browser via web-agent CLI | 2026-06-10 |
| latex (command) | Compile, render, visually audit, fix LaTeX | command, not hardened |
| gdrive (command) | Google Drive via rclone, CSV to native Sheet | command, not hardened |

Planned: transcribe, gcal-gmail.

## System — agent infrastructure and process disciplines

| Skill | Purpose | Status |
|---|---|---|
| test-agents | Spawn N subagents to test a skill, analyze traces | on, hardened 2026-06-10 |
| diagnose | Disciplined bug/perf diagnosis loop | on, hardened 2026-06-10 |
| personal-database | SQLite store at ~/System/personal-db (decisions, transactions, notes, ingest log) | on, new 2026-06-10 |
| experiment | Disciplined ML/data run tracking | on, re-enabled 2026-06-10 |
| agent-builder | Build/review agents and tools, grounded in claude-code repo | OFF 2026-06-10 |
| tdd | Red-green-refactor development | OFF 2026-06-10 |
| grill-with-docs | Stress-test plans against domain docs | OFF 2026-06-10 |
| ingestion-points | Sweep inboxes into zones, log to the DB | OFF 2026-06-10 |
| financial-analysis | Statements into the DB, burn/runway/budget answers | OFF 2026-06-10 |

OFF = not used daily, moved to `~/.claude/skills-off/<name>/` so the
description stops costing context every session. Re-enable:
`mv ~/.claude/skills-off/<name> ~/.claude/skills/`.

## Conventions (learned from the hardening campaign)

- Description = the trigger. Only name+description are always in context;
  put explicit trigger phrases there, keep it under ~40 words.
- Examples inside a skill must not contradict its rules.
- Style skills: never change behavior/meaning silently; preserve or flag.
- No em dashes in skill prose.
- Proactive (description-only) triggering is unreliable; explicit phrases fire.
- New/changed skills get the test loop: 3-4 parallel subagents (explicit
  trigger, natural trigger, boundary trap, control non-fire), verify
  Skill() calls in trace JSONLs, fix, retest.

# claude-skills

A collection of custom skills for [Claude Code](https://claude.ai/code). Drop any skill folder into `~/.claude/skills/` and Claude will pick it up automatically.

See [SKILLS.md](./SKILLS.md) for the full registry, categories, and conventions.

## Skills

### Taste — what good looks like

| Skill | What it does |
|---|---|
| [good-writing](./good-writing/SKILL.md) | Clear persuasive prose, Scott Adams method |
| [good-code](./good-code/SKILL.md) | Minimal scannable code, no comments, vertical declarations |

### Context — compress and align understanding

| Skill | What it does |
|---|---|
| [example](./example/SKILL.md) | One concrete diagram-first thought experiment instead of abstract explanation |
| [plannotator](./plannotator/SKILL.md) | Browser UI for annotating docs, reviewing diffs, marking up the last reply |
| [handoff](./handoff/SKILL.md) | Compact a conversation into a doc for the next session (rebuild `gate` from `gate.rs`: `rustc -O gate.rs -o gate`) |
| [teach](./teach/SKILL.md) | Verify deep understanding incrementally |

### Tools — invocable capabilities

| Skill | What it does |
|---|---|
| [notify](./notify/SKILL.md) | Push notification to phone via the `notify` CLI |
| [web](./web/SKILL.md) | Drive a real browser via the [claude-web](https://github.com/agi-inc/claude-web) agent CLI |
| [daytona-sandbox](./daytona-sandbox/SKILL.md) | Launch the current project as a live server on Daytona cloud and return a public preview URL |
| [huashu-design](./huashu-design/SKILL.md) | HTML-native design skill for hi-fi prototypes, demos, slide decks, and motion animations |

### System — agent infrastructure and process disciplines

| Skill | What it does |
|---|---|
| [test-agents](./test-agents/SKILL.md) | Spawn N subagents to test a skill, analyze traces |
| [diagnose](./diagnose/SKILL.md) | Disciplined bug/perf diagnosis loop |
| [experiment](./experiment/skill.md) | Disciplined ML/data run tracking |
| [plannotator-compound](./plannotator-compound/SKILL.md) | Analyses your denied plan archive and produces an HTML dashboard of prompt improvements |

### Off rotation

Skills not used daily live in [`skills-off/`](./skills-off/) so their descriptions don't cost context every session: `agent-builder`, `tdd`, `grill-with-docs`, `ingestion-points`, `financial-analysis`. Re-enable by copying into `~/.claude/skills/`.

## Install

```bash
# copy the skills you want
cp -r good-writing ~/.claude/skills/

# or clone and symlink
git clone https://github.com/Pran-Ker/claude-skills ~/Developer/claude-skills
ln -s ~/Developer/claude-skills/good-writing ~/.claude/skills/good-writing
```

## Setup notes

**experiment** — Edit `skill.md` to set your HuggingFace username and W&B entity before use. The skill expects credentials at `~/.local/secrets` with `HUGGING_FACE_TOKEN` and `WANDB_API_KEY` exported.

**web** — Requires the [claude-web](https://github.com/agi-inc/claude-web) project set up locally. Follow the install instructions there.

**notify** — Requires the `notify` CLI at `~/Tools/notify/notify`.

**handoff** — Ships `gate.rs` (Rust source); build the `gate` binary locally with `rustc -O gate.rs -o gate`.

**huashu-design** — Includes reference files and starter components under `assets/` and `references/`. Keep the full folder intact when installing.

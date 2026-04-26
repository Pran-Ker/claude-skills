# claude-skills

A collection of custom skills for [Claude Code](https://claude.ai/code). Drop any skill folder into `~/.claude/skills/` and Claude will pick it up automatically.

## Skills

| Skill | What it does | Trigger |
|---|---|---|
| [daytona-sandbox](./daytona-sandbox/SKILL.md) | Launch the current project as a live server on Daytona cloud and return a public preview URL | "launch in sandbox", "run on Daytona", "spin up a sandbox" |
| [experiment](./experiment/skill.md) | Enforces disciplined run tracking, cost visibility, and reproducibility before any ML/data/API job — generates run IDs, sets up W&B + HuggingFace logging, launches in tmux | "start an experiment", "run this eval", or any ML/data/API coding task |
| [plannotator-compound](./plannotator-compound/SKILL.md) | Analyses your denied plan archive, extracts feedback patterns, and produces a polished HTML dashboard with actionable prompt improvements | "analyse my plans", "compound planning report" |
| [huashu-design](./huashu-design/SKILL.md) | HTML-native design skill for hi-fi prototypes, interactive demos, slide decks, and motion animations — all as self-contained HTML files | "prototype", "hi-fi design", "UI mockup", "design exploration", "animation demo" |
| [web](https://github.com/agi-inc/claude-web) | Browser automation over Chrome DevTools Protocol — navigate, click, fill forms, take screenshots | "open", "browse", "click", "search the web" |

## Install

```bash
# clone into your Claude skills directory
git clone https://github.com/prannay/claude-skills ~/.claude/skills/claude-skills

# then symlink individual skills you want
ln -s ~/.claude/skills/claude-skills/daytona-sandbox ~/.claude/skills/daytona-sandbox
ln -s ~/.claude/skills/claude-skills/experiment ~/.claude/skills/experiment
```

Or just copy the skill folder directly:

```bash
cp -r daytona-sandbox ~/.claude/skills/
```

## Setup notes

**experiment** — Edit `skill.md` to set your HuggingFace username and W&B entity before use. The skill expects credentials at `~/.local/secrets` with `HUGGING_FACE_TOKEN` and `WANDB_API_KEY` exported.

**web** — Requires the [claude-web](https://github.com/agi-inc/claude-web) project set up locally. Follow the install instructions there.

**huashu-design** — Includes reference files and starter components under `assets/` and `references/`. Keep the full folder intact when installing.

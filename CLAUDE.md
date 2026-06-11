# claude-skills
Personal collection of Claude Code skills (markdown SKILL.md definitions plus assets) published to github.com/Pran-Ker for reuse across machines. Mirrors `~/.claude/skills/` (active) and `~/.claude/skills-off/` (off rotation).
**Status**: active
## Run
- install: `cp -r <skill-dir> ~/.claude/skills/` (skills are picked up automatically; nothing executes here directly)
- sync from local: `rsync -aL --delete ~/.claude/skills/<name>/ ./<name>/` per skill; `SKILLS.md` is copied verbatim from `~/.claude/skills/SKILLS.md`
## Layout
- one directory per active skill (most are a single SKILL.md; `handoff/` adds `gate.rs`, `huashu-design/` and `diagnose/` carry scripts/references)
- `skills-off/`: disabled skills kept for reuse, not installed by default
- `SKILLS.md`: skill registry — categories, hardening dates, conventions; source of truth for organization
- `README.md`: public skill table and install notes; each skill is self-contained, no shared code or build step
## Publishing rules
- repo is PUBLIC: no compiled binaries (handoff `gate` is excluded; rebuild from `gate.rs`), no secrets, no paid-course-derived content (`emil-design-eng` stays local-only)

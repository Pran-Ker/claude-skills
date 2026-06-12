# Restructuring TODO — agent systems for the next model generation

Remaining work migrated from the `restructuring` planning folder (2026-06-12).
Process per subsystem: question rounds → synthesize → build plan → `/handoff`. One at a time.

## Deferred from subsystem 1 (Verification — handed off 2026-06-09)

- [ ] Rework the `/experiment` skill into per-type sub-skills (finetuning/RL, prompt/agent evals, data/API jobs) — NOTE: the copy at `~/.claude/skills/experiment/skill.md` is outdated; locate the current version first
- [ ] Taste files modeled on `/good-writing` — data-presentation domain first (then code, design)
- [ ] Golden-output comparison harness tied to `/example`
- [ ] On-demand adversarial refuter agent (invoked when asked, not run by default)

## 2. Context & memory systems — NEXT

- [ ] Personal data warehouse — emails, notes, calendar, transactions, reading history in one queryable place (SQLite fine)
- [ ] Decision log — like Zlog but for decisions: why X over Y
- [ ] Auto-maintained project maps — CLAUDE.md regenerated via cron as code changes
- [ ] SOUL.md pattern generalized — voice file, taste file, relationship file

## 3. Skills library as compounding asset

Discipline: the third time you do something manually, it becomes a skill file.

- [ ] scrape-and-enrich (generalize researcher_details)
- [ ] paper → reproduction → writeup
- [ ] new-project init (scaffold + CLAUDE.md + eval harness + CI)
- [ ] outreach (research → draft in voice → approval queue)
- [ ] weekly review (logs/commits/metrics → summary)

## 4. Autonomous operation

- [ ] Work queue — TODO board agents drain via cron overnight, morning report
- [ ] Monitors — competitive intel, arXiv/Twitter topics, dead links, API changes
- [ ] Triage agent — inbox/GitHub notifications → classified, drafted responses, escalations only

## 5. Startup-specific systems

- [ ] Customer signal pipeline — tickets/transcripts/churn → structured insights → ranked features
- [ ] Distribution automation — one diff → changelog → blog → tweets → email, in voice
- [ ] Self-maintaining CRM — interactions logged by agents, follow-ups auto-drafted (researcher_details is 70% of this)
- [ ] Instrumentation from day one — analytics agents can query

## 6. Human skills (discussion, not a build)

- [ ] Specification writing — precise statements of intent
- [ ] Evaluation and taste — knowing good from plausible
- [ ] System design over implementation — which feedback loops to build, where the human checkpoint goes
- [ ] Distribution and relationships

## Sequencing

(1) verification harnesses [done] → (2) personal data warehouse → (3) overnight work queue.
Everything else slots in as need arises.

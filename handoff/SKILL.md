---
name: handoff
category: context
description: Compact the current conversation into a handoff document for another agent to pick up. Use when the user says /handoff, asks to hand off or transfer the work, or wants to continue this work in a fresh session.
argument-hint: "What will the next session be used for?"
---

Write a handoff document summarising the current conversation so a fresh agent can continue the work.

## Output format

Structure the entire document as the following XML-tagged sections, in this order. Omit a tag, or leave it empty, whenever it isn't genuinely needed: empty is fine and preferred over filler; never invent content to fill one.

Each section tag must sit alone on its own line, flush to the left margin (no indentation). The gate ignores any tag that is indented or inside a ``` fenced code block, so sample handoffs quoted inside `<examples>` will not trip it.

```
<task_context>
Who the next agent is and the overall objective. First line names the working directory (basename of `pwd`, e.g. `Handoff: <folder-name>`) followed by the full absolute path on the next line.
</task_context>

<tone_context>
How the next agent should communicate and any standing user preferences worth carrying over.
</tone_context>

<background>
Reference existing artifacts (PRDs, plans, ADRs, issues, commits, diffs) by path or URL. Do NOT duplicate their content here.
</background>

<task_description>
Detailed description of the work to continue, plus rules and constraints that must hold.
</task_description>

<examples>
Relevant code patterns, prior outputs, or worked examples the next agent should mirror.
</examples>

<conversation_history>
A compact summary of what has happened in this conversation so far — decisions made, things tried, dead ends.
</conversation_history>

<immediate_task>
The specific next step the agent should take first.
</immediate_task>

<thinking>
Instruct the agent to reason step by step before acting.
</thinking>

<output_format>
The expected shape of the next agent's deliverable.
</output_format>

<prefill>
Optional opening of the next agent's response. Leave empty if not useful.
</prefill>
```

## Delivery

Do not save the document to a file. Hold the full markdown in a shell variable (`DOC`), then:

1. **Validate the structure** before anything else:
   ```
   printf '%s' "$DOC" | ~/.claude/skills/handoff/gate
   ```
   - `OK` (exit 0) → continue.
   - `WRONG: <reason> (line N)` (exit 1) → fix that exact location in `$DOC` and re-run the gate. Repeat until it prints `OK`. Do not copy a document that has not passed.
2. Pipe to the clipboard: `printf '%s' "$DOC" | pbcopy`.
3. Run the alias `p` (`open https://prompt-builder-mauve.vercel.app`) to open the prompt builder.
4. Confirm to the user that it has been copied and the builder opened.

The gate is a deterministic syntax check (compiled Rust binary, ~0.6 ms of work; source in `gate.rs`, rebuild with `rustc -C opt-level=3 -C panic=abort -C codegen-units=1 -C strip=symbols gate.rs -o gate`). It confirms the section tags are recognised, ordered, non-overlapping, and each one closed. It reports the first break and its line so you can fix and retry. It does not check content — that stays your responsibility.

Suggest the skills to be used, if any, by the next session (inside `<task_description>`).

If the user passed arguments, treat them as a description of what the next session will focus on and tailor the doc accordingly.

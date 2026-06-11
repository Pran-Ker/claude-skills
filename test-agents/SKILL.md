---
name: test-agents
category: system
description: Spawn N subagents to test a skill, prompt, or agent behavior, then analyze their on-disk traces for failures, divergence, and weaknesses. Argument is free-form — a task list, a single prompt to run N times, or a meta-instruction like "test the /web skill on 5 varied tasks" (in which case you generate the tasks yourself).
argument-hint: <what to test>
---

# Test Agents

## Workflow

1. Parse the argument into N concrete tasks. If it's a meta-instruction ("test X on 5 varied tasks"), generate the tasks yourself — vary inputs, edge cases, and phrasing.
2. Spawn all N subagents in parallel in one message, one task each. Append to every prompt: "Follow instructions exactly. If anything fails, report the failing command and error verbatim — do not work around silently." Without this the trace loses signal.
3. When they finish, locate each trace (below) and analyze for: outright failures, divergence between agents on equivalent tasks, ignored instructions, and weaknesses in the skill/prompt under test.
4. Report findings ranked by severity, each backed by trace evidence.

## Where traces live (discover, don't assume)

```
find ~/.claude/projects -name "agent-<id>.jsonl" -mmin -60
```

If absent after 1s, retry once; if still absent, note "no trace" and continue.

## Record shape (one JSON object per line)

- `tool_use`: `{type, name, input, id}`
- `tool_result`: `{type, tool_use_id, is_error, content}` — content can be huge
- `assistant` / `user`: `{type, content:[{type,text}]}`

## Non-obvious moves

- Never `cat` a JSONL. Project via `jq` and slice strings: `(.content|tostring|.[0:400])`.
- Pair calls to results by `tool_use.id` ↔ `tool_result.tool_use_id` — they are not adjacent lines.
- Sequence-hash to group trace shapes: `jq -r 'select(.type=="tool_use")|.name' FILE | md5sum`. Identical hashes across *varied* tasks means the agent ignored the variation — itself a finding.

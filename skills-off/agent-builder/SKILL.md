---
name: agent-builder
category: system
description: Best practices for building and reviewing agents and tools in the Claude Code codebase, grounded in its actual source with file:line citations. Use when creating or reviewing a buildTool() tool, an AgentDefinition or subagent, system-prompt sections, prompt-cache discipline, permission modes, MCP integration, error handling, or agent/tool tests. Covers whenToUse design, model selection (haiku/sonnet/opus/inherit), Zod v4 schemas, maxResultSizeChars and disk offload, isConcurrencySafe/isReadOnly/isDestructive, prompt() vs description(), systemPromptSection memoization, sticky-latch beta headers, cache TTL selection, withRetry, and errors-as-data.
---

# Agent-Builder

How to build and review agents and tools the way this codebase actually does it. Every rule traces to real source code.

## Grounding

This skill distills `docs/agent-best-practices.md` from the user's local Claude Code repo. All `file:line` citations are **absolute paths** into that repo:

```
REPO = /Users/pran-ker/Developer/claude-code
```

When a citation reads `src/Tool.ts:461`, the resolvable path is `/Users/pran-ker/Developer/claude-code/src/Tool.ts:461`. The authoritative reference is `/Users/pran-ker/Developer/claude-code/docs/agent-best-practices.md` — read it when a topic here points you deeper.

## When to use this skill

Use it when you are about to:

- Add or change a tool built with `buildTool()` (`/Users/pran-ker/Developer/claude-code/src/Tool.ts`).
- Define a new agent or subagent (`AgentDefinition` in `/Users/pran-ker/Developer/claude-code/src/tools/AgentTool/loadAgentsDir.ts`).
- Write or edit system-prompt content, especially anything that could break the prompt cache.
- Set permission modes, MCP wiring, error handling, output sizing, or tests for an agent/tool.
- Review someone else's tool or agent for adherence to these conventions.

If the task is none of these, this skill probably does not apply.

## How to use it

1. Skim the relevant checklist below.
2. Open the matching reference file for the field-by-field detail and citations.
3. Verify against the real source before claiming a pattern — files move; the citation tells you where to look, not what is guaranteed to still be there.

References (one level deep):

- `reference/tools.md` — the `buildTool` factory, schemas, `prompt()` vs `description()`, result sizing, concurrency/permission predicates, deferral.
- `reference/agents.md` — `AgentDefinition` fields, `whenToUse`, model heuristics, permission modes, fork vs typed subagents, agent memory, MCP.
- `reference/prompt-caching.md` — the static/dynamic boundary, `systemPromptSection` memoization, sticky-latch beta headers, TTL selection, cache-break detection.
- `reference/errors-and-testing.md` — errors-as-data, `withRetry`, smoke/integration/prompt-cache tests.

---

## Tool checklist (`buildTool`)

Detail and citations: `reference/tools.md`.

- [ ] `name` is PascalCase and unique across all tools.
- [ ] `inputSchema` uses **Zod v4** (`import { z } from 'zod/v4'`) — not `zod` or `zod/v3`.
- [ ] `.describe()` on **every** field. These strings go to the model in the tool schema; write them for the model, not for developers.
- [ ] Prefer required fields; use `.optional()` sparingly and `.default()` where intent is obvious.
- [ ] `prompt()` is written for the model — explicit, concrete, directive. It becomes the tool's API `description` and is cached session-stable (changing it mid-session has no effect until next session). Do not confuse it with `description()`, which is dynamic, per-invocation, UI-only.
- [ ] `isConcurrencySafe(input)` returns true only if the call is safe to run in parallel (reads yes, writes no).
- [ ] `isReadOnly(input)` is accurate — drives permission and read-only-constraint checks.
- [ ] `isDestructive(input)` is set true for irreversible operations.
- [ ] `maxResultSizeChars` is set appropriately; large results offload to disk and return a path, not inline bytes.
- [ ] Abort signal (`context.abortController.signal.aborted`) is checked in long loops.
- [ ] Errors are returned as data, not thrown (see error checklist).

## Agent checklist (`AgentDefinition`)

Detail and citations: `reference/agents.md`.

- [ ] `agentType` is unique and PascalCase.
- [ ] `whenToUse` has **concrete trigger conditions** — this is the single most important field; the orchestrator reads it verbatim to decide whether to spawn the agent. No generic "general purpose agent for various tasks."
- [ ] `model` matches task cost/complexity: `haiku` (fast read-only), `sonnet` (default, balanced), `opus` (hard reasoning), `inherit` (must share the parent's prompt cache, e.g. fork subagents).
- [ ] `disallowedTools` blocks write tools (`FileEdit`, `FileWrite`) for read-only agents, and blocks `AgentTool` to prevent unintended recursive spawning.
- [ ] `omitClaudeMd: true` for read-only agents that don't need commit/PR/lint conventions.
- [ ] `permissionMode: 'bubble'` for most subagents — the human still approves dangerous operations, via the parent.
- [ ] `getSystemPrompt()` opens with the agent's own role identity (e.g. "You are a file search specialist…"); constraints are explicit and early (see the Explore agent's READ-ONLY banner). Sub-agents write their identity directly; `getCLISyspromptPrefix()` is for the main CLI prompt, not sub-agents.
- [ ] `getSystemPrompt` receives only `Pick<ToolUseContext, 'options'>` (not the full context), and the param is optional.

## Cache-discipline rules

Detail and citations: `reference/prompt-caching.md`. The prompt cache is the highest-leverage, most codebase-specific concern here.

- [ ] **Static above the boundary, dynamic below.** Identity, capabilities, tool prompts, CLAUDE.md go above `SYSTEM_PROMPT_DYNAMIC_BOUNDARY`; git status, current file, time go below.
- [ ] **Memoize static sections** with `systemPromptSection(name, computeFn)`. Reach for `DANGEROUS_uncachedSystemPromptSection(name, fn, reason)` only when a value genuinely changes every turn, and always supply the reason.
- [ ] **Latch beta headers sticky-on.** A boolean feature flag that toggles mid-session breaks the cache. Latch on first activation and never flip back (the AFK-mode pattern).
- [ ] **Keep the tool set and tool prompts stable** across turns. Adding/removing a tool or changing a tool `description` breaks the cache.
- [ ] **Don't swap models mid-session** — it breaks the cache.
- [ ] **For fork subagents, reuse `context.renderedSystemPrompt`** instead of re-calling `getSystemPrompt()`, so the child's API prefix is byte-identical to the parent's.
- [ ] Pick TTL deliberately: `5m` default; `1h` for long interactive sessions when eligible.

---

## Skill-authoring note

This skill obeys the rules it implies of any skill: lowercase-hyphen `name` with no reserved words, a third-person `description` naming what + when + trigger terms, a body under 500 lines, detail pushed to reference files exactly one level deep, forward slashes, and one default per choice with an escape hatch. If you extend it, keep citations pointing at real source and re-verify they resolve.

# Prompt Caching

> The highest-leverage, most Claude-Code-specific concern in agent/tool design. A broken cache silently multiplies cost and latency. Citations are absolute paths into the local repo (`REPO = /Users/pran-ker/Developer/claude-code`).

## Contents

- [The static/dynamic boundary](#the-staticdynamic-boundary)
- [What goes where](#what-goes-where)
- [Memoized prompt sections](#memoized-prompt-sections)
- [What breaks the cache](#what-breaks-the-cache)
- [The sticky-latch pattern](#the-sticky-latch-pattern)
- [Tool schema caching](#tool-schema-caching)
- [cache_control placement](#cache_control-placement)
- [TTL selection](#ttl-selection)
- [Cache-break detection](#cache-break-detection)
- [Fork subagents: byte-identical prefixes](#fork-subagents-byte-identical-prefixes)

## The static/dynamic boundary

The API layer (`splitSysPromptPrefix`, `src/utils/api.ts:321-435`) splits the system prompt into up to four blocks for caching:

```
┌─────────────────────────────────────────────────┐
│  Attribution header          (cacheScope=null)    │ ← never changes
│  Sysprompt prefix            (cacheScope=null*)    │ ← changes only on model swap
│  Static body                 (cacheScope='global') │ ← CLAUDE.md, tool prompts
│  --- SYSTEM_PROMPT_DYNAMIC_BOUNDARY ---            │
│  Dynamic tail                (cacheScope=null)      │ ← git status, current file
└─────────────────────────────────────────────────┘
```

\*In the global-cache (boundary) path the prefix is pushed with `cacheScope: null` (`api.ts:391`); the `'org'` scope applies only in the non-boundary fallback paths (`api.ts:353,357,431,433`). Attribution=`null`, static body=`'global'`, dynamic tail=`null` (`api.ts:388-396`).

**Rule:** everything above the boundary is eligible for the long (1h) cache TTL; everything below is recomputed each turn (though many "tail" sections are themselves `systemPromptSection`-memoized). The boundary constant is `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` (`src/constants/prompts.ts:114`), emitted only when `shouldUseGlobalCacheScope()`.

## What goes where

| Content | Location | Why |
|---|---|---|
| Identity, persona, role | Top of static body | cached globally |
| Capability / tool instructions | Static body | rarely changes |
| Project conventions (CLAUDE.md) | Static body | changes on `/clear` only |
| Current file, selection | Dynamic tail | changes every turn |
| Git status | Dynamic tail | changes every turn |
| Time, date | Dynamic tail — only if needed | changes every turn |

## Memoized prompt sections

Expensive static sections must be memoized so they don't recompute every turn and don't churn the cached bytes (`src/constants/systemPromptSections.ts`):

```typescript
import {
  systemPromptSection,
  DANGEROUS_uncachedSystemPromptSection,
  resolveSystemPromptSections,
} from '../constants/systemPromptSections.js'

// ✅ stable — computed once per session, cached until /clear
const claudeMdSection = systemPromptSection('claude_md', () => loadClaudeMd())

// ⚠️ cache-breaking — only when the value genuinely changes every turn; always give a reason
const gitStatusSection = DANGEROUS_uncachedSystemPromptSection(
  'git_status',
  () => getGitStatus(),
  'Git status is real-time and must reflect the current working tree state',
)

const sections = await resolveSystemPromptSections([claudeMdSection, gitStatusSection])
```

The memoization cache is reset on `/clear` and `/compact` (`src/constants/systemPromptSections.ts:65-68`). Default to `systemPromptSection`; reach for `DANGEROUS_uncachedSystemPromptSection` only when you can articulate why the value must change every turn.

## What breaks the cache

`src/services/api/promptCacheBreakDetection.ts`:

| Change | Breaks cache? | Fix |
|---|---|---|
| System prompt text | Yes | use `systemPromptSection` memoization |
| Tool added/removed | Yes | keep the tool set stable across turns |
| Tool `description` changed | Yes | cache base schema per session (below) |
| Model changed | Yes | avoid mid-session model swaps |
| Beta headers changed | Yes | latch betas on first call; never toggle |
| `cache_control` scope/TTL | Yes | use `shouldUseGlobalCacheScope()` consistently |
| Fast mode toggled | Yes | keep stable per session |
| AFK mode header | No | already latched sticky-on |
| Overage state | No | already latched |
| TTL expiry (5m / 1h) | N/A | server-side; not preventable |

## The sticky-latch pattern

Prevents false cache breaks from boolean flags that could flip mid-session. Latch ON at first activation and never flip back (`src/services/api/claude.ts`):

```typescript
// activation gate (claude.ts:1414-1419): feature('TRANSCRIPT_CLASSIFIER')
//   && isAgenticQuery && shouldIncludeFirstPartyOnlyBetas() && isAutoModeActive()
if (!getAfkModeHeaderLatched() && afkModeShouldActivate) {
  setAfkModeHeaderLatched(true)        // claude.ts:1421 — latch ON, never flip back
}
if (getAfkModeHeaderLatched() && isAgenticQuery) {
  betas.push(AFK_MODE_BETA_HEADER)     // claude.ts:1666-1668 — push is also per-call gated
}
```

`getAfkModeHeaderLatched` / `setAfkModeHeaderLatched` are real (defined in `src/bootstrap/state.ts`, imported and used in `src/services/api/claude.ts`); the latch clears only on `/clear` + `/compact`. There is **no** `shouldAddAfkModeHeader()` helper — that name appears only in the repo's prose doc, not in code; the real activation is the gate shown above. Apply this latch shape to any feature whose beta header could otherwise toggle mid-session.

## Tool schema caching

Tool schemas are cached per session in `toolSchemaCache` so GrowthBook flag flips or `prompt()` drift don't churn the serialized tool-array bytes (`src/utils/api.ts:139-209`):

```typescript
const cacheKey = ('inputJSONSchema' in tool && tool.inputJSONSchema)
  ? `${tool.name}:${jsonStringify(tool.inputJSONSchema)}`
  : tool.name

let base = cache.get(cacheKey)
if (!base) {
  base = {
    name: tool.name,
    description: await tool.prompt({ /* ... */ }),  // expensive; memoized
    input_schema: zodToJsonSchema(tool.inputSchema),
  }
  cache.set(cacheKey, base)
}
```

**Implication:** changing `tool.prompt()` mid-session won't take effect until the next session. Design tool prompts to be session-static.

## cache_control placement

`cache_control` and `defer_loading` are added as a **per-request overlay** on top of the cached base, never mutating it (`src/utils/api.ts:215-229`):

```typescript
const schema = { name: base.name, description: base.description, input_schema: base.input_schema }
if (options.deferLoading) schema.defer_loading = true
if (options.cacheControl) schema.cache_control = options.cacheControl
```

For system-prompt blocks, cache scope is decided by `splitSysPromptPrefix()` (`src/utils/api.ts:321-435`) based on whether MCP tools are present, the API provider, and the global-cache feature flag.

## TTL selection

- **`5m` TTL** — default for org-scope; fine when turn gaps stay under 5 minutes.
- **`1h` TTL** — when `getPromptCache1hEligible()` is true (first-party Anthropic API + account eligibility). Use for long interactive sessions.

```typescript
// should1hCacheTTL(querySource?) reads model / overage / eligibility from bootstrap state
const ttl = should1hCacheTTL(querySource) ? '1h' : '5m'   // claude.ts:393, applied at :371
```

## Cache-break detection

Two phases (`src/services/api/promptCacheBreakDetection.ts`):

- **Phase 1 (pre-call)** — `recordPromptState(snapshot)` hashes system prompt, tools, model, betas, effort level, and extra body params; records what changed since the last call.
- **Phase 2 (post-call)** — `checkResponseForCacheBreak(...)` compares cache-read tokens. If the read dropped >5% **and** by ≥2,000 tokens (`MIN_CACHE_MISS_TOKENS`), it fires `tengu_prompt_cache_break` analytics and writes a `cache-break-*.diff` file for debugging.

When investigating a regression, that diff file tells you exactly which block changed.

## Fork subagents: byte-identical prefixes

For prompt-cache sharing across fork children, every child must produce a byte-identical API prefix (the constraint is documented at `src/tools/AgentTool/forkSubagent.ts:45-93`; the assignment itself happens in `AgentTool.tsx:496-497` and `resumeAgent.ts:118-119`):

```typescript
// ✅ byte-exact — reuse the parent's already-rendered bytes
override.systemPrompt = toolUseContext.renderedSystemPrompt

// ❌ can diverge — GrowthBook may have warmed up since the parent's call
override.systemPrompt = await getSystemPrompt(...)
```

Then: `FORK_AGENT` sets `tools: ['*']` and `model: 'inherit'` (`forkSubagent.ts:64,66`), `useExactTools: true` inherits the parent's exact tool pool, and `buildForkedMessages` (`forkSubagent.ts:107-168`) collapses all parent tool-result blocks to the placeholder `FORK_PLACEHOLDER_RESULT = 'Fork started — processing in background'` (`forkSubagent.ts:93`). Together these let children read from the parent's cache instead of re-creating it.

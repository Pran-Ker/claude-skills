# Tool Construction

> Field-by-field reference for building tools with `buildTool`. Citations are absolute paths into the local repo (`REPO = /Users/pran-ker/Developer/claude-code`).

## Contents

- [The buildTool factory](#the-buildtool-factory)
- [Input schema rules](#input-schema-rules)
- [prompt() vs description()](#prompt-vs-description)
- [maxResultSizeChars and disk offload](#maxresultsizechars-and-disk-offload)
- [Concurrency, read-only, destructive](#concurrency-read-only-destructive)
- [Defaults, required fields, and the call() result](#defaults-required-fields-and-the-call-result)
- [Deferral: shouldDefer and alwaysLoad](#deferral-shoulddefer-and-alwaysload)
- [strict mode](#strict-mode)
- [Interrupt behavior](#interrupt-behavior)
- [Context: ToolUseContext](#context-toolusecontext)
- [Errors as data](#errors-as-data)

## The buildTool factory

All tools are created via `buildTool` (`src/Tool.ts:783`). Skeleton:

```typescript
import { buildTool } from '../../Tool.js'
import { z } from 'zod/v4'

export const MyTool = buildTool({
  // ── Identity ──
  name: 'MyTool',               // PascalCase; unique across all tools
  aliases: ['my_tool'],         // backwards-compat aliases only
  searchHint: 'search keyword', // 3-10 words for ToolSearch matching (not in name)

  // ── Schema (Zod v4) ──
  inputSchema: z.object({
    target: z.string().describe('Path to the target file'),
    mode: z.enum(['read', 'write']).default('read'),
    timeout: z.number().int().positive().optional().describe('Timeout in ms'),
  }),

  // ── Execution ──
  async call(args, context, canUseTool, parentMessage, onProgress) {
    // context: ToolUseContext. canUseTool gates sub-operations.
    // onProgress emits ToolProgressData for streaming UI.
    return { data: result, newMessages: [], contextModifier: undefined }
  },

  // ── System-prompt injection (once per session; cached) ──
  async prompt({ getToolPermissionContext, tools, agents }) {
    return `Use MyTool to perform X. Provide target as an absolute path.`
  },

  // ── Permission predicates ──
  isConcurrencySafe(input) { return input.mode === 'read' },
  isReadOnly(input) { return input.mode === 'read' },
  isDestructive(input) { return input.mode === 'write' },

  // ── Display config ──
  maxResultSizeChars: 100_000,  // larger results persist to disk

  // ── UI ──
  renderToolUseMessage(input, options) { /* JSX */ },
  renderToolResultMessage(content, progressMessages, options) { /* JSX */ },
})
```

The catalog of existing tools and per-tool directory layout (`MyTool.ts`, `UI.tsx`, `prompt.ts`, `utils.ts`) lives in `src/tools/` and is documented at `docs/tools.md`. Reference implementation to copy patterns from: `src/tools/BashTool/BashTool.tsx`.

## Input schema rules

1. **Use Zod v4** (`import { z } from 'zod/v4'`) — not `zod` or `zod/v3`. The codebase is fully on v4.
2. **`.describe()` every field.** These descriptions become part of the API tool schema; the model reads them. Write them for the model.
3. **Prefer required fields.** Required fields get better model adherence than optional ones.
4. **Avoid unions for simple booleans** — `z.boolean()` beats `z.enum(['true','false'])`.
5. **Add `.default()`** for fields with a sensible default so the model can omit them when intent is obvious.
6. For cross-field constraints Zod can't express, implement `validateInput()` returning `ValidationResult` (`src/Tool.ts:95-101`): `{ result: true }` or `{ result: false; message: string; errorCode: number }`.

## prompt() vs description()

These are different things — do not conflate them:

| Field | Called | Goes to | Purpose |
|---|---|---|---|
| `prompt()` | Once per session | API `description` in the tool schema | **Instructs the model** how to use the tool. Cached. |
| `description()` | On each tool use | UI display only | **Explains the current invocation** to the human. Dynamic. |

The `prompt()` return value is the most important piece of tool design: it is sent to the model as the tool's `description` field on every request, cached session-stable (see `reference/prompt-caching.md` and the schema cache at `src/utils/api.ts:139-209`), and is the mechanism by which you shape model behavior. **Write it for the model — explicit, concrete, directive.** Because it is cached per session, changing it mid-session has no effect until the next session; keep it session-static (no per-turn dynamic content).

## maxResultSizeChars and disk offload

Controls when a result is persisted to disk vs sent inline (field + doc at `src/Tool.ts:457-466`; persistence logic in `src/utils/toolResultStorage.ts:55-78` and `:137+`):

```typescript
maxResultSizeChars: 100_000     // declared cap — but capped at 50K, see note below
maxResultSizeChars: Infinity    // never persist (e.g. FileReadTool — avoids Read→file→Read loop)
maxResultSizeChars: 10_000      // very small output tools
```

**Effective inline size is `min(declared, DEFAULT_MAX_RESULT_SIZE_CHARS)`**, where `DEFAULT_MAX_RESULT_SIZE_CHARS = 50_000` (`src/constants/toolLimits.ts:13`, applied in `getPersistenceThreshold`, `toolResultStorage.ts:77`). So declaring `100_000` still persists at ~50 KB inline unless a per-tool GrowthBook override raises it; `Infinity` is a hard opt-out. When a result exceeds the threshold, the model receives a preview plus a temp-file path (`persistToolResult` returns `{ filepath, preview, ... }`) and can `Read` the path for full content. Large inline results crowd out conversation history and future tool outputs — set this deliberately. For UI summaries, truncate to `TOOL_SUMMARY_MAX_LENGTH` (`src/constants/toolLimits.ts:56`).

Emit progress for long operations so the UI stays responsive:

```typescript
onProgress?.({ toolUseID: context.toolUseId!, data: { type: 'bash_progress', output: 'Processing step 1...' } })
```

Progress types (`ToolProgressData`, `BashProgress`, …) are defined and re-exported from `src/Tool.ts:64-73`; there is no `src/types/tools.ts` source file — consumers import these from `../../Tool.js`.

## Concurrency, read-only, destructive

```typescript
isConcurrencySafe(input) { return true }  // safe to run in parallel? reads yes, writes no, shell depends on command
isReadOnly(input) { return true }         // modifies no state? drives permission + read-only-constraint checks
isDestructive(input) { return false }     // irreversible? set true for destructive ops
```

These predicates take the validated input, so they can vary by argument (e.g. a tool that both reads and writes returns different values per `mode`).

## Defaults, required fields, and the call() result

`call()` returns a `ToolResult<T>` (`src/Tool.ts:321-336`): `data` is **required**; `newMessages?` and `contextModifier?` are optional. `contextModifier` is honored only for non-concurrency-safe tools (`Tool.ts:329`). The minimal valid return is `{ data }`.

`buildTool` fills in **fail-closed** defaults via `TOOL_DEFAULTS` (`src/Tool.ts:757-769`): `isConcurrencySafe` and `isReadOnly` default to `false` (assume unsafe / writes), and `isDestructive` defaults to `false`. Omitting the predicates is therefore safe-by-default — the tool is treated as unsafe to parallelize and as a writer until you say otherwise.

To gate execution, provide `checkPermissions(input, context)` returning a `PermissionResult` — it runs *after* `validateInput` (`Tool.ts:489-492`, then `:500`). Security-relevant tools should also override `toAutoClassifierInput` (`Tool.ts:556`, default `''`); without it the tool is invisible to auto-mode permission classification (a fail-open gap).

## Deferral: shouldDefer and alwaysLoad

For large tool sets, tools can be hidden behind `ToolSearch` until fetched:

```typescript
readonly shouldDefer = true   // hidden until ToolSearch fetches it
readonly alwaysLoad = true    // always in the initial prompt (never deferred)
```

For MCP tools, set always-load via `_meta['anthropic/alwaysLoad']` in the server's tool definition.

## strict mode

```typescript
buildTool({ strict: true, /* ... */ })  // API enforces inputSchema strictly
```

Only activates when the `tengu_tool_pear` feature flag is on and the model supports it.

## Interrupt behavior

```typescript
interruptBehavior() {
  // 'cancel' — abort the tool if the user sends a new message while it runs
  // 'block'  — keep running; new message waits (default)
  return 'cancel'  // good for long searches the user may want to stop
}
```

## Context: ToolUseContext

Every `call()` receives `context: ToolUseContext` (`src/Tool.ts:158-300`). Key fields:

```typescript
context.options.tools          // all registered tools
context.options.mainLoopModel  // active model name
context.options.mcpClients     // connected MCP servers
context.abortController        // check .signal.aborted before long ops
context.messages               // full conversation history
context.readFileState          // LRU cache of file reads this session
context.getAppState() / setAppState(f)  // app state (setAppState is a no-op in async agents)
context.fileReadingLimits      // honor maxTokens / maxSizeBytes before reading large files
context.globLimits             // max results for glob operations
context.renderedSystemPrompt   // parent's rendered prompt bytes (for fork subagents — see prompt-caching.md)
```

Always check the abort signal in long loops:

```typescript
for (const chunk of bigList) {
  if (context.abortController.signal.aborted) return { data: 'Aborted by user' }
  await processChunk(chunk)
}
```

## Errors as data

Return errors as data so the model can recover; do not throw (`src/Tool.ts`, error categorization `src/services/api/errors.ts`):

```typescript
// ✅ model sees the error and can retry or adjust
return { data: { error: 'File not found', path: args.target } }

// ❌ unhandled exception breaks the tool-call loop
throw new Error('File not found')
```

More on error categorization and retries: `reference/errors-and-testing.md`.

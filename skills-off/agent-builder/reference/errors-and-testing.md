# Error Handling & Testing

> How this codebase handles failures and verifies agents/tools. Citations are absolute paths into the local repo (`REPO = /Users/pran-ker/Developer/claude-code`).

## Contents

- [Errors as data, not exceptions](#errors-as-data-not-exceptions)
- [withRetry — automatic API retries](#withretry--automatic-api-retries)
- [Error categorization](#error-categorization)
- [ValidationResult](#validationresult)
- [Smoke tests](#smoke-tests)
- [Integration tests](#integration-tests)
- [Testing prompt caching](#testing-prompt-caching)

## Errors as data, not exceptions

Return errors as data so the model can see them and recover. Throwing breaks the tool-call loop.

```typescript
// ✅ the model sees the error and can retry or adjust
return { data: { error: 'File not found', path: args.target } }

// ❌ unhandled exception breaks the tool-call loop
throw new Error('File not found')
```

## withRetry — automatic API retries

The query engine wraps every API call in `withRetry`; you rarely call it directly. It is an **async generator**, not a plain awaitable, and its real signature (`src/services/api/withRetry.ts:170-178`) is:

```typescript
async function* withRetry<T>(
  getClient: () => Promise<Anthropic>,
  operation: (client: Anthropic, attempt: number, context: RetryContext) => Promise<T>,
  options: RetryOptions,
): AsyncGenerator<SystemAPIErrorMessage, T>
```

`RetryOptions` (`withRetry.ts:127-142`): `maxRetries?` (default `DEFAULT_MAX_RETRIES = 10`), `model`, `fallbackModel?`, `thinkingConfig`, `fastMode?`, `signal?`, `querySource?`, `initialConsecutive529Errors?`. The retry policy is internal — base delay `BASE_DELAY_MS = 500` (`withRetry.ts:55`) and the retry/skip decision in a private `shouldRetry(error)` (`withRetry.ts:696`); there is **no** caller-supplied `shouldRetry`, `maxAttempts`, or `baseDelayMs`. Don't hand-roll retry around API calls — let the engine's `withRetry` own it.

## Error categorization

`getAssistantMessageFromError` converts API errors into user-visible messages (`src/services/api/errors.ts:425`); `categorizeRetryableAPIError` (`errors.ts:1163`) classifies retryable statuses. Posture:

- `APIConnectionTimeoutError` / `APIConnectionError` — network failure; retryable (`withRetry.ts:753`).
- `APIUserAbortError` — user aborted; **never** retry. Handled in `withRetry.ts` (thrown when `signal.aborted`), not in `errors.ts`.
- `APIError` status 429 — rate limit; retried with backoff.
- `APIError` status 529 — overloaded; retried **only** for foreground query sources (`FOREGROUND_529_RETRY_SOURCES`, `withRetry.ts:62-89`) and capped at `MAX_529_RETRIES = 3`; background sources bail immediately.

The retry decision lives inside `withRetry`; tools generally don't implement their own.

## ValidationResult

For cross-field constraints Zod can't express, implement `validateInput()` returning `ValidationResult` (`src/Tool.ts:95-101`):

```typescript
type ValidationResult =
  | { result: true }
  | { result: false; message: string; errorCode: number }
```

## Smoke tests

Every tool has a smoke test asserting structural validity. The shipped pattern (`tests/smoke/tools.test.ts`) imports concrete tools and checks each one's registered identity and schema:

```typescript
import { describe, expect, it } from 'vitest'
import { BashTool } from '../../src/tools/BashTool/BashTool.js'

describe('BashTool', () => {
  it('has a valid structure', () => {
    expect(BashTool.name).toBe('Bash')          // the REGISTERED name, not the export identifier
    expect(typeof BashTool.call).toBe('function')
    expect(BashTool.inputSchema).toBeDefined()    // a non-null Zod object
  })
})
```

`tool.name` is the **registered** tool name (`'Bash'`, `'Read'`), which usually differs from the exported symbol (`BashTool`, `FileReadTool`) — assert against the registered string. Validate, at minimum: the tool exports a valid `Tool` object, `name` is the expected string, and `inputSchema` is defined. (`getEmptyToolPermissionContext` exists at `Tool.ts:140` if you additionally want to exercise `prompt()`, but the shipped smoke test does not.)

## Integration tests

For tools that call external services (`tests/integration/api.test.ts`, MCP at `tests/integration/mcp.test.ts`):

1. Hit the real API with a test credential.
2. Assert on the **shape** of the result, not the content (content is non-deterministic).
3. Verify error handling for expected failure modes (timeouts, 429, auth failure).

## Testing prompt caching

To verify an agent doesn't break the cache, run two identical consecutive calls and assert the second hits the cache:

```typescript
const first = await callAgent(input)
const second = await callAgent(input)

// SDK usage fields are *_input_tokens — guard for undefined
expect(second.usage.cache_read_input_tokens ?? 0).toBeGreaterThan(0)
expect(second.usage.cache_creation_input_tokens ?? 0).toBe(0)
```

The second call should read from cache (`cache_read_input_tokens > 0`) and create nothing new (`cache_creation_input_tokens === 0`). The field names are `cache_read_input_tokens` / `cache_creation_input_tokens` — not `cache_*_tokens`. If the cache doesn't hit, consult `reference/prompt-caching.md` and the cache-break diff file written by `checkResponseForCacheBreak`.

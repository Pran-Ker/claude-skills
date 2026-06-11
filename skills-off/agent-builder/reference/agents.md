# Agent Definition

> Field-by-field reference for defining agents and subagents. Citations are absolute paths into the local repo (`REPO = /Users/pran-ker/Developer/claude-code`).

## Contents

- [The three agent types](#the-three-agent-types)
- [Mandatory and common fields](#mandatory-and-common-fields)
- [whenToUse — the most important field](#whentouse--the-most-important-field)
- [Built-in agent template](#built-in-agent-template)
- [Custom agent (markdown format)](#custom-agent-markdown-format)
- [Model selection heuristics](#model-selection-heuristics)
- [System prompt design](#system-prompt-design)
- [Permission modes](#permission-modes)
- [Fork vs typed subagents](#fork-vs-typed-subagents)
- [Agent memory](#agent-memory)
- [MCP integration](#mcp-integration)

## The three agent types

Every agent is one variant of `AgentDefinition` (`src/tools/AgentTool/loadAgentsDir.ts:106-165`):

| Type | Source | When |
|---|---|---|
| `BuiltInAgentDefinition` | `source: 'built-in'` | Shipped with the binary; dynamic prompt via `getSystemPrompt()` |
| `CustomAgentDefinition` | user/project settings | Markdown files in `.claude/agents/` |
| `PluginAgentDefinition` | `source: 'plugin'` | Loaded from installed plugins |

Type guards (`isBuiltInAgent`, etc.) live just below at `src/tools/AgentTool/loadAgentsDir.ts:167+`.

## Mandatory and common fields

`BaseAgentDefinition` (`src/tools/AgentTool/loadAgentsDir.ts:106-133`):

```typescript
type BaseAgentDefinition = {
  agentType: string        // unique name shown in UI and analytics; PascalCase
  whenToUse: string        // plain-English trigger sent to the orchestrator (see below)
  tools?: string[]         // explicit allowlist (default: full set)
  disallowedTools?: string[]  // tools this agent must never call
  model?: string           // type is just string; values 'haiku'|'sonnet'|'opus'|'inherit' (defaults to Sonnet when omitted)
  effort?: EffortValue     // reasoning effort: an EFFORT_LEVELS value or an int
  permissionMode?: PermissionMode  // see PERMISSION_MODES below
  maxTurns?: number        // hard cap on agentic turns
  omitClaudeMd?: boolean   // skip CLAUDE.md hierarchy (read-only agents)
  background?: boolean      // always run async; never blocks the parent
  memory?: AgentMemoryScope // 'user' | 'project' | 'local'
  hooks?: HooksSettings    // session-scoped hooks registered when the agent starts
  mcpServers?: AgentMcpServerSpec[]
  requiredMcpServers?: string[]  // agent unavailable unless these are configured
  skills?: string[]        // skill names to preload
  initialPrompt?: string   // prepended to the first user turn
  isolation?: 'worktree' | 'remote'  // isolated worktree; 'remote' is ant-only (schema gate, :94-97)
}
```

The Zod schema validating markdown frontmatter is `AgentJsonSchema` at `src/tools/AgentTool/loadAgentsDir.ts:73-99`.

## whenToUse — the most important field

The orchestrator reads `whenToUse` verbatim when deciding which agent to invoke. Write it as a **concrete trigger condition**, not a generic description.

**Bad:** `'General purpose agent for various tasks'`

**Good** (the real Explore agent, `src/tools/AgentTool/built-in/exploreAgent.ts:61-62`):

```
'Fast agent specialized for exploring codebases. Use this when you need to quickly find files
by patterns (eg. "src/components/**/*.tsx"), search code for keywords (eg. "API endpoints"),
or answer questions about the codebase (eg. "how do API endpoints work?"). When calling this
agent, specify the desired thoroughness level: "quick" for basic searches, "medium" for
moderate exploration, or "very thorough" for comprehensive analysis across multiple locations
and naming conventions.'
```

Note it names example inputs and spells out each thoroughness level rather than abbreviating — that specificity is what makes routing reliable. (Reproduced verbatim from source; don't shorten it.)

## Built-in agent template

Pattern, modeled on `src/tools/AgentTool/built-in/exploreAgent.ts:64-83`:

```typescript
import type { BuiltInAgentDefinition } from '../loadAgentsDir.js'

export const MY_AGENT: BuiltInAgentDefinition = {
  agentType: 'MyAgent',          // PascalCase, unique
  whenToUse: '...',              // concrete trigger conditions
  source: 'built-in',
  baseDir: 'built-in',
  model: 'haiku',                // haiku for fast/cheap read-only work
  permissionMode: 'bubble',      // surface permission prompts to the parent
  omitClaudeMd: true,            // read-only agents skip project conventions
  disallowedTools: [
    AGENT_TOOL_NAME,             // prevent recursive sub-spawning unless intentional
    FILE_EDIT_TOOL_NAME,         // lock down writes for read-only agents
    FILE_WRITE_TOOL_NAME,
  ],
  getSystemPrompt: () => `...`,   // canonical Explore form: takes no args
}
```

`getSystemPrompt` is typed `(params: { toolUseContext: Pick<ToolUseContext, 'options'> }) => string` (`loadAgentsDir.ts:140-143`) — only `options` is available, not the full context, and the param is optional (Explore omits it). `permissionMode: 'bubble'` above is illustrative; the real Explore agent doesn't set it. Note Explore's model is `process.env.USER_TYPE === 'ant' ? 'inherit' : 'haiku'` — ants inherit the main model so they share its cache; external users get haiku for speed.

## Custom agent (markdown format)

Create `.claude/agents/my-agent.md`:

```markdown
---
description: One-line trigger description for when to spawn this agent
tools: [Read, Bash, Glob, Grep]   # explicit allowlist preferred over full toolset
model: haiku
permissionMode: bubble
maxTurns: 20
---

You are a specialist agent for X. Your job is ...

## Constraints
- Only read files, never write
- ...
```

The frontmatter `description` becomes the agent's `whenToUse` (`src/tools/AgentTool/loadAgentsDir.ts:478`) — write it as a concrete trigger, by the same rules as above. The body becomes `getSystemPrompt()`.

## Model selection heuristics

| Use case | Model | Rationale |
|---|---|---|
| Fast read-only search | `haiku` | speed + cost |
| Code generation, review | `sonnet` (default) | balance |
| Complex reasoning, architecture | `opus` | quality |
| Subagent that must match parent | `inherit` | cache sharing |

`inherit` is critical when the parent's prompt cache must be shared by the child — see fork subagents below and `reference/prompt-caching.md`.

## System prompt design

Open with role identity, then put constraints early and explicit. The Explore agent (`src/tools/AgentTool/built-in/exploreAgent.ts:24-56`) leads with a loud `=== CRITICAL: READ-ONLY MODE ===` banner enumerating every prohibited operation before describing strengths — constraints first, capabilities second.

Scope matters for the identity line. For the **main CLI** system prompt, get it from `getCLISyspromptPrefix()` (`src/constants/system.ts:30-46`) rather than hard-coding — it handles the Vertex override and SDK mode and returns the correct one of:

- "You are Claude Code, Anthropic's official CLI for Claude."
- "You are Claude Code … running within the Claude Agent SDK."
- "You are a Claude agent, built on Anthropic's Claude Agent SDK."

Built-in **sub-agents** are different: they write their own role identity directly in `getSystemPrompt()`. The Explore agent opens with a literal `"You are a file search specialist for Claude Code, Anthropic's official CLI for Claude."` — it does **not** call `getCLISyspromptPrefix()`. So for a sub-agent, lead with a specialist identity line of your own; reserve `getCLISyspromptPrefix()` for the top-level CLI prompt.

Structure, memoization, and the dynamic boundary are covered in `reference/prompt-caching.md`.

## Permission modes

`src/utils/permissions/PermissionMode.ts`:

| Mode | Behavior |
|---|---|
| `default` | Show permission prompts to the user |
| `acceptEdits` | Auto-approve file edits; prompt for shell commands |
| `bypassPermissions` | No prompts; all operations auto-approved |
| `plan` | Planning mode — present a plan, don't execute |
| `dontAsk` | Suppress prompts without full bypass |
| `bubble` | *(internal)* Forward permission prompts to the parent agent |
| `auto` | *(internal)* ML classifier decides |

The full `PERMISSION_MODES` set is in `src/utils/permissions/PermissionMode.ts`; `bubble` and `auto` are internal-only (not in the external set). Use `bubble` for most subagents — the human still sees and approves dangerous operations, just via the parent. To gate a tool's own execution, implement `checkPermissions(input, context)` returning `PermissionResult` (`{ granted: true }` or `{ granted: false, reason }`). User-configured `alwaysAllowRules` / `alwaysDenyRules` are glob patterns like `Bash(git:*)`, matched via the pattern logic in `src/tools/BashTool/bashPermissions.ts`.

## Fork vs typed subagents

| Pattern | When | How |
|---|---|---|
| **Fork** | Child needs full parent context | omit `subagent_type`; uses `FORK_AGENT` |
| **Typed** | Child is a specialized role | set `subagent_type: 'Explore'` etc. |
| **Background** | Independent parallel work | set `background: true` in the agent def |

Prevent recursive forking: the system detects fork children via `FORK_BOILERPLATE_TAG` and rejects nested forks (`src/tools/AgentTool/forkSubagent.ts:78-89`). Guard your agents with `disallowedTools: [AGENT_TOOL_NAME]` or check `isInForkChild()` at call time.

For cache-sharing fork children, all children must produce **byte-identical API prefixes** (constraint documented at `src/tools/AgentTool/forkSubagent.ts:45-93`; the message collapse itself is in `buildForkedMessages`, `:107-168`): pass `renderedSystemPrompt` from the parent context (assigned in `AgentTool.tsx:496-497`), use `tools: ['*']` with `useExactTools` to inherit the parent's exact tool pool, and collapse parent tool-result blocks to `FORK_PLACEHOLDER_RESULT = 'Fork started — processing in background'` (`forkSubagent.ts:93`). Detail in `reference/prompt-caching.md`.

## Agent memory

Persistent memory across sessions via the `memory` field (`src/tools/AgentTool/agentMemory.ts`):

```typescript
memory: 'user'     // <userBase>/agent-memory/<agentType>/
memory: 'project'  // <cwd>/.claude/agent-memory/<agentType>/
memory: 'local'    // <cwd>/.claude/agent-memory-local/<agentType>/  (gitignored)
```

Path and scope resolution are in `src/tools/AgentTool/agentMemory.ts:34-62` (the directory is `agent-memory/`, not `agents/`).

## MCP integration

Reference servers in an agent (`src/tools/AgentTool/loadAgentsDir.ts:57-68`):

```typescript
mcpServers: [
  'slack',                          // reference an existing server by name
  { my_server: { command: '...' } } // inline definition
]
requiredMcpServers: ['slack', 'github']  // agent unavailable unless configured
```

MCP tools are prefixed `mcp__<server>__<tool>` — use the prefix when listing in `tools`/`disallowedTools`. For analytics, MCP tool names are sanitized to `'mcp'` to avoid leaking server names into telemetry (`src/services/api/promptCacheBreakDetection.ts:182-185`). Tool registration and `inputJSONSchema` handling for MCP servers live in `src/services/mcp/client.ts` — **not** in `mcpValidation.ts`, which only enforces output token/size limits (`getMaxMcpOutputTokens`, `getContentSizeEstimate`). A server tool must provide `name`, `description` (its `prompt()` equivalent), and a valid `inputSchema`. Non-tool data is exposed via `ReadMcpResourceTool` / `ListMcpResourcesTool` and prefetched at session start (prefetch logic in `src/services/mcp/client.ts`).

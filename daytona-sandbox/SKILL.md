---
name: daytona-sandbox
description: Launch the current project as a live server on Daytona cloud and return a public preview URL. Use when user says "launch in sandbox", "run on Daytona", "deploy to sandbox", "spin up a sandbox", or "test on cloud".
argument-hint: [port]
---

Launch the current project on Daytona cloud infrastructure and return a public preview URL.

## Prerequisites check (run in parallel)

1. **Daytona CLI**: `daytona --version` — if missing: `brew install daytonaio/cli/daytona`
2. **API key**: `echo $DAYTONA_API_KEY` — if empty: `source ~/.zshrc`
3. **Git remote**: `git remote get-url origin` — if none, tell user to push to GitHub first

## Step 1 — Detect project type

| File | Install | Start | Port |
|---|---|---|---|
| `package.json` | `npm install` | `scripts.start` in package.json, else `npm start` | 3000 |
| `requirements.txt` | `pip install -r requirements.txt` | check `Procfile` web line, else ask user | 8000 |
| `pyproject.toml` | `pip install .` | check `Procfile` web line, else ask user | 8000 |
| `go.mod` | `go build -o app .` | `./app` | 8080 |

If `$ARGUMENTS` was passed, use it as the port. If the start command is ambiguous, ask the user before proceeding.

## Step 2 — Create sandbox

```bash
DAYTONA_API_KEY=$DAYTONA_API_KEY daytona sandbox create
```

Capture the sandbox ID from the output. Save it:
```bash
scratchpad "daytona sandbox <ID> — <project name>"
```

## Step 3 — Clone and install

```bash
REMOTE=$(git remote get-url origin)
daytona sandbox exec <ID> "git clone $REMOTE app"
daytona sandbox exec <ID> "cd app && <INSTALL_COMMAND>"
```

If the repo is private, ask the user for a GitHub token or to temporarily make it public.

## Step 4 — Start the server

```bash
daytona sandbox exec <ID> "cd app && nohup <START_COMMAND> > server.log 2>&1 &"
sleep 3
daytona sandbox exec <ID> "curl -sf http://localhost:<PORT> > /dev/null && echo ok || tail -20 app/server.log"
```

If the health check fails, print the log output and stop. Common fix: app is binding to `127.0.0.1` instead of `0.0.0.0`.

## Step 5 — Get preview URL

```bash
daytona sandbox info <NAME_OR_ID>
daytona preview-url <NAME_OR_ID> --port <PORT>
```

URL format: `https://<PORT>-<TOKEN>.daytonaproxy01.net`

## Step 6 — Report to user

```
Live at: https://<PORT>-<ID>.preview.daytona.app

Sandbox ID: <ID>
Stop:   daytona sandbox stop <ID>
Delete: daytona sandbox delete <ID>
```

## Teardown (when user asks to stop)

```bash
daytona sandbox delete <ID>
```

## Common errors

- **Clone fails**: private repo — ask for GitHub token
- **Port unreachable**: app binding to 127.0.0.1 — needs 0.0.0.0
- **Auth error**: run `daytona whoami` to verify API key
- **Unknown start command**: always ask rather than guess

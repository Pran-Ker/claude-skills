---
name: experiment
description: Enforces disciplined run tracking, reproducibility, and safe execution before any implementation, benchmark, data-processing job, or API-heavy script. Use when starting any ML/data/API coding task.
---

You are a coding agent. Before starting any implementation, evaluation, benchmark, data-processing job, API-heavy script, or deployment, follow these principles in order.

After each step report `✅ Step N — result` or `⚠️ Step N — what's needed`. Print the updated checklist after every step.

## 1. Load credentials and confirm tracking destinations

First, load credentials into the shell:

```bash
source ~/.local/secrets  # or wherever you store your secrets
export HF_TOKEN="${HUGGING_FACE_TOKEN}"
export WANDB_API_KEY="${WANDB_API_KEY}"
```

Verify both are set:

```bash
[[ -n "$HF_TOKEN" ]] && echo "HF_TOKEN ✓" || echo "HF_TOKEN ✗"
[[ -n "$WANDB_API_KEY" ]] && echo "WANDB_API_KEY ✓" || echo "WANDB_API_KEY ✗"
```

If either is missing, stop and tell the user.

The following are pre-configured. Confirm the project-specific fields only:

**Hugging Face**
- Username: `<YOUR_HF_USERNAME>`
- Ask user: repo name, public or private

**W&B**
- Entity: `<YOUR_WANDB_ENTITY>`
- Ask user: project name, run name convention if any

**Weave (for agent or LLM call loop runs only)**
- Entity: `<YOUR_WANDB_ENTITY>` (same W&B key)
- Ask user: Weave project name
- Only required if the job involves an AI agent or LLM call loop

## 2. Generate a Run ID

Run this before every run:

```bash
RUN_ID=$(python3 - <<'EOF'
import json, random, datetime, sys
from pathlib import Path
project, task, method, variant = sys.argv[1:5]
reg = Path('~/.claude/run_registry.json').expanduser()
d = json.loads(reg.read_text()) if reg.exists() else {}
d[project] = d.get(project, 0) + 1
reg.parent.mkdir(exist_ok=True)
reg.write_text(json.dumps(d, indent=2))
ts = datetime.datetime.now().strftime('%Y%m%d-%H%M%S')
rand = ''.join(random.choices('abcdefghijklmnopqrstuvwxyz0123456789', k=4))
print(f'RUN_{d[project]:03d}_{project}_{task}_{method}_{variant}_{ts}_{rand}')
EOF
python3 /dev/stdin PROJECT TASK METHOD VARIANT)
echo "Run ID: $RUN_ID"
```

Substitute PROJECT, TASK, METHOD, VARIANT from context before running.

Schema: `RUN_<seq>_<project>_<task>_<method>_<variant>_<YYYYMMDD-HHMMSS>_<rand>`

Example: `RUN_004_mathagent_eval_lora_rank16_20260424-143022_k7p9`

Rules:
- Never create manually, never reuse, never overwrite an existing run directory
- Sequence increases monotonically per project in `~/.claude/run_registry.json`
- Timestamp must include seconds; random suffix required
- Apply consistently across: HF logs, W&B run name, Weave run name, tmux session, Zlog folder, checkpoints, and artifacts

## 3. Check compute

Run: `nvidia-smi`, `df -h`, `free -h`, `lscpu`, `python --version`, `pip freeze`

Confirm the run is feasible with the current setup. Stop and report if it is not.

## 4. Check git state

Run `git status` and `git log --oneline -3`.

Report: current branch, last commit hash, and any dirty files. Ask before running from a dirty repo. When committing, include the run ID, the goal, and any unique context so git log is self-explanatory without external references.

## 5. Logging

Track everything. Machine logs store the full picture:

- Run ID, timestamp, git commit, branch, dirty status
- Hostname, OS, Python version, all package versions, CUDA, GPU
- Full config, all hyperparameters, CLI args, safe environment variables
- Dataset name, version, splits, sample counts, preprocessing steps
- Model name, checkpoint path, tokenizer, training settings, optimizer, seed
- All metrics and losses per step, eval results, latency, throughput, memory usage
- Errors, warnings, failed cases, recovery steps
- All artifacts, checkpoints, and summaries

The `summary.md` written to `Zlog/<run_id>/` contains only the human-important subset for quick scanning:

- Run ID and timestamp
- Git commit and branch
- Model and dataset
- Key hyperparameters and seed
- Primary eval metric and best score
- Runtime

Never log: secrets, API keys, credentials, raw personal data, full `.env` files

## 6. Folder structure

The skill creates two folders at the project root. Do not touch anything else in the project.

**`Zexp/`** — scaffolding (created once per project, not per run)
```
Zexp/
  run.sh          — script that launches the project; supports --resume
  experiments.log — running list of all run IDs, status, and one-line result summary
  config.json     — hyperparameters the user can edit between runs
```

**`Zlog/<run_id>/`** — human-readable summary (created per run)
```
Zlog/<run_id>/
  summary.md      — what ran, result, key metric, one-paragraph human summary,
                    W&B dashboard URL, Weave trace URL (agent runs only)
  reproduce.sh    — exact command + pip freeze + env vars; queries HF for the latest
                    checkpoint automatically when run with --resume
  best_score.json — the single most important metric from this run

  logs/           — machine files; store everything, no exclusions
    metrics.jsonl
    environment.json
    stdout.log
    stderr.log
    (all trace and artifact files)
```

## 7. Hugging Face storage

Mirror the Zlog structure exactly in HF under:

```
logs/<project_name>/<run_id>/summary.md
logs/<project_name>/<run_id>/reproduce.sh
logs/<project_name>/<run_id>/best_score.json
logs/<project_name>/<run_id>/logs/  (all machine files)
```

Scan for secrets before uploading. Exclude `.env` and raw credentials.

## 8. W&B tracking

Track only what is human-understandable or directly actionable:

- Primary eval metric (loss, accuracy, perplexity, score — whichever applies)
- Best score so far
- Learning rate (if training)
- GPU memory peak (if GPU job)
- Runtime and throughput

Enable system metrics. Do not add charts unless the user asks.

## 9. Weave (agent runs only)

If the job is an agent or LLM call loop, log all traces to Weave: individual calls, inputs, outputs, latencies, and retry counts. Use the same project name as W&B unless the user specifies otherwise.

## 10. Launch in tmux

All long-running jobs must use a named tmux session:

```bash
tmux new -s <run_id>
# reattach: tmux attach -t <run_id>
```

Tell the user: session name, command, stdout/stderr log paths, reattach command. Always save stdout and stderr to `Zlog/<run_id>/logs/`.

## 11. Pre-launch checklist

Go through every item. Do not skip.

- [ ] Tracking destinations confirmed (HF repo name, W&B project, Weave if applicable)
- [ ] Run ID generated and registered in `~/.claude/run_registry.json`
- [ ] Compute feasibility confirmed
- [ ] Git state clean or dirty repo explicitly approved
- [ ] Credentials loaded (not printed)
- [ ] Disk space sufficient
- [ ] HF and W&B API access verified
- [ ] `Zexp/` exists; `Zlog/<run_id>/` created and empty
- [ ] tmux available
- [ ] Small test run passed (see below)

## 12. Test run first

Before the full job, always run a minimal end-to-end test: a small subset of data, steps, or API calls that exercises every stage — data loading, model or API call, metric logging, HF upload, W&B logging, and tmux capture. Confirm all stages pass before launching the full job.

## 13. Reproducibility and cross-device checkpointing

Save with every run: git commit and branch, dependency versions, docker image if used, random seeds, exact command, hardware details.

The `reproduce.sh` in `Zlog/<run_id>/` must be sufficient to fully reproduce or resume the run from any machine. It must include:
- `pip install -r requirements.txt` (or equivalent)
- the exact training command
- `--resume` flag that queries HF for the latest checkpoint automatically

Checkpoints for long jobs are saved locally and uploaded to HF **incrementally during the run** — not just at the end. This means:
- Any device can pull the latest checkpoint from HF and resume
- Checkpoint paths on HF follow: `checkpoints/<project_name>/<run_id>/step_<N>/`

Never overwrite a checkpoint without confirmation. Each step checkpoint is a new upload, not an overwrite.

## On failure

If the job crashes:
1. Do nothing destructive first — capture read-only state (`nvidia-smi`, `free -h`, `tail stdout.log`)
2. Verify the last HF checkpoint is intact before any recovery action
3. Write `crash_report.json` to `Zlog/<run_id>/logs/` and append a crash section to `summary.md`
4. Append to `metrics.jsonl` — never overwrite
5. Mirror crash artifacts to HF
6. Close the W&B run with `exit_code=1`
7. Update `experiments.log` with CRASHED status and last good checkpoint path
8. Do not start a new run ID for a resume — it is a continuation of the same experiment
9. Require a test run (small number of steps) on the new machine before the full resume
10. Append to existing logs (`tee -a`) — never overwrite crash evidence

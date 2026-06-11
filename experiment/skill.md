---
name: experiment
category: system
description: Enforces disciplined run tracking, reproducibility, and safe execution before any implementation, benchmark, data-processing job, or API-heavy script. Use when starting any ML/data/API coding task.
---

After each step report `✅ Step N: result` or `⚠️ Step N — what's needed`. Print the updated checklist after every step.

## Checklist

Follow these steps in order. Reprint this checklist after every step with each item marked `✅` done, `⬜` pending, or `⚠️` blocked:

- [ ] 1. Load credentials and confirm tracking destinations
- [ ] 2. Check setup — feasibility check + short test run
- [ ] 3. Check git state — commit if dirty
- [ ] 4. Logging — folder setup + side-car logging scripts
- [ ] 5. W&B tracking
- [ ] 6. Launch experiment — named tmux session
- [ ] 7. On progress — monitor + crash handling + phone updates
- [ ] 8. Reproducibility and cross-device checkpointing
- [ ] 9. Create a backup

## 1. Load credentials and confirm tracking destinations

First, load credentials into the shell:

```bash
source ~/.local/secrets
export HF_TOKEN="${HUGGING_FACE_TOKEN}" # HF username: Pran-Ker
export WANDB_API_KEY="${WANDB_API_KEY}" # W&B entity: hebbarpran-warping
# export any other API keys this run needs
```

Verify every key the run needs actually works (a cheap authenticated call each).

Propose a run ID of the form `RUN_<NNN>_<task_name>_<repo>` and confirm it with the user. Use that exact name consistently across W&B, HF, and Weave.

Make sure these runs are grouped together in W&B and HF unless mentioned otherwise.

## 2. Check setup

Run a subagent to check whether the run is feasible or not, like checking `nvidia-smi`, `df -h`, `free -h`, `lscpu`, `python --version`, `pip freeze` and other details in the code.

Do a test run, making sure it actually works (stop it after a few minutes once everything is confirmed working).

Proceed only after the test run has actually succeeded; report exactly what was verified.

## 3. Check git state

Run a subagent which checks `git status` and `git log --oneline -3`. And if the git is dirty commits it (locally). The goal is to have a checkpoint for reproducing the results.

## 4. Logging

Have multiple agents go through how the experiment runs and initialize the experiment folder setup and logging scripts.
Logging script:
Create external scripts that track everything in the experiment. If it is a new project, you will need to launch a subagent to verify that the endpoint we are using to run the experiments is tracking all the details and store the logs in files using an external side-car program. Create them in exp/__helper__

- Run ID, timestamp, git commit, branch
- Hostname, python environment, CUDA, GPU
- Full config, all hyperparameters, CLI args, safe environment variables
- Dataset name, version, splits, sample counts, preprocessing steps
- Model name, checkpoint path, tokenizer, training settings, optimizer, seed
- All metrics and losses per step, eval results, latency, throughput, memory usage
- Errors, warnings, failed cases, recovery steps
- All artifacts, checkpoints, and summaries

The `summary.md` written to `exp/<run_id>/` contains only the human-important subset for quick scanning:

- Run ID and timestamp
- Git commit and branch
- HF and W&B links
- Model and dataset
- Key hyperparameters and seed
- Primary eval metric and best score
- Tinker Checkpoints (only look into this if we are using it)
- Runtime

Never log: secrets, API keys, credentials, raw personal data, full `.env` files

Create a folder at the project root (which is added to .gitignore). Do not touch anything else in the project.

**`exp/<run_id>/`** — human-readable summary (created per run)
```
exp/<run_id>/
  summary.md      — what ran, result, key metric, one-paragraph human
  reproduce.sh    — exact command + pip freeze + env vars; queries HF for the latest
                    checkpoint automatically when run with --resume
  best_score.json — create this once completed, it contains the most important metric from this run, which we track

  logs/           — machine files; store everything, no exclusions; runs, steps, api calls, intermediate checkpoints, solution files.
```

Verify the logs capture every value the experiment emits: compare the metrics a test run prints against what actually landed in the log files. Silently dropped values are a known failure mode here — check for them explicitly.


## 5. W&B tracking

Track only what is human-understandable or directly actionable and make sure it is ordered based on usefulness. Sometimes the tracking is in the codebase, so you will need to make changes to it with a subagent:

- Primary eval metric (loss, accuracy, perplexity, score — whichever applies)
- Best score so far
- Learning rate (if training)
- GPU memory peak (if GPU job)
- Runtime and throughput

Enable system metrics. Do not add charts unless the user asks. Weave from W&B is only for agent runs, logging the steps.

## 6. Launch experiment

All long-running jobs must use a named tmux session:

```bash
tmux new -s <run_id>
# reattach: tmux attach -t <run_id>
```

## 7. On progress

Monitor at short intervals until you're confident the run is healthy, then switch to long intervals.

If the job crashes:
1. Do nothing destructive first — capture read-only state (`nvidia-smi`, `free -h`, `tail stdout.log`)
2. Verify the last HF checkpoint is intact before any recovery action

Send phone updates with `notify` (`~/Tools/notify/notify`) at the moments that matter —  on completion, and `notify -p 1` on failure. Add at most a few sparse mid-run milestones (e.g. ~25/50/75% or major phase boundaries, `notify -t "Give detailed background and results in extremely simple, easy to understand format"`)

## 8. Reproducibility and cross-device checkpointing

Launch a subagent to ensure that we are saving with every run: git commit and branch, dependency versions, docker image if used, random seeds, exact command, hardware details.

The `reproduce.sh` in `exp/<run_id>/` must be sufficient to fully reproduce or resume the run from any machine. It must include:
- `pip install -r requirements.txt` (or equivalent)
- the exact training command
- `--resume` flag that queries HF for the latest checkpoint automatically

Checkpoints for long jobs are saved locally and uploaded to HF **incrementally during the run** — not just at the end. This means:
- Any device can pull the latest checkpoint from HF and resume
- Checkpoint paths on HF follow: `checkpoints/<project_name>/<run_id>/step_<N>/`

Never overwrite a checkpoint without confirmation. Each step checkpoint is a new upload, not an overwrite.

## 9. Create a backup

Zip `exp/<run_id>/` and save it to `~/Research/experiment_backup/<repo_name>/`.

Update the repo's CLAUDE.md to record that this skill has configured the repo: which steps are already done (secrets wiring, logging scripts, exp/ structure) so future runs only repeat the per-run steps.
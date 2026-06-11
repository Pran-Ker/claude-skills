---
name: notify
category: tools
description: Send a push notification to Prannay's phone via the `notify` CLI. Use only when explicitly asked to notify, or when another skill (e.g. experiment) directs it. Batch aggressively and send the fewest notifications possible.
---

# Notify

Send notifications with the `notify` CLI (`~/Tools/notify/notify`, alias `n`).

## Rules

- Run only when explicitly asked, or when another skill directs it.
- Budget: at most 3 notifications per task. One, at the end, is best. Batch related updates into a single message.
- Every notification carries three parts: brief background (the reader likely won't remember the task), the result, and any follow-up or ask.

## Flags

`-t` title · `-p -2..2` priority (1 high, 2 emergency) · `-d` also pop a desktop notification · `-w DUR` delay send. Run `notify --help` for the full surface.

## Example

Asked to "notify me the results of the experiments": wait for runs to finish, group them, send one notification:

> Results for the ML experiments on <repo_name>:
> 1. Mse_Norm (lower better) = 19.2
> 2. Mse_Norm (lower better) = 23.4
> Done with 3 of 7 experiments, 4 remaining. Should I also run on Gemini?

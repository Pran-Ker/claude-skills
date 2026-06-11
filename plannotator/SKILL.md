---
name: plannotator
category: context
description: Open Plannotator's browser UI for annotating documents, reviewing
  code changes or PRs, or annotating the last assistant message, then act on
  the returned feedback. Use on /plannotator, "annotate this", "review this in
  plannotator", "open it in plannotator", or "let me mark up your last reply".
---

# Plannotator

One skill, three subcommands of the CLI at `~/.local/bin/plannotator`. Pick by
what the user wants to mark up. Always run the command yourself with Bash;
never ask the user to paste shell syntax. The command blocks until the browser
session ends, so run it in the background for long reviews.

## annotate: a document, URL, or folder

```bash
plannotator annotate <path-or-url>
```

For markdown files, converted HTML, URLs, or folders. When the user wants to
review something I produced (a plan, changelog, report), write it to a file
first, then annotate that file.

## review: code changes or a PR

```bash
plannotator review [optional-pr-url]
```

For the current worktree's diff, or a PR URL if given.

## last: the previous assistant message

```bash
plannotator last
```

Targets the latest rendered assistant response. Do NOT send any commentary or
status text before running it; a preamble would become the thing annotated.

## After the session ends

1. Annotations returned: address each one directly in the same conversation.
2. Approval/LGTM-style result: acknowledge the review passed and continue.
3. Session closed with no feedback: say so briefly and continue.

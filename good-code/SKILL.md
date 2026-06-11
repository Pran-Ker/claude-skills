---
name: good-code
category: taste
description: Write or reformat code to be minimal and scannable. No comments,
  vertical declarations, labeled blocks, flat control flow. Trigger on
  /good-code, "clean this up", "make this readable/scannable", or whenever
  reformatting or writing fresh code where the user wants clarity.
---

# Good Code: minimal, scannable, segmented

Write for a reader who scans top-to-bottom to grasp intent in seconds, not
line-by-line. Vertical rhythm, clear name → action mapping, logical grouping.
Shorter is a proxy for clearer, but never at the cost of readability.

## Rules
- No comments. Exception: one short label (3-5 words) per non-obvious block
  when a name alone can't convey what's happening.
- Minimize length: fewest tokens/lines that still read clearly. Expressions
  over statements, early returns over nesting, standard-library idioms over
  reinvention. Never golf into obscurity.
- Declarations one per line: function parameters/arguments, struct/dataclass
  fields, enum members, and grouped assignments stay vertically listed so key
  variables are instantly scannable. Never collapse a multi-line parameter or
  field list to save space.
- Group code into labeled logical blocks separated by ONE blank line; one
  blank line between top-level definitions. No double blanks, no trailing
  whitespace.
- Names carry the meaning comments would: verbs for functions, nouns for
  data, full words over abbreviations.
- Flat control flow: guard clauses first, happy path last, max ~2 levels of
  nesting.
- Each block does one thing; scanning the first token of each line should
  reveal the shape.
- Keep existing behavior identical unless told otherwise. Cleanup instincts
  that change semantics are behavior changes: `== True` → truthy check,
  "fixing" a mutable default, list → set membership when the caller sees the
  mutation. Preserve the original semantics, or change them and flag it in
  one line above the code block. Never change behavior silently.
- Never delete a comment that records an external constraint (bug number,
  workaround, spec quirk): compress it into the block label and keep the why.

## When rules conflict
- Removing a comment loses information the code can't recover → encode it in
  a name or a 3-5 word block label.
- A line exceeds reasonable width → break at logical boundaries, never
  arbitrarily.
- Length vs scannability → scannability wins, and one-per-line declarations
  always win over compaction.

## Example

Before:
```python
def p(d):
    # filter active users and get their emails
    r = []
    for u in d:
        if u['active']:
            r.append(u['email'])
    return r
```

After:
```python
def active_emails(users):
    return [u["email"] for u in users if u["active"]]
```

Keep declarations vertical (do NOT collapse):
```python
def _build_config(
    *,
    adapter_dir: str,
    target_agent_path: str,
    task_model: str,
    benchmark: str,
    jobs_dir: str,
    working_dir: str,
    include_tasks: list[str] | None,
    n_concurrent: int,
    run_timeout: int,
):
```

## Output
For a pure reformat request: the rewritten code in a single fenced block,
no prose, no explanation. Sole exception: one line above the block flagging
an intentional behavior change or deliberately preserved suspicious behavior.
When part of a larger task, apply the rules to every line of code you
produce and keep surrounding prose to a minimum.

---
name: example
category: context
description: Show imagined output as one concrete, diagram-first thought
  experiment a smart non-expert can read on first pass. Trigger on /example,
  "what would this look like", "give me an example", and ALWAYS before
  answering "explain X" or "how does X work" questions.
---

# Example: imagine the output

Show the finished thing; don't describe it. One concrete scenario a smart
non-expert can follow on first read. The example is an alignment anchor,
not the full answer: the user will dive into one step or redline the
prototype next, so every part must be addressable (numbered steps,
named parts).

## Output shape (text mode)
1. **Headline**: one plain sentence saying what the thing is. Zero jargon.
2. **The shape**: a small diagram (ASCII flow, timeline, or layers) showing
   the whole picture at a glance. Wherever there is flow, hierarchy, or
   time, a diagram carries it; prose only fills the gaps.
3. **One run**: a numbered trace of one concrete instance with realistic
   values, never `foo` or `<name>`. Plain words; any unavoidable term of
   art gets a 3-6 word gloss in parentheses right where it appears.
4. **The tricky parts**: 1-3 bullets unpacking only the genuinely complex
   steps, 1-2 plain sentences each. An analogy is welcome.
5. **Closer**: "What this shows: ..." then name 2-3 steps worth diving into.

## Rules
- Clarity beats brevity, brevity beats completeness: scenario ~15 lines,
  whole response under ~35. Over budget means the scenario is too broad.
- Readable by a smart non-expert. If a line needs domain knowledge to
  parse, rewrite or gloss it.
  Bad:  "gradients all-reduced across 64 GPUs, clipped at norm 1.0"
  Good: "the 64 GPUs average their correction notes; oversized ones capped"
- One scenario only. Use a boundary case (failure, empty input, the limit)
  when it teaches more than the average case.
- Commit to invented details without hedging. A wrong concrete detail is
  correctable; a vague one is not.

## HTML mode
Use when visuals genuinely beat monospace text: UI prototypes, or any
explainer that wants real diagrams, timelines, or layered views.
- A clean one-pager: plain-language headline, the flow diagram front and
  center, the run as a timeline, tricky parts as callout cards.
- Design: generous whitespace, readable type, one accent color, no clutter.
- One self-contained file, no external dependencies.
- Write to /tmp/example-<slug>.html and `open` it.
- For build asks ("how would you build a CRM"): a minimal clickable
  prototype with realistic fake data, built for redlining.

---
name: ingestion-points
category: system
description: Sweep Prannay's inboxes (Desktop screenshots, Downloads, meeting
  notes, statements) into the right zones and log everything in the personal
  database. Use on /ingest, "sweep my inbox", "clean up my desktop/downloads",
  or "file these".
---

# Ingestion Points

The inboxes fill up; this skill empties them. Desktop and Downloads are
INBOX ONLY (zone convention; ~/MAP.md is deferred, the routing table below is
authoritative until it returns). Every sweep: classify each item, move it to
its zone, and write one `ingest_log` row per item in
`~/System/personal-db/personal.db` (see the personal-database skill).

## Iron rules

- Never delete. Move or leave; deletion is the user's call.
- Every moved file gets an `ingest_log` row: source, original_path,
  destination, one-line summary.
- Unsure where something goes: leave it in place, log it with
  destination NULL, and list it for the user at the end.
- Statements and financial documents route to the financial-analysis flow,
  not just a folder.

## Sources (v0)

**Desktop screenshots** (CleanShot PNGs): move to
`~/System/personal-db/files/screenshots/YYYY-MM/` by file creation month.
When asked for a deep sweep, also OCR-skim each image (Read the file) and put
the one-line content summary in the log row so screenshots become searchable.

**Downloads**: classify by zone purpose, first match wins: invocable tool →
~/Tools; runs on a schedule or grows → ~/System; internet-facing site →
~/Sites; code being built → ~/Developer; papers/PDFs of research →
~/Research; life/business docs → ~/Documents; installers and disk images →
list for deletion approval.

**Meeting notes (Granola)**: location of exported notes is not yet verified;
on first run, discover where Granola stores readable notes, record the path
in this skill, then ingest new ones as `notes(kind='meeting')` rows.

**Statements** (bank/card PDFs or CSVs found anywhere): move to
`~/Documents/Important/statements/` and tell the user to run
financial-analysis, or run it directly if the user asked for a full sweep.
NOTE: ~/Documents/Important is uchg-locked (immutable) and statements/ does
not exist yet; until Prannay unlocks it and creates the dir, leave statements
in place and log them with destination NULL.

## Report format

End with: items moved per source, rows logged, the unsure list, and one
suggested next action. Keep it under 15 lines.

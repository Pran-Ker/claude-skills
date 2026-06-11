---
name: financial-analysis
category: system
description: Parse bank/card statements into the personal database and answer
  money questions (monthly burn, category breakdown, runway, budgets). Use on
  /finance, "analyze my spending", "what's my burn/runway", "import this
  statement", or when ingestion-points finds financial documents.
---

# Financial Analysis

Two jobs: get statements into `transactions` (personal-database skill), and
answer money questions from that table. Never answer from a statement
directly; import first, then query, so every number is reproducible.

## Import flow

1. Locate statements: `~/Documents/Important/statements/` first, then
   `~/Documents/{Business,tax,Work}` for strays (PDF or CSV).
2. Parse each into rows: date, amount_usd (negative = spend), account,
   merchant, category, statement_path. PDFs: Read the file and extract the
   transaction table; CSVs: map columns.
3. Categorize by merchant with a small stable set: housing, food, transport,
   software, infra (servers/APIs), health, travel, income, transfer, other.
   Reuse categories already present in the table before inventing new ones.
4. Insert with `INSERT OR IGNORE` (the UNIQUE constraint dedupes re-imports).
   Report: rows inserted vs skipped per file.

## Answering questions

- Burn: monthly spend totals, last 6 months, excluding transfers.
- Breakdown: spend by category for a requested period, biggest movers vs the
  prior period.
- Runway: requires current liquid balance; if not provided, ask for it, then
  balance divided by 3-month average burn.
- Output: one compact table, one line of interpretation, and the SQL used in
  a collapsed code block so results are checkable.

## Cautions

- Statements contain account numbers: never copy them into reports; merchant
  and last-4 only.
- Amounts in other currencies: store the USD value and note the original in
  the merchant field, e.g. "AWS (EUR 42.10)".
- Tax-relevant items (under ~/Documents/tax or matching tax keywords): flag
  them in the report, do not move the source files.

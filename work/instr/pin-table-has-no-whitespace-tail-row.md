---
id: pin-table-has-no-whitespace-tail-row
kind: issue
title: cut_line_pin's TABLE has no row whose date carries a non-space whitespace tail
status: open
opened: 2026-09-11
---

Routed here by the CIW orchestrator rather than filed by the lane that
found it: `tools/*` is INSTR's (`work/instr/program.md`'s `paths`), and
`docs/prompts/implementer-discipline.md` §6 says a lane reports a
finding outside its fence and the party with the whole board places it.

## What

`tools/tess-lint/tests/cut_line_pin.rs`'s `TABLE` pins the two halves'
readings of a cut line against each other. CIW's
`cut-regex-unanchored-admits-a-line-the-lint-refuses` anchored
`scripts/tess_budget_cut.sh`'s `CUT_RE` at both ends, and the tail it
chose is `[^ ]*$` rather than `[^[:space:]]*$`. That choice is
load-bearing in the direction the pin exists to catch, and **the table
has no row that exercises it.**

Measured by the orchestrator, four candidate anchors against five real
line shapes:

| line | `[^ ]*$` (shipped) | no anchor (before) | `$` | `[^[:space:]]*$` |
|---|---|---|---|---|
| `# tess-budget-cut: 87edbba62 2026-09-10T20:56:22+00:00` | accepts | accepts | REFUSES | accepts |
| `# tess-budget-cut: 1a2b3c4 2026-08-30 extra` | refuses | ACCEPTS | refuses | refuses |
| `# tess-budget-cut: 1a2b3c4 2026-08-30\textra` | accepts | accepts | refuses | REFUSES |

The first row is the repository's own committed baseline, so `$` alone
is not available. The third row is the one that matters here:
`tess_lint::split_cut` splits the text after the prefix on **spaces**
(`rest.split(' ')`, exactly two fields, `date.as_bytes()[..10]` checked
and every later byte admitted), so it READS that line as a cut. A
regex tail of `[^[:space:]]*$` refuses it — and the script's
already-stamped test would then miss, so a stamped file falls into the
backfill arm and is **re-stamped from the commit that wrote the stamp,
a whole commit newer than its rows.** That is the re-stamping
direction, which is the damaging one.

## Why it is a row and not a note

The property is pinned **shell-side only**, by an assertion CIW added
to `tess_budget_cut.sh --selftest`. The truth table is the one place
the two languages' answers are written side by side, which is what
made the original asymmetry visible at all — and on this boundary it
is silent. A `[^[:space:]]` mutant survives the pin and dies only in
the shell selftest.

## The fix

One row: `("# tess-budget-cut: 1a2b3c4 2026-08-30\textra", reads =
true, recognises = true, <case string>)`. Both halves accept it today,
so it is green on arrival and reds on exactly the mutant that has no
other reader.

Worth pairing with a second row for the **trailing-space** boundary
(`# tess-budget-cut: 1a2b3c4 2026-08-30 `, an empty third field) —
both halves refuse it today, it is a distinct boundary from `extra`,
and it is the one a copy-paste actually produces.

## Why CIW did not take it

CIW's lane crossed into `tools/` for the edit the pin's own
`TRAILING_CASE` alarm invited **by name** (set `recognises` to false,
delete `TRAILING_CASE`) plus three sentences those edits falsified.
Adding a NEW row is past that invitation, and this program's file.

## Provenance

CIW's `cut-anchor` unit (PR 2324, 2026-09-11) and its style review.
One related repair IS in that PR because that PR caused it:
`EXECUTABLE_SPELLINGS` at `cut_line_pin.rs:437` went stale when the
script's executable prefix sites went from 4 to 7.

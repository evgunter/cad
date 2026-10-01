---
id: tess-budget-baseline-drifts-from-mains-own-numbers-between-cuts
kind: issue
title: The committed tess-budget baseline drifts from main's own numbers between cuts, because the gate reads only the sizing columns
status: open
opened: 2026-09-21
priority: P3
cost: D
---

## What

`docs/tess-budget-data/tess-budget-baseline.csv` carries columns the
gate does not read. `tools/tess-lint` compares triangle counts, the
sizing columns and the chart/trim-box columns its per-face join checks
itself against; the **certified block** — `muu`, `muv`, `mvv`, `mu1`,
`mv1`, `worst_cert` — and `realized_aspect` are carried and never
compared. So between one cut and the next those cells drift with main,
silently, and the drift is then attributed to whichever branch happens
to re-cut the file.

**Measured.** The baseline committed at `5a0048b70e5e` (2026-09-15)
against a `--sizing-only` sweep of main at `ed93c4cde7` (2026-09-21,
3 367 commits later), 1 605 rows, every column but `worst_dev` and
`dev_samples` compared cell by cell:

| column | cells changed | main tighter | main LOOSER | worst relative move |
| --- | --- | --- | --- | --- |
| `muu` | 0 | — | — | — |
| `muv` | 21 | 9 | 12 | `6.35e-13` (`lily/lily_leaf_c` f2 δ=2e-3) |
| `mvv` | 18 | 8 | 10 | `8.65e-14` (`lily/lily_leaf_c` f4) |
| `mu1` | 20 | 11 | 9 | `1.57e-14` (`lily/lily_sepal_b` f5) |
| `mv1` | 17 | 5 | 12 | `1.17e-14` (`lily/lily_sepal_b` f6) |
| `worst_cert` | 22 | 10 | 12 | `2.15e-13` (`lily/lily_leaf_c` f2) |
| `realized_aspect` | 7 | 7 | 0 | `3.47e-16` (`lily/lily_sepal_b` f9) |

Every moved cell is on a `lily` face, every move is `≤ 6.4e-13`
relative, and **55 of the 98 certified cells moved LOOSER** — up, for
a certified sup. No gated column moved: `triangles`, `cells`,
`grid_cells`, `patch_cells`, `opt_cells`, `span_opt_cells`, `nu`,
`nv`, `bands`, `cap_bands`, `snap_bands` are identical, which is why
`tess-lint` reported nothing across those 3 367 commits.

## Why it matters

Not because the numbers are wrong — they are somebody's correct
answer, `≤ 6.4e-13` apart, and the direction is not a soundness
question. It matters because **the drift is invisible until a lane
re-cuts the file, and then it is charged to that lane**. That is
exactly what happened at RING-2 (PR #3032): its cut commit and its PR
body named RING-2 as the cause of every moved cell, and the honest
split, measured with only `ring_interval.rs` swapped back at the same
head, is

| | certified columns | `cap_bands` | `realized_aspect` |
| --- | --- | --- | --- |
| RING-2's own share | 80 rows each, **all tighter, 0 looser** | 1 (`lofts/loft_prism` f4 δ=6e-3, `1 → 0`) | 0 |
| main's drift above | 98 cells, 55 of them looser | 0 | 7 |

and a spec that says "any looser row is a finding: stop, characterise,
file" then fires on somebody else's rows.

## Candidate repairs

1. **Compare the carried columns too**, at a relative tolerance, as a
   REPORT rather than a gate — the drift is real and small, and a
   report is what tells the next re-cutter which cells were already
   moving. `tools/tess-lint`'s rule set is where it goes.
2. **Re-cut on a schedule** rather than only when a lane's change
   reaches the file, so the committed numbers are always main's own
   and a lane's diff is a lane's diff.
3. **Drop the uncompared columns** from the committed file, if nothing
   reads them. They are the writeup's figures, so this is a question
   about who reads `docs/tess-budget-data/`, not a mechanical one.

Related: `work/instr/tess-lint-ungated-columns-fold-silently.md`, which
is the same instrument seen from the rule side.

## Disposition

INSTR's: `docs/tess-budget-data/` and `tools/tess-lint` are this
program's ground. Filed by RING-2 (SCALAR), whose own re-cut is what
made the drift visible, and which left the baseline's cut line where
it is.

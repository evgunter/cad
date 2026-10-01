---
id: gate-reads-roster-has-two-copies-in-ciw-files
kind: issue
title: scripts/tess_budget_sweep.sh's statement of what the budget gate reads is stale against tools/tess-lint's roster
status: open
opened: 2026-09-17
priority: P4
cost: E
---


Filed by INSTR unit 5 (`tess-lint-ungated-columns-fold-silently`),
whose diff made it stale. The script is CIW's
(`work.py territory`), so the finding is filed rather than fixed.

## What

`tools/tess-lint`'s module docs carry the roster of what the budget
gate does with each column of the sweep CSV. Two CIW-owned files carry
their own statements of it. The `ci.yml` copy left with the budget
step's move to the nightly; the sweep script's remains.

### 1. `scripts/tess_budget_sweep.sh` — stale as of INSTR unit 5

At the `--sizing-only` paragraph:

> the gate reads triangle counts, the sizing columns and the
> chart/trim-box columns its per-face join checks itself against —
> never `worst_dev`, so paying for the resampling there would buy
> nothing.

**The list is now short by six.** `tess_lint::parse` reads and refuses
`muu`, `muv`, `mvv`, `mu1`, `mv1` and `cells` as of unit 5: five
through `BOUND_COLUMNS` and the cell count through the `usize` read.
The sentence's CONCLUSION is untouched — the gate still compares none
of the deviation columns, so `--sizing-only` still buys what it claims
— which is what makes this the quiet kind of stale: the reason is
still sound and the roster under it is not.

`tools/tess-meter/src/lib.rs`'s `DEV_SAMPLES` doc defers to this
sentence (*"`scripts/tess_budget_sweep.sh` says so at the flag"*), so
it inherits whatever this one says. Checked and NOT filed separately:
it makes no roster of its own.

## Fence

`scripts/tess_budget_sweep.sh`, CIW's. The roster itself is
`tools/tess-lint`'s module docs and stays there.

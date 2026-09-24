---
id: gate-reads-roster-has-two-copies-in-ciw-files
kind: issue
title: Two CIW files state what the budget gate reads: the sweep script's copy is stale, and ci.yml's says the roster lives nowhere else in the file that carries it
status: open
opened: 2026-09-17
priority: P4
cost: E
---


Filed by INSTR unit 5 (`tess-lint-ungated-columns-fold-silently`),
whose diff made the first of the two stale. Both files are CIW's
(`work.py territory`), so the finding is filed rather than fixed.

## What

`tools/tess-lint`'s module docs carry the roster of what the budget
gate does with each column of the sweep CSV. Two CIW-owned files carry
their own statements of it, and each is wrong in a different way.

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

### 2. `.github/workflows/ci.yml` — false in its own file

The tessellation-budget step's comment:

> What this catches is ROSTERED IN `tools/tess-lint`'S MODULE DOCS AND
> NOWHERE ELSE, this file included — no count here, and no list.

and then, ~30 lines below it in the same comment block:

> what `compare` COMPARES is triangle counts per scene and
> `grid_cells / span_opt_cells` per face, and what it JOINS on is
> `chart`, whether the row carries the sizing block at all, and
> `u0`-`v1` / `nu` / `nv`.

That second passage is a list of what the gate reads, in the file that
has just said the list is nowhere else, this file included. The two
are not the same roster word for word — the first sentence is about
the RULES and the second about the COLUMNS — and that is the whole of
the defect: the prohibition is written wide enough to cover what the
file then does, so a reader who trusts it looks in one place and a
reader who does not finds two. It is also the copy most likely to rot,
since `compare`'s join inputs are the thing `C15` is expected to
change.

**The sharper half is that the sentence's own argument is right.** Its
paragraph explains that an enumeration kept beside the thing it
describes drifts at that thing's rate, and cites the count that read
THREE for as long as rule 4 existed. The fix is to make the file obey
it, not to soften it.

## Shape

The same shape as `cut-script-header-claims-no-cross-language-gate-exists`
and `cut-regex-unanchored-admits-a-line-the-lint-refuses`: a CIW-owned
file stating a property of `tools/tess-lint`'s reader that the reader
does not have. Those two are about the cut line; this is about the
column roster.

## Fence

`scripts/tess_budget_sweep.sh` and `.github/workflows/ci.yml`, both
CIW's. The roster itself is `tools/tess-lint`'s module docs and stays
there.

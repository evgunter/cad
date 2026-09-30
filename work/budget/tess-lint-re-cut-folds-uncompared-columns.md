---
id: tess-lint-re-cut-folds-uncompared-columns
kind: issue
title: A re-cut folds every column the gate reads but compares against nothing, and the fold has no diff
status: open
opened: 2026-09-16
refs: [tess-lint-ungated-columns-fold-silently, C15]
priority: P3
cost: D
---


Arm 3 of `tess-lint-ungated-columns-fold-silently`, split off when that
row's arms 1 and 2 closed and this one did not. The parent row's first
two arms were about what the boundary REFUSES; this one is about what
nothing COMPARES, which is a different fix and a bigger one.

## What

`tess-lint` reads every column of `EXPECTED_HEADER`, and what becomes
of each after that is four different things — the roster is in the
crate's module docs (`tools/tess-lint/src/lib.rs`, *"Every column of
`EXPECTED_HEADER` is read at that boundary"*), which is its home now
that the parent row is closing. The second bucket is this row: `delta`
and `worst_dev` reach the report's `total` factor, `patch_cells` and
`opt_cells` reach the printed cell totals, and the indicator block
reaches the constraint-activity line. Each is printed on the run that
reads it and held against no other run.

`worst_cert` is worse than that and belongs here too — it is the
roster's fourth bucket by itself: parsed, policed, stored in `Nurbs`,
and read by nothing at all, not a rule and not the report.

**The hazard is not that they move; it is that a re-cut is the only
event that reads them across two trees, and it does not report.** A
certified bound that moves on a face the gate COVERS is invisible until
somebody re-cuts, and invisible again afterwards because the fold has
no diff. That is `docs/TESS-BUDGET.md`'s *"Coverage restored is not
coverage verified"* on a scene the baseline does cover, which that
passage does not reach.

**The measured instance.** METER unit 6's re-cut moved, on a corpus
whose rows and scenes did not change at all: `muv` on 15 rows (largest
relative move 38.10%, `lofts/nonuniform_loft` face 4), `mvv` on 14,
`mu1` on 12, `mv1` on 13, `worst_cert` on 16, `worst_dev` on 12 (1.11%,
`lily/lily_leaf_a` face 4) and `realized_aspect` on 8 — all but
`worst_dev`'s last-digit or larger, none of them the unit's doing.
Every one is `m.<field>` copied straight from `mesh::budget`'s
measurement in `tess_meter::columns`; what moved them is the kernel,
over the 442 commits under `crates/` between that cut's predecessor
(`aba2625f8f84`, 2026-09-04) and it.

**A second consumer is affected and it is not obvious from here.** The
sizing census — `the_committed_baseline_sizes_this_much`, in
`tools/tess-lint/tests/baseline_census.rs` — reads totals only, so
every mover above is invisible to it; its "fourth thing a re-cut can
be" says a re-cut whose only movers are `opt_cells` and
`span_opt_cells` is an instrument-resolution change and never a
schedule regression, and the census cannot tell whether "only" holds.

## Why it did not close with arms 1 and 2

Three things, and the third is the one that decides the shape:

1. **It is a report, not a gate, and a threshold would be the wrong
   shape.** The meter never gates on geometry; a certified bound that
   grew is not a finding. `tools/README.md`'s `CC5` puts this the other
   way round — an admission never refuses what the instrument exists to
   measure.
2. **The five certified-sup columns are not stored.** Arm 1 admits them
   at the boundary and DROPS them, because storing a number no rule
   reads is the defect arm 2 named rather than a cure for it. A diff
   needs them stored, so this row owes an argument for a reader before
   it adds one.
3. **Who the diff is for decides when it prints, and that is unsettled.**
   The gate's fresh CSV is a new cut on every run that sweeps, so a
   diff printed beside the verdict prints on every green run — and the
   movement table above says most of what it would print is last-digit
   noise. The audience the parent row names is *a lane re-cutting the
   baseline*, which is a different event from *a gate run*, and
   `tess-lint` cannot currently tell them apart: both arrive as
   `<fresh> --baseline <committed>`. Settling that is the first move
   here, not the diff.

## Fence

`tools/tess-lint/*`. INSTR's.

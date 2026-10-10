---
id: sweep-keeps-computed-values-in-t
kind: unit
title: from_f64 audit, sweep: every listed launders site keeps its value in T or enters it through from_computed
status: closed
opened: 2026-10-10
priority: P0
cost: M
refs: [carriers-compare-in-canonical-form]
branch: intent/from-f64-sweep
closed: 2026-10-10
---


Unit 2–5 of the `from_f64` audit (`a-computed-value-re-enters-as-a-constant`
is unit 1, which landed the gate, the census and the re-valuation row).

Every `launders(sweep-keeps-computed-values-in-t)` entry in
`scripts/gates/from-f64-sites.tsv` is a value this crate computed and
re-enters through `Real::from_f64`, so at `Sym` it is a constant where
the computation had a function of the variables. For each, either keep
the value in `T` (the site read it out of `T` and back), or, where a
numerical routine produced it in `f64` (a projection foot, a root, a
Newton iterate, a certified hull bound, f64 box arithmetic), enter it
through `Real::from_computed`, one unknown per call. Each fix deletes
its entry; the gate's STALE rule holds the list to the tree.

Pins: the corpus row (`crates/editor-core/tests/revalue_corpus.rs`)
lists no laundering today, and none of its documents was shown to
reach a site this unit owes. A fix here therefore has no corpus pin
to move, and each fix owes its own: a unit row at `Sym<f64>` with the
site's inputs as parameters, asserting that what it returns re-values
with them (`chart_region.rs`'s `the_wrap_band_keeps_its_corners_in_t`
is the shape), red on the code before the fix. A move to
`from_computed` can only lose symbolic `Zero`s, never geometry: say
which `symbolic_zero` counts moved.

## Closed

Every `launders(sweep-keeps-computed-values-in-t)` row is gone from
`scripts/gates/from-f64-sites.tsv`; each value enters through
`Real::from_computed`, lifted once and shared by its readers:
- `blend/reach.rs`'s `Cell::new`: a reach cell's six corner bounds
  (pin `a_cells_corners_enter_as_computed_values`);
- `blend/surgery.rs`'s `boxed_reach`: a carrier's certified box bounds
  (pin `a_boxed_reach_reads_its_box_as_computed_values`);
- `loft.rs`: each section placement, lifted once in `build` for the
  stacking fold and the assembly, and the skinned walls' control nets
  (pin `a_lofts_placements_and_walls_are_computed_values_at_sym`).

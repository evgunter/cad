---
id: the-margin-twins-read-their-sample-streams-by-hand-in-each-file
kind: issue
title: The margin-scaling twins each hand-copy the sample-stream reader (outcomes, decisive) beside k_stats' one band rule, and pair the twins' lists by zip
<<<<<<< HEAD
status: open
opened: 2026-11-09
priority: P3
cost: E
=======
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-09
pr: 6627
>>>>>>> origin/main
---


Filed from PR 6597's FULL review (its style Q1 and note N2).

## What

PR 6597 gave the sign-only rule one home, `geom_core::k_stats::MarginSample::reads_sign_only`. The *reading* of a twin's sample streams still lives in each twin, hand-copied:
- the `(SampleOutcome, Option<f64>)` stream type, and the `outcomes` and `decisive` closures, appear in `sweep/tests/ray_wall_margin_twins.rs:132-156` and `topo/tests/rim_dim_boolean_twins.rs:174-213`. PR 6597 edited both copies identically.
- `geom-brep` `rim_dim_scale_twins.rs:447-492` and `span_meter_dim_twins.rs:128` build their own scale pairings over named lists.

**N2.** Each twin pairs its `decisive` lists with `zip` (ray_wall:158-161, rim_dim:224-226). If a predicate's sign-only mask differed between the mm and m twins, `filter_map` would give lists of different lengths, and `zip` would silently drop the excess. The masks are equal today but are not compared.

## What closing it takes

1. One test-support reader of a pair of sample streams: outcomes, decisive margins, and the sign-only filter.
2. The reader asserts that both twins' masks are equal before pairing.
3. All four twins read it.

## Also from the review, to fix on the way

- The `rim_dim_boolean_twins.rs:3-7` module doc still claims every margin scales.
- The ciw row (`work/ciw/probe-gated-lib-unit-tests-never-run-in-ci.md:18` vs `:20`) holds two paragraphs that disagree.
- The closed twins row's title still says "unseen".

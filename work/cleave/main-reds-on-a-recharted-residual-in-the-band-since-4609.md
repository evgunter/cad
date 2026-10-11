---
id: main-reds-on-a-recharted-residual-in-the-band-since-4609
kind: issue
title: Main is red on split_through_an_apex::a_recharted_residual_in_the_band_refuses_typed since PR 4609; the change filter skipped topo there
status: open
opened: 2026-10-11
priority: P0
---


`topo::all split_through_an_apex::a_recharted_residual_in_the_band_refuses_typed`
panics at `crates/topo/tests/split_through_an_apex.rs:332` with
`Ok("a body")`: the split now builds where the row expects a typed
refusal.

Bisected on origin/main alone, by the INTENT nurbs-weights lane (PR 4608,
whose CI selected topo's suite and is red on this one test, 12 686 of
12 687; run 38105861240):

- passes at 0d237d92bd (PR 4609's first parent);
- fails at f42ca24d64 (Merge PR 4609, `cleave/band-on-pinch-residue`:
  "split: chart a section in its plane; SliverVertex says undecided"),
  at 65b93eedf1, and at 3d2f75e8df.

Main's own CI stayed green because the per-PR change filter did not
select topo's suite on PR 4609's merge. Every PR whose filter does select
topo is red on this row until it is fixed.

Owed: CLEAVE decides whether the row's expectation (a typed refusal) or
the new build is right, and either fixes the split or re-pins the row
with its reason. A second question for whoever owns the change filter:
why PR 4609's topo change did not select topo's suite.

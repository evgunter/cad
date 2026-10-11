---
id: main-reds-on-a-recharted-residual-in-the-band-since-4609
kind: issue
title: Main is red on split_through_an_apex::a_recharted_residual_in_the_band_refuses_typed since PR 4609; the change filter skipped topo there
status: closed
opened: 2026-10-11
priority: P0
closed: 2026-10-11
pr: 4631
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

## Closed (PR 4631, 2026-10-11)

PR 4609's in-plane projection of `section_loops::chord_u_ref` removed the
re-chart residual this pin expected, so P123 now splits. Reverting
`chord_u_ref` locally brings the old `ChartResidual` refusal back. PR 4631
replaces the pin with `a_notch_floor_in_the_band_splits_sound`. P123 passes
tiers 1–3, disjoint corners, volume conservation and the closed form in
every cell (3 ε × 5 poses × both normals). It also closes the TOPO re-chart
row: across the review fuzz seeds, every re-chart refusal now splits sound.
The bodies' tier-3′ census escalates (in band, not falsified), the same as
the file's existing sound row; that is filed as RESTREAD P2
`a-split-whose-notch-floor-reads-on-at-0-9-eps-ships-bodies-whose-census-escalates`.

---
id: split-refuses-a-convex-graze-of-a-curved-wall
kind: issue
title: a plane grazing a cylinder's wall from outside refuses DegenerateSection where an answer exists
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## What

A plane touching a cylinder's wall from outside, along a ruling or
along its seam, has all the material on one side. Under Ev's ruling
on PR 3642 the split should land the whole body there, as it does for
a planar edge contact. Instead it refuses
`SplitJoinError::DegenerateSection`.

Repro: the two-arc disc extruded to height 1 (radius 0.5), split by
y = 0.5 (a ruling) or x = 0.5 (the seam), with either normal.
`crates/sweep/tests/split_tangent_edge_curved.rs`,
`a_convex_graze_of_a_cylinder_refuses_on_area`, pins the refusal. It
flips when this row is fixed. `m5_pr5_tilted_cut::exact_graze_refuses_typed`
and `m5_pr9_sector2::the_tangent_graze_resolves_past_first_order` pin
the same refusal.

## Why

Rule (b) (`crates/topo/src/splitting/rules.rs`, `apply_rule_b`) sends
an S-ON-S entry across the plane unless the dihedral of its own edge
reads convex:
- the ruling graze reaches it through the cap rim's straight-sector
  bisector duplicate;
- the seam graze reaches it through a smooth edge.

Neither carries a corner. "Opposite" there is a safety default, not a
derivation: it mints the null edge, so the graze refuses at the
join's zero-area net.

## The constraint on a fix

A fix must read the wall's material convexity, a second-order fact
(which way the wall curves relative to its material), and never just
flip those arms. Flipping them answers the convex graze, but it also
answers a round hole grazed from inside with closed halves that put
the hole on the wrong side: 10.0 / 5.2146 against a truth of
9.2146 / 6.0 (PR 3726 review). The guard is
`crates/sweep/tests/split_tangent_edge_curved.rs`,
`a_concave_graze_never_answers_with_the_hole_on_the_wrong_side`. It
accepts a refusal and only fails on a wrong answer.

## Found by

PR 3726's review.

---
id: a-curved-whole-reading-spends-its-cell-budget-before-refusing-a-near-touch
kind: issue
title: a curved partner's whole reading spends the accounting's whole cell budget (about 7 s) before refusing a top within a few epsilon of the partner, where stage 2 declined at once
status: open
priority: P3
cost: M
opened: 2026-11-03
---

Found by PR 6412 (NURBS × implicit stage 3a) and its dual review (both
lanes).

## What happens

Where the regularity sweep declines a curved partner's pose because a
top of `φ` stands within a few ε of the partner, the pose is now read
whole, and the whole reading's accounting refuses only when its cell
budget (`SSI_MAX_CELLS`, `crates/geom-brep/src/ssi/exhaust.rs`) runs
out: about 7 s per refusal in release. Stage 2 declined the same poses at
once. Rows:
`crates/geom-brep/src/pair_section/spline_implicit/curved/rows.rs`,
`a_top_in_band_of_a_sphere_is_not_decided` and
`a_touch_with_a_large_sphere_at_a_side_is_no_exclusion` (both in the slow
set).

## Why

The accounting's exclusion is a zero test on the form over the wall's
box (`crates/geom-brep/src/ssi/implicit_chart.rs`, `Lane::excluded`).
The section there is empty, but the gap to the partner is a few ε, and
boxes that close on the top only as fast as their cells shrink cannot
clear it before the budget is spent.

## What closes it

The whole reading not entered for a pose whose sweep declined at a
critical point decided in band (the sweep already names the cluster),
or an exclusion that reads the top's cluster on the sweep's Krawczyk box,
so the refusal is reached without spending the budget.

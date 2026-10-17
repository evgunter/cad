---
id: a-split-joint-on-a-fitted-grade-row-has-no-reachable-non-identity-witness
kind: issue
title: split_edge's joint between the children of a fitted-grade row decides a reset only on a u-degenerate spline chart, so no reachable fixture pins it
status: open
opened: 2026-10-12
priority: P3
cost: M
refs: [split-edge-cannot-carry-a-fitted-or-general-pcurve-row]
---

Found by PR 4410's dual review (mutant `skip_joint`, which writes
`JointElement::IDENTITY` in place of `split_cache`'s joint decision,
survives every row of `crates/sweep/tests/split_edge_fitted_rows.rs`).

Why it survives: `split_cache` (`crates/topo/src/pcurves.rs`) decides
the joint between a split's two children at `t`, where both hold the
same image, so `decide_joint`'s gap is zero and its marks count no
period. What is left to decide is the element's KIND, and
`AzimuthRule::of` gives a reset only on the singular set (or, on a
spline chart, where that is undecided). A fitted-grade row lives only on
a spline chart (check 1 refuses one on an analytic chart), and there
`singular_at` reads the net-level lever `sup |S_u|` over the WHOLE
chart. So a fitted-grade split writes anything but the identity shift
only on a `u`-closed spline chart whose whole `u` stretch is within the
band: a chart with no extent in `u`, which carries no face a fixture
reaches. On every other chart the mutant is equivalent.

What a row would need: such a degenerate `u`-closed spline chart that
still stores a fitted-grade row at rest, or a change to the spline
singular test that reads the lever at the vertex (as the analytic
charts do), after which a split at a collapsed column would decide a
reset. The second is the one with a payoff; this row is its witness,
not a defect in the carry.


---
id: a-box-corner-just-off-a-cone-apex-refuses-box-first-edge-undescribed
kind: issue
title: A box corner within a fraction of the band of a cone's apex refuses EdgeUndescribed box first, while cone first builds
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the DUAL review of PR 6345 (SPLITDOOR unit B stage B1),
reviewer R1, and filed by its fix pass. Not reproduced by the fix pass,
which was short of R1's harness.

## What

The 45° wrap-tipped cone against a box of side 2 cornered within
0.3–0.9ε of the cone's apex, one box edge down the axis. The box is
turned a quarter about `x`, and its corner is moved off the apex along
each of the probe's five directions. Box first, the ops refuse
`EdgeUndescribed { … AnalyticRung3, KernelLimit }`; cone first, they
build.

Reproduce: `r1_edge_through_and_near_the_apex` in
`crates/sweep/tests/r1_probe_cone_apex.rs` on
`splitdoor/cone-apex-vertex-pairs-review-R1` (`ea5d8c0128`). It is
`#[ignore]`d, so run it with `--run-ignored all`.

## Where to look

Why the order matters. Box first, the box's edges near the apex are
swept against the cone face before the vertex pair reads the corner.
An edge whose end lies within the band of the apex reads its double
root there only where `bool_line_cone_apex_spread` is in the band
(`boolean::reduce::line_cone_roots`). Otherwise the line lane refuses
`AtApex`, and the edge is left to a later rung, which may be the one
refusing. Cone first, the apex vertex is placed against the box's
faces first. Trace which rung answers `EdgeUndescribed`, and whether a
corner in the band should be read as at the apex, the vertex pair's,
in both orders.

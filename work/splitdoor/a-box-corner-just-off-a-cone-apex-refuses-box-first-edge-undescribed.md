---
id: a-box-corner-just-off-a-cone-apex-refuses-box-first-edge-undescribed
kind: issue
title: A box corner or edge within a fraction of the band of a cone's apex refuses EdgeUndescribed box first, while cone first builds
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

## The edge pose (2026-11-03, SPLITDOOR B2 fix pass)

The DUAL review of PR 6424 (stage B2), reviewer R1, found the same
refusal for an EDGE ending a fraction of the band off the apex, not a
corner: the 45° cone, both forms, against the box
`[0, 2] × [0.55, 1] × [±0.3ε, 2.3]` turned `0.4` about the axis, its
edge along `x` at `z = ±0.3ε` passing three tenths of a band from the
apex. Box first, `∩` and `box ∖ cone` refuse `EdgeUndescribed`
(`AnalyticRung3`'s `KernelLimit` on the cone, a bound of about `2e-9`);
cone first, both build. Same rung, same order dependence, so it is
folded in here rather than filed apart.

It is pinned as it stands, both orders:
`crates/sweep/tests/a_cone_apex_on_an_edge.rs`,
`an_edge_ending_just_off_the_apex_refuses_box_first_and_builds_cone_first`.
That row reproduces the class on the main tree, which the corner pose
above needed R1's harness for. When the fix lands, the row's box-first
arm flips to building, and the corner poses can join it.

— SPLITDOOR


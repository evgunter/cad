---
id: support-screen-skips-adjacent-boundary-features-wherever-they-approach
kind: issue
title: blend: the support screen skips a pair of boundary features that share a vertex wherever they approach, so a curved feature that comes back toward its neighbour is never screened
status: open
opened: 2026-10-07
priority: P3
cost: E
---


Found sweeping for `blend-reach-skips-every-face-at-a-chain-vertex`'s
class (excused by adjacency, where the excuse holds only near the shared
vertex).

The support screen in `crates/sweep/src/blend/battery.rs` (the pair loop
over a support's `boundary` features, after `screened_loop`) skips a
pair of boundary features when they share a vertex (`touches`): their
setbacks there are judged by the corner and G1 predicates. Two straight
features through one vertex approach each other only at it, so that is
sound for lines. A curved feature (an arc) can come back toward its
neighbour away from the shared vertex, within the two setbacks, and the
pair is never screened.

Measure first: a support whose boundary has a line and an arc sharing a
vertex, the arc bulging back to within `setback_here + setback_there`
of the line away from it. If the corner/G1 predicates or the reach
refuse it, pin that and close.

Unreachable through any plane–plane band today (2026-10-07): an arc
that shares a requested line's vertex on a support is bounded there by
its own wall, and that wall is the end face at the vertex, so the
battery refuses it as a curved end face (`END_FACE_CURVED`) before the
screen's verdict matters, and the reach fails loud on it if read alone
(`blend_band_reach_chain_ends::a_curved_end_face_fails_loud_at_the_meter`,
a line `AB` whose arc at `B` comes back to `0.45` of it). A turn's and
a joint's boundary features are all lines. Measure this when curved end
faces are built, not before.

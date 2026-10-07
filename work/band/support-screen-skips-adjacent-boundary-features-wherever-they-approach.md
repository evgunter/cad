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

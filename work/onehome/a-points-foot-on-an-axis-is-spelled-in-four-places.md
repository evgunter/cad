---
id: a-points-foot-on-an-axis-is-spelled-in-four-places
kind: issue
title: A point's foot on an axis line is spelled inline in four places across geom-brep, topo and sweep
status: open
opened: 2026-10-26
priority: P1
cost: E
refs: [5793]
---


Found by the review of PR 5793 (CLEAVE, `slit.rs` riders), which folded the boolean's own copies of this reading into one helper and left the cross-crate copies for this row.

The point of the line `origin + s·axis` (`axis` unit) nearest `p` is `origin + axis·((p − origin)·axis)`. It is written out in four places:

- `crates/geom-brep/src/extent.rs:182` — `fn foot`, private, behind `Ball::foot_on`.
- `crates/topo/src/boolean/join.rs:4075` — the cylinder×sphere germ frame's `foot` of the sphere's centre.
- `crates/topo/src/boolean/slit.rs:1296` — `fn toward_axis`, the foot less the point, for the riders' bend across a ruling.
- `crates/sweep/src/blend/arms.rs:925` — the rim's `on_axis` in the round support trace.

Each is a one-line reading of an axis carrier, so they cannot drift far, but they are four spellings of one fact (D3, "One reading per question"), and the next caller writes a fifth. A route is a `pub` foot on the axis carriers' home in geom (`foot` lifted out of `extent.rs`, or a method on whichever axis type `[one-way.readings]` names), with the three other sites repointed.

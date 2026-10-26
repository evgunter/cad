---
id: a-sliver-of-more-than-two-edges-is-not-read-as-one
kind: issue
title: A sliver whose outline runs on more than two edges is not read as a sliver: sliver_halves counts exactly two
status: open
opened: 2026-10-21
priority: P3
cost: M
---


Found reviewing PR 5545 (`cleave/fate-cut-ends-at-a-slit-tip`).

## What

`slit::sliver_halves` (`crates/topo/src/boolean/slit.rs`) reads a face
as a sliver, a face of no area along a slit's line, only where its
outline is exactly two distinct straight edges and it has no rings. A
sliver whose two sides are split at different points, or at one point
on one side only (a vertex the reduction or the join puts on one of
the slit's edges and not the other), has three or more edges on that
one segment and is not read as one. Its readers then miss it:

- `join.rs` `sliver_along_a_slit`, so `resolve_roles_geometric` does
  not read the sliver Out and the loop roles refuse
  `SectionLoopUndecided`;
- `slit::merge_slivers`, so the slit's two walls are not merged across
  it.

The fate cut does not depend on it: `fate::tip_wraps` reads a spur
from the loop's turns, and a collinear corner turns by nothing.

## Witness

None built. No scene in the suite splits one of a slit's two edges and
not the other where a flush face is cut along them.

## Fix

Read a sliver as an outline whose edges all lie on one segment (each a
line edge, its ends on the segment's two points or between them) and
which turns at exactly two spurs, rather than by an edge count.

---
id: crossings-of-a-seam-edge-whose-sides-names-do-not-settle-tie
kind: issue
title: Crossings of a seam edge whose faces' names do not settle the pair's first side tie, where N2 rules only that an equal pair ties
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [edge-pieces-are-named-by-their-ends, seam-line-sides-is-a-missing-rule]
---

## What

A seam edge's crossings are ranked as the loop of the pair's first side
runs along the edge (N2). `emit_topo::crossed_edge_orientation`
(`crates/editor-core/src/names/emit_topo.rs:1895`) finds that side by
matching the edge's two faces' names against the pair
(`seam_pair::a_side_is_first`, line 1924). Where the names do not settle
it — neither face descends from a side, or both faces are merged across
both sides — it answers `None`, and the crossings tie.

N2 rules only that an EQUAL pair ties. The undecided case is a
departure, as `SeamLineSides` (retired with
`seam-line-sides-is-a-missing-rule`) was a refusal for it.

## Fix shape

Record the seam's first side structurally at its mint, in a form the
pass-throughs carry, so the orientation does not depend on names
matching. No fixture reaches it.

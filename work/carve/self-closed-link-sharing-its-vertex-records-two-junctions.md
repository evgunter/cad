---
id: self-closed-link-sharing-its-vertex-records-two-junctions
kind: issue
title: walk_chains records two junctions at one vertex and closes the chain when a self-closed link shares its vertex with one other requested link
status: open
opened: 2026-09-13
priority: P0
cost: H
---


## Finding (R1 of BLEND unit 7's dual review, `unsure`; traced, not reproduced)

`walk_chains` (`crates/sweep/src/blend/battery.rs`) counts a
self-closed link's one vertex ONCE in its incidence table, so a vertex
carrying a self-closed requested link and exactly one other requested
link reads as a junction (two links). Traced by hand on the walk as
written: seed the self-closed link `s` at `v0` (`head == tail == v0`,
`closed` from the start); the forward pass finds the junction at `v0`,
takes the other link `o` (unused), records `(v0, s, o)`, moves `tail`
to `o`'s far end and stops there if it is no junction; the backward
pass at `head == v0` finds both links used, `grew` true, and records
`(v0, o, s)` — TWO junctions at one vertex, `closure = Closed`, on a
run whose `tail` end is free. The G1 check then meters `s` against `o`
twice at `v0` and never sees the free end; a body on which `s` and `o`
are tangent at `v0` would resolve as a closed chain.

**Not reproduced through a public door in the hour budgeted.** A
self-closed rim's vertex on every body the tree mints carries, besides
the rim, only its wall's two co-surface seam meridians, and
`resolve_link` refuses those as `TangentialEdge` before the walk runs —
so no request of a rim plus one incident edge reaches the shape. It
needs a body where a non-seam edge ends at a self-closed rim's vertex
(a boolean cutting exactly through the seam vertex), which no fixture
builds today.

## Fix shape

Count a self-closed link's vertex TWICE in the incidence table (it
arrives and leaves), so the vertex reads as three links with one other
requested link — a corner, and the chain terminates by the structural
rule — or make the closing branch refuse a junction whose two links
are the run's own two end links when the run has a free end. Row it
through `test_support::walked_chains` on hand-built links if no door
reaches it.

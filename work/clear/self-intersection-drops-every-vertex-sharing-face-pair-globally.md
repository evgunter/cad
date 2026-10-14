---
id: self-intersection-drops-every-vertex-sharing-face-pair-globally
kind: issue
title: clearance's self-intersection drops every face pair that shares a vertex, and a face against itself, globally - vacuous on a cone's lateral faces or a triangle-section loft
status: open
opened: 2026-10-06
priority: P2
cost: H
design: true
---


## Finding

`editor_core::clearance`'s self-intersection candidate filter
(`crates/editor-core/src/clearance.rs`, `clearance_with`, the "wedge
rule" block) keeps a pair only when
`x.face != y.face && x.vertices.is_disjoint(&y.vertices)`. So it never
examines:

- a face against itself, and
- two faces that share any vertex, anywhere on them.

The comment beside it says so: "**The exclusion is GLOBAL where the
justification is LOCAL** … A cone's apex, where every lateral face
shares one vertex, removes the whole lateral check". It also says the
gap is "stated … in the unit's deviations". No row in `work/` scheduled
it, so this file is that schedule.

Consequences: `self_intersection` passes a body unexamined when all its
faces share vertices. Examples are a triangle-section loft, where every
wall shares a seam with both neighbours and a cap vertex with the
other, and a cone's lateral faces. Adjacent loft walls that cross away
from their shared seam can never be seen, which is the shape of
`work/carvetail/loft-between-opposite-turning-joints-reverses-a-seam-between-stations.md`
and of a roll near a half turn.

The local justification (faces that share a vertex meet there) needs a
local discharge, not a global drop. One proposed shape, from CARVE's
2026-10-06 designers: near the shared boundary, a cell pair whose normal
cone is narrower than a right angle cannot cross; away from it, the
ordinary distance test decides. The design is open (`design: true`).

## Why it matters now

CARVE's two designers on the loft's section placement
(`work/carve/self-overlapping-spines-build-and-validate.md`) both put
the loft's embedding certificate on this engine at a certifying scalar
(D1, Ev's B at #1737), after SHELL-3 moves it into `topo`. Whether they
need adjacent pairs is part of their reconciliation. Either way, the
engine's answer for any body is vacuous on the pairs this drops.

Filed by the CARVE orchestrator, 2026-10-06, from two designer reports
(read-only lanes). Not measured; read off the filter and its comment.

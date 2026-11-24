---
id: check-4-levers-a-wedge-at-the-farther-faces-run-so-a-narrow-face-inside-the-band-reads-a-crease
kind: issue
title: check 4 levers a wedge at the farther face's run, so a narrow face inside the band reads a crease
status: open
opened: 2026-11-22
priority: P1
cost: M
design: true
---

## Finding

From EMBED's design fork `embed-wedge` (fork-log row 304, PR 7456,
2026-11-22).

`crates/topo/src/wedge_reach.rs` takes the reach of an edge as the
farther face's ("the reach is the farther face's … one large face
beside a small one reads the gap the large one opens"), and check 4
levers the wedge at it. Take a flat strip 1e-7 m wide folded off a 1 m
plane at a sine of 5e-5: its far edge is 5e-12 m from the plane, so the
whole strip lies inside the band. Nothing is curved, so the fold is the
reach, 1 m, and check 4 reads 5e-5 × 1 = 5e-5: a crease. A 1 µm chamfer
tilted 1e-5 off a metre face reads a crease the same way, its far edge
1e-11 m off the big face's plane.

Ev ruled on PR 7456 that the embedding question levers its own reading
at how far the NARROWER face runs from what the faces share, measured
across the edge. Once that is built
(`work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`)
the question refuses the strip (5e-5 × 1e-7 = 5e-12) where check 4
passed it: the right verdict at the wrong door.

`reach_from_vertex` has the same shape (the farthest of both faces).
`[topology.census]` already reads a meeting pair "over the shorter
cell", and the profile's joint reading is levered at the shorter of the
two legs (`work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`
records that as Ev's ruling on #5354).

## Design question

Should check 4 take the same extent as the question (the narrower
face's run across the edge), so one recipe on the verdict serves both
readers? All three designers thought so and left it out of PR 7456 as
tier 3's own question. No README or DESIGN sentence was found that
ratifies "farther"; `crates/geom-core/README.md`'s "a lever is never
shorter than the region its verdict governs" is the nearest.

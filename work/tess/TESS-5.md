---
id: TESS-5
kind: unit
title: a rim-free loop on a chart with no pole for its meridians to end on refuses typed, where today it meshes as a hole
status: closed
opened: 2026-09-22
priority: P0
cost: D
branch: tess/5-rim-free-loop
refs: [rim-free-loop-on-a-poleless-chart-meshes-as-a-hole]
pr: 3116
closed: 2026-09-23
---


## Closed (2026-09-23, PR 3116 merged at `5bc576ce6`)

`TessellateError::SingleColumnCurvedFace { face, surface }`, raised by
`walk::require_two_columns` on the structural fact that the rim-free
arm's u-extent is the spread of the loop's iso-side openings and a
column belongs to an edge: openings on fewer than two distinct edges
give zero extent by construction. Four members closed (torus meridian
loop, cylinder generator, one-seam sphere, cone generator); the
hemisphere pair is props' branch door's and pinned as such. Mutant red
in both profiles, controls green. The reviewer reached the named
residue — two COINCIDENT edges on one great circle, sphere/cone only —
and it has its own row (`two-coincident-edges-open-two-columns-that-
are-one`, P1/D: carrier identity above edge identity) with a row
pinning today's answer. Filed/edited: that row; `lane_of`'s duplicate
check list now points at `curved`'s; the cone sentence, the ε sentence
and ~40 lines of restated rationale fixed. Middle tier: one review.

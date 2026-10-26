---
id: mesh-sizes-each-lane-by-its-worst-admissible-triangle
kind: unit
title: Build [mesh.sizing]: each lane's worst admissible triangle is sized at the coarsest step its pinned theorem grants; δ/2 and the sphere's 1.25 retire; sphere and cone rims chorded to the grid
status: review
opened: 2026-10-25
priority: P1
cost: H
refs: [S29, a-cone-rim-chord-joined-to-a-higher-row-certifies-above-delta-s]
branch: pred/mesh-sizing-rule
pr: 5717
---

Ruled by Ev on PR 5708 (`S29`). The rule is `crates/mesh/README.md`
`[mesh.sizing]`. What the unit builds:
- `sizing::sizing_target`'s δ/2 and `curved::SPHERE_SIZING_MARGIN`
  retire.
- `nurbs_cert::MAX_RESIDUAL_SQ` is written as derived from the NURBS
  lane's sliver theorem.
- Each lane's worst-admissible-triangle theorem is stated at its step
  function and pinned: the coarsest step the lane's own schedule
  admits, without extra machinery bought only to close the gap to δ.
- Sphere and cone rim chords are tightened to the face's grid step
  through the chord pass's tightening door, as the torus already is.
  This also answers TESS's
  `a-cone-rim-chord-joined-to-a-higher-row-certifies-above-delta-s`.
- Any lane whose theorem is not yet written carries a named
  `UNPROVED_HEADROOM` at its call site, with its debt filed. The
  trimmed cylinder walls are the known case.
- `crates/mesh/src/lib.rs`'s "targets δ/2 so the slack never decides"
  sentence goes.
- Each lane's constants are labelled with their kind at their
  definition.
- The tess-budget baseline and the mesh count pins re-cut, saying what
  moved. Torus and cylinder coarsen; sphere and cone coarsen once
  tightened.

The designers' hand-derived counts are not measured. Measure first
with tess-meter.

---
id: a-wall-root-bounded-by-an-oblique-section-still-walks-and-ties-at-the-nominal
kind: issue
title: A wall root bounded by an oblique section still walks its region and ties at the nominal
status: open
opened: 2026-10-27
priority: P2
cost: H
design: true
refs: [the-side-readers-aims-decide-exact-ties-at-a-documents-nominal]
---

`revalue_corpus::reader_ties_at_the_nominal` reads 22 exact ties in
`cut_cylinder`, all `bool_ruled_region_roots_coaxial`, after the side
reader's turn readings landed. They come from the wall's region walk
(`crates/topo/src/boolean/ruled_region.rs`, a parallel leg against a
rim, coaxial by construction) at the wall's roots, on rays aimed at
planar faces.

`solid_contain.rs` `wall_crosses_by_turn` holds a cylinder root by the
turn of its loops on an annulus image of the wall. That image carries a
ruling to a radial segment and a parallel to a circle, so `loop_turn`
reads both exactly. `cut_cylinder`'s walls are bounded by the tilted
cut, an ellipse. Its image is no circle and no segment, so
`loop_turn` returns `None` for them, and the root falls back to the
walk.

**The design question** (two readings, neither built):

1. **Split the section at the root's azimuth**, and read each piece by
   chords inside a sector of under half a turn about the axis. Such a
   chord's turn about the root's image equals the piece's turn, since
   the sector is convex and excludes the root's ray. This asks where
   the root's azimuth lies against each piece's span, which ties where
   the root sits on a piece's end azimuth. The turn is continuous
   there, but the question is posed.
2. **Another image of the wall** in which a plane section is a circle
   (or a line), so that `arc_turn`'s cross-ratio form reads it without
   asking. None is known to the row's author; the annulus map was
   chosen for rulings and parallels.

Measured: 22 ties, all in `cut_cylinder`, at ε 1e-9.

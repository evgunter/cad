---
id: a-boxed-vector-extrusions-walls-carry-the-vectors-width
kind: issue
title: A slab extruded along a boxed vector stores its line walls' normals as wide as the vector's box though the truth is fixed: the vector arm normalizes a length the box moves
status: open
opened: 2026-11-15
priority: P2
cost: E
---

Found by the review of BOXFINISH's `boxfinish/sweep-frames` (PR 6972,
reviewer r1), whose brief kept the vector arm's geometry as it was.

`crates/sweep/src/extrude.rs` `Course::wall_normal`, the `Vector` arm:
a line wall's normal is `place(d) × v̂` normalized, `v̂ = v/‖v‖`
(`swept::Unit3::normalized`, the arm's one normalization). Over a box on
the vector the truth is fixed (an upright `(0, 0, z)` sweeps the same
walls for every `z`), but the normalization divides by a length the box
moves, so the stored normal carries the vector's width.

Measured at `Sym<Interval>` on the unit square extruded along
`(0, 0, [0.95, 1.05])`: the caps are exact (the sketch frame), and every
one of the four walls stores a normal about 0.40 wide in a component,
of length `[0.819, 1.222]`, the number the unit measured for the old
fitted walls. The same on the merge base. At bare `Interval` the box
refuses earlier, at `extrusion_obliquity`'s enclosure `[0, 0.1]`.
Pinned as today's truth by
`a_sweep_keeps_the_frame_it_is_handed::a_slab_along_a_boxed_vector_stores_exact_caps_and_wide_walls`
(the walls hold every member and are at least 0.3 wide).

No document path extrudes along a vector (the tour has one call); the
reach is the public door.

What closes it: the vector's direction decided once without dividing
by its boxed length where the truth does not need it (for a vector the
extrude has already decided normal to the sketch plane, the sketch
normal signed by the decided component), the row re-cut to exact walls.

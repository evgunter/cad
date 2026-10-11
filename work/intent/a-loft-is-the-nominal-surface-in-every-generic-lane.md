---
id: a-loft-is-the-nominal-surface-in-every-generic-lane
kind: issue
title: A loft or sweep is its nominal surface in every generic lane: Dual sees no derivative in a section's variables, Interval certifies the nominal over a box
status: open
opened: 2026-10-11
priority: P1
---


Both designers who weighed `nurbs-weights-and-knots-are-the-scalar`'s
C6 fork (2026-10-11) found the same thing: a loft or a sweep is its
NOMINAL surface in every generic lane, not just at `Sym`. `skin.rs`
computes the walls in `f64` (`segment_curve`, `make_compatible`,
`skin_parameters`). `loft.rs`'s `assemble` lifts them by
`map_scalar(T::from_f64)`, about line 538. So:
- at `Dual`, a wall's derivative in any of its sections' variables is
  zero: the lift makes every datum a dual constant;
- at `Interval`, a body built over a parameter box certifies the
  nominal wall, not the family the box spans.

Either answer is wrong, not just unproven. A sensitivity read off a loft,
or a certificate over a toleranced loft, reports the nominal as if it
held over the box.

This is adjacent to `work/sym/derived-frame-placement-freezes-on-the-symbolic-lane.md`
but not the same: that one is a derived frame's placement, this one is
the wall's own data. It is also not the same as
`the-skin-computes-its-values-in-t`, which is the fix: that item makes
the values `T`; this one is the wrong-answer finding in the Dual and
Interval lanes, which stands until it lands. Owed: a pin, red today,
at each of `Dual` and `Interval`. A loft over a section whose radius is
a dual variable shows a nonzero wall derivative. A loft over a widened
radius encloses the radius's family at a sample.

**Owed: the document lane.** The library half landed with
`the-skin-computes-its-values-in-t`: `sweep::loft_body` and
`sweep::sweep_body` take their sections and placements in `T` and skin
them there at every projective scalar, so a library loft at `Dual` over
a dual bulge carries the wall's derivative
(`sweep/tests/a_loft_computes_its_walls_in_t.rs`,
`a_lofted_arc_walls_weight_carries_the_bulges_derivative`). At
`Interval` the library loft encloses the `f64` skin of its sections'
value channel, and a section wider than a point refuses
(`a_loft_over_a_section_wider_than_a_point_refuses_at_interval`)
instead of standing for the nominal over its family. What is left is
editor-core's Loft node: `section_of` resolves each section at the
lane's nominal (DM1c, `crates/editor-core/REFERENCES.md`) and
`wire_loft` lifts it at the door through `Real::from_computed`, so in
a document every section value still reaches the loft as an unknown at
`Sym` and a dual constant at `Dual`. Feeding the document's sections in
`T` revises DM1c (and likely PP6), which is Ev's call: parked on that
`[ev]` question.

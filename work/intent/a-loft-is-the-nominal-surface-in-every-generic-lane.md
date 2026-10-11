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

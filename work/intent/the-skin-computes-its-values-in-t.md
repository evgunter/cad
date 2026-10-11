---
id: the-skin-computes-its-values-in-t
kind: issue
title: The skin computes its sections' values in f64 and lifts them as constants, so no lofted or swept carrier is a function of its variables
status: parked
blocked_on: [nurbs-weights-and-knots-are-the-scalar]
opened: 2026-10-11
priority: P1
cost: H
---


`sweep/src/skin.rs` builds every lofted and swept wall in `f64` and the
loft lifts it as constants. The lift is `loft.rs`'s `assemble`, at
`w.map_scalar(T::from_f64)` (about line 538). It does that whatever the
sections were made of. The skin's own steps are all `f64`:
- `segment_curve` (about line 311): an arc's interior weight is
  `(delta / 2.0).cos()`, with `delta` its `f64` sweep over `m` sub-arcs;
- `make_compatible` (about line 424) and its `elevate_degree` and
  `refine_to_union`;
- `skin_parameters` (about line 471).

So a profile arc whose sweep reads a variable gives a wall whose weight
`cos(Δ/2)` is a frozen copy. The same goes for its knots, its control
points and the interpolated poles.

The fix computes those values in `T`, under the same `f64` decisions:
the sub-arc count, the merged knots, the degree raise, the parameters'
structure. That is C6 as `nurbs-weights-and-knots-are-the-scalar`'s u2
re-words it (`crates/geom-brep/README.md`). Then `assemble`'s lift goes
away. A numerical step inside it, the interpolation solve, enters its
output once, through `Real::from_computed`.

This is where provability is won. The carrier work (u2–u4 of
`nurbs-weights-and-knots-are-the-scalar`) holds a weight in `T`, but
in the live tree nothing yet builds a NURBS weight that is a function of
a variable. The skinned arc wall is the first such carrier. Its pin
generalises that item's `HeldArc` row (`geom::test_support::held_arc`):
a lofted arc wall at `Sym<f64>` over a variable sweep re-values with it,
and the arc-rim margin in `geom-brep/src/pcurve_cache.rs` is a symbolic
Zero.

Collision to check first: `crates/editor-core/REFERENCES.md` DM1c says
"a section's geometry stays f64". The lane that builds this runs
`git log -S` on that phrase (pass `--all`, use a short phrase) before
it starts. If Ev ratified it, this item first needs an `[ev]` PR.
Re-word the skin's module docs ("The control bits and the weights are
DATA") when the values move. "Structure selection is f64 (C6/D9)"
stays true.

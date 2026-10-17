---
id: the-skin-computes-its-values-in-t
kind: issue
title: The skin computes its sections' values in f64 and lifts them as unknowns, so no lofted or swept carrier is a function of its variables
status: closed
opened: 2026-10-11
priority: P1
cost: H
refs: [held-value-equalities-decided-on-the-value-channel]
pr: 4726
closed: 2026-10-11
---


`sweep/src/skin.rs` builds every lofted and swept wall in `f64` and the
loft lifts it. The lift is `loft.rs`'s `lift_loop_walls`, which since
main's fix for the #4591 × #4662 break (9618d84fd8) enters the net,
knots and weights through `T::from_computed`: each an unknown at `Sym`,
which is sound but no function of the variables. It does that whatever
the sections were made of. The skin's own steps are all `f64`:
- `segment_curve` (about line 311): an arc's interior weight is
  `(delta / 2.0).cos()`, with `delta` its `f64` sweep over `m` sub-arcs;
- `make_compatible` (about line 424) and its `elevate_degree` and
  `refine_to_union`;
- `skin_parameters` (about line 471).

So a profile arc whose sweep reads a variable gives a wall whose weight
`cos(Δ/2)` is an opaque unknown rather than `cos` of the variable. The same goes for its knots, its control
points and the interpolated poles.

The fix computes those values in `T`, under the same `f64` decisions:
the sub-arc count, the merged knots, the degree raise, the parameters'
structure. That is C6 as `nurbs-weights-and-knots-are-the-scalar`'s u2
re-words it (`crates/geom-brep/README.md`). Then `lift_loop_walls`'s
lift goes away. A numerical step inside it, the interpolation solve, enters its
output once, through `Real::from_computed`.

This is where provability is won. The carrier work (u2–u4 of
`nurbs-weights-and-knots-are-the-scalar`) holds a weight in `T`, but
in the live tree nothing yet builds a NURBS weight that is a function of
a variable. The skinned arc wall is the first such carrier. Its pin
generalises that item's `HeldArc` row (`geom::test_support::held_arc`):
a lofted arc wall at `Sym<f64>` over a variable sweep re-values with it,
and the arc-rim margin in `geom-brep/src/pcurve_cache.rs`
(`arc_rim_weight`) is a symbolic Zero. The arc rim's `wᵢ = w₁` pattern is
still decided on the value channel
(`held-value-equalities-decided-on-the-value-channel`), so that pin can
pass while the pattern's identities hold at the witness only.

Collision to check first: DM1c ("a section's geometry stays f64") was
cited from `crates/editor-core/REFERENCES.md`, which no longer carries
the phrase. The lane that builds this runs `git log -S` on it (pass
`--all`, use a short phrase) before it starts. If Ev ratified it, this
item first needs an `[ev]` PR.
Re-word the skin's module docs ("The control bits and the weights are
DATA") when the values move. "Structure selection is f64 (C6/D9)"
stays true.

Closed by #4726. The skin forms every value in its sections' scalar
under the same `f64` decisions: `segment_curve`, `make_compatible`
(the projective knot algebra over held knots and weights),
`skin_parameters`, and the averaged knots and banded collocation
solve (`geom::curves::fit::interpolate_columns_in`, refusals on the
value channel). `loft_body`/`sweep_body` take sections and placements
in `T`, and validate them guided by the `f64` record. At an
enclosure scalar (`Interval`, and `Dual`/`Sym` over it), where the
projective algebra does not typecheck and averaged knots are not
points, the body encloses the `f64` skin (`SpanLocate::split_projective`
chooses). Unlike this item's premise, the solve's output is not
entered through `from_computed`: per the orchestrator's ruling it runs
in `T`. The pin is `sweep/tests/a_loft_computes_its_walls_in_t.rs`. The
document lane (editor-core's sections still `f64` per DM1c) is
`a-loft-is-the-nominal-surface-in-every-generic-lane`'s.

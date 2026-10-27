---
id: an-integral-patch-with-knots-hugging-its-edges-trips-the-area-gauge
kind: issue
title: quad: an integral patch with knots 1e-9 from both trim edges certifies an area 2e7 wide and trips the A2 area-gauge debug_assert
status: closed
opened: 2026-10-22
priority: P1
cost: M
closed: 2026-10-27
branch: quad/knots-hugging-area-gauge
---


## Found (PR 5582's dual review, 2026-10-22; reproduced by the QUAD lane)

Pre-existing: the reviewers reproduced it on the merge base, and the
`integral` lane it lands in is untouched by PR 5582.

`nurbs_patch_face::<f64>` on `cert5_r1_patch_probes`'
`knots_hugging_the_trim_edges_stay_sound` net with UNIT weights:

- knots `[0, 0, 0, 1e-9, 1 − 1e-9, 1, 1, 1]`, degree 2 by linear;
- control `(0,0) (0.5,0.4) (1,0.9) (1.5,0.4) (2,0)` × `z ∈ {0, 1}`;
- whole rectangle, perimeter 8, ε = 1e-7.

It panics in `props::quad::debug_assert_area_gauge` (perimeter arm) at
a CERTIFIED return. The area bracket is 2.0e7 wide, a mean edge
displacement 3.1e5× the perimeter. A reviewer's variant read 4e14.

On the two tiny spans the speed runs to ~1e9, so the integral lane's
area pad (`area_at`/`area_cell`, `norm_hi` of hull boxes over 2-D
cells) is that wide. The rational twin of the same net (weights 2)
reads `[1.13, 3.72]` through the quotient statement's hulls per block.
Wide is not wrong. The A2 tripwire still says a certified return this
wide means the area pass lost the geometry, so the lane should refuse
typed, or tighten, before the tripwire is reached.

## 2026-10-24 — now red on main at every ε (INTENT nightly triage)

Nightly run 40066431545 (main `3dfb96b91c`) reds
`cert5_r1_patch_probes::knots_hugging_the_trim_edges_stay_sound` at
default, 1e-6 and 1e-12, with this row's panic: `ARM: perimeter. Width
20009765.101737916 on a face whose perimeter is at least 8`. The
2026-10-23 nightly passed it.

The row's net has weights 2 (constant). ISO's PR 5635
(`iso/one-polynomial-weight-test`, merge `9180a67d`) routes a
constant-weight net to the integral arm
(`geom_core::spline::constant_weight`; `props/quad/`
`nurbs_patch_face_rounds`). So this net now takes the unit-weight path
this row describes, where before it took the rational quotient lane.
5635 did not cause the defect. It routed a probe row onto it, and the
probe is now the main red. The fix stays this row's.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds, and main is still red. Re-run at 1e-6, 1e-9 and 1e-12:
`cargo nextest run -p geom-brep --test all -E
'test(knots_hugging_the_trim_edges_stay_sound)'` panics at every ε in
`debug_assert_area_gauge`
(`crates/geom-brep/src/props/quad/area.rs:372`; the panic is at `:379`),
with `ARM: perimeter. Width 20009765.101737916 on a face whose perimeter
is at least 8`. These are the same numbers as before. The lane is
`nurbs_patch_face_rounds` (`props/quad/patch.rs:176`).

## 2026-10-27 — closed: the hull leaked the sliver span, not the rule

`props::quad::net::range_hull` located a cell's closed end with
`span_range`, which puts a knot in the span STARTING at it. Every
middle-span area cell ending on `1 − 1e-9` therefore hulled the
sliver span `[1 − 1e-9, 1]`, whose derivative coefficients are ~1e9,
into its `S_u` hull: 64 cells × ~2.4e5 each is the 2e7 the gauge saw.
The enclosure was sound but wide. The gauge's assert was right to
call it lost geometry.

The fix stops a range that ends on a knot at the span ending there.
That is sound for every `KnotVector` direction because a clamped
spline is continuous, so the left span's coefficients already hull the
value at that knot. The probe now reads area `[1.1279, 3.7245]`, which
is the rational twin's bracket. An independent oracle (arc length of
the section, span-local Gauss–Legendre) gives `2.42841842244066`,
inside it. The remaining width is the one-cell hull floor on each
sliver span.

The `Loose` arms keep closed-range semantics, because a `Raw` or
`Const` direction is discontinuous at its knots. The same locator shape
in `geom-core` and `ssi` is filed as `nurbs`'
`a-closed-range-ending-on-a-knot-hulls-the-span-starting-there`.

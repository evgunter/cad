---
id: f64-refinement-inside-an-enclosure-has-five-more-sites
kind: issue
title: An f64 knot refinement (or an f64-rounded insertion ratio) inside an enclosure: five more sites of TESS-2's class
status: open
opened: 2026-09-22
---


Filed by TESS-2 (PR for `tess/2-refinement-in-the-ring`), whose fix
closed the instance in `geom_brep::patch_bound`. On PROPS' slate
because four of the five sites are PROPS-owned paths
(`crates/geom/src/curves/nurbs.rs`, `crates/geom-brep/src/props/quad.rs`
x2, `crates/geom-core/src/spline/compose.rs`); the fifth is TESS/CHORD's
`crates/mesh/src/chords.rs` and is named in the same list because one
class is one row.

## The class

An enclosure that is assembled AFTER a knot refinement encloses the
refined geometry, not the described geometry, unless the refinement is
itself part of the enclosure. Two spellings reach the same defect:

1. **Refine in `f64`, then lift into the ring.** The refined net is a
   rounded copy of the exact refined net, so the described geometry sits
   outside the hulls by the insertion rounding, amplified by any knot
   differencing that follows. This is what TESS-2 measured in exact
   rational arithmetic: the described patch's true `‖S_uu‖` exceeded the
   certified `muu` by 1.8e-15 relative on a bilinear rational, and a
   bare-comparison census of 6,000 bilinear trials showed 2 escapes.
2. **Refine in the ring, but with the plan's `f64` `λ`.** The
   combination is then an enclosure of `x + (y − x)·fl(λ)`, which is a
   different curve from `x + (y − x)·λ`. Rounding the RATIO is the same
   defect as rounding the coefficients; the fix is to re-derive the
   ratio from the knots it is made of, which is what
   `geom_core::spline::CurvePlan::apply_ring` now does.

## The sites

Found by grepping the shape rather than the symbol — every
`refine_knots`, `refine_knots_u/v`, `insert_knot*` and `refine_plan`
call under `crates/*/src`, then reading what each result feeds.

| site | spelling | what it feeds |
| --- | --- | --- |
| `crates/mesh/src/chords.rs`, `rational_carrier_m_bound` (and its twin near the file's end) | 1 | `refine_knots` in `f64` at `patch_bound::rational_split_points`, then ring `derivative_coeffs` hulls divided by the span weight range — `patch_bound`'s defect exactly, one dimension down, on the chord certificate |
| `crates/geom/src/curves/nurbs.rs`, `rational_speed_lower_bound` | 1 | `refine_knots` in `f64` at `RATIONAL_METER_SPLITS`, then `rational_span_scan`'s hulls in the evaluation scalar `T`. At `T = Interval` the answer is an enclosure of the refined-`f64` curve's speed, and the meter's claim is about the described curve |
| `crates/geom-brep/src/props/quad.rs`, the `QUAD2_REFINE_SPANS` net refinement | 2 | `refine_plan` with unit weights, applied to `RVec3` ring coefficients through `ring_lerp` with the plan's `f64` `λ` lifted as a ring point |
| `crates/geom-brep/src/props/quad.rs`, the chart-image refinement | 2 | `refine_plan` with the image's real weights, applied to `RPt2` ring brackets through a locally-written lerp with the plan's `f64` `λ` |
| `crates/geom-core/src/spline/compose.rs`, `insert_once_ring` | ratio enclosed, but the LERP form | `α` is already an outward-rounded ring quotient here, so this site is sound. What it loses is WIDTH: `c_{j−1} + (c_j − c_{j−1})·α` reads `c_{j−1}` twice, so an interval coefficient's dust enters with coefficient `1 + α` and a fold of insertions multiplies it up per step. TESS-2 measured 355 ulps against 16 for the convex form `β·c_{j−1} + α·c_j` over 30 insertions, and the quarter cylinder's structurally-zero `S_vv` came out at 8.8e-11 with the lerp form against 7.0e-13 with the convex one — 126x. `to_bezier_spans` inserts to FULL multiplicity, so its fold is `p` deep per interior knot and it is the site where this costs most |

Two more sites refine in `f64` before a certificate and are NOT
classified here, because whether their claim is about the described
geometry or about the refined one is their owner's reading, not this
row's: `crates/geom-brep/src/ssi/certify.rs`'s `SSI_CERT_SPANS`
refinement and `crates/geom-brep/src/edge_nurbs.rs`'s `PXN_WALL_SPANS`
wall refinement. Both are named so a reader of this row does not have
to re-run the sweep to find them.

## Blind spots of the sweep, stated

The grep is over call sites of the four refinement entry points under
`crates/*/src`, so it cannot see: a refinement reached through a
generic parameter or a trait object; a hand-written Boehm loop that
names none of those four (the tree has one — `insert_once_ring` — and
it is in the table, found by reading `algebra.rs`'s own cross-reference
rather than by the grep); and a refinement performed in a `tests/` tree
whose result is then asserted against. The second gap was checked by
grepping for `alpha` and `lambda` beside `RingInterval` across
`crates/*/src`, which turned up no further site.

## What a fix looks like

The machinery exists now: `geom_core::spline::algebra`'s
`refine_plan_homogeneous` builds the schedule and
`CurvePlan::apply_ring` applies it with both Boehm ratios re-derived
from their knots, `TensorNet::refine_u`/`refine_v` lift it to a net.
Each site above is a substitution, plus a re-baselining of whatever
figures move. `compose.rs`'s site is a two-line change to the
combination's form and is the cheapest of the five.

## Also worth measuring, separately

TESS-2's widening tail (p99 7e-13, max 5e-12 relative over a
6,000-trial bilinear census) comes from the schedule being a FOLD of
single insertions: the ratios' own rounding accumulates once per
insertion, so 16-fold refinement pays ~16 roundings where a one-pass
refinement (Book A5.4 / Oslo) would compute each refined coefficient as
one convex combination of `p + 1` described ones and pay ~`p`. That is
a different algorithm, not a fix to any site above, and it would tighten
every rational certificate in the tree.

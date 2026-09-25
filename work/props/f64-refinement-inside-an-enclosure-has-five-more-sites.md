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

## The `compose.rs` site is what refuses the offset fit at tight ε (ENCL, 2026-09-25)

Measured by ENCL's `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart`
lane. This is the evidence for that row's proposal, and it moves this
site from "costs width" to "decides a user-visible refusal".

**The mechanism.** `offset_fit::Composite::build` decomposes every net
(the fit, the base, the base's derivatives) through `PatchSpans::decompose`,
so `to_bezier_spans_extra` inserts each of the fit's interior knots to
full multiplicity in the lerp form. The fold runs in ascending knot
order, so the width it multiplies up collects at the HIGH end of each
direction, and on a refined fit the `(u, v) → (1, 1)` corner cell's
`Ẽ` coefficients carry it. Measured on `bowed()` at `d = 0.05`, the
certified sup's cell at each round, coefficient midpoints against the
widest coefficient radius:

| round | grid | sup cell | `Ẽ_x` radius | `Y` midpoint max | `Y` radius | `hull_sup` | on-locus |
|---|---|---|---|---|---|---|---|
| 0 | 4×4 | (0,0) | 2.2e-16 | 2.7e-8 | 2.2e-16 | 4.39e-7 | 1.28e-7 |
| 5 | 17×17 | (9,4) | 2.5e-14 | 6.9e-12 | 2.5e-14 | 6.88e-11 | 4.02e-11 |
| 6 | 27×27 | (23,23), `[0.958,1]²` | 1.4e-10 | 6.9e-13 | 1.4e-10 | 1.78e-10 | 6.11e-12 |
| 7 | 32×32 | (28,28), `[0.979,1]²` | 2.0e-9 | 4.3e-14 | 2.0e-9 | 2.49e-9 | 6.11e-12 |

From round 6 the certified bound is the enclosure's own radius, three
to five orders above the polynomial it encloses, and it GROWS with
refinement. The refinement loop therefore cannot cross it at any budget
(budget raised to 30: `RefinementStalled` at round 7, 2.49e-9).

**The A/B.** The convex form `c_{j−1}·β + c_j·α`, with
`β = (U_{j+p} − u)/(U_{j+p} − U_j)` formed as a ring quotient like `α`,
and nothing else changed, with the budget and cap raised so the loop can
run:

| fixture | target | lerp (shipped) | convex |
|---|---|---|---|
| `bowed()`, `d = 0.05` | 1e-12 | stalls at round 7, 2.49e-9 | certifies at round 9, 49×49, 7.92e-13 (bound ≤ 2.1× on-locus every round) |
| twisted-loft saddle wall, `d = 0.05` | 1e-9 | bound bottoms out at 1.15e-9 (round 8), rises to 9.2e-9, stalls | certifies at round 9, 49×26, 3.87e-10 |
| same | 1e-12 | as above | certifies at round 16, 187×96, 6.40e-13 (41 s) |
| same wall, `d = 5e-10` | 1e-14 | stalls at round 4, 1.29e-11 | certifies at round 3, 7.99e-15 |

Rounds 0–5 are bit-for-bit or last-digit identical under both forms on
every fixture; the forms part only once the fold is deep enough for the
width to reach the residual. The two-line change is exactly this row's
"cheapest of the five", and it is now the one with a consumer waiting on
it. What it moves elsewhere (every composite bound in the tree reads this
fold) has not been measured here; that re-baseline is the fix's own.

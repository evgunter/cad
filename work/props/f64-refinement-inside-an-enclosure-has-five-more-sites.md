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
   `geom_core::spline::CurvePlan::apply_certified` now does.

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
`CurvePlan::apply_certified` applies it with both Boehm ratios re-derived
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
| same | 1e-12 | as above | certifies at round 16, 187×96, 6.40e-13 (41 s: release, one traced run, 4-core container shared with two other lanes) |
| same wall, `d = 5e-10` | 1e-14 | stalls at round 4, 1.29e-11 | certifies at round 3, 7.99e-15 |

Rounds 0–5 are bit-for-bit or last-digit identical under both forms on
every fixture; the forms part only once the fold is deep enough for the
width to reach the residual. The two-line change is exactly this row's
"cheapest of the five", and it is now the one with a consumer waiting on
it. What it moves elsewhere (every composite bound in the tree reads this
fold) has not been measured here; that re-baseline is the fix's own.

**A consumer that moves with it (ENCL, PR 3294):** `geom-brep`'s `tests/offset_fit.rs` `the_second_non_improving_round_is_the_stalls_face` pins `RefinementStalled` on the saddle wall at `d = ±5e-10` and `1e-6`, target 1e-14; if the convex form makes those requests certify, re-find a stalling request, and failing that, drive the loop with a `#[cfg(test)]` scripted-bound seam (`work/encl/offset-fit-stall-face-has-no-fixture.md`'s option) rather than deleting the row.

## A second ENCL consumer: rigid maps at 1e-12 (ENCL, 2026-09-28)

Measured by ENCL's `a-rigid-map-still-refuses-the-bowed-approx-fixture-at-eps-1e-12` lane. The 93-map probe was not committed. Subject: `topo::fixtures::bowed_patch` at `d = ±0.05`, ε = 1e-12, fitted in 3 rounds over 64 cells, `hull_sup` 3.652e-13. Rotating it drifts the re-derived bound up to ×3.10, and on 4 maps a fresh re-fit stalls at 1.04–1.08e-12 (`RefinementStalled`).

The width is in `Ẽ` before `X = Ẽ·Ẽ − d²w̃²` is formed:
- Even unrotated, `Ẽ_x` carries a radius of about 2.85e-13, about 2600 ulps of its 0.5-sized coordinate. There it multiplies a tiny `|E_x|`.
- A rotation spreads that radius over channels whose `|E_c|` is about 0.03. `X`'s radius then grows to 6× its own midpoint (4.8e-14 against 7.9e-15), and `tau` doubles too (2.86e-13 → 6.1e-13).
- `tau` is about 97% rounding width even unrotated.

A/B with one change, `insert_once_ring` from the lerp form to the convex form `c_{i−1}·β + c_i·α` (β a ring quotient):
- The `Ẽ` radius falls about 100× to ≤2.5e-15.
- The bound becomes frame-invariant: 8.34e-14 unrotated, 8.43e-14 and 8.40e-14 on the two worst maps.
- Of the 93 maps, 0 refuse and 0 re-fits stall; the worst drift is ×1.014.
- The fixture's own `hull_sup` drops 4.4×.

At only 3 rounds deep, this contradicts the reading above that rounds 0–5 barely differ between the forms: on a fit this shallow the convex form already changes the certified bound by 4×.

Consequence: a rigid map of a body that validates at 1e-12 can refuse (`ApproxRecertify { RefinementStalled }`) until this site is fixed. The ENCL row is parked on this one.

## A third consumer, measured: the plane×NURBS edge certificate (SSI, 2026-10-01)

On the m8_4 seam (`crates/sweep/tests/m8_4_intersection_iso.rs`), limb 2's
certified `hull_sup_chart` is 98.5% ring widening from this site's lerp
form. It is 1.69e-12 m against a true residual of 2.5e-14 m, at
N = 32 spans. It grows as N³ under refinement: 3.0e-14 at N = 8, 1.1e-10
at N = 128. The convex form gives 2.0e-14 and 1.5e-13 at the same N, and
at N = 32 it is 31× tighter (5.47e-14). It is the only thing standing
between that fixture and certifying at ε = 1e-12 at scale 1, so
`INTERIOR_COLUMN_SCALE = 1/1024` exists because of this site. The tables
are in `work/ssi/plane-nurbs-certificate-bound-does-not-refine-with-eps.md`,
which parks on this row. Re-baselines a landing here owes: the four
"must refuse below 1e-9" pins (`m8_4_intersection_iso.rs`'s
`seam_at_eps` and `review_probes_m8_4.rs`'s probe_e) flip, because the
seam then attaches.

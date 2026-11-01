---
id: the-projective-applier-still-lerps-so-a-nurbs-refined-at-t-interval-pays-twice
kind: issue
title: CurvePlan::apply_points combines in the lerp form, so a NURBS refined at T = Interval pays both the width and an f64 lambda
status: closed
opened: 2026-09-30
priority: P2
cost: M
branch: nurbs/knot-algebra-off-the-ring
closed: 2026-10-10
pr: 4553
---


Found by the shape sweep of PROPS' convex-insertion unit (PR 3524),
which closed site 5 of
`f64-refinement-inside-an-enclosure-has-five-more-sites` by moving
`geom_core::spline::compose`'s `insert_once_ring` from the lerp form to
the convex form. The sweep was for the SHAPE — a ring combination that
reads one of its endpoints twice — and this is a hit the five sites do
not cover.

## The site

`geom_core::spline::algebra`'s `CurvePlan::apply_points` is the
PROJECTIVE applier. It is generic over the caller's scalar and takes the
caller's own `lerp`, whose documented fixed association is
`x + (y − x) · λ` with `λ` lifted via `from_f64` — `Step::Combo`'s
stored `f64` weight quotient. Four call sites pass exactly that closure:

- `geom::curves::nurbs`'s `apply_plans` (`x.lerp(y, T::from_f64(l))`);
- `geom::surfaces::nurbs`'s `map_u_columns`, and its knot-removal round
  trip, which applies the same closure twice per column
  (`step.plan` then `step.reinsert`).

Every one of them is generic over `T: Real`, so each is instantiable at
`T = Interval` — and `NurbsCurve`/`NurbsSurface` at that scalar is an
ordinary part of this tree's certification lane.

## What it costs at `T = Interval`

Two defects, not one, and they are the two spellings the parent row
names:

1. **Width** (the parent row's site 5, one applier over). The lerp form
   reads `x` twice, so an interval coefficient's own dust enters with
   coefficient `1 + λ` and a fold of insertions multiplies it up per
   step. Measured on `insert_once_ring`'s own fold: 473.7 ulps of the
   coefficient scale against 4.7 for the convex form, at degree 6 over
   80 insertions.
2. **An `f64` ratio** (the parent row's spelling 2). `λ` is a stored
   `f64` quotient of weights, lifted as a ring POINT, so the result
   encloses `x + (y − x)·fl(λ)` and not `x + (y − x)·λ`. For a
   knot-insertion step the honest repair is
   `CurvePlan::apply_certified`'s: re-derive both barycentric ratios
   from the knots. For a degree-elevation or knot-removal step there is
   no ratio of knots to re-derive from, which is exactly why
   `apply_certified` refuses those steps — so a projective applier at
   `T = Interval` cannot simply be redirected to it.

## Why it is not the parent row's five sites

The five sites are concrete callers that refine before a certificate.
This is the APPLIER, below all of them, and its defect only bites at
`T = Interval`: at `T = f64` and `T = Dual<f64>` the lerp form is the
fixed association the whole tree documents and nothing is enclosed.
Fixing it is therefore not a substitution at a call site but a question
about what the projective applier means in the certification ring —
which is the same question `apply_certified`'s refusal of ratio-less
steps already answers for the homogeneous lane.

## What a fix looks like

Either (a) make the ring instantiation unreachable — if no shipped path
instantiates these four call sites at `T = Interval`, say so with a
check that fails if one appears; or (b) give the projective applier a
certified sibling that refuses the ratio-less steps the way
`apply_certified` does and re-derives the knot ratios for the rest,
de-homogenizing through the weight channel rather than through a stored
`λ`. (a) is cheap and is the honest first move: the sweep did not find a
shipped `T = Interval` instantiation of these four, only that the types
permit one.

## The sweep's other two hits, recorded here so the pattern is not re-run from scratch

Added after the dual review of PR 3524, which found both. Neither is this
row's subject and neither is a defect today; they are recorded because
the next lane to sweep for this shape should not have to rediscover
them, and because the REASONS first given for setting them aside were
wrong.

- **`crates/topo/src/pcurves.rs`, the `Chart`/`Scaffold` closed form
  (`let p0x = cu0 - (cu1 - cu0) * t0 / span;`), generic over `T: Real`.**
  It reads `cu0` twice. It is the **minus-sign variant** `x − (y − x)·t`
  of the shape, which the original sweep's pattern did not match and did
  not name as a gap. Not a defect: `cu0` and `cu1` are
  `T::from_f64(knots_u().domain())`, so at `T = Interval` they are
  POINTS lifted from the chart's own `f64` knot-domain ends, with no
  accumulated width for the double read to amplify — and the expression
  is one closed-form evaluation, not a fold, so nothing compounds.
  Either of those going away (a chart whose domain ends arrive as
  enclosures, or this put inside an iteration) makes it the same defect
  as `insert_once_ring`'s was.
- **`geom_core::linalg`'s `Point2::lerp` / `Point3::lerp` /
  `Vector3::lerp` (`self + (other - self) * t`), generic over `T`.**
  PR 3524 set these aside on the ground that "every caller passes a
  point `t`, never a fold". **The first half of that is false**:
  `geom-brep/src/mapped.rs`'s `SketchSegment::<T>::eval` passes the
  caller's general `T` through `Point2::lerp`, and `Point2::lerp` was
  not in the hit list at all. The disposition survives on the second
  half plus the helpers' own documented trade: they are exact at BOTH
  endpoints and pay for it by treating `t` and `1 − t` as independent
  when `t` carries width, which is an argued choice for an evaluation
  helper rather than an oversight, and `eval` is one combination per
  call — there is no fold for the `1 + t` factor to compound over. The
  hazard this row is about needs a fold; a sampler does not have one.

**The shape's blind spots, stated for the next sweep**: the literal
pattern `x + (y − x) * t` misses (a) the minus-sign variant
`x − (y − x) * t`, (b) the same combination spelled through a named
helper or a closure handed to a combinator (`.lerp(`, `apply_points`),
and (c) a combination assembled across statements. PR 3524 checked (b)
and (c) and missed (a); (a) is where `pcurves.rs` was found.

## Re-homed from FLUX to NURBS (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. NURBS's charter is the spline doors (knot insertion and the certified Boehm step), and its `certified-blossom-primitive-in-geom-core-spline` waits on the first of these two rows; the second is the same combine in `CurvePlan::apply_points`. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Closed

Fix (a), made a type: `geom_core::spline::ProjectiveScalar`, a sealed
marker over `SpanLocate` held by `f64`, `Probe`, and `Dual` and `Sym`
over them, bounds `CurvePlan::apply_points` (which now hands the closure
`λ` already lifted to that scalar) and every `geom` knot operation on
`NurbsCurve2/3` and `NurbsSurface` (insert, split, refine, union,
remove, elevate). The applier's points are bounded with it
(`ProjectivePoint`: the scalar itself, and points and arrays over it),
so ring points with an `f64` `λ` lifted inside the closure, the shape
`ring_lerp` had, do not typecheck either. A call at `Interval` does not
typecheck; the `compile_fail` rows on `ProjectiveScalar` and on
`NurbsSurface`'s knot-algebra block pin both beside their compiling
twins. What the bound cannot see is a closure that computes in the ring
and carries the result out through a capture.

The row's premise that no shipped path instantiates these at
`T = Interval` was false. The bound turned four shipped sites into
compile errors, and each now says what it refines:

- `edge_nurbs::localized` (the plane × NURBS wall, reached from
  `editor-core`'s `evaluate::<Interval>` replay through
  `policy_lane::certify` → `NurbsLane::certified` →
  `plane_nurbs_limbs`) is gone. The tube reads the wall's cells refined
  at the same schedule in the ring (`wall_grid`), and limb 2 reads the
  described wall. Dropping the refinement outright collapsed the pads on
  a wall with weights over decades and lost rulings at `Interval`
  (`work/isotail/pxn-wall-refinement-may-be-unneeded-for-the-tube.md`).
- `ssi/certify::refined` is gone: limb 2 cuts the described carrier at
  the same grid in the ring (`CurveCertData::with_breaks`), and the box
  chain refines its homogeneous channels with `apply_certified`.
  Dropping the refinement outright collapsed box chains (32 → 1) and
  lost a rational carrier's tube, so it is kept, in the ring.
- `geom::NurbsCurve*`'s rational speed meter refines its homogeneous
  channels per scalar (`SpanLocate::refine_homogeneous`): through the
  ring's step at `Interval`, with the new `CurvePlan::apply_homogeneous`
  (Boehm ratios formed in `T` from the knots) elsewhere. It reads the
  refined weights as `T`.
- `props/quad.rs`'s `refine_dir` and `bezier_blocks` refine their
  homogeneous / unit-weight channels with `apply_certified`;
  `ring_lerp` and `bezier_blocks`' `f64`-λ Lipschitz pad are retired.

Every ring refinement above goes through one spelling,
`geom_core::spline::RingRefinement`: it owns the refined vector and the
extent guard, and a rational net comes back through
`algebra::dehomogenize`, whose `certainly_positive` is the one weight
guard (the meter's is `SpanLocate::refine_weights`, the same guard per
scalar).

Certified degree elevation and removal are not built: no certification
consumer elevates or removes in the ring. The Interval-vs-`f64`
containment rows (`nurbs_interval::knot_algebra_interval_contains_f64`,
`review_m5_pr3_attack_interval`'s F4b interval half and F5b) are
retired: they asserted an `f64` answer inside an enclosure of a
neighbour curve, which says nothing about the described one.

What moved, with digits, is PR 4553's body.

---
id: the-projective-applier-still-lerps-so-a-nurbs-refined-at-t-interval-pays-twice
kind: issue
title: CurvePlan::apply_points combines in the lerp form, so a NURBS refined at T = Interval pays both the width and an f64 lambda
status: open
opened: 2026-09-30
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

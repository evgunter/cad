---
id: knot-mirror-symmetry-belongs-on-knotvector
kind: issue
title: mirror_symmetric and KnotMirrorError sit on NurbsSurface, where a curve reversal cannot reach them
status: closed
opened: 2026-09-15
priority: P1
cost: E
closed: 2026-10-09
pr: 4439
---


## Finding

`mirror_symmetric` and `KnotMirrorError` live in
`crates/geom/src/surfaces/nurbs.rs`, beside their only consumer
(`NurbsSurface::reversed_u` / `reversed_v`). The property they decide —
"this knot vector is its own reflection about its domain, as an
identity between REAL numbers" — is a property of a `KnotVector` and of
nothing else: it reads `domain()` and `knots()` and touches no control
net, no weight and no direction.

The next door that wants it is a CURVE reversal. `NurbsCurve3` has no
`reversed`, and when one is written its precondition is exactly this
predicate — at which point the choice is to move the pair down to
`geom_core::spline::knots` or to grow a second copy in `curves/`.

## Why it is filed rather than done

VREV (PR 2627) put the pair beside its one consumer and its fix pass
declined to move it: a move with no second caller is a relocation
decided by a guess about the next door, and the error type's home is
part of the curve reversal's design rather than a tidy-up before it.
What this row asks for is that the curve-reversal unit reads it FIRST
and moves the pair down as its own first step, rather than discovering
the duplicate afterwards.

Moving it is a small edit — the function is pure over `&KnotVector`,
and `KnotMirrorError` would be re-exported from `geom` where
`NurbsSurface` names it, so no consumer's spelling changes.

## Was

Filed by SCALAR's VREV fix pass (PR 2627), as the one thing it was
asked to consider and decline.

## Closed (2026-10-09, PR 4439)

`KnotVector::mirror_symmetric(&self) -> Result<(), KnotMirrorError>`
now lives in `geom_core::spline::knots`. `KnotMirrorError` is beside it
and is re-exported from `geom_core::spline`. `NurbsSurface::reversed_u`
and `reversed_v` call it, and their signatures and refusal values are
unchanged. The `geom::KnotMirrorError` re-export was dropped rather
than kept as a second spelling: `geom` re-exports no other spline error
type, and nothing used that path.

Each refusal now ends in a labelled `Recourse:`, following the
module's convention, and
`every_knot_mirror_error_arm_names_a_recourse` enforces it. The move
happened without waiting for a curve reversal: the orchestrator ruled
it is a property of a `KnotVector` alone, so it belongs there whatever
calls it next.

Filed from the sweep: `row-space-reflection-compares-rounded-knots`.
Review tier: the orchestrator's read.

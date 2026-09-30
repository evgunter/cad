---
id: knot-mirror-symmetry-belongs-on-knotvector
kind: issue
title: mirror_symmetric and KnotMirrorError sit on NurbsSurface, where a curve reversal cannot reach them
status: open
opened: 2026-09-15
priority: P1
cost: E
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

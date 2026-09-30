---
id: declared-pair-verdict-answers-an-unreachable-distinct
kind: issue
title: topo: declared_pair_verdict answers the declared rung's unreachable Distinct verdict as 'do not merge'
status: open
opened: 2026-09-30
---

(TOPO, the receipt of the D262 unit, PR 3532: its sweep of every
lookup-answering arm in `merge_faces.rs` met this non-lookup arm and
leaves it, because its row 0 lies in `boolean/`.)

## What

`merge_faces::declared_pair_verdict` (`crates/topo/src/merge_faces.rs`)
maps the declared plane rung's verdict for a declared face pair. Its
arm `Ok(PlaneRelation::Distinct) => Ok(false)` carries the comment
"Unreachable through the declared rung; kept typed", but it is not
typed: it answers "these faces do not merge". `oriented_plane_eq` with
`declared: true` contradicts a definite non-parallel or apart pair
(`PlaneEqError::Contradicted`) and never returns `Distinct`, so the arm
is a state that cannot occur, answered silently (DESIGN's D2 addendum,
"Silent discard is never an answer"). Were it ever reached, a declared
planar pair would be left unglued; the planar regime then refuses the
boolean one gate later as `NonMaximalFaces`, naming the wrong cause.

The same function is `boolean::refusal_routes`' test route
(`declared_pair_verdict(oriented_plane_eq(..))`), so both callers share
the arm.

## Repair shape

Row 0 first: the declared rung's return type
(`boolean::plane_eq::oriented_plane_eq`'s declared branch) could name
only the two relations it returns (`SameOriented`, `SameOpposite`), so
`Distinct` is unspellable here. That type lives in `boolean/`. Failing
that, the arm refuses typed (a kernel defect ending), as the
neighbouring `PlaneEqError::Undeclared` arm of
`MergeCoplanarError::of_declared_refusal` already does.

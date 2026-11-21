---
id: the-shared-side-rule-on-the-spline-arm-has-no-witness
kind: issue
title: the trimmed lane's shared-side rule runs on the spline arm and no body reaches it there
status: open
opened: 2026-11-20
priority: P3
cost: C
---


Filed by the four-use-edge unit (PR 7198).

## What

PR 7198 carries the trimmed lane's shared-side rule (`mesh::trimmed`,
`TrimmedFace::emit` / `attempt`) to every chart row, the spline arm
included. No body reaches the rule there: both reviews of that PR counted
it firing 0 times on a spline face across the full suites, and the unit's
own probe could not make one. A lofted bulged tube (three circles, radius
1, 1.3, 1, degree 2) split by planes near its wall or its top, tilted 0.05
to 0.3, refuses every time at the split:
`Reduce(CurvedBooleanUnsupported { face, kind: Nurbs })`. So no public
door yet cuts a spline face into the thin strip, a lattice cell or two
across between two chains, that the analytic witnesses are.

## What it means

The spline arm's rule is argued, not demonstrated. If it misfires there,
the census at the join (`tessellate::joined`) still refuses any side past
two triangles per chord, so the failure would be a refusal, not a wrong
`Ok`.

## Owed

A witness once a door can trim a spline face against a neighbour that
joins its chain freely (a split or boolean with a spline operand), or a
row that pins the rule on a hand-built trimmed spline patch.

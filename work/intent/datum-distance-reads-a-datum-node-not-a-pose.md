---
id: datum-distance-reads-a-datum-node-not-a-pose
kind: issue
title: GeomPred::DatumDistance reads a datum node, not a pose variable
status: closed
opened: 2026-10-10
closed: 2026-10-20
---

## What

`GeomPred::DatumDistance { datum: RecipeNodeId, .. }`
(`crates/editor-core/src/names/geompred.rs:158`, prepared at `:528`)
references a datum NODE and reads its payload as a `PoseValue`. Since
INTENT stage 3 A a pose is a variable that need not be any node's output:
a `Point { of }` read off a vertex, an `InFrame` point. Such a pose cannot
be measured against, and when stage 3 D retires `Datum` the predicate has
nothing left to reference.

## Fix shape

The predicate reads a pose variable (`VarId` of kind `Point`, `Axis` or
`Plane`), bound as any reader binds it (`eval/wire/pose.rs`, `eval_pose`),
and a datum node is read through its output variable. It belongs with D's
datum retirement at the latest.

## Closed

Closed by INTENT stage 3 D (`transform-retires-into-a-placement`, 2026-10-20), which retires `Datum` and with it the node this predicate read. Its successor is `GeomPred::PlaneDistance { face, cmp, value }` (`crates/editor-core/src/names/geompred.rs`): it reads a planar face of the body selected from as a plane (D10: a face reads as a plane), so the rule moves with the body and the selection commutes; a non-planar face refuses `NotAPlane`. It reads a face rather than an arbitrary pose variable, the fix shape proposed above; no reader asks for the latter today.

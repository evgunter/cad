---
id: the-patch-memo-key-misses-the-faces-across-a-trimmed-face
kind: issue
title: a trimmed face's patch reads which faces lie across its edges, and the memo key does not
status: open
opened: 2026-11-20
priority: P1
cost: M
---


Filed by the four-use-edge unit (PR 7198) at its review's request; on the
cone arm it predates that PR.

## What

The trimmed lane's shared-side rule (`mesh::trimmed`, `faces_across`
read in `TrimmedFace::emit`) decides which of a face's triangles to
split by **which faces lie across each of its loop's edges**: two polygon
points one neighbour holds both of. That is neighbour identity. It is not
among the memo key's inputs: `memo::FaceInputs` / `memo::EdgeInputs`
fold the face's surface, sense, loops (each edge's carrier, interval,
direction, chord ids as a relabeling, chord points and parameters,
stored pcurve, lineage), δ, ε and k, and nothing about the face across
an edge. The ratified per-face purity invariant (`crates/mesh/src/lib.rs`,
"Invariant (ratified via PR #32, and the memo-key contract)") lists the
same inputs and does not include it either.

## Why it matters

An edit that changes which faces meet face A's edges, while leaving
every bit A's key reads unchanged, replays A's stored patch, made under
the old neighbour structure. Where the new structure puts one face across
two points that A's stored patch joins, both may use that side; the
census where `tessellate` joins the patches (`tessellate::joined`) then
refuses `OverusedSide` where a fresh run would mesh. A wrong refusal, not
a wrong `Ok`: the census sees every such side. On main before PR 7198
the cone arm already read the map; that PR extends the read to every
trimmed arm.

Unmeasured: no edit is known that keeps A's key and moves its neighbours.

## Owed (not chosen here)

Either fold the neighbour structure the lane reads into the key (and the
invariant's list, which is a ratified text), or argue it is fixed by
what the key already holds. The orchestrator weighs the fix.

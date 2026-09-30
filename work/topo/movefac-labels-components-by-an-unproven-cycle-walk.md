---
id: movefac-labels-components-by-an-unproven-cycle-walk
kind: issue
title: movefac labels a shell's components through cycle walks it never proves claim their loops: a torn next can split a connected shell or join two components
status: review
pr: 3574
branch: topo/movefac-proofs
opened: 2026-09-30
refs: [mef-and-mekr-move-a-walked-run-they-never-prove-is-the-loops]
priority: P3
cost: M
---

## What

Found by the second pass of the walk-proofs unit (PR 3511).

`Body::movefac` (`crates/topo/src/movefac.rs`, the component labelling
before its mutation phase) walks each loop's `next` cycle from
`first`, follows every member's mate to the neighbouring face, and
then moves each component's faces into a shell of its own. The walk
reads no `parent_loop`: a torn `next` diverted through another loop
labels the faces that loop's members border, which can join two
components the shell really has, and one closed past a member misses
the faces that member borders, which can split a connected shell. The
move then re-partitions faces on the strength of the walk. Not
measured.

## The shape to give

Each walk proves it claims its loop (`Body::require_run_of`,
`RunExtent::Part`) and refuses `LoopCycleBroken` naming it before the
labelling is used. Pin one row per direction.

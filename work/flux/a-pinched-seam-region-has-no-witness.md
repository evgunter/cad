---
id: a-pinched-seam-region-has-no-witness
kind: issue
title: check 6 leaves a pinched seam region unread, and no body reaches that branch
status: open
opened: 2026-10-12
priority: P3
cost: M
refs: [a-pair-of-seam-siblings-flipped-together-is-silent]
---

## What

`crates/topo/src/validate.rs`, `seam_region`, marks a seam region
`pinched` when a boundary cycle passes through one vertex twice. Check 6
then leaves the region unread, because the side readers take simple
loops. No row reaches that branch: a mutant that ignores it survives
every suite (second review of PR 4746).

## Why no witness

A pinch needs a face that meets the region at one vertex alone. On one
sphere that face would lie on a second sphere entity coincident with the
first. The second review of PR 4746 measured both routes:

- `mef` minting such a face on a new coincident entity refuses
  `RechartUnvouched`: the run's edges are at rest on the old chart.
- STEP import merges coincident `SPHERICAL_SURFACE`s into one entity
  (`review_4746_2_import::review_4746_2_bowtie`).

A tangent pinch (a face touching the region along a tangent) has cusp
corners, which `sphere_loop`'s `props_sphere_loop_cusp` refuses before any
side is read.

## What closes it

Either of these:

- a door that reaches a coincident second sphere entity, plus a row on
  it that goes red when the branch is removed;
- the branch's deletion, once it is shown no body can reach it.

Until then the branch errs toward exemption, never toward a wrong read.


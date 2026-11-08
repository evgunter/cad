---
id: the-march-escalates-where-a-branch-crosses-a-c0-crease
kind: issue
title: a march whose branch crosses a C0 crease of a NURBS wall escalates on its step there, on the plane lane and the 1x2 lane alike
status: open
priority: P3
cost: M
opened: 2026-11-03
---

Found by PR 6412 (SECTARMS, NURBS × implicit stage 3a) and confirmed on
the plane lane by its dual review (R2).

## What happens

A tent creased along `x = ½` (a knot of full multiplicity), cut by the
unit sphere sunk 0.02 below the ridge's top, is one loop about the top.
Read whole on the 1×2 lane, its march escalates on its step where it
crosses the crease: `SectionError::Escalated` with predicate
`ssi_step_progress`
(`crates/geom-brep/src/pair_section/spline_implicit/curved/rows.rs`,
`a_loop_round_a_ridges_top_declines`). The same tent cut by the plane
`z = 0.28` through `PlaneSection` escalates on `ssi_step_progress` in the
same way (R2's probe), so the behaviour is the march's, not the lane's.

## Why

The step's rungs read the carrier's curvature at the step's start
(`crates/geom-brep/src/ssi/march.rs`, `march`, `relative_rungs`), from
the wall's third-order jet (`ders3`). At a C0 crease the chart turns a
corner, the one-sided jets disagree, and the step that straddles it
never predicts a state on the locus, so it halves into the band.

## What closes it

A march that cuts its step at a crease of the wall it reads (as the
regularity sweep already cuts the rectangle at its creases,
`spline_implicit/sweep.rs`), or a typed refusal naming the crease.
Either way, a row on both lanes.

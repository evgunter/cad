---
id: the-boolean-sectors-read-a-wrap-tip-as-a-one-edge-slit-tip
kind: issue
title: the boolean's sector and insert stages read a wrap edge's tip (a pole reached by one wrap edge) as a one-edge slit tip
status: parked
opened: 2026-10-29
priority: P1
cost: M
blocked_on: [loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk]
---


Ev ruled on `[ev]` PR 6014 (2026-10-29, fork row 215) that a chart singularity (a sphere's pole, a cone's apex) with no written edge to it is reached by ONE wrap edge, walked out and back: the wrap tip (DESIGN `[topology]`, the seam clause and tiers 2/3), "if the complexity it adds is tolerable", asking that keeping that complexity down be investigated. The probe that measured each reader is on branch `probe/foreign-wraptip` (edits E1–E7).

Owed here: `topo::sector_shape::SlitTips` takes a wrap edge's tip as a one-edge slit tip (its corner a full turn, as a slit's tip already reads), and `boolean::insert::strut_faces_first` orders a germ along the tip's edge (the probe's E4 and E5, which were invasive there; before them `Escalated{BisectorSide}` at margin exactly 0 and `CurvedBooleanUnsupported`). Read the corner through the shared loop walk (`work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`) rather than spelling it here. Not owed here: the one-site section loop on a curved face, which is owed under either form (a one-wall extrude reaches it).

Signed: (FOREIGN orchestrator)

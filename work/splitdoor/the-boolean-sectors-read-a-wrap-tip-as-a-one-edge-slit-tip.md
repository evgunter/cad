---
id: the-boolean-sectors-read-a-wrap-tip-as-a-one-edge-slit-tip
kind: issue
title: the boolean's sector and insert stages read a wrap edge's tip (a pole reached by one wrap edge) as a one-edge slit tip
status: open
opened: 2026-10-29
priority: P1
cost: M
---


Ev ruled on `[ev]` PR 6014 (2026-10-29, fork row 215) that a chart singularity (a sphere's pole, a cone's apex) with no written edge to it is reached by ONE wrap edge, walked out and back: the wrap tip (DESIGN `[topology]`, the seam clause and tiers 2/3), "if the complexity it adds is tolerable", asking that keeping that complexity down be investigated. The probe that measured each reader is on branch `probe/foreign-wraptip` (edits E1–E7).

Owed here: `topo::sector_shape::SlitTips` takes a wrap edge's tip as a one-edge slit tip (its corner a full turn, as a slit's tip already reads), and `boolean::insert::strut_faces_first` orders a germ along the tip's edge (the probe's E4 and E5, which were invasive there; before them `Escalated{BisectorSide}` at margin exactly 0 and `CurvedBooleanUnsupported`). Read the corner through the shared loop walk (`work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`) rather than spelling it here. Not owed here: the one-site section loop on a curved face, which is owed under either form (a one-wall extrude reaches it).

Signed: (FOREIGN orchestrator)

## Released (2026-10-30)

`loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk` closed with PR 6088. The shared structure is `Body::both_sides_face`, read through two views: `surface_cycles` (region) and `chart_image` (spur-free: integrators and chart readers). Read the tip through these, never by a turn of your own. (RESTCURVE orchestrator)

## Held (2026-10-30)

Re-parked on `sphere-faces-take-a-per-face-chart-or-pole-vertices` (a ruling open on `[ev]` PR 6085). Ev pointed out that a sphere face other than the whole sphere has a singularity-free chart, which may retire the pole vertex and the wrap tip for spheres. Do not build on the wrap tip until it is ruled. (RESTCURVE orchestrator)

## Released (2026-10-31)

Ev ruled on `[ev]` PR 6085, decision 3 (fork-log row 222): keep the wrap tip. A tip is any valence-1 end of a wrap edge, and the wrap edge is the face's seam kept to its end. A chart reader reads a tip as the seam, through the lift's reset at the singular vertex. A chart-free reader drops it, keeping the singular vertex as the side's datum. The hold is lifted. (RESTCURVE orchestrator)

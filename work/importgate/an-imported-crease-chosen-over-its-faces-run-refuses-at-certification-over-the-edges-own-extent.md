---
id: an-imported-crease-chosen-over-its-faces-run-refuses-at-certification-over-the-edges-own-extent
kind: issue
title: an imported crease chosen over its faces' run can refuse at certification, which reads it over the edge's own extent
status: open
opened: 2026-11-21
priority: P3
cost: M
---


Found by `oneround/lever-sites-brep`'s fix pass (PR 7219, 2026-11-21),
answering its review's question whether a crease the import chooses over
its faces' run completes as a body.

## What

Two readers lever one crease at different lengths.

- `step_import::build::choose` reads whether an edge's two planes cross
  over how far the two faces run, from below
  (`step_import::reach::faces_run`): 4 m for the fixture below.
- Certification's attachment gate reads the same crossing over the
  edge's own extent (`geom_brep::certify`, the crossing angle "levered by
  the arm it is read over"): about 1 cm there. `topo::wedge_reach` reads
  a built edge's faces off their vertices, the same order of length.

Between the two, `choose` builds a crease that certification cannot
show to cross, and the import refuses there. It is a typed refusal and
no body is built; the cost is its wording, which names the edge's
certification where the cause is that the file's two planes are nearly
one.

## Measured

The fixture is `build::tests::folded_lid`: two unit cylinders whose
walls cross along two rulings 2 cm apart, with two lids folded along
that chord. Each run is `build_one_solid` then the at-rest gate.

| run ε | file ε | fold | outcome |
|---|---|---|---|
| 1e-9 | 1e-6 | 1e-7 | refuses, `step_planes_cross` (one plane over the run) |
| 1e-9 | 1e-6 | 2e-6 | refuses, in the file's band |
| 1e-9 | 1e-6 | 3e-6 | refuses at edge 6, an end residual in the run's band |
| 1e-9 | 1e-6 | 1e-5 | finishes |
| 1e-6 | 1e-6 | 1e-5 | refuses at edge 1: "the faces are not shown to cross (margin 1e-7 m of the crossing angle)" |
| 1e-6 | 1e-6 | 4e-4, 1e-3 | refuses at edge 1: the crossing angle in the run's band |
| 1e-6 | 1e-6 | 1e-2 | finishes |
| 1e-12 | 1e-6 | 1e-5, 4e-4 | refuses at an end residual (the crease is ill-conditioned by `u/fold`) |
| 1e-12 | 1e-6 | 1e-3 | finishes |

At the default run ε with a file ε of 1e-6 there is no fold `choose`
builds and certification refuses for the crossing: `choose` needs
`fold·4 > 1e-5` and certification `fold·0.01 > 1e-8`. The window opens
where the file's ε is near the run's: at a run ε of 1e-6, folds from
`2.5e-6` to `1e-3`.

## What would close it

One of: the import refuses such a crease itself, in the file's words
(its two planes are one over the edge), by also reading the crossing
over the edge's own extent before it builds; or certification reads a
crease's crossing over the faces it bounds. Which is a choice about what
an edge's crossing is consumed over, and `wedge_reach`'s pin
(`work/oneround/lever-doors-pinned-sites-await-a-geometry-read.md`)
meets the same one.

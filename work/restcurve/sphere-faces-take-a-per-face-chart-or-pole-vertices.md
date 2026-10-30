---
id: sphere-faces-take-a-per-face-chart-or-pole-vertices
kind: ruling
title: Does a sphere face read a per-face singularity-free chart (stereographic from a point off the face) instead of pole vertices and wrap edges, and what does a cone apex then need?
status: open
opened: 2026-10-30
priority: P1
refs: [6085, 6014, 6097, validity-refuses-an-interior-chart-singularity]
---

Raised by Ev on `[ev]` PR 6085 (2026-10-30, in the comments). A sphere face that is not the whole sphere is a disc, possibly with holes, so it has a singularity-free chart: stereographic projection from a point outside its closure. Only the whole sphere needs a singular point.

The pole as a vertex, and the wrap edge that reaches it (Ev's PR 2850/4549 rule, PR 6014's wrap tip, PR 6054's refusal), are then an artefact of the sphere's one fixed latitude/longitude chart.

Open questions:
- Does each sphere face carry its own chart, stored or derived?
- What do the pcurves (check 8) and import's normalization read?
- Does a cone's apex, which is a real geometric singularity, still need a vertex, and does it still need a wrap edge?

Until Ev rules, the wrap-tip rows are parked here, and PR 6097 (the wrap-tip tier rules) is held unmerged. The orchestrator asked Ev whether to run a designer pair on it. (RESTCURVE orchestrator)

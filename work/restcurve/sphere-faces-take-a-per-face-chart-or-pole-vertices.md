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

## A wrap tip is a seam whose top collapsed (Ev's question on PR 6085, 2026-10-30)

In the fixed latitude/longitude chart, a polar cap is the rectangle [u₀, u₀+2π] × [v_rim, π/2]. Its wrap edge is the cap's SEAM, run up to the pole. The two passes of that seam sit a period apart in the chart. The top side (the pole segment) has zero length in 3-D, so in the loop the seam's up-pass and down-pass are adjacent, and the structural spur test fires on them. That gives the edge two readings:
- a **seam** for chart readers: the mesher, the lift, and integrals in the lifted coordinate;
- a **spur** for chart-free integrals (Gauss–Bonnet, 3-D flux), whose two passes cancel there.

PR 6088's chart image drops it as a spur for EVERY integrator. Whether that is right for a lifted-coordinate integrator (the quadrature lane on a sphere or cone face) with a wrap tip is unverified. PR 6097's rows measured a hemisphere and a cone correctly, but not through every lane. No door mints a tip today, so nothing live is wrong. Whatever this ruling decides must settle it: either per-face charts remove the seam and the collapsed top entirely, or the chart image treats a wrap tip as a seam. (RESTCURVE orchestrator)

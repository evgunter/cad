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

## Options, as of Ev's PR 6085 thread (2026-10-30)

Ev asked whether a pole vertex must lie on a loop, so that the seam has to become an edge. It need not. The forms to weigh:
1. **Wrap edge** (PR 6014's ruling). The seam is made real up to the pole.
2. **Two half-meridians meeting at the pole.** These were the designers' convergence on fork row 215.
3. **A one-vertex loop at the pole** (STEP `VERTEX_LOOP`). This form was never weighed. In the latitude/longitude chart the cap becomes the band case: the rim has winding +1, and the collapsed top, a zero-length loop, has winding −1. Readers already handle bands. What it costs:
   - tier 2's empty-loop refusal would change;
   - half-edge walkers would have to accept a loop with no half-edges;
   - the top segment's chart image would be synthesised;
   - import would keep `VERTEX_LOOP` rather than split it.
4. **A per-face stereographic chart from a point off the face.** No pole exists at all, except on the whole sphere. The cone apex stays a real singularity.

## Weighed (pole-form fork, fork-log row 220, 2026-10-30)

The Opus/Fable pair converged on its first reports:
- **Keep the wrap tip**, one form for both sphere and cone.
- **A tip is any valence-1 end of a wrap edge.**
- **The wrap edge is the face's D1 seam, kept to its end.**
- **The rejected forms:** the one-vertex loop (about 9 special cases against about 6, spread wider, and the cap would be the one seamless closed face), the per-face chart, and two half-meridians.

The recommendation is posted on `[ev]` PR 6085 as decision 3, with the DESIGN diff on that branch. This ruling closes on Ev's answer there.

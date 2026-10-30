---
id: import-splits-a-foreign-face-or-edge-at-each-c0-knot
kind: issue
title: STEP import splits a foreign face or edge at each interior C⁰ knot at its door, so nothing imported carries a crease inside a cell
status: review
opened: 2026-10-28
priority: P0
cost: M
refs: [loft-of-three-or-more-sections-at-degree-one-is-unmeshable]
pr: 6013
branch: foreign/c0-knot-split
---


## Ruling

Ev's ruling on [ev] PR 5913 (2026-10-28, Decision 2: "S, then"), now DESIGN [topology]: every face chart and edge carrier is C¹ by its knot structure strictly inside its cell; a construction whose carrier is not puts an edge or vertex there; import splits at its door; no reader carries a crease arm.

## Owed

At the import door, split each foreign face chart and edge carrier at every interior knot of multiplicity at least its degree. The split is exact: the knot vector already separates the sub-nets. Mint an edge (or a vertex on a curve) at each knot, carrying its pcurves through the split. The joint is a smooth seam where the foreign surface was smooth there, and a crease edge where it was not. No numeric verdict decides which, because both are edges.

**Sequencing:** land it before the tier-3 rule.

Filed by the TESS orchestrator at the ruling.

## How import satisfies the rule (FOREIGN designer pair, 2026-10-29)

Measured on the unit's branch (draft PR 6013): the standard rational-quadratic conic (equal arcs, weights 1 : cos(θ/2) : 1) is C¹ as a locus at every arc joint, not C², and C⁰ by its knot structure, which tests the homogeneous spline: the weight function's slope flips sign at the joint, so the double knot never comes out. Every torus, sphere, cone or swept circle a file writes as a spline is therefore cut at every joint, at the exporter's span count: the QUAD acceptance quarter torus goes from 2 faces to 162 and its import from 5.3 s to 125–146 s (release).

Ruled (Ev, `[ev]` PR 6014, fork row 216, option (i)): the split stands as built, exact. Its cost is met elsewhere: promotion first (`work/exch/step-import-promotes-torus-sphere-and-cone-nets.md`), so a foreign torus, sphere or cone has no knots and is never cut; and per-span quadrature schedules (below). No spline-to-spline rewrite: none is exact (in exact arithmetic on the fixture's floats no surface joint is exactly C¹; kinks of about 1e-16 to 1e-15 rad), and an ε_in-certified one would spread the weights 4–40× and raise the degree. A non-analytic rational surface (a swept circle, a revolved spline profile) imports as one face per span the exporter wrote; its seams are ordinary edges.

Parked on the quadrature row: this lands once a split rational conic costs what the whole one did, so the nightly does not take the 30-minute debug import.

Separately, the at-rest quadrature's round-zero schedules are per face (`AREA_CELLS_PER_AXIS`, `AXIS_INIT_PIECES`, `HULL_BLOCKS_PER_AXIS`, `REFINED_SPANS_PER_AXIS`), so a tile gets a whole patch's grid; they become per knot span (`work/quad/the-at-rest-gates-cost-grows-with-the-face-count-of-a-split-conic-patch.md`).

## Landing (Ev, 2026-10-29)

Built and held as **draft PR 6013** (branch `foreign/c0-knot-split`), which stays open while this row is parked. Ev: it stays open "as long as it's clear what it's waiting on … and whose responsibility it is to merge", and it is addressed soon, so this row and its blocker are P0.
- **Waits on** `work/quad/the-at-rest-gates-cost-grows-with-the-face-count-of-a-split-conic-patch.md` (P0): until the quadrature reads per knot span, a split rational conic costs about 25× its whole patch at the at-rest gate.
- **Who lands it: QUAD's orchestrator** (Ev, in chat, 2026-10-30: FOREIGN has no live orchestrator, and the remaining steps sit on QUAD's ground: the re-time is QUAD's acceptance, the conflict is in the conic test QUAD's unit re-baselines, and `rectangle_winding` is in `quad_lane.rs`). The row stays FOREIGN's. QUAD:
  1. dispatches the single FULL review now, on the frozen head `8e3c6dab`, since the split's correctness does not depend on the per-span schedules;
  2. once the QUAD row closes, merges main into the branch and resolves the conic test;
  3. re-times the conic quarter torus, in release and debug;
  4. runs the fix pass on the review's findings, un-drafts, and merges, closing this row with `pr: 6013`.
- **Alongside:** EXCH's `step-import-promotes-torus-sphere-and-cone-nets` (P0) takes the common conics out of the split's reach, but the split does not wait on it.

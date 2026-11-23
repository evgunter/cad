---
id: a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line
kind: unit
title: A face bounded by chart iso lines alone that is no rectangle meshes at the density its δ allows; the cures are measured, then one is built
status: dispatched
opened: 2026-11-16
priority: P1
cost: H
branch: tesslane/iso-run-density
---



Unit 4b of the chart lane (`work/tesslane/plan.md`, The chart lane), cut from unit 4 (`a-curved-face-is-routed-by-its-domain-not-its-carriers`) on 2026-11-16.

## What it is

The plan's sizing rule has two halves: a boundary run on a chart iso line gets a lattice line through it and sizes at δ, and a run that crosses lattice lines sizes at the one named `UNPROVED_HEADROOM` until its lemma is proved (unit 5). Unit 4 routes a notched face to the chart lane and meshes it at the lane's present target, the headroom's share of δ, on the uniform lattice. This unit is the first half of the rule: on a face whose every boundary run is an iso line, the lattice takes a line through each run, every boundary point is a lattice point, and the face sizes at δ with no headroom, as a rectangle does.

## Why it waits

Where a lattice point may sit beside the boundary is the question before Ev on PR 6966 (the lattice keeps clear of the boundary; every winding is resolved). A lattice line laid through a run puts lattice points on the boundary on purpose, and two runs at nearly one coordinate put two lattice lines nearly on each other, so this rule and that one are one design and are cut together once that PR is answered.

## Scope

- Only a face bounded by iso runs alone changes its lattice and its target. Every such face refuses on main before unit 4, so nothing that meshes on main moves. A face with one oblique run keeps today's lattice and target until unit 5.
- It is density, not reach: the faces mesh without it, at about twice the triangles a rectangle of the same size takes.

## Done when

The notched pipe walls of `reach_probes` mesh at δ with no headroom and their triangle counts are stated beside the counts unit 4 pinned; a lattice line within rounding of a run or of another line is one line, pinned on a body that reaches it; `[mesh.sizing]`'s wording of the headroom names which faces it covers.

- 2026-11-16 — **PR 6966 is answered and merged**: a lattice node is not laid within a clearance of its own step from the boundary. This unit now waits on the unit that builds that rule and on unit 4, which are both in the lattice's and the router's code; it is cut after them, as the rule above read against the clearance as built.

- 2026-11-17 — **"About twice the triangles" above holds only at coarse δ, and not on a ruled face.** Measured by three designers on the fork of that date (row 279 of `docs/DESIGN-FORK-LOG.md`): the chart lane lays rows a cylinder or cone rectangle does not need, and the ratio grows as δ shrinks. A unit 45° cone, full turn: 8 against 58 triangles at δ 0.1, 24 against 604 at 0.01, 72 against 6168 at 0.001; slender cones 150 to 500 times. With its rows switched off the same lane emits 100 against the rectangle lane's 72, and that remainder is the headroom's finer rim chords. The ruled-chart half of this unit is cut out as unit 4c (`a-ruled-face-with-a-split-side-meshes-at-the-density-of-the-face-without-it`); this unit keeps the sphere and torus rectangles.

- 2026-11-17 — **Unit 4 closed (PR 7041); this unit now waits on unit 4c.** The notched walls mesh on the chart lane's uniform lattice with its headroom. A lattice line laid through each boundary run is a constraint on the triangulation taken to get the bound, which is the question Ev answered on PR 7046 for the ruled charts (improve the bound's method before constraining the triangulation), so this unit is not built as written above. Unit 4c measured, on ruled faces, that sizing at δ without the headroom is what cures the thin-neighbour residual; what that says for a notched face on each chart is read once 4c has landed, and this unit is then re-specified or retired.

- 2026-11-22 — **Unit 4c closed (PR 7096); this unit is open and is to be re-specified before it is dispatched.** What 4c measured that bears on it: on a ruled face, sizing at δ with triangles made row against row beat every cure that kept the lattice, and refinement on the certificate did not converge with the one rule tried. The title above (a lattice line through each boundary run) is one cure among several and is a constraint on the triangulation, which Ev's answer on PR 7046 puts last. The re-specification is 4c's shape: measure first on the notched walls and the notched sphere faces (the density of today's chart-lane mesh against the same face sized at δ; which cures reach it, in Ev's order of preference: the bound's method, then the least constraint), stop and report, then build one.

## Unit (2026-11-22, TESSLANE)

**Letter H. Tier: dual review.** Whatever is built changes how the chart lane sizes and triangulates a class of faces (and the rims their neighbours share), each candidate adds or removes a mechanism in that lane, and "the density its δ allows" has no oracle but measurement. As unit 4c was. The two stages below are one unit; stage 1's report is the orchestrator's to rule on and is not reviewed by the pair.

Branch `tesslane/iso-run-density`, cut from `tesslane/ruled-density` at `a020a3edca` (PR 7096's closing head, reviewed and waiting on its hosted run): merge main once that lands.

**This section replaces "What it is", "Scope" and "Done when" above**, which describe one cure (a lattice line through each run) as the unit. The sections above are kept as its history.

### The class

A curved face with no ring whose outer loop's every edge lies on an iso line of its chart (a rim or a ruling on a cylinder or cone; a parallel or a meridian on a sphere or torus) and which is NOT a rectangle of that chart: a pipe wall notched through its seam, a ball less a quarter-turn wedge. `tessellate::route_of` sends it to the chart lane because the shape door decides "no rectangle" (`Trim::Read`), and the lane meshes it on its uniform lattice at the headroom's share of δ (`UNPROVED_HEADROOM`), each triangle held by its certificate. Bodies in hand: `reach_probes::a_notched_or_holed_pipe_wall_meshes` and `a_notched_sphere_face_meshes_on_the_sphere_row`; no door builds a notch on a cone or a torus (PR 7041). That list is a hypothesis: find the class by instrumenting the route, not by this sentence.

### What is not known, and is stage 1's to measure

- **How dense these faces are today against what δ allows.** Unit 4c found the chart lane 7 to 86 times a ruled rectangle's triangles, growing as δ shrinks, most of it lattice rows a ruled face does not need. Nobody has taken the same ratio on a notched wall or a notched sphere face. The reference to measure against is stated by the unit and argued, not assumed here; two candidates to consider: the rectangle lane's count on the face's bounding iso-rectangle scaled by area, and the sum of the rectangle lane's counts over the iso-rectangles the face divides into along the lines through its notch's corners.
- **How common the class is**: its faces counted over the `mesh`, `sweep` and `step-import` suites and the tour, by instrumenting `route_of`.
- **Which cure reaches the reference**, in the order Ev's answer on PR 7046 gives (the bound's method first, then the least constraint on the triangulation):
  1. the triangulation left alone, the face sized at δ without the headroom, and refinement where a certificate refuses, with at least two refinement rules: the lane's centroid insert, and one that splits the offending triangle's edge of widest span (unit 4c measured only the first, and it did not converge); on a ruled chart, also with the lattice's rows off;
  2. the least constraint: the iso lines through the notch's corners held as constraint edges, so the triangulation sees iso-rectangles, with rows off on a ruled chart;
  3. no triangulation step: the face divided into iso-rectangles along those lines, each meshed as the rectangle lane or unit 4c's walk meshes it, with the conformity of the triangles along the dividing lines shown;
  4. a lattice line through each boundary run (this file's first design).
  These four are the orchestrator's list, not the edge of the search: a cure the measurement suggests is measured too.

### Owed, in two stages

**Stage 1: measure, then stop and report.** Build each cure only as far as measuring it needs, behind a switch that does not ship. Corpus: the bodies above at δ 0.1 to 0.001 (finer where the ratio is still moving) and ε 1e-9, 1e-6 and 1e-12, placed obliquely as well as on the axes; notches of several widths and depths, a notch across the seam and one beside it, two notches on one wall, and on the sphere a wedge below, beside and through a pole. For each cure and cell: meshes or refuses and why; triangles against today's and against the reference; `check_mesh`, the winding rule, the certificate's worst reading, sampled deviation; for cure 1, the passes to converge and the wall time against today's. Also: what each cure does to the chord counts of the rims the face shares, and so to its neighbours. **Report the table and a recommendation, and build nothing further until the orchestrator answers.** If the measured ratio is small enough that no cure earns its mechanism, say so: retiring the unit is an outcome.

**Stage 2: build the cure the orchestrator names**, with rows that pin a relation (against the reference, read in the same run) rather than raw counts, mutants, the §5 sweep, and patch digests showing that no face outside the class and its rim-sharing neighbours moves. If the cure changes what `[mesh.sizing]` says a lane does, the new text is proposed to Ev with the measurement, as an `[ev]` PR of its own, not edited here.

### Not this unit's

- A boundary run that crosses the chart's lines (the plan's unit 5).
- Faces the rectangle lane takes today (`rectangle-faces-move-to-the-chart-lane`, unit 6, which is Ev's).
- Which rule the mesh applies to a side two faces can both make (`the-mesh-cures-a-side-two-faces-can-both-make-three-ways`, with designers). Stage 1 says whether any cure makes such a side on a class face and stops there.

**Pre-push for stage 2**, on the pushed head: fmt, `scripts/run-gates.sh`, the doc gate, clippy for the workspace (all features), the viewer with `--features app` and `pncad-py`; `mesh` whole with the default profile (slow set included) at the three ε; `test_binding_census` against a wheel if a public type moved; the three `tools` roots; `sweep`'s meshing modules; the viewer's slow-set rows pinned on mesh output (`display_budget`, `review_pick_r2`); a test costing 0.5 s or more goes in the slow set with its measured time.

### Stage 1's result and the ruling (2026-11-23)

Measured by the implementer over 210 cylinder-wall and 60 sphere-face cells (δ 0.1 to 0.001; ε 1e-9, 1e-6 and 1e-12, identical in every column; axis and oblique placements, identical), against the rectangle lane's own count summed over the iso-rectangles the face divides into (`r_dec`):

| cure | cylinder cells meshing | cylinder triangles / r_dec (min / median / max) | sphere cells meshing | sphere ratio |
|---|---|---|---|---|
| today (headroom, uniform lattice) | 210 | 1.2 / 22 / 223 | 60 | 1.65 / 1.96 / 4.4 |
| sized at δ | 192 | 1.0 / 11 / 115 | 60 | 0.75 / 0.95 / 3.0 |
| sized at δ, no rows, no refinement | 194 | 1.00 / 1.02 / 1.17 | 60 | as above |
| 1a: δ, no rows, refinement at the centroid | 210 | 1.00 / 1.02 / 1.40 | 60 | 0.75 / 0.95 / 3.0 |
| 1b: δ, no rows, refinement splitting the widest side | 210 | 1.00 / 1.02 / 1.20 | 60 | 0.75 / 0.95 / 3.0 |
| 2: 1a with the iso lines through the corners held | 210 | as 1a | 60 | 0.83 / 0.98 / 3.2 |
| 4: a lattice line through every run, at δ, rows kept | 210 | 1.5 / 13 / 120 | 60 | 0.86 / 0.99 / 1.02 |

One refinement round clears every refusing cell. The neighbours across the class faces' rims lose 2.7% of their triangles and none gains one. The class is thin in the suites: seven `mesh` tests reach it, and the census of `sweep`, `step-import` and the tour is not finished. Only cylinder walls and sphere faces reach it. The route's `Trim::Read` is wider than the class (a wall cut by a tilted section reaches it too), and a holed wall (`Trim::Structural`) behaves as the class does.

**Ruled: cure 1b, keyed to the class.** The class is one fact of the route, decided once: every loop image lies on an iso line, the face is no rectangle, the chart is a cylinder or a sphere, the holed sibling included. Such a face lays no rows on a cylinder and refines where a certificate refuses (the midpoint of the widest side that is no constraint, the centroid where there is none), under a named termination constant. Nothing in it reads ε.

**Not ruled here: sizing the class at δ.** `[mesh.sizing]` keeps `UNPROVED_HEADROOM` on a lane without its theorem and says the cure is the theorem; a refinement step standing where the theorem stands changes what that clause decides. It goes to designers and then to Ev as its own `[ev]` pull request with this table. The unit builds the δ target at one named site and does not merge before Ev answers; on a no, that site returns to the headroom. The implementer measures first what the cure earns with the headroom kept.

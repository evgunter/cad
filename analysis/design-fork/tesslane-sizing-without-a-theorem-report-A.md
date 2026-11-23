## For Ev

**Recommendation (likely).** Two moves, and the second is the actual question:

1. **Cylinder lattice rows come out of the chart lane** (sure; not a question). The cylinder's cell theorem is about the angular direction only. Along the axis the surface is straight, so rows buy no accuracy. Most of the measured excess comes from the rows, not from the half-of-δ factor. With the factor kept and rows off, notched pipe walls go from 1.2–223× the reference count to 1.00–1.50×, all 210 cells mesh, the worst triangle reads 0.69 δ, and no new mechanism is needed. This applies `[mesh.sizing]`'s existing rule ("the coarsest step the theorem grants"). It is a defect fix and does not touch the page. It is cylinder-only: with rows off, cone faces refuse (measured below).

2. **Replace "each lane has a theorem / UNPROVED_HEADROOM until it does" with: step from the chart's cell bound at δ; the certificate places a point where one is missing.** The final state:
   - **Step.** Every chart-lane face sizes its lattice and chords its rims at δ, from the per-chart *cell bound* (the closed-form bound on a triangle inside one lattice cell). That bound is already proved and pinned for every chart.
   - **Triangulation.** It is "any division into triangles". There is no lemma about what a constrained Delaunay triangulation does beside a trim chord.
   - **Certificate and refinement.** Every triangle is certified, as today. A triangle over δ gets one interior point: the midpoint of its widest side that is not a boundary side, or its centroid if every side is a boundary side. The face is re-triangulated, under a named work budget. If the budget runs out, the face refuses typed (`CertificateExceeded`), as today.
   - **What goes.** `UNPROVED_HEADROOM` is deleted on every analytic arm, not only for notched walls. So is the page's list of kinds of worst triangle (half a cell, a Delaunay sliver), which the ruled-face walk already falsified.

   The headroom is a safety factor under another name: 0.5 was chosen, not derived. The page forbids safety factors in the line above.

**Premise check.**
- (a) "No theorem" is narrower than it reads (likely). The cell bound exists for every chart. What is missing is only *containment*: how far past one cell a triangle beside an oblique trim chord can reach.
- (b) Soundness never rested on the step (sure). `[tolerance.chordal]`'s promise is held triangle by triangle by the certificate. The step decides two other things: how dense the mesh is, and how often a face refuses. The headroom pays for fewer refusals with uniform density everywhere. The certificate already says exactly which triangle needs a point.
- (c) This shape is already in the code. The spline faces of the same lane refine on a refused certificate, and its docs say "the certificate, as everywhere, is the guarantee". DESIGN `[tolerance.measure]` uses the same shape for measurement: refine until a predicate resolves, within a stated work budget.

**Ratified text I would change.** In `[mesh.sizing]`: "Each lane has a theorem…" and "A lane without its theorem yet keeps an explicitly named UNPROVED_HEADROOM…".
- Provenance: both lines are from S29 (`d0b1563d291`, PR 5708). In that PR, the headroom precondition was designer text you approved in passing. Your own words there were "as big as possible without any extra work" and "generally aim to never overrefine". On PR 7046 you asked that triangulation work "at the level of abstraction of any division into triangles so that it never deviates by more than delta".
- A containment lemma is a claim about the triangulation's behaviour, which is what you asked not to depend on. A global half-δ is the over-refinement you asked to avoid.
- I read "without extra work" as "don't work to land exactly at δ", not "no local repair". Whether one refinement round counts as extra work is yours to say (unsure).
- Proposed text: *Each chart has a cell bound: a closed-form bound on a triangle within one lattice cell, pinned beside its step function. A lane sizes at the coarsest step that bound grants at δ. Every triangle is certified; one over δ gets a point and the face is triangulated again, within a named budget, else the face refuses typed.*

**Measured** (whole `mesh` suite, release, one ε; I changed nothing that ships):

| configuration | rows | result |
|---|---|---|
| today | kept | 546/546 pass |
| δ, no refinement | kept | 7 faces that mesh today refuse on the certificate (tilted cylinder walls up to 2.0× δ, cone section and ring faces, a torus ring, two crossed rods, a notched pipe) and 1 folds (a tube near a sphere's pole) |
| δ + refinement, budget 6 rounds | kept | all certificate refusals cured except the wide tilted cylinder (r = 20, 2.0× δ) |
| δ + refinement, budget 40 rounds | kept | the tilted cylinder meshes, and a needle funnel the suite expected to refuse now meshes. One face still refuses: the near-pole tube folds (`Folded(Sphere)`). That is a fold, not a deviation, and refinement does not reach it. |

The rest are pins of today's bytes and of the headroom itself, which move as designed. With rows also off on cones, 9 cone faces refuse even with the headroom kept, so rows stay on cones.

**The options as final states.**

- **A: the rewrite above** (recommended).
  - For: density at δ everywhere, by the stated measurements about 2× fewer triangles on spheres and about 1.4× fewer on cylinders, and fewer points on the rims shared with neighbours. No unexplained constant. One rule for every lane, the spline faces included. Refinement points are interior only, so neighbours never see them. The mesh is still a function of (body, δ).
  - Against: a face may take several re-triangulations (up to tens on a wide tilted wall). One suite face, the near-pole sphere tube, turns from meshing into a typed fold refusal unless its fold gets its own cure. The work budget is a named constant: a work bound like `MAX_COUNT`, not sizing.
  - Reversible: yes, it is one target site.
- **B: keep the page, headroom until a lemma exists, rows off on cylinders.**
  - For: no refinement on analytic faces, and the suite stays as it is.
  - Against: spheres and every oblique face stay at about 2× forever unless someone proves a lemma about triangulator behaviour (two attempts so far, both marked unsure). It also keeps the 0.5 beside the "no safety factor" line.
- **C: δ plus refinement for the notched-wall class only** (the current interim ruling).
  - Against: the class is a fact about routing, not about meaning. The reason for the headroom (no containment bound) is the same on every chart-lane face, so C puts two rules on one question. Not recommended.

**What becomes of the debt rows (under A).**
- The two "has no worst-triangle theorem" rows (cylinder, sphere) close as superseded.
- The containment-lemma unit is retired, replaced by two smaller rows: the refinement budget, set by measurement with a typed refusal past it; and the near-pole sphere fold.
- One follow-on question, not decided here (unsure): the spline faces' `LATTICE_SHARE` also spends a share of δ for the same purpose and could be measured against the same rule.

## For the orchestrator

- **Measured vs read.** Whole `mesh` suite (default profile, 546 tests, release, ε 1e-9 only) under `ISO_CURE` = none, `delta`, `rowsoff`, `delta,refine_edge` and `delta,rowsoff,refine_edge`. I also ran a scratch edit, `MAX_GRID_RETRIES` 6 → 40, on four tests only, then reverted it; the tree is clean. Raw output: `/tmp/claude-0/-home-user-cad/fb1e15c1-4d32-594f-bc8e-32180b8de3b0/scratchpad/c_*.txt`, `cure.txt` and `r40.txt`. Everything else in the report is read, not measured. I did not run the probe harness or the corpus.
- **Error in the brief.** "Applied to every chart-lane analytic face, the same switches make faces refuse" mixes three causes:
  - The cone refusals come from `rowsoff`, whose probe also strips cone rows via `development().is_some()`. Rows matter on a cone in this lane.
  - The tilted-cylinder, torus, rods and notched-pipe refusals come from sizing at δ, and refinement cures them. The wide tilted wall needs more than the shared budget of 6.
  - The sphere fold is neither, and refinement does not touch it.
  
  So 1b's "one round on every cell" holds in the class but not lane-wide.
- **Unchecked claims.**
  - "Cell bound proved for every chart" is taken from the unit-5 row and `sizing.rs` docs; I did not audit the pins.
  - Refinement termination has no proof; it is a budget plus typed refusal.
  - The refinement cap is currently shared with the fold and shared-side retries (`MAX_GRID_RETRIES`). A design under A probably wants its own named budget.
- **Defects off the question (not filed; I commit nothing).**
  - (1) `a_needle_funnel_cut_at_its_apex_refuses_typed` changes refusal cause, or meshes, under δ + refinement. Its pin is about the cause.
  - (2) The near-pole tube on the ball folds at δ but not at δ/2. The step is preventing a fold that no rule names. That is worth a row whichever option is chosen.
  - (3) The `trimmed.rs` probe at line 664 applies `rowsoff` to cones despite the "cylinder" comment in the measurement brief. The implementer's cylinder-only table is consistent with this, but a lane-wide run is not.

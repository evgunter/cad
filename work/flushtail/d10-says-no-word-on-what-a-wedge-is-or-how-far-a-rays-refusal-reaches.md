---
id: d10-says-no-word-on-what-a-wedge-is-or-how-far-a-rays-refusal-reaches
kind: issue
title: D10 does not say what a wedge is, how far past the band a ray's refusal reaches, or whether a winding number may read containment where no ray can
status: open
opened: 2026-10-20
priority: P3
cost: M
design: true
refs: [5422]
---

Three questions the edge-meeting ray left on the ratified text. Each was measured by PR 5422's reviews; settling any of them changes `[coincidence.booleans]` or `[verdicts]`, so it is Ev's call (an `[ev]` PR, weighed first by a designer pair if it forks).

1. **What a wedge is.** D10 says "a ray that meets a corner or a wedge reads nothing" and does not define "wedge". PR 5422 reads it as *a face the ray meets at two of its edges*.
   - Both reviews measured that this rule is load-bearing. Without it, the half rule is applied twice on a narrow face, and `point_in_solid` reads wrong on needles under 2Kε, 14–21 points of 400 in places.
   - The second review measured that the narrower reading gives the same results as the PR's first, broader one, which added a clause for two met edges sharing a vertex.
   - Should D10 state the definition?
2. **How far past the band a ray's refusal reaches.** D10 says the ray "refuses only on two faces that do not meet whose crossings lie in band of each other". An edge's passage can slide along the ray, so two faces the census reads definite apart can still have passes in band.
   - Measured on the second review's re-check at `fc93825299`: a staircase of two boxes with faces 1.3–1.8Kε apart refuses an order-row pair in 8 of 8 cases at ε 1e-9 and 1e-6 (`review_r2_a_staircase_of_two_corners`, branch `flush/edge-meeting-ray-review-r2` @ 8aa59db083).
   - So the reach is about 3Kε: the band plus two slides. `side_at_infinity`'s doc hedges this with "about"; nothing states the bound.
   - The refusal restricts, so no reading is wrong; the cost is lost rays. Should the bound be stated, or tightened, for example by deciding a pair from the census's face–face read?
3. **A winding number for containment.** Inside a needle narrower than 2Kε, every point lies in band of the boundary, so no tie-break-free ray can read a role there; the volume reads it, as ratified.
   - For `point_in_solid`, though, the second review argued that a generalised winding number at the query point is perturbation-invariant over a cluster whose endpoints are definite, and would read containment with no tie-break.
   - May an instrument read containment that way where the ray reads nothing?


# SECTARMS — the log

## 2026-10-28 — opened

(SECT orchestrator) Cut from SECT at Ev's request in chat, SECT being
over its budget. SECT keeps the torus-pair
thread (teapot wall 2), its main red and two follow-ups; this track takes
the section object's remaining arms, six rows moved by `git mv` with ids
and bodies unchanged. No orchestrator yet.

- 2026-10-28 — (SECT orchestrator) The closed-edge and C5-residue rows moved on to SECTSITE before this track opened. The tracker's weights are now E 1, M 3, H 8, so six rows were 38 against 30. Dispatchable here: 4, for 22 points.

## 2026-10-31 — picked up

(SECTARMS orchestrator) Track picked up; status `active`.

- **The P0 NURBS × cone/sphere row goes to the designer pair** (blinding record `analysis/design-fork/nurbs-curved-section`). Both were handed the problem only: how the section of a NURBS face against a curved analytic face is computed and certified as an arm of the value, and where its tangency is decided. Each is asked whether the answer also covers the NURBS × torus row.
- **The skew-cylinder and walk-order rows go to a survey lane first.** Both were filed before the section-first design. The skew row asks for a frame, which that design retires. The walk order may close by retirement (the plan's own note). The survey reports scope, cost and tier for each before an implementer is dispatched.
- The cone-frames row waits on the cone's metres conversion (plan, Order 3) and keeps its design flag.
- 2026-10-31 — **Survey in** (skew cylinders, walk order; probe in the lane's private clone, uncommitted). Neither row closes by retirement.
  - **Skew cylinders:** the row's poses still refuse `GermFrameUnsupported`. Coplanar axes with unequal radii are a second unserved family. The row is re-scoped onto the value: cylinder × cylinder becomes an arm of `pair_section`, re-costed H with `design: true`. The question underneath is how the value bounds and levers a pair with no bounded side (parallelism needs a length, a near-parallel ball grows as `r/θ`, parallel components are open lines), plus whether crossing nodes (the bisector ellipses) live on the value. The cone-frames row has the same question. It goes to a designer pair once the P0 fork, which gives the value a second domain, has converged, so the two answers are designed against each other rather than in parallel.
  - **Walk order:** three spellings now (the carried lane's foot parameter, the frame lane's turn, the split's walk). The cheap closure is a row holding them against each other on a tilted ellipse on a cylinder and a circle on a sphere. Dispatched as `sectarms/walk-order-held` (M), tier **single style review**: test-only plus visibility seams, and the risk is a row that cannot go red.
  - **Filed:** `conic-germ-pairs-rank-by-the-frames-turn-not-the-foot-parameter` (SECTARMS, P1 H), the move that would retire the frame lane's order. `apex/a-near-right-angle-cylinder-pair-stops-at-a-first-order-sector-side` (P1 M), the survey's off-row stop at 1.5707963 rad.
  - **Load:** dispatchable now P0 H + P1 3×H + M = 35 against 30. A split is owed if the slate stays this size after the P0 fork settles; the natural seam is the unbounded-pair arms (skew cylinders, cone frames, conic pairs) against the NURBS arms.
- 2026-10-31 — **P0 fork, reconciliation.** First reports and round 1 recorded (analysis-side notes). Both designers converged on one implicit × chart arm over the knot rectangle, open components, and a pose decision on the composite before any march. The two crossed on one item, whether an isolated interior tangent point is decided Zero, so round 2 is running.
- 2026-10-31 — **P0 fork converged; `[ev]` PR 6121 opened** (fork row 223, record `analysis/design-fork/nurbs-curved-section`).
  - **What the PR proposes:** one implicit × chart arm, `φ = f∘S` over the knot rectangle, with the rectangle as the value's second domain and open components. Plane × NURBS folds in (row 217's arm, PR 6026), and so does the torus row. Tangency and coincidence are decided once in a pre-march classification (the hull; the boundary pass; one regularity sweep, with W5 as its plane instance). An isolated interior touch with a definite Hessian is Zero. C5's carve-out is replaced for implicit partners.
  - **Rounds:** round 1 crossed on the interior touch, round 2 converged, round 3 was my check against row 217 / PR 5990, which neither designer had cited. Both found the design consistent and generalising.
  - **Premise corrections carried in the PR:** the row's fitted-cubic vs per-span fork is false, no demo joins on the arm alone, and klein also needs cone × NURBS.
  - **Off-question findings:** `boundary_section` and `sweep_chart_plane` hand-roll `f∘C` beside `geom_core::spline::compose`, and the route table's note plans a chart tube for the analytic partner. Both retire inside the arm's build (named in the PR), so neither is filed separately. Measure before the build: does klein's rim read Zero or in band at ε 1e-9 (cap tilt 3.99e-4 rad)?
  - The P0 row keeps `design: true` until Ev answers.
- 2026-10-31 — **Walk order held (PR 6126; `the-walk-order-is-spelled-twice` closed).** `join_walk_order_rows.rs` holds the boolean's turn order against the split's walk on a tilted ellipse and a circle, plus the carried lane's foot order on the circle (no carried ellipse exists). Each lane gets one scrambled order, ranked through production's `keep_nearer`, both senses, on and off the conic.
  - **Tier: single style review.** APPROVE-WITH-FIXES with one MINOR: the boolean lanes were unscrambled, so a no-op ranking passed ccw. Fixed. Mutants N, N2 and A–G now go red in each sense alone.
  - **What the off-conic rows found:** the orders never diverge, but at a steep flank the split's walk decides two sites the band cannot order, and refuses them as `NotAlternating`, a face claim, where the boolean escalates. Filed `the-splits-walk-decides-off-conic-sites-the-band-cannot-order` (P3 M), held by a disabled red.
  - **Retiring the frame lane's spelling** remains `conic-germ-pairs-rank-by-the-frames-turn-not-the-foot-parameter`.

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
- 2026-10-31 — **Unbounded-pair fork converged; `[ev]` PR 6127 opened** (fork row 224, record `analysis/design-fork/unbounded-pair-section`). It covers the skew-cylinder and cone-frame rows.
  - **The design:** one region per operation, the ball about the operands' overlap, owned like the band. A pair's domain is its own ball cut by that region. Components can be open, or end at a node (D2's page). Every node is a recorded Zero tangency in closed form, with no node march built. Cylinder × cylinder goes on the value in every pose, on closed-form margins. The cone pose is `section_cert/ruling.rs`'s sweep with its double root decided. C5's no-face-extent sentence is restated.
  - **Rounds:** round 1 crossed on nodes and on whether D2 lets a branch end at a node. I settled D2 myself: the sentence is in the geom-brep README's Edge descriptions. Round 2 converged.
  - **Coupling:** shares the "component with ends" type with PR 6121. The two builds are sequenced so `Components`/`CarriedComponent` change once.
  - **Build-time items (not forks):**
    - Reading a pinch as the guard's `Touch` needs its own argument (A).
    - A Zero double root needs an upper clearance bound at the root (A).
    - `of_pair_section` must read a node's connected set as one component (B).
    - The cylinder march door seeds on the thinner wall's chart cut by the region, since a 3-D tree is `(L/r)³` (B).
  - **Off-question, retires inside the build:** `cylinder_cylinder_section` decides radii before parallelism, so parallel unequal radii reach only the frame.
  - Both rows keep `design: true` until Ev answers.
- 2026-10-31 — **Ev's questions on PR 6121 open a third fork** (record `analysis/design-fork/section-trace-domain`). Ev asked why analytic pairs are traced over a ball rather than a chart, and whether general NURBS and well-behaved analytic charts should share the knot-rectangle form. I answered the factual parts on the PR: the trace space is chosen by where each side has a compact, regular, exact description, and analytic carriers are whole closed or infinite surfaces. The architectural question went to a fresh designer pair: in which space and over which domain each kind pair is traced, and whether 6121 and 6127 together are the final state or a local optimum. Neither PR merges before this is back.
- 2026-10-31 — Walk order: PR 6126 style review APPROVE-WITH-FIXES. MINOR: the boolean lanes were not scrambled, so a ranking that decided nothing passed the ccw walk. Fix pass dispatched: scramble every lane, rank through production's pick, add off-conic sites. **A class to watch:** the test rows hand-build `Reach`/`Turn::facing` records in three places (`join.rs` nearer_along rows, the new walk-order rows, `partners`' tail), so a production change to `Reach` passes them silently. If a fourth copy appears, give the builder one test home.
- 2026-10-31 — **Trace-domain fork converged** (fork row 225, on PR 6121). The two PRs stand in substance under one per-side rule: each side enters through its exact description, residual else chart, and the domain is the trace space's compact part. Analytic charts gain nothing; the cone's chart is a named fallback only. Edits pushed:
  - **6121 C3:** the domain rule, three trace shapes, and rim/cut/node ends.
  - **6121 C5:** an indefinite-Hessian node is decided Zero, as 6127 decides it. A collapsed side not decided clear of the partner refuses before the sweep. Spline × spline is worded as unbuilt.
  - **6127:** the region is scoped to ℝ³ pairs, and `section_cert::Reach` retires into it.
  - Corrected on the PR: my "compact, regular, exact" rule and my collapsed-side gradient claim were both wrong.
  - Build-time notes: closed nets need nothing (D1's seam edge). An interior fold belongs to surface validity, outside SECTARMS. `ChartedNurbs::mint` refuses a zero sup speed, not a collapsed side, so the pre-sweep refusal must read C4's collapsed decision.
- 2026-10-31 — **Walk order held (PR 6126; `the-walk-order-is-spelled-twice` closed).** `join_walk_order_rows.rs` holds the boolean's turn order against the split's walk on a tilted ellipse and a circle, plus the carried lane's foot order on the circle (no carried ellipse exists). Each lane gets one scrambled order, ranked through production's `keep_nearer`, both senses, on and off the conic.
  - **Tier: single style review.** APPROVE-WITH-FIXES with one MINOR: the boolean lanes were unscrambled, so a no-op ranking passed ccw. Fixed. Mutants N, N2 and A–G now go red in each sense alone.
  - **What the off-conic rows found:** the orders never diverge, but at a steep flank the split's walk decides two sites the band cannot order, and refuses them as `NotAlternating`, a face claim, where the boolean escalates. Filed `the-splits-walk-decides-off-conic-sites-the-band-cannot-order` (P3 M), held by a disabled red.
  - **Retiring the frame lane's spelling** remains `conic-germ-pairs-rank-by-the-frames-turn-not-the-foot-parameter`.
- 2026-10-31 — **Ev approved both forks.** PR 6121 merged ("cool, i think this design sounds good!"). The P0 row is decided and the torus row is covered by it. PR 6127 was approved ("ok, this looks good!") and merges next; the skew-cylinder and cone rows are decided. Fork rows: the NURBS arm is **225** and the trace domain **226** (renumbered at the merge, since main took 223 for another fork); the unbounded pair is 224. Earlier entries here cite 223 and 225; read them as 225 and 226. PR 6126 merged, closing the walk-order row.
- 2026-10-31 — **Process findings (mine):**
  - **A brief restated Ev's question in my words.** "Local optimum" came back in both designers' reports and in my summary as if Ev had asked it, and Ev caught it. A designer brief quotes Ev's question verbatim and keeps any framing of mine visibly separate.
  - **A chained shell command committed a conflicted merge.** It ran `git merge … && python3 …; …; git commit`, and the `;` after a failed merge let the commit run, pushing conflict markers to the 6121 branch. I fixed it forward. The rule I now follow: a merge in a script stops the whole script on a non-zero exit, and markers are checked with `git grep` before any commit.
- 2026-10-31 — **Next:** the builds.
  - The NURBS arm is staged behind SPLICE's PR 6026, open and idle since 10-29. That PR adds `ComponentShape::Open { reaches }` and the `(Plane, Nurbs)` arm, and the W5 row follows it.
  - The cylinder × cylinder build depends on nothing open, but it shares the component-ends type with 6026, so its spec reuses whatever 6026 lands rather than adding a second.
- 2026-10-31 — **Cylinder × cylinder: phase 0 plan accepted; stage 1 dispatched** (`sectarms/cylinder-pair-on-the-value`).
  - **Measured:** the ℝ³ march already handles two cylinders. Seeding is cheap at the current floor.
  - **The stages:**
    - **1** (geom-brep, H, **dual**): the pose with the region and the shared end type, which folds `BranchEnd::Slab`/`SlabInBand` into one cut end.
    - **2** (M, single full): the region at the glue door, plus `section_door` and `section_cert` reading the value.
    - **3** (topo, H, dual): the join reads the value, and the frame's cylinder arm retires.
    - **4** (conditional): chart seeding, only if a witness row shows region-scale seeding misses a loop.
  - **Not waiting on SPLICE's PR 6026** (590 commits behind, idle). Stage 1 builds the end type in a shape 6026 adopts mechanically; a coupling note is on SPLICE's log.
  - **Adjudication of the plan's findings:**
    - **(1)** The accounting cost grows with the region, and 6127's worked example refuses `CellBudget` rather than listing arcs. Filed `a-near-parallel-pairs-accounting-grows-with-the-operation-region` (P1 H, design). It goes to designers, and to Ev if it moves C3. Ev is told the example overclaimed.
    - **(2)** Count components only when the pose's ball lies inside the region, else march. Accepted.
    - **(3)** `cylinder_cylinder_section` becomes the pose's closed-form constructor. Accepted.
    - **(4)** `section_cert::Reach` also serves the cone and torus arms. This unit removes only the cylinder use, and the rest goes to the cone row (noted there).
    - **(5)** The march domain is the meet of two boxes, a sound superset. Stage 1 adds that evidence to SSIEDGE's `ssi-r3-slab-is-not-geometry`.
- 2026-10-31 — (SPLICE orchestrator) **Reply on PR 6026's coupling.** SPLICE prefers to land 6026 first. It is in its final fix pass after two rounds of review (a concurrent pair, then a re-review) and should merge today. Please have the cylinder × cylinder unit adopt 6026's shapes (`ComponentShape::Open { reaches }` and `pair_section_with`'s march door) and generalise them in one place, rather than changing two signatures at once. On W5: SPLICE will read `[ev]` PR 6121 before building `w5-loop-free-stage-before-the-plane-nurbs-whole-reading`, and will build it as the plane instance of the regularity sweep if Ev approves that.
- 2026-10-31 — **Accounting-cost fork → `[ev]` PR 6137 as a stated split** (fork row 227, record `analysis/design-fork/region-accounting-cost`).
  - **Agreed:**
    - For every ruled pair, the ruling sweep is the pose's reading, and seeds come from it.
    - A 1-D coverage of the ruling angle and a one-to-one end-and-witness match both exist.
    - Ruled pairs get no 3-D accounting.
  - **Split:** does the coverage run at runtime as the closing (A), or in tests as the oracle with cardinality closing (B)? It is a reading of `[one-way.readings]`, which permits both "stages of one proof" and "a second method … only as a test oracle".
  - **Why stopped:** four crossovers, each round's newest argument shown to the other in the last, so it was stopped as a split rather than a fifth round.
  - **To file once the build row is written:** `ssi-domain-extent-means-three-things` (SSIEDGE P2), `a-long-ssi-arc-fails-limb-three-over-its-chain` (P2), and a faster general sweep for marched poses (P3, with the two failed steering prototypes as evidence).
- 2026-10-31 — **Accounting fork decided** (PR 6137 merged; fork row 227). Ev chose B and placed its oracle as a `debug_assert`: cardinality with one-to-one end/witness matching closes, and the ruling-angle coverage is DESIGN's row-5 debug assertion ("in production we don't want to waste time double checking things"). The ruling sweep, one-to-one matching and sweep seeding become a new cylinder-unit stage, after stage 1 and before stage 3. My ruling (2) is retired.
- 2026-10-31 — **Cylinder stage 1 open: PR 6139**, frozen at `b61ee71`, CI green. **Dual concurrent review dispatched** (arm: concurrent, unit difficulty H), R1 and R2 on the same head with identical briefs and eight claims to falsify. Also cleared the finished lanes' clones and the cylinder lane's target for disk (90% → 32%).
- 2026-10-31 — **Cylinder stage 1 (PR 6139) reviewed and fixed.**
  - **Dual concurrent review** on `b61ee71`: both reviewers APPROVE-WITH-FIXES, and the one MAJOR is bilateral. Coaxial cylinders of unequal radii refused `CoincidentSurfaces` because the reorder dropped main's premise that the radii were already decided equal.
  - **Fix pass `a865b8f`:**
    - coincidence needs both coaxial Zero and radii Zero;
    - closed forms obey the domain through one `placement()`;
    - rows added for the unequal chord, the off-centre region, coplanar unequal radii, pipes and bosses;
    - the parallel ladder lives in one place;
    - docs say the march domain is the cube of one ball.
  - **Two calls I accepted:**
    - `crossing_ball`'s `|e|/2` never binds (measured), so it stays a conservative bound;
    - the crossing `Pinched` arm stays row 1, because `Band::new` admits an escalation edge under twice the zero.
  - **Behaviour that moved:** the replace_face neighbour gate now refuses `VertexDisagreement` (filed on SHELF, P3).
  - **Filed** `a-one-loop-cylinder-pair-refuses-winds-unseparated-at-coarse-eps` (P2), unmeasured.
  - **Class to watch:** reordering margin reads can drop a premise a later read relied on. That is what made this MAJOR. A unit that reorders pose decisions sweeps every later read for that premise.
- 2026-10-31 — **PR 6139 merged**: cylinder stage 1, with its dual-review row as the last commit. **Stage 2 dispatched** (`sectarms/cylinder-pair-readers`, tier single full): the operation's region at the glue door, and `section_door` and `section_cert` reading the value. The ruling-sweep stage and stage 3 follow, one at a time, because disk holds about one warm lane plus reviewers.
  - **New process rules on main today** (Ev on #6132): a drawn third of design forks get a second Opus designer, and reconciliation runs until all designers agree or a round changes no one's mind, replacing the crossover rule. Both apply to this track's forks from now on.
- 2026-10-31 — (SPLICE orchestrator) **PR 6026 merged (`6335ffd3`); W5 dispatched** (`splice/w5-loop-free-sweep`).
  - 6026 adopted 6139's shapes: `ComponentShape::Open { ends: ComponentEnds { rims, .. } }`, with SPLICE's exact-sides `ChartReach` as the type in `rims`. `pair_section_with` is folded into `pair_section(a, b, region, lane, band)`.
  - W5 is built per PR 6121 as the plane instance of the pose classification's regularity sweep, in `pair_section/plane_spline.rs`, and the guard reads its verdict. It may move W0 (`section_cert::nurbs_plane`) into the pose as stage 1.
  - Krawczyk isolation and the Hessian decision are left to your NURBS × implicit build, with the plug-in point named in W5's PR.
- 2026-10-31 — **Stage 2 open: PR 6151** (`cf488e3`, CI green).
  - **Region at the glue door.** `section_door` and `section_cert` read two cylinders through the value, and node-joined arcs read as one component.
  - **A guard I didn't ask for:** a `carried_at_the_join` gate keeps cylinder pairs off the join's carried lane until stage 3, because a node site desyncs before then.
  - **SPLICE's 6026 merged mid-stage** and was absorbed: its march door and rim reaches.
  - **Eight rows moved to the slow set** (1.3–4.1 s), because the guard now marches cut pairs. The ruling-sweep stage should bring them back.
  - **Held before review:** GREEN's new P0 (18 sweep rows red on main) names 6139 as a suspect, so the same lane bisects it first. P0 work outranks the P1 review.
- 2026-10-31 — **GREEN's P0 is not ours.** All 18 rows were red before PR 6139 (17 with identical panics at 6139's parent, at its merge and at stage 2; the 18th still running after 22 minutes at the parent). The evidence is on GREEN's row, and 6139 leaves its suspect list. **Stage 2's review dispatched** (single full, on `cf488e3`).
- 2026-10-31 — (SPLICE orchestrator) **W5 landed (PR 6150, `8b890892`).** The `(Plane, Nurbs)` arm your NURBS × implicit build generalises is now in place.
  - **The stages in code:**
    - stage 1, `PlaneSplinePose::hull`: apart, coincident (D10) or in band;
    - a door that refuses a collapsed side not decided clear over its whole row (`chart_singularity::spline_collapsed_row`);
    - stage 2, C3's own pass (`PlaneSection::pass()`), run once and carried to `all()` and `read_whole`;
    - stage 3, `plane_spline/sweep.rs` on the shared recursion (`exhaust::SweepDuty::Regular`), with a union-find same-side join.
    - The guard reads `Components::Reaching(Reached)`; `Reached::new` refuses an empty reach.
  - **Krawczyk and Hessian isolation** plug in at `Declined::Floor`.
  - **A class the review found, which bears on your build:** W5 first ran its sweep ahead of stages 1 and 2. A wall within the band everywhere, and a boundary graze, then cleared where main refuses them as tangent. Run the hull's coincidence call and the boundary pass before the sweep for every implicit `f`. The re-review also showed that the sweep alone reads a long thin in-band wall as loop-free, so the guarantee lives in stages 1 and 2.
  - **Also:** "excluded or regular ⇒ loop-free" is unsound without the same-side join, because an excluded cell can hold the loop's extremum.
- 2026-11-02 — **Resumed after the weekly limit** (it hit on 10-31 during the stage-2 fix pass; nothing had been pushed).
  - **Stage 2 fix pass resumed.** The full review returned APPROVE-WITH-FIXES with no MAJOR. Adjudicated: the join's held lane gets the same gate as the carried lane; two staging gaps (records at the region vs the frame's reach; the gate's retirement) are named on the skew row; rows pin the straddling-march and tangent-ruling paths; `region_for`'s kind census moves to geom-brep; the string-keyed control flow becomes typed.
  - **The P0 NURBS × implicit arm is unblocked:** SPLICE's 6026 and W5 (PR 6150) landed. Its phase-0 plan is dispatched (`sectarms/nurbs-implicit-arm-build`). SPLICE's stage-order class (run the hull's coincidence call and the boundary pass before the sweep, keep the same-side join) is in its brief.
- 2026-11-02 — **The P0 NURBS arm's plan is accepted** (four stages; build after cylinder stage 2 merges, which shares `pair_section.rs`, `section_cert.rs` and `glue.rs`):
  - **Stage 1** (H, dual): classification for every implicit `f`, in C5's order (hull, door, pass, sweep), plus topo's routing.
  - **Stage 2** (H, single full): critical points decided (a 2×2 Krawczyk, the Hessian), plus a crease pass.
  - **Stage 3** (H, dual): the curved whole reading on the rectangle (`LocalSystem<1,2>`).
  - **Stage 4** (H, single full): fold the plane in and retire the ℝ⁴ plane lane.

  Filed `the-join-reads-a-spline-implicit-section` (P0 H, parked on the P0), the join half the four stages leave out. No demo pin moves in any stage. Teapot, klein and lily each stop upstream first, and lily's stop has moved to SECTSITE's plane × torus frame (corrected on the torus row).
- 2026-11-02 — **The plan's findings, adjudicated:**
  - **(F1) The crease pass.** On a disc of the rectangle, an extremum of `φ` on a crease is a critical point of `φ` along that crease line, or lies at its end, so a 1-D regularity pass along each crease completes the sweep's proof. Accepted as part of the one regularity sweep, not a new decision. C5's sentence still decides the same thing, and the crease pass is how the sweep decides it on a creased wall. Stage 2 builds it, and Ev is told. Klein's half-tube is the case.
  - **(F2)** The coincidence hull needs a lower bound on the metre divisor, per kind. Build-time.
  - **(F3)** A decided touch makes small closed loops at nearby levels, so "loop-free is the sweep's verdict" holds outside decided touches, and the same-side join exempts them. A wording correction that lands with stage 2.
  - **(F4)** C3 states the 1×2 trace in the present tense with no code yet. That stays untrue until stage 3 or 4. Accepted as drift with a schedule.
  - **(F5)** Fixed: the torus row's stop.
  - **Load:** the slate is now well over budget (the NURBS arm is about 4 H, the cylinder unit's remaining 2 stages, the cone, the conic pairs). A split along the semantic seam (the NURBS arm vs the unbounded analytic pairs) is owed at the next natural seam.

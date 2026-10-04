# Review — PR #4025 "join: the transverse cylinder × sphere germ frame"

Frozen head `e207aeb5` (PR commits `4d141a69`, `aaea5ca3`; base merge of main `e4a0a0d1`). Probes: `crates/sweep/tests/cylinder_sphere_{frame,door}_review_probes.rs` (ignored rows) and `review-probes/` (Python: analytic frame sweep, independent volume oracle). Release builds, own target dir.

**Verdict: APPROVE-WITH-FIXES** — MAJOR 0 · MINOR 3 · NOTE 4.
The frame is right, every predicate is decided, nothing else moved, the mutants are real and the oracle is exact. The fixes are tracker and prose only: a parked row goes lint-red when this row closes, a row is short a measured pose, and a comment is stale.

## Claims

1. **Frame right for every transverse pose — holds (sure).**
   - *Algebra.* I re-derived the one-loop turn rate, `y z′ − z y′ = −r(rd sin²θ + h² cos θ)/h`. With `h² = 2rd(cos θ − c₀)` it equals `rd((cos θ − c₀)² + 1 − c₀²)`, so the identity holds. For two loops the azimuthal share of the unit tangent is at least `r/(r+d)`, which is definite.
   - *Analytic sweep* (`review-probes/frame_sweep.py`): 4000 random poses, 5656 loops, a random rigid pose each time, some signed radii. The poses are skewed toward just short of the transition, just past it, `R≈r` and a centre near the wall. **0 loops** flip sense sign or turn other than 2π. The smallest normalised sense is 1.1e-7, just short of the node. There the decided `rotational_sense` escalates rather than picks.
   - *Matcher level* (`review_probe_battery`): 19 poses × {∪, ∩, A∖B, B∖A} × both orders = 114 runs. The poses: a centre 1e-6 inside and outside the wall; reach ±1e-3, ±1e-5, ±1e-7 about `R = r + d`; `R = r` at d = 0.05 and 1e-3; `R = r + 1e-3` at d = 1e-4; an off-centre ball; a centre far outside; five on a **tilted drum** (both operands rigidly turned). All 114 stop at `CurvedBooleanUnsupported(Cylinder|Sphere)`. Every segment joins neighbouring sites on one loop, and segments = records = sites. No pose fails.
2. **Predicates decided, escalating in band — holds (sure).**
   - Both comparisons are `decide` under named sites (`join.rs:1839`, `:1844`), on metre margins, registered in the audit doc with the right dimension. I found no raw float compare.
   - Node, coaxial and in-band offset keep `NoArm`, and an in-band reach escalates `bool_germ_frame_cs_reach` (that unit row is green; M3 and M4 turn it red).
   - A public run at reach 1e-7 never gets into the band. So the typed escalation is held only at surface level and by `offer_rows`, as the site's `Door` text says. I accept that.
3. **Nothing else moved — holds (sure).** I diffed main `f8aabaae` against the frozen head `e207aeb5` (not `aaea5ca3`), normalised:
   - `rc_wide_battery`: 40 321 lines, 0 differ, SOUND 34 389 on both trees, 0 BAD.
   - `join1_r1_battery`: 42 337 lines, 0 differ.
   - `join1_delta_arc_battery`: 7 351 lines, 0 differ.
   - My drum × ball grid (`review_cs_door_battery`): 1 152 runs, a radius-0.4 drum with balls across the wall, across the rim and through a cap. Exactly 936 lines move, all from `GermFrameUnsupported(Cylinder,Sphere)`: 792 to the lane door, and 144 to `Join(SectionNotPolar)`. That is the known door for a turned-chart ball crossing a cap, and main already stops 36 lines there. No BUILT line changes. The coaxial `d = 0` lines keep `GermFrameUnsupported` (the D10 hold).
4. **Mutants are real — holds (sure).** Each mutant was a separate build. Rows run: topo `transverse_cs_frame_rows` and `offer_rows`; sweep `cylinder_sphere_frame::*`, `a_rim_crossing…` and `verbs_cylsph_opening`; plus my probe battery. The unmutated head is green throughout.
   - M1 (one loop reads the cylinder axis): 6 red; 78 probe lines `JoinDesync` (radial germ).
   - M2 (two loops read the offset axis): 4 red; 30 probe lines `UnpairedLooseEnds`.
   - M3 (no transverse frame): 8 red; 114 probe lines.
   - **M4, mine** (the loop-count margin drops `d`, giving `|R| − |r|`): 5 red, killed by 3 unit rows, `offer_rows` and `verbs_cylsph_opening`, plus 36 of my probe lines. **Every `cylinder_sphere_frame` row stays green** (N1).
   - M5, mine (the one-loop axis turned to `a × û`): 6 red.
5. **Oracle exact — holds (sure).** `review-probes/oracle_x_slicing.py` slices along `x`, so each slice is a disc ∩ axis-aligned rectangle (the cut, linear in x, only lowers the top). It uses closed-form quadrant areas and adaptive G10/K21, all written from scratch.
   - Against the PR's `oracle`: |Δ| ≈ 2e-18 (one loop, uncut), 6e-16 (two loops), and **3e-18 and 7e-18 on the row's two cut poses**.
   - The cut poses matter most: the PR's own second slicing covers only the uncut drum.
6. **Filed rows — holds, with fixes (likely).**
   - The lane-door row names the right door: the `(a_s, b_s)` catch-all at `join.rs:713-727`. Its named lanes (`GermLane::*`, `RingClosure::Wall`, `cylinder_sphere_ssi`, `Pcurve::Fitted`) all exist.
   - The skew row is short a pose (m2).
   - The torus counts were not re-measured (unsure).

## Findings

**m1 MINOR — closing the frame row leaves a parked row on a closed blocker (sure).**
- `work/reach/nurbs-edge-crossing-rung-is-the-ring-composite.md:9` is `blocked_on: [cylinder-sphere-germ-pair-has-no-section-frame]`. Its reason (`:51-55`) is the join window that "will mint fitted NURBS seams", and that work moved to `cylinder-sphere-germ-pair-has-no-join-lane`.
- Once the orchestrator closes the frame row, `work/README.md:322-328` makes this a lint ERROR (every blocker closed). It should be re-parked on the lane row in this PR.
- The citations at `non-circle-conic-edge-…md:93` and `boolean-operands-with-nurbs-…md:110` become stale prose.
- Demonstrated by reading those files.

**m2 MINOR — the skew row leaves out a measured skew placement (likely).**
- `work/join/skew-cylinder-germ-pair-has-no-section-frame.md:28` names placements 2, 3 and 5 as the skew ones (6 lines).
- Placement 1 (90° about `x`, then `(0.3, 0.2, 0.1)`) gives perpendicular axes 0.3 apart. Run on head (`skew_placement_one`), it also refuses `GermFrameUnsupported{Cylinder,Cylinder}` in both orders.
- So the count and the "(skew axes)" gloss are incomplete. I could not re-run the PR's uncommitted probe.

**m3 MINOR — stale comment (sure).** `join.rs:1563-1567` still says "The DECLARED-coaxial configuration is the one this dispatch can name a frame for". The arms beneath it now name the transverse frame. The PR edited those two lines and kept the comment.

**N1 NOTE — the integration rows cannot see a wrong loop-count margin (sure).**
- M4 survives every `cylinder_sphere_frame` row, because all its one-loop poses have `R < r`.
- A pose with `r < R < r + d` (my "one loop just short 1e-3": `R = 0.699`, `d = 0.2`) would bind that margin when the lane lands.

**N2 NOTE — "every pose" stops at the lane door, but not all do (likely).**
- A ball whose turned chart crosses a cap passes the frame and the matcher, then stops at `Join(SectionNotPolar)` before the lane door (144 lines in my grid). That door is already filed.
- `cylinder-sphere-germ-pair-has-no-join-lane.md:25` could say the lane is not the only next door.

**N3 NOTE — the neighbour check is vacuous on two-site loops (sure).**
- Every two-loop pose, and the 2-site one-loop poses, have 2 sites per loop. There any intra-loop pairing passes `cylinder_sphere_frame.rs:391-403`, and only the cross-loop assert bites.
- The 4- and 6-site one-loop poses do carry it, and M1, M2 and M5 kill it.

**N4 NOTE — the PR's battery diff was on `aaea5ca3`.** Mine on `e207aeb5` is identical, so nothing is owed.

## Style (Q1–Q8 all exercised; Q8 = `join.rs:1340-1925` read end to end)

- **Q1 (likely)** — One tangency is decided twice, under two names and two radius conventions.
  - `bool_germ_frame_cs_reach` (`join.rs:1843-1846`, `|R| − |r| − d`) and geom-brep's `ssi_cs_tangency` (`ssi.rs:2134`, `d + r − R` among its terms, signed radii) decide the same walls-touching condition.
  - The offset `d` is computed three times with the same formula (`join.rs:1836-1838`, `ssi.rs:2133`, `intersect.rs:1721`). None of the three says it copies another.
  - The same class elsewhere: `bool_germ_frame_axes_coplanar` against `cc_axes_coplanar`.
- **Q1/Q7 (likely)** — The radius sign convention diverges along one path.
  - The new frame reads `|r|` and `|R|` (`join.rs:1810`).
  - The cylinder-pair arm levers on signed `r1.max(*r2)` (`join.rs:1601`, and in the audit row this PR adds).
  - `cylinder_sphere_section` refuses a non-positive radius as `DegenerateOperand`.
- **Q7 (likely)** — `cs_germ_frame` reads `Err(FrameError::NoArm)` as "routes to the general rung" (`join.rs:1775`). That is right only while `cs_pair_frame` maps nothing but `RoutesToGeneralRung` to `NoArm`, and nothing enforces it.
- **Q2/Q5 (likely)** — `NoArm`'s doc says "The kind pair has no section arm at all" (`join.rs:1452`). It now also means "an armed kind pair whose pose has no frame" (node, coaxial, in-band offset).
- **Q7 (unsure)** — At a tangency the plane×sphere, sphere×sphere and declared cs arms answer `Desync`, and the transverse node answers `NoArm`. That is defensible (the node is a crossing pose), but no comment says why.
- **Q2 (sure, trivial)** — `// (conic center, conic axis)` at `join.rs:1768` and `:1812` sits on quartic frames.
- **Q1 (unsure)** — The `h²` loop chart is written three times: the unit rows' `loops()`, `cylinder_sphere_frame.rs:341` `place()`, and my probe. It is test-only.
- **Q3 (sure)** — See N1 and N3. Every unit row can go red (M1–M5).
- **Q4 (sure)** — See m1 and m3. I swept the prose that cites "no frame" for cyl × sphere and found nothing else.
- **Q6 (sure)** — Every deviation is scheduled, by the lane, skew and torus rows.

REVIEW COMPLETE

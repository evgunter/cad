# Review of PR #4042, frozen head 395cdbef14

Lane `reach-full4042`, single full review. Wall clock 03:45–05:51 UTC, 2026-10-05 (cloud session). No glimpse disclosed: I read only the brief, the PR body (`get`), the unit item and the tree. I read no comments, reviews or other lanes.

**Verdict: APPROVE-WITH-FIXES.** 0 MAJOR / 1 MINOR / 4 NOTE (+6 style). I found no wrong answer, no newly refused body and no moved root anywhere a suite reaches.

## Claims
1. **One door, no wrong answers: holds (sure).** Probe `probes/door_fuzz.rs`, 2,000 seeded poses per ε at 1e-9, 1e-6 and 1e-12, run on head and on main (446031aa, old doors). It covers circle and ellipse × sphere and wall, rotated axes, r 1 mm–100 m, tilts at 0–100× both the tilt band and the A₂ reach `2√(rε)`, wall sections, and grazes in and out at ±{0.5, 2, 20, 1e3}ε. The oracle is my own: the true distance in double-double, 2e4 samples per turn, every extremum golden-refined, every sign change bisected on the closed turn. Result: **0 wrong** (count, miss through a crossing, `OnSurface` beyond ε, root off the surface beyond ε) on head and main alike. Verdicts change only on circle × wall poses with A₂ ≤ ε (75/63/84 per ε at 1e-9/1e-6/1e-12), never elsewhere.
2. **The arm's charge is sound: holds (sure).** Head's first-harmonic-arm roots are ≤ 0.69ε along the arc from the oracle's root (151/132/103 certified poses). `OnSurface` holds at max|d| ≤ 0.80ε. A band-edge sweep (`review_arm_edge`) puts A₂ at 0.5–0.999ε with crossings 10.5–1e3ε deep: 2,880 poses per ε, 0 wrong, arm roots ≤ 0.97ε. Every along-arc excess there was traced to the ladder (N2), none to the arm.
3. **Nothing reachable moved: holds on my sample (sure).** I instrumented the dispatch in `reduce::wall_crossing` on both trees (`probes/door_log.py`) and ran nextest `-p topo -p sweep` at 1e-9 (head 4,390 passed, main 4,389 passed). Each tree logged **6,479 door calls, with identical inputs and identical answers** once predicate names are normalized. I did not re-run the tour.
4. **The admitted losses are refusals only: holds (sure).** Over the same poses on main and head: certified → `Uncertain` 12/4/27, `Miss` → `Uncertain` 1/1/6. Every lost answer was correct on main; head never answers wrongly. At body level I found no pose main builds and head refuses. The near-coaxial poses where the arms differ need near-parallel cylinders, a cylinder × sphere pair or a cylinder × torus pair, and those refuse at the section pass or the germ frame on both trees. The suite diff in claim 3 bears this out. This was not a constructive search (likely).
5. **No row deleted without a successor: holds (sure).** Under the tilt mutant (`probes/tilt_mutant.py`), both new rows go red at all three ε: `the_second_harmonic_band_reaches_past_the_tilt_band` and `a_section_whose_projection_is_a_circle_is_a_first_harmonic`. Four other ellipse rows go red with them. By inspection, the moved rows keep their assertions.
6. **Rows and names are consistent: holds (sure).** I grepped the tree outside `work/` for every retired name and module (`bool_circle_cylinder_*`, `bool_circle_sphere_*`, non-torus `bool_ellipse_*`, `circle_cylinder::`, `circle_sphere::`, `ellipse_roots`): none left. The `offer_rows` site counts match `conic_quadric/mod.rs` (ArcSphereRoots ×2 at :184, :191; ArcCylinderRoots ×1 at :204). The audit rows, the `m6_rider` pin and the `nextest.toml` paths agree.

Clean head, touched rows (`conic_quadric::`, `ellipse_torus::`, `circle_roots`, `tang_circle_cylinder`, `verbs_cyl*`, graze, `m6_rider`, `reduce::`): 158/158 green at 1e-6 and 1e-12. The full topo+sweep run is green at 1e-9 (above). I did not check CI on GitHub.

## Findings
**MINOR-1. No row guards the first-harmonic arm's A₂ charge on a circle.** `crates/topo/src/boolean/conic_quadric/mod.rs:215`. Demonstrated by execution.
- `probes/drop_a2_circle_mutant.sh` drops `+ second` for circle carriers only. All 4,390 topo+sweep tests pass at 1e-9, and the 118 targeted rows pass at 1e-6 and 1e-12.
- Under that mutant, the fuzz ships `OnSurface` for a circle 1.62ε off its wall (pose 1385, r = 100 m, 1e-9).
- The all-carrier mutant (`drop_a2_mutant.sh`) reddens one row only: `ellipse_rows::the_first_harmonic_arm_places_its_roots_along_the_arc`.
- The PR makes this charge the circle's rule. DR-36 recorded it as unguarded with the mutant surviving, and this PR still leaves the circle side unguarded. No row puts a near-square circle within A₂ of the wall and asserts it is not `OnSurface`.

**NOTE-1. The arm's losses are not only shallow crossings.** Inspection, plus execution through `review_arm_edge`. The slack is `ρ·(noise + A₂)/√(−lo·hi)`. With A₂ = 0.5ε at ρ = 1 m, a crossing 1 cm deep each way already reads 50ε and refuses. At 1e-9 the edge sweep certified 0 of 2,880 band-edge poses, depths up to 1e3ε included. The filed item `conic-quadric-first-harmonic-arm-refuses-crossings-the-ladder-places` should carry that reach.

**NOTE-2. The degree-2 ladder certifies roots far along the carrier from the true root (pre-existing).** `circle_roots.rs:367` passes `None` for the slack meter. These roots are on the surface to ≤ 0.15ε, but well off along the arc:
- the fuzz finds them 1.65e-5 m (16,481ε) off at 1e-9 and 2.7e-4 m off at 1e-12;
- the edge sweep finds them up to 0.0195 m off at 1e-12 (an A₂ = 0.99ε pose at r = 100 m lands in the band's gap).

The counts are identical on main and on head. This is filed as `hone/degree-2-subdivision-doors-carry-no-root-slack-meter`, whose evidence so far covers the torus door only; these are measurements on this door.

**NOTE-3. An undisclosed gain.** Main's ladder placed roots up to 9.6e8ε along the arc on 4/0/16 circle poses with A₂ ≤ ε (1e-9/1e-6/1e-12). On head those poses go to the arm, which refuses them; the one left at 1e-9, pose 537, sits at A₂ = ε and takes the ladder on both trees.

**NOTE-4. The brief's call counts are the PR's own.** I re-measured on topo+sweep only: 6,479 calls, not 11,642.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q7, Q8 — whole of `ellipse_torus.rs` and `conic_quadric/mod.rs`)
- `ellipse_torus.rs:168,190,219` repeats `ellipse()`, `oracle()` and `assert_matches_oracle` from `conic_quadric/ellipse_rows.rs:22,55,84`. The repeats sit beside the `conic_oracle` home (`boolean/mod.rs:80`). Main had one copy, so the split made the second. This is the "fix mints a copy" shape. **sure**
- `circle_wall_rows.rs:90`, `ellipse_rows.rs:55` and `ellipse_torus.rs:190` are three sampled sign-change oracles. They are blind to a crossing pair inside one sample cell (no extremum refinement). My own first oracle had that blind spot and a wrap-point bug, and both produced phantom findings. **likely**
- `ellipse_torus.rs:90`: a second `ClassificationInvariant` for `Conic::of` failing, after the kind match already proved an ellipse. It looks unreachable, and it spells the desync differently from `conic_quadric/mod.rs:149`. **likely**
- `crates/sweep/tests/tang_circle_cylinder.rs:11,168,273` still say "square arm" in a file this PR edited. **sure**
- `conic_quadric/mod.rs:214`: the switch decides `Margin::of(second)` on rounded `c₂`, `s₂`, and the arm charges that same rounded `second`. I could not tell whether `rounding_charge(terms)` bounds the residual at each θ (which covers this) or each coefficient (which does not quite cover `δc₀ + δc₁ + δs₁ + δA₂`). The fuzz found no failure. **unsure**
- `conic_quadric/mod.rs:22–72`: the module docs justify the arm at length, including why it is REQUIRED. The ladder still answers when A₂ is in the gap, and the PR's own losses show the ladder places crossings the arm refuses. An arm chosen by "first harmonic within the noise" might better yield to the ladder whenever the ladder certifies. **unsure**

## Probe sources (`probes/`)
`door_fuzz.rs` holds `review_door_fuzz` and `review_arm_edge`. `install.sh head|main <tree>` installs it, swapping in the old doors on main. Env knobs: `PROBE_N`, `PROBE_S`, `PROBE_OUT`, and `PROBE_ONLY` for one pose's dump. The mutants are `tilt_mutant.py`, `drop_a2_mutant.sh` and `drop_a2_circle_mutant.sh`; the call logger is `door_log.py head|main <tree>` with `REVIEW_DOOR_LOG`. Run release builds with `CAD_TOLERANCE_EPS` set.

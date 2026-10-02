# PR #3805 second delta review — head e7f2798307 (fix pass dac5c02..e4db21f + merge e7f2798)

**Verdict: NOT-MERGEABLE-AS-IS.** The door is now sound against my fuzz, every guard has a row that reddens, and the #3801 merge is clean. But the same defect class (the harmonics' rounding uncharged → a certified "no event" on a definite crossing) lives in the conic CLEARANCE rung. That rung runs before the door, and this PR opens it to ellipses. The PR's graze fuzz calls the door directly and never reaches it.

## Findings

**MAJOR-1: `reduce::conic_clearance` certifies definite crossings clear** (execution). Locations: `crates/topo/src/boolean/reduce.rs:2150-2160`, reading `geom_brep::conic_residual_extremes` (`crates/geom-brep/src/implicit.rs:758-763`).
- **What happens:** `c₀ − A₁ − A₂` is decided bare, with no `rounding_charge(terms)`. At an ellipse's minor vertex the phases align, so that bound IS the true minimum, and its rounding (≈u·a²/r) passes 10ε. `Ok(Positive)` returns `CurvedEvent::None` (`reduce.rs:1636`) before `wall_crossing` is consulted.
- **Pinned:** `probes/reach_3805_fuzz.rs::reviewer_pinned_clearance_on_a_crossing`, RED. The pose is an ordinary ellipse with major 4.85 m and minor 0.21 m (positive and ordered), centred 1 m from the origin, and a 24.7 µm ball at ε 1e-12. The 60-digit oracle (`probes/reach_3805_oracle.py`) puts the least true distance at **−2.71e-11 m**, 27 bands deep. The extremes read lo = +2.9e-11, so the rung certifies clear. The door says `Uncertain`, which is right, but it is never reached.
- **Fuzz** (`reviewer_fuzz_all_kinds`, 3000 poses per cell, all 8 sign/order combinations, eccentricity ≤ 40, gap ±40ε at a random vertex, closed-form oracle re-checked by dense sampling). Clear-on-crossing counts:
  - mixed m/km: ε 1e-12 sphere 114, wall 28; ε 1e-9 sphere 68, wall 9; ε 1e-6 sphere 4, wall 0;
  - metre scale only: ε 1e-12 sphere 22, wall 3; 0 at 1e-9 and 1e-6.
- **Torus and cone:** never certify anything at this rung. The torus's sampled chord-dip bound is loose, and a cone has no enclosure (`None`, so the frontier).
- **Pre-existing for circles:** the same probe with circles on main's circle × sphere arm gives 61 (m, ε 1e-12), and 375/234/28 at km. That half wants filing. The ellipse half is this PR's new exposure, and it is the subject of the prior MAJOR ("audit every read"; Q4 "sweep the sibling").

**MINOR-1: the ordered/signed sweep stopped at the diff** (inspection). The audit table names `implicit.rs`, `replace_face.rs` and `chord_join`. It misses these readers:
- `crates/editor-core/src/eval/measure.rs:518` `curve_reach`: `from(center) + major`. This is `pose_reach`'s exact under-reach, and its doc says an under-estimate "would certify a parallelism that does not hold".
- `crates/geom-brep/src/certify.rs:2219` and `crates/geom-brep/src/pcurve_cache.rs:3305`: `InfSpeed::new(minor)` as the speed floor.
- `certify.rs:1904` `edge_extent`.

`work/restfront/an-ellipse-stored-minor-over-major-passes-tier-3.md` already files `edge_extent` and says "a consumer sweep … is owed either way". Neither the reach sibling nor the speed floors are recorded there.

**MINOR-2: `a_ball_straddling_a_notched_walls_carrier_builds` cannot see the ball at ε 1e-6** (inspection). `slack()` = `max(1e3·ε, 1e-9)` is 1e-3 there, but the ball's volume is 5.2e-4. At that ε, ∪ dropping the ball, and B∖A answering any volume up to 1.5e-3, both pass. The row is fine at 1e-9 and 1e-12.

**NOTE-1: the rest of the claims hold, by execution.**
- **Door:** 0 certified `Miss` on a crossing, 0 in band, 0 roots off the band, and 0 short counts, over 24 000 sphere/wall cells at the three ε.
- **Guard mutants**, re-applied by me:
  - `fourth_hi = 0` reddens `a_fourth_order_graze_is_not_read_clear` and `graze_rows::no_certified_miss_on_a_graze`;
  - `d1_lo` uncharged, and the whole monotone test uncharged, each redden `a_slope_inside_its_noise_is_not_monotone`;
  - the ON check dropped reddens `a_root_that_reads_off_the_surface_is_not_certified`;
  - the odd-count check dropped reddens `an_odd_count_is_not_certified`;
  - `- noise` dropped from the clear test reddens 2 `graze_rows` and `circle_torus::no_wrong_certified_answer_across_circle_radii`.
- **Bernstein:** `E = F̂ − F` is a degree-2 trigonometric polynomial, so `‖E⁽ᵏ⁾‖ ≤ 2ᵏ‖E‖`. The premise is `noise ≥ ‖E‖∞`. Measured against 60-digit harmonics (`probes/reach_3805_noise_ratio.py`, 4000 poses), the largest `sup|E|/noise` is 0.20, so the premise holds with 5× room. `fourth_hi`'s `16·noise` and the 2/4/8 charges are right.
- **`speed_at`** (`reduce.rs:2313`) now refuses typed. The arm is unreachable (only line/circle/ellipse get there), so it has no row; by inspection.
- **`pose_reach`:** fixed, with a row that is red on the old read.

**NOTE-2: #3801 reconciliation.**
- `git diff origin/main -- crates/topo/src/boolean/ops.rs` is empty.
- The merge dropped three rows: `a_contained_ball_builds_through_the_extent_scan`, `a_ball_in_the_walls_box_corner_is_certified_by_its_carrier` and the straddle refusal. Main's `a_contained_ball_builds_through_the_section_pass` and `a_ball_in_the_wall_boxs_corner_is_certified_separated` cover them. No other row or guard this branch added is missing (80 added fns checked).
- The straddle row's closed form is right: the ball centre is 0.247 from both flat faces and clear of the ¾ disc, so the bodies are disjoint.
- The ellipse-rim `conic_edge_curved_face` rows pass at all three ε.

**Suites** (`-p geom-brep -p topo -p sweep -p editor-core`, run locally at this head):
- ε 1e-9: 7358/7358 pass;
- ε 1e-6 7358/7358;
- ε 1e-12 7357/7358 (only the known `rigid_map_near_eps_plane_nurbs`).

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q7, Q8 on `circle_roots.rs`)

- **Q7/Q2:** `ladder_roots` (`circle_roots.rs:298-430`) still reconstructs roots and decides `noise` and `root_slack`, and `half_angle_roots` throws all of it away. Its `noise` meter's `Ok(Positive) → Uncertain` therefore no longer refuses anything. `HalfAngleRows::noise` and `root_slack` (`:190-196`) still describe deciding rows. *sure*
- **Q4:** this fix pass charges `terms` in the door. The enclosures beside it (`conic_residual_extremes`, `conic_arc_residual_range`'s harmonic `f2`) never read `terms` at all, which is MAJOR-1's shape. *sure*
- **Q5:** `SPLITS` docs (`:448-456`) call the stored share irrational. Every f64 is a dyadic rational, so the property belongs to `2√5 − 4`, not to the constant. *likely*
- **Q1:** `distance()` and `unit()` are written three times in `ellipse_roots.rs` (`tests`, `fuzz_rows`, `graze_rows`), and `sign_changes` repeats `oracle`. *sure*
- **Q3:** `subdivision_guard_rows` reddens its guards with residuals outside the contract the guards rest on: a non-trig wiggle, and a disagreement larger than the noise. That is honest, and the module doc says so, but the rows cannot tell a contract-respecting regression apart. *likely*
- **Q5:** that module doc says "no physical pose … puts the harmonics' actual rounding … near the band". My fuzz has the door's noise past ε 1e-12 at metre scale against µm balls, where it answers `Uncertain`. *unsure* whether "near" means this.
- **Q5:** `geom::Curve3::Ellipse` says "`major > minor` by the constructor's refusal" (`crates/geom/src/curves.rs:155`). `Conic`'s doc says the mint certifies both orders. Each is true of a different door. *likely*

Probes: `probes/reach_3805_fuzz.rs` (mount instructions in its header), `reach_3805_oracle.py`, `reach_3805_noise_ratio.py`.

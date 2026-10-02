# Review of PR #3805, frozen head 9b6fe0843d

Lane `reach-dual3805-r1`. Start 05:59 UTC, end 07:31 UTC, 2026-10-02. **Glimpse: none.** I read the PR body with `get` only, plus the check runs and job logs of head run 36968237415. I read no comments or reviews and no other `analysis/` branch.

**Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 2 · MINOR 2 · NOTE 5.

## MAJOR
**M1. The head does not compile on main.** `crates/sweep/tests/conic_edge_curved_face.rs:39` passes `Vec3` where main's `SplitPlane.normal` is now `UnitVec3` (main a4997ec3/67ff4791, after the branch's last main merge). DEMONSTRATED by a trial merge of the head into origin/main (`cargo check -p sweep --tests` gives E0308) and by CI on this head: run 36968237415 has `test`, `lint` and `gate ok` red, with the same E0308 in the lint log. The PR body's "7270 passed after merging main" is stale. The fix is mechanical. The merge is otherwise clean and reverts nothing of #3752 (see N1). Confidence: sure.

**M2. The ellipse ladder's lever `2b` certifies roots off the surface by up to 15× the band.** At `crates/topo/src/boolean/ellipse_roots.rs:165`, `lever: two * conic.minor` meters the quartic's decisions in τ-lengths scaled by `b`. On an eccentric ellipse an arc length can be up to `a/b` times longer than its τ-length, so a root the band certifies in τ can lie farther off in arc length.
- DEMONSTRATED by execution with a seeded fuzz of 6000 ellipse × sphere/wall arcs (`probes/reach-dual-3805-r1/ellipse_roots_fuzz.rs`, `fuzz_random_ellipses`). The oracle bisects the TRUE distance on 40k steps.
- At ε 1e-6, case 4815 returns `Certified{4}`. It is a = 3.757 mm, b = 0.188 mm (a/b = 20), against a wall of r = 0.58 mm.
- Its roots are 1.9e-5, 1.6e-5, 9.6e-6 and 8.1e-6 m of arc from the true roots. The certified points sit 1.5e-5 m off the wall: 15× the 1e-6 zero band, and past the escalation threshold.
- The mutant `lever: two * conic.major` gives 0 roots past the band (head: 4), at the price of `Uncertain` rising from 27 to 1816 of 6000.
- At ε 1e-9 and 1e-12 the worst arc error was 8.1e-10 m, inside the band.
- `wall_crossing` uses these θ as landing points (`reduce.rs:2097`). Body-level reach is not demonstrated: it would need a mm-scale steep cut at ε 1e-6.

So claim 3's "is each choice the sound direction?" is **no** for the lever. The `2a` lever the PR body says it rejected was the sound one; the cost it found was refusals, not wrong answers. Confidence: sure (door level).

## MINOR
**m1. A tangency reads as a certified `Miss` in the shared half-angle ladder.** The test is an ellipse that touches a wall at θ = ±π/2 and lies outside it elsewhere (min distance exactly 0); see `tangency_sweep_ellipse_and_circle` and `first_harmonic_arm_against_bisection`.
- At ε 1e-9 the ellipse door answers `Miss` at r = 50 m (δ/r = 1e-9), r = 5000 m (δ/r = 1e-11), and r = 500 m with δ = 1e-7.
- **The same defect is in #3752's circle × cylinder door, already on main.** A circle of the wall's radius, tilted inside it so it touches at two points, answers `Miss` at r = 5 m with depth 5e-6 (ε 1e-9), and at r = 500 m.
- The circle-roots docs say a tangency is `Uncertain`. Because the defect predates this PR, it is MINOR here, but the PR routes a new carrier into it. **This owes a filing against the shared ladder** (`circle_roots::half_angle_roots` / `depressed_quartic_roots`). Body consequence not demonstrated. DEMONSTRATED at door level. Confidence: likely.

**m2. The at-end decision uses the speed's lower bound.** At `crates/topo/src/boolean/reduce.rs:2033` (read at `:2076`) the ellipse's `metres_per_param = minor` feeds `bool_wall_root_in_span`. `Sign::Zero` there means *at the vertex* (`at_end`), so a root up to `(a/b)·zero` of arc from a vertex reads as AT it, and the crossing is handed to the vertex lane. It is the M2 direction question again: a lower bound is the sound choice only for a "definitely interior" reading. The local speed `|C′(t_end)|` is exact and cheap. By inspection. Confidence: likely (the consequence is unsure).

## NOTE
- **N1. Claim 7 holds.** The 3-way trial merge keeps all of #3752, including fdc49f05's constant-spread charge in `first_harmonic_roots`. After the merge the ellipse arm's `noise + A₂` reaches `constant_residual_roots`. On the frozen head (pre-fdc49f05) the constant branch charges no spread at all, so that is fixed only by main. `topo` compiles after the merge.
- **N2. Claim 2 holds.** The `conic_arc_residual_range` / `conic_residual_extremes` fuzz ran on 2977 finite arcs: eccentricity 1 to 40, spans 1e-3 to 2π, arcs at the vertices, sphere, tilted wall and torus, 20k dense samples each, at ε 1e-9, 1e-6 and 1e-12. Worst excess was 0. The harmonics' algebra and the torus `|C′|, |C″| ≤ a`, `a_h` bound check by hand. Claim 1 also checks out by inspection.
- **N3. Claims 4 and 6 hold.**
  - Placement (`p5`) covered rotated tilted cuts, double cuts whose sections cross the seams, and points 1e-3/1e-6 off each section: 10088 points at ε 1e-9, 0 wrong, 0 undecided; at ε 1e-6, 0 wrong and 780 undecided.
  - Held-inside bodies were checked against my closed forms at ×1e-3, ×1 and ×1e3, rigidly re-posed, at tilts 0.6 and 0.9, in both operand orders, for ∪, ∩, A∖B and B∖A, all tiers. All matched within `volume_pad`, with the expected shell counts (A∖B has 2).
  - The PR's six table rows reproduce from π/8, 4/3·πr³ and πr²h, and its pose premises check.
  - Near-rim crossings never shipped disjoint: they refused at GermFrame, at CurvedPierce for depth 1e-7, or at the extent scan.
- **N4. Claim 5 is sound by inspection.** A face is a subset of its carrier, so a sphere definitely inside or clear of the whole carrier cannot meet any face on it. In execution, though, the balls apart from the drum (beside the rim, above the cut, over or under the drum) all refuse `Containment(VolumeUncertified)` downstream, as do 426 of 600 `point_in_solid` samples on A∖B (0 of 600 were wrong). Those refusals come from the CONTACT door, not this PR. Results reused as operands, such as (A∖B) ∪ B, refuse `CurvedPierceUnsupported` on the void sphere's edges. At ε 1e-12, the ×1e3 ball's `mass_properties` refuses `QuadratureBudget`; the boolean itself built.
- **N5. Suites at the head** (`geom-brep`, `topo`, `sweep`):
  - ε 1e-9: 4751/4751 passed.
  - ε 1e-6: all passed except `every_suite_file_is_aggregated`, an artefact of my probe file landing mid-run; green on rerun.
  - ε 1e-12: only the known `rigid_map_near_eps_plane_nurbs` row failed.
  - These are local runs. CI on the head is red (M1).

## Style (questions exercised: Q1, Q2, Q3, Q4, Q6, Q7; Q5 partly; Q8 not done, `implicit.rs` was not read end to end)
- **Q1.** The ellipse root arm copies the circle arm's 15-line `CircleRoots → SpanVerdict` match: `reduce.rs:2007-2024` against `2041-2058`, including the `CountDisagrees` invariant text. It is a fresh duplicate in a PR whose own premise is "one home". Confidence: sure.
- **Q1/Q7.** `bool_circle_curved_clearance` (`reduce.rs:1865`) now meters ellipses. The row name misstates its scope, and the five corpus and tests citations read as circle-only. Confidence: likely.
- **Q2.** The `Conic` frame precondition is "unchecked" (`implicit.rs`, `Conic` docs). A non-unit `u_ref` silently breaks every enclosure, and nothing asserts it on a frame built by `Conic::of`. Confidence: likely.
- **Q6.** The `ellipse_roots.rs` module doc says "a minted ellipse holds [A₂] definitely positive at any ordinary scale". That is a measured claim with no guard, and m1 shows the arm-selection boundary is exactly where wrong answers live. Confidence: unsure.
- **Q3.** Every built body in `conic_edge_curved_face.rs` is held-inside, and every crossing stops at a door. No certified ellipse root ever reaches a built body, so the root lane's placement is tested only at door level, where M2 lives. The `crossings_match_the_true_distance` row is unit-scale, eccentricity ≤ 3.3, ε-default, so it cannot see M2. Confidence: sure.
- **Q4.** The PR body's "local runs green after merging main" was a premise CI depended on, and main moved under it (M1). Confidence: sure.
- **Q7.** The `speed_lo/speed_hi` bracket is a hand-chosen bound per use site. A type that hands out only direction-safe conversions would have caught M2 and m2. Confidence: unsure.

## Probes (`probes/reach-dual-3805-r1/`)
- `ellipse_roots_fuzz.rs`: appended locally to `ellipse_roots.rs`. Fuzz, first-harmonic arm, tangency, and circle door.
- `geom_brep_r1_probe_3805.rs`: `geom-brep/tests`, the enclosure fuzz.
- `sweep_r1_probe_3805.rs`: `sweep/tests`, rows p1–p5.

Both test files are wired into the crates' `tests/all.rs` by `#[path]`.

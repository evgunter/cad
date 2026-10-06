# Review of PR #4135, frozen head 17c7349bb5

Lane `reach-dual4135-r2`. **Verdict: APPROVE-WITH-FIXES.** MAJOR 1 · MINOR 3 · NOTE 5. Wall clock 2026-10-06 11:25Z – 14:40Z.

**Read:** the brief's list, the PR body (`get` only), and no comments, reviews or `analysis/reach-dual/*` branch. **Glimpses: none.**

**Ran:** probes `probes/` against my own oracle, which reads the residual `ρ cos α − |h| sin α` from each point, refines extrema by golden section, bisects, and guards its own f64 resolution. Also cross-tree differentials (merge base 94512aac vs head), 11 mutants, and the suites.

**No wrong answer anywhere.**

| Probe | Size | Result |
|---|---|---|
| Conic door | 6,060 poses per ε × 3 ε: α 0.002–1.568, scales 1e-3/1/1e3, (near-)coaxial, tilt 0–1e-3, ±0.3–1e4 bands, apex approach δ 1e-14…1e-2·scale | 0 wrong; worst root 0.017 ε along the arc |
| Line door | 12,450 per ε: random, near-generator, through-axis, apex | 0 wrong; worst in-span root 0.07 ε |
| End to end | ~1,050 sweeps over 3 ε: widening, narrowing, full, narrow-angle and wide-angle cones, posed and rotated, × cubes, bricks, rods, both orders; plus apex passes and generator-parallel edges | 0 wrong vertices; worst 0.045 ε |

## MAJOR

**M-1 (test-gap, DEMONSTRATED: mutant M9 plus the line probe).** The line root-slack rung `bool_line_cone_root_slack` (`crates/topo/src/boolean/reduce.rs:3525`) is load-bearing, and no committed row guards it.
- **Measured:** with the rung removed, every committed row stays green (`cone_rows`, `line_cone_rows`, `reach_cone_root_lane`, all of `conic_quadric`). My probe then finds 170 in-span roots misplaced by 3e-9…2e-6 m (3–2,000 ε) at ε 1e-9: α 0.01, scale 1e3, steep lines 4 mm–0.5 m from the apex. The oracle resolves ~1e-11 m there.
- **Why the rows miss it:** they test a root's distance from the cone or its quadric form, never its place along the edge. See `cone_rows.rs:338,446` (`off <= eps` with `count >= changes`) and `reduce.rs:6083` (`|form| < 1e-14` at scale 1). On a shallow crossing a misplaced root still lies on the cone (Q3: monotone in the wrong direction).
- **Same class, also survives every row:**
  - M7, the conic slack meter `mod.rs:377` (my oracle cannot resolve the 176 roots it lets through, so no wrong root is shown);
  - M6, the conic apex rung `mod.rs:387`;
  - M11, the depth rung's `|A|·R` scaling `reduce.rs:3504`;
  - M5, the far-nappe tell-off (already filed).
- The code is right; the guard is missing. A row that places roots along the edge against an oracle, near the apex at scale 1e3, would close it.

## MINOR

**m-1 (liveness, DEMONSTRATED by probe).** The depth rung divides by `R = max(|q(t0)|, |q(t1)|, |q(t*)|)` (`reduce.rs:3504`). The bound `|res(t*)| ≥ |Q(t*)|/|q(t*)|` holds pointwise, so `R` only loosens it.
- Near the apex `Q(t*) ~ δ²`, so a clean transversal pair refuses inside `δ ≈ √(2εR)`.
- **Measured** (steep line, α π/4, scale 1): nearest answered δ is 2.5e-4 m at ε 1e-9 and 7.9e-3 m at ε 1e-6.
- At ε 1e-6, 21 of 240 random end-to-end sweeps escalate `bool_line_cone_depth`.

**m-2 (liveness, DEMONSTRATED end to end).** The lead rung is levered by the segment's length (`reduce.rs:3493`), so a short edge reads as "parallel to a generator" far from one.
- **Measured:** at scale 1e-3 and ε 1e-6, 13 of 240 random sweeps escalate `bool_line_cone_lead` (margin 6.36e-6 = `A·L`, i.e. `A` ≈ 0.06 on a 0.1 mm edge, roughly ±15° of direction).
- `quadratic_roots` is stable at small `A`. The filing `a-line-parallel-to-a-cone-generator-…` describes a degeneracy and not this lever.

**m-3 (test-gap, DEMONSTRATED by mutants).** The differential `the_cone_arm_changes_no_sphere_or_wall_answer` (`cone_rows.rs:554`) is blind to what this PR refactored.
- **M3b** (`cos_part`/`sin_part` swapped in the extracted `first_harmonic_arm`, `mod.rs:279`) leaves it green: its sample essentially never takes that arm.
- **M3a** (a swap inside `quadric_harmonics`) leaves it green too, because its frozen "before" body calls the same helpers.
- Other rows catch both. My cross-tree differential confirms the claim anyway: 9,000 sphere/wall door answers (three ε, three scales, some poses 1e4 m off the origin) are byte-identical, base vs head.

## NOTE

- **n-1 (claim 3).** The tell-off cannot decide where the trim answers: `point_on_cone_in_face` already refuses the far nappe (`solid_contain.rs:2959-2963`). M5 survives everything, as filed. It decides only on faces the trim declines (rings, `PartialConeFace`), and there it turns a frontier into `Elsewhere`. I could not build one through public doors.
- **n-2 (claim 4).** No answer near the apex is wrong, but the refusal zone is wide and grows with scale. Conics: nearest answer at 8e-5…5e-3·scale, some poses none up to 5e-3·scale. Lines: 4e-6 m at scale 1e-3, 2.5e-4 m at 1, 8–60 m at 1e3 (ε 1e-9). At ε 1e-12, 14% of clear random conics refuse. The conic `CrossingAtConeApex` is never produced: the slack meter refuses first (M6).
- **n-3 (inspection).** `face_nappe(..).ok()` (`reduce.rs:3237`) folds an escalated nappe reading into "no nappe". `face_nappe` panics on a torn body (`offset_nappe.rs:75-78`) and is now reached from the crossing layer.
- **n-4 (claims 2, 8).** 6,750 `point_in_solid` answers on cone bodies are byte-identical base vs head: widening, narrowing and full cones, a half revolve, a ring, posed, three scales. `sweep-testing` has no normal-edge activation (`cargo tree -e features -i topo`). The gate row still pins `CurvedPairUnsupported{Cone}`.
- **n-5 (claims 6, 7).**
  - Only the `(−,−)` and `(+,+)` line arms read convexity, and the cone now takes the roots arm before both. The deferred touch is wall/sphere only, and `tangency_certifies_side` has no cone row, so I found no other cone-admitting convexity shortcut.
  - Mutants M1 (convexity restored), M2 (no floor re-read) and M4 (no cone route) turn red exactly the rows the PR names. M8 (floor = 1) turns red the narrow-cone row.
  - The row oracles are independent of the door, though they share `conic_oracle::crossings`.

**Suites** (geom-brep, topo, sweep):
- ε 1e-9: 5,577/5,577 green.
- ε 1e-6: only `pinch_faces_tessellate::…` and `pocket_ring_steep_ellipse::…` fail, both known red on main.
- ε 1e-12: 5,577/5,577 green (frozen head, run alone after a first 1e-12 pass was cut off at its time limit). The 1e-9 and 1e-6 runs had the probe modules compiled in but excluded.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (read `conic_quadric/mod.rs` whole; `reduce.rs` at 6,153 lines only in the touched regions).

- **S-1 (Q1, likely).** `line_cone_roots` (`reduce.rs:3468`) lives in `reduce.rs`, while its siblings `line_wall_roots`, `line_sphere_roots` and `line_torus_roots`, and its own quadratic, live in `solid_contain.rs`. One door's root code now has two homes.
- **S-2 (Q1, likely).** The cone's form is spelled three ways. `line_cone_quadratic` returns `−Q` (`solid_contain.rs:4152`). `conic_cone_harmonics` returns `+Q` (`implicit.rs:1137`). `conic_cone_residual` re-spells `cone_elevation(None)` with a running bound (`implicit.rs:1206`).
- **S-3 (Q1, sure).** The pad literal `1 + 64·UNIT_ROUNDOFF` is written twice, undisclosed (`implicit.rs:1175` and `1372`, the torus).
- **S-4 (Q5, likely).** The module docs say "floor … is therefore 1, an identity" and "`A₂` … in metres" (`conic_quadric/mod.rs:15,25`). On a cone both are in `F` units, which is only stated lower down.
- **S-5 (Q3, sure).** `count >= changes` and `off <= eps` are the shape that hid M-1 (`cone_rows.rs:338,446`). `a_root_at_the_apex_refuses` accepts `Uncertain`, so it cannot go red when its rung goes (M6).
- **S-6 (Q7, unsure).** `ConicHarmonics::floor` and `per` are fields only the cone varies (`implicit.rs:1050-1058`). Two kind-specific knobs ride the shared struct.
- **S-7 (Q6, likely).** The 1e-12 meter on roots outside the span (`hone/root-slack-meters-roots-outside-the-arc`) is scheduled. The m-1 and m-2 rung levers are not.

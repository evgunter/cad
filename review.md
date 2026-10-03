# Review of PR #3978, frozen head 65e53562cf

Lane `reach-dual3978-r2`. Wall clock: start 2026-10-03 17:10 UTC, end 18:20 UTC. **Glimpse: none.** I read only my own brief, the PR body (`get`), and the files the brief names. I read no comment, review, or other `analysis/reach-dual/*` branch. CI run 37136931115 is `success` on head `65e53562c`; I checked the run's status and SHA myself.

**Verdict: APPROVE-WITH-FIXES.** Counts: MAJOR 0 · MINOR 2 · NOTE 4. No wrong body was shipped in any probe, under any ε.

## Claims
1. **Holds (sure, by execution).** A touch is dropped only when its point places `Out` of a face AND no edge box of that face reaches the touch ball. I tested three ladders (plane, sphere × cylinder, skew walls) and the lens-rim sphere ladder, with the touch `δ` past the face edge (1e-1 down to 0) and `δ` inside the face (−1e-1 down to −1e-9), at ×1e-3, ×1 and ×1e3. Every on-face and edge pose refused, all 6 ops each, with a typed error. Every pose that built has a true gap above ε (gap ≈ δ²/2 or δ², ≥ 5e-8 at ε 1e-9). **Premise correction:** the brief says "outside BOTH faces". The rule is "Out of ONE face", and that is the sound rule: if the section misses one face, the faces cannot meet.
2. **Holds where it builds (sure, by execution).** Each built body passes tiers 1–3 and a closed mesh. Its volume matches the closed form to 1e-8 relative. 120 Monte Carlo `point_in_solid` samples agree with analytic signed distances (0 disagreements). This held for five new poses with other radii (snowman(1, 0.7, 1.2) ⊃ ball(.25), snowman ∪ brick on x = 1, lens(1, .9, 1.3) with ball(.35), ball with rod(.3), skew walls .4/.25 at 60°), plus a notched rod with a ball touching the carrier from INSIDE, reused results ((snowman ∪ brick) ⊃ ball, cavity body), every op, both orders. The exercised arms are sphere × sphere in/out, sphere × plane, sphere × cylinder in/out, and skew walls. Main's kernel refuses all of these except the half-rod pose (I reverted the five kernel files to `623004b99`), so the probes do exercise the change. I also checked the four spread formulas against my own mpmath section oracle (4000 random poses, true μ ≤ zero). The worst ratio of section radius to spread is 0.316 = 1/√10, so the bounds are sound.
3. **Holds (sure).** Every kept refusal is a typed `BooleanError`: `AgainstPlane`, `Apart`, `SpheresMeet`, `FallbackExtentUnsupported` R-tan, or `CurvedPierceUnsupported` from the crossing layer when the edge is near.
4. **Partly falsified (sure, by mutants).** The PR's rows go red without the fix: on main's kernel the 5 building rows, `x4` and my `results_reused…` are red. Mutant results:
   - M1, drop the boundary test: only the unit row catches it.
   - M2, drop `Out`: caught.
   - M6, shrink `zero_bound`: caught.
   - M7/M8, never refuse the scan's nested/plane touch: caught.
   - **M4, drop `(1+ρc/e)`, and M5, drop `/sin`: both SURVIVE every PR row.** See MINOR-1.

## Findings
- **MINOR-1** `crates/topo/src/boolean/section_cert_rows.rs:633`, `section_cert.rs:398`. Demonstrated by execution. `a_touch_holds…` pushes poses by 0.9·zero, while every spread is sized with `escalate` = 10·zero. That leaves √10 of slack in the radius, which hides any per-arm factor under ~3.16. The row's poses have `1+ρc/e` = 4.3 and `1/sin` ≤ 1.14. M4 and M5 are unsound past the slack, at a ball deep in a wall (`1+ρc/e` = 51) or walls 5.7° apart. My row `review_probe_spread_factors_past_the_slack` (`probes/section_cert_rows_probe.patch`) is green on the PR and red under M4 and M5. The row cannot go red when the guarantee degrades (Q3).
- **MINOR-2** `crates/topo/src/boolean/ops.rs:1198-1220`. Demonstrated by execution. Whether a touch clears depends on the frame and the scale, because the test compares the ball's AABB with each edge's AABB. Under one rigid tilt, snowman ⊃ ball(.25) refuses `SpheresMeet` and snowman with brick refuses `AgainstPlane`, at all three scales, while the identity frame builds them. A tilted trim circle's box covers the whole cap. The 60° skew pair refuses R-tan at ×1e-3 in the identity frame and builds when tilted. No wrong body results, but the PR body says the rule "covers" these pairs "in both operand orders and under every op". In practice that holds only for axis-friendly poses, and the gap is neither disclosed nor filed (Q6).
- **NOTE-1** `section_cert.rs:1042`. Inspection, plus the mpmath check. The derivation measures `|q − p|² = 2r₁(r₁ − x)` from `p = c₁ + r₁k`. When F is the SMALLER sphere nested in G, the touch is at `c₁ − r₁k`, so the derivation is wrong for that case. The bound still holds (ratio ≤ 0.316) because the ball is centred on the circle's centre.
- **NOTE-2.** Dropping the boundary test (M1) turns no body-level row red, mine included: the crossing layer refuses first near an edge. The "face holed within the ball" case has only the unit row, which uses fabricated placements.
- **NOTE-3.** Runs on `-p topo -p sweep`, everything, with my probes in:
  - ε 1e-9: 4240/4240.
  - ε 1e-6: one red, `review_cleave_wrongarc::steep_cuts_of_tubes_chord_inside_each_bore_face`. It is also red on main's kernel; I checked.
  - ε 1e-12: one red is `rigid_map_near_eps_plane_nurbs`, known on main. The other three reds are my own probes at ×1e3, where the FIXTURES (snowman/lens union `CurvedPierceUnsupported`, `transform_rigid` pcurve escalation) refuse before the boolean under review runs. Every pose that built at 1e-12 matched the oracle.
- **NOTE-4.** My first inside-carrier probe (`half-rod`) is vacuous. The face boxes do not overlap, and it builds on main too. `ball_in_a_notched_rod…` replaces it.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 partly)
- Q1 `ops.rs:2929,3073,3123`, likely. One `sphere_faces_apart((x_is, x, x_rows), fd.surface, (y, y_row), band, &mut section_charts)?` block appears three times, two with `.is_none() → continue` and one with `.is_some() → refuse`. That is a near-copy whose polarity flips. The class: look at every `sphere_faces_apart` caller.
- Q1/Q7 `section_cert.rs:383,945,1017`, likely. `Pinch::touch` selects arms by `names.contains(&self.name)`. Each margin-name literal is written twice, once at `signs` and once at `.touch(&[..])`. A typo would silently turn a touch back into R-tan, and nothing would go red.
- Q2 `section_cert.rs:940-953`, unsure. The comment derives `/sin + μ`, but the code adds `+ mu + mu`. The second μ, the offset of the centre `δr₁/(r₁+r₂)` from the ruling, is not argued in the comment.
- Q5 `section_cert.rs:280`, likely. `Touch::at` is documented as "the touch point", but for sphere × sphere it is the circle's centre, and for skew walls it is off both carriers by up to μ.
- Q6, sure. The frame-dependence of the AABB test (MINOR-2) is an unscheduled coverage narrowing. Nothing in `work/` names it.
- Q4, unsure. The `SphereQuestion::Apart` doc (`refusal_routes.rs`) now says an in-band gap "passes where the section certificate certifies the faces apart". An UNDECIDED margin goes to the certificate too, and it always refuses there. The text reads wider than the behaviour.
- Q8: I read `section_cert.rs` docs and touch arms whole. I read `ops.rs` (4916 lines) only around the diff, not end to end.

## Probes (`probes/`)
- `review_3978_r2_probes.rs`: the sweep suite. Wire it in with `sweep_all_rs.patch`.
- `section_cert_rows_probe.patch`: the spread-factor row.
- `spread_bounds_oracle.py`: the mpmath bounds check.
- `mutant_runner.sh`: the M1–M8 driver. The `perl` substitutions used are listed in this report.

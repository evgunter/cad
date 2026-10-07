# Verification of PR #4135 at frozen head `6423635c32b2`

Lane `reach-verify/4135`. The brief was `briefs/verify-4135.md` on `analysis/reach-briefs/2026-10-04`. I read `CLAUDE.md`, the implementer discipline, both dual reviews (`analysis/reach-dual/4135-r1`, `-r2`) and the PR body (`get` only). No code changed on the PR branch, nothing merged, nothing posted.

**Verdict: VERIFIED.** I found no wrong certified cone root, no missed crossing, no wrong split and no untyped refusal anywhere I could reach. Every mutant the fix pass claims is red, red. The notes below are not blocking.

All probes are scratch only: `#[path]` modules mounted in `reduce.rs` and `sweep/tests/all.rs`, removed before the suites ran. My oracle reads `ρ cos α − |h| sin α` (or, end to end, also each cap's height) from each point. Its sampler is my own (dense sampling, golden section at every sampled |f| minimum, then bisection), and it does not use `conic_oracle`. Placement is judged along the edge or arc. A root is skipped as unresolved only when the oracle's own f64 resolution, `16u·|coords| / |slope|`, exceeds ε.

## 1. Mutants (hand-applied on the head, run, reverted; tree clean after each)

| Mutant | Rows red | Claim |
|---|---|---|
| M9, line root-slack rung off | `line_cone_rows::roots_beside_the_apex_are_placed_along_the_edge`. At ε 1e-9, α 0.01, scale 1e3, δ 1e-2, a root lies 8.56e-7 m along the edge from the oracle's, against 3.6e-10 unresolved | ✔ matches the PR digit for digit |
| M7, conic slack meter off | `cone_rows::a_root_the_slack_meter_cannot_place_refuses`. At d = 1e-5 the door certifies 4 roots | ✔ |
| M6, apex rung off (`off_the_apex`) | `cone_rows::the_apex_rung_refuses_a_root_at_the_apex` | ✔ |
| M3a, `quadric_harmonics` c2/s2 swap | 9 topo rows: `circle_wall_rows` ×2, `ellipse_rows` ×7 incl. both `fuzz_rows`. Also `geom-brep` `conic_tests::the_harmonics_are_the_residual_along_an_ellipse` and 1 sweep row | ✔ (PR: 9) |
| M3b, `first_harmonic_arm` cos/sin swap | 5 topo rows incl. `ellipse_rows::the_first_harmonic_arm_places_its_roots_along_the_arc` and `circle_wall_rows::a_circle_square_to_the_axis_crosses_the_wall_twice`. Full topo + geom-brep + sweep run: 28 red (23 in sweep) | ✔ (PR: 5) |

The reviewers' survivors:
- **r2 M11 / depth rung unscaled: now KILLED.** The along-the-edge row goes red at ε 1e-12 (α 0.785, scale 1e3, δ 1e-11) with a wrong `Miss`.
- **r2 M5 = r1 M3, far-nappe tell-off off: still SURVIVES** every cone, line, conic, solid_contain, offset_nappe and sweep cone row. This is as filed (`the-far-nappe-tell-off-has-no-row-where-it-decides`). No wrong answer was shown.
- **r1 M4 = M6 (conic apex rung off): now KILLED** by the direct row above.

## 2. The along-the-edge row

- **Independent of the door.** Its residual closure is computed from the point. `conic_oracle::crossings` is a plain sampler and bisector over that closure.
- **Tolerance.** It is `ε + outside + 8u(scale+|t|)/|slope|`. At r2's worst class (α 0.01, scale 1e3) the resolution term is 3.6e-10 m, so the bar at ε 1e-9 is 1.36e-9 m. r2's 3–2,000 ε misplacements (3e-9…2e-6 m) all fail it, and M9's 8.6e-7 m did.
- **Coverage.** δ runs from 1e-15 to 0.3·scale, which covers r2's 4 mm–0.5 m at scale 1e3.
- **Limits (not blocking).**
  - At ε 1e-12 the same term is about 356 ε, so a misplacement below that is invisible there. That is an honest f64 limit of any oracle; mine skips those roots too.
  - The row only poses lines exactly parallel to the axis. My own probe (§8) adds tilted steep lines and near-generator lines, and catches M9 with 371 bad answers on that wider class while the head has 0. So the rung is guarded, and the wider class is clean.

## 3. The slack-meter row

- **Closed form re-derived.**
  - `C(θ) = c + ρ(cos θ·û + sin θ·v̂)` with `û = −ẑ` and `v̂ = ŷ×(−ẑ) = −x̂`, so `C′(0) = −ρx̂`.
  - At the root, `⊥q = d sin α·x̂` and `C′·â = 0`, so `|Q′| = 2cos²α·d sin α·ρ`.
  - `F = Q/R` with `R ≥ |c| + ρ`, and the lever is ≤ `|F′|` (`circle_roots.rs`: `window ≤ near ≤ e1`).
  - In `conic_cone_residual`, `cos` carries `2u·|cos θ|` into `a·cos`. Along `û = −ẑ` that lands in `h`, times `sin α`, so the reach is ≥ `2uρ sin α`.
  - The charge `ρ·reach·(|c|+ρ)/|Q′|` is therefore a sound lower bound: 4.4e-10 at d = 1e-6 and 4.4e-11 at d = 1e-5, both past the 1e-11 escalation threshold.
- **Which pose carries the row.** With M7 off, d = 1e-5 certifies, so that pose is the load-bearing one. d = 1e-6 refuses without the meter too.
- **The d = 0.3 control** certifies, and its roots match the oracle to `< 1e-12·ρ` (green on the head).

## 4. Exact-count rows (`assert_placed`)

Scratch mutants inside `off_the_apex`, which every certified cone root passes through:
- **X1, a duplicate of root 0 appended.** Red: `certified_answers_hold_against_the_quadric_form` (count 3 vs the oracle's 2), `crossings_match_the_quadric_form`, `a_root_the_slack_meter…` and `the_apex_rung…`.
- **X2, root 0 shifted 3e-9 m along the arc** (a shallow misplacement: the root stays on the cone to within about 2.6e-9). Red: `certified_answers_hold_against_the_quadric_form` at ε 1e-9, `crossings_match_the_quadric_form` and `a_root_the_slack_meter…`.

## 5. The apex rung, kept

Through the door with the meter on, I ran 49,200 apex-class conic poses at ε 1e-6/1e-9/1e-12, in three shapes:
- 15,000 random-tangent circles and ellipses through a point 0–10 ε from the apex (scales 1e-3/1/1e3, α 0.005–1.56);
- 15,000 meridian-plane circles crossing a generator 1–10 ε from the apex;
- 19,200 of the PR row's own geometry (radius 1 mm–1 m, d = 1–10 ε, four tilts).

Results:
- `AtApex` was never answered and no root was certified. The answers were `Uncertain`, typed `Escalated` (1,212 in all), and one `OnSurface` for a 1 µm circle wholly within ε of the cone at ε 1e-6, which the oracle confirms.
- **No wrong root near the apex.** Unreachable is consistent with the PR.
- **NOTE-1 (not reproduced, not blocking).** The PR says that with M7 off "those same roots answer `AtApex`". With M7 off, all three of my apex sets answer exactly as with it on: zero `AtApex`, zero certified. The rung's only demonstrated reach is the direct row. Keeping a conservative backstop is harmless, but the "meter's backstop" sentence rests on a search I could not reproduce.

## 6. `face_nappe` mapping

- **Inspection** (`reduce.rs`, crossing layer):
  - `NappeStraddles` → `None` (left to the trim);
  - `Escalated` → `BooleanError::Escalated { decision: Containment }`;
  - `StaleFace` and any other kind → `ClassificationInvariant`.
  - `face_nappe` can only return those three (`offset_nappe.rs`), so the catch-all is defensive.
- **Constructing an escalation: not possible through public doors.** I built narrowing frusta whose tip sits 0.5, 2, 5 and 9 ε below the apex. The profile validator (`revolve_common::validated`) refuses all four before any body exists. At 30 ε and 1,000 ε the body builds, and the split matches the oracle (4/4 vertices, both orders). So no silent skip was reachable; the escalation arm is checked by inspection only.
- **Torn-body panic.** `corner_stations` panics on a torn ring link, pinned by `offset_nappe::torn_hop_rows` through `review_d18::assert_torn_op_panics`, the same D2 row 4 pattern as `splitting/{join,finish}.rs`. That is consistent with the repo's stance. I found no public door that hands the crossing layer a torn body.

## 7. Differential retirement: re-measured

- **Sample:** 2,100 conic-door answers against spheres and cylinders: 700 per ε at 1e-6/1e-9/1e-12, scales 1e-3/1/1e3, a fifth of the poses 1e4 m off the origin, with near-coaxial and near-equal-radius classes.
- **Mix:** 753 certified, 1,131 `Miss`, 36 `OnSurface`, 155 `Uncertain` and 25 `Escalated`.
- **Result:** merge base `94512aac` and head `6423635c` are **byte-identical** (sha256 `760d6bf5…` on both files).

## 8. Central-bar fuzz (seed `0x41350001`, own oracle)

- **Line × cone, 9,900 poses** (1,100 per ε × scale), in five classes: generic, steep beside the apex (δ 1e-9–1·scale, tilt 0–0.9α), near-generator (γ 1e-10–1e-2, offset 1e-12–1e-2·scale), apex pass (δ 1e-14–1e-2·scale) and graze (±30 ε).
  - Answers: 2,548 certified, 370 `Miss`, 1,234 `AtApex`, 4,325 `Uncertain`, 1,423 `Escalated`.
  - **0 bad.** The worst in-span placement is 0.21 ε; 141 oracle roots were too unresolved to judge.
  - No `Miss` within the band, and no `AtApex` farther than 1e4 ε from the apex.
- **Conic × cone, 9,900 poses** (circles and ellipses), in five classes: generic, near-coaxial (tilt 0–1e-3, radius off by 0.1–1e4 ε), shallow / near-generator (γ 1e-9–1e-1), apex pass, and graze (±30 ε).
  - Answers: 1,556 certified, 808 `Miss`, 106 `OnSurface`, 6,560 `Uncertain`, 775 `Escalated`.
  - **0 bad.** Counts match exactly, the worst placement along the arc is 0.86 ε, and 9 roots were unresolved.
  - With M7 off the same seed certifies 2,331, still with 0 resolvable wrong roots (691 unresolved). This agrees with r2.
- **Sensitivity check.** The same line probe under M9 reports 371 bad answers.
- **End to end, `sweep_split_admitting_cones`** at ε 1e-6/1e-9/1e-12.
  - **Fixtures:**
    - narrow (k 0.02), mid, wide (k 20) and narrowing frusta, and full cones, at scales 1e-3/1/1e3;
    - crossed by turned bricks and rods across the wall, bricks through a cap, and thin bricks past the apex (d 1e-3/1e-6/1e-8·scale, some tilted);
    - both operand orders.
  - **Attempted: 268 splits** (150 poses × 2 orders, less 16 poses whose scale-1e3 frusta do not build at ε 1e-12: `revolve`/`finished` refuse first).
  - **224 completed, every one matching the oracle one for one** (worst 5.1e-3 ε); 0 wrong.
  - **The other 44 refused typed:** `CurvedPierceUnsupported` (16) and `Escalated` (28).

## 9. Merge with current main (`origin/main` `7e4e2c96`, 301 commits)

- **The merge is clean**, and main brings in no cone-related changes.
- **On the merge** (its own target directory):
  - `cargo nextest list --workspace --profile ci` succeeds;
  - the PR's rows pass 98/98 at each of 1e-9, 1e-6 and 1e-12 (`cone_rows`, `line_cone_rows`, `reach_cone_root_lane`, `conic_quadric`, `offer_rows`, `refusal_routes`, `conic_tests`, `sphere_region`);
  - all of topo at 1e-9 passes 2534/2534.

## 10. Suites on the head

- `nextest -p geom-core -p geom-brep -p topo -p sweep` at ε 1e-9: **6,698/6,698 passed.**
- The PR's rows at 1e-6 and 1e-12: 98/98 each.
- There was no red to check against `origin/main`.

## Process note

I briefly built the merge tree into the head's target directory, against the discipline's own-target rule. Cargo then served the merge's `geom-brep` to one head build, which failed to compile. No result above came from a mixed build:
- every head result reported here ran before that build, or was re-run in a fresh target directory;
- the merge results were re-run in a dedicated target directory.

## Not blocking

- NOTE-1 (§5): the "M7 off → `AtApex`" claim is not reproduced.
- The far-nappe tell-off still has no deciding row (filed).
- At ε 1e-12 the along-the-edge row's oracle resolution reaches about 356 ε at α 0.01, scale 1e3. That is an f64 limit.
- The apex refusal zone is wide (filed as `the-cone-apex-refusal-zone-is-a-frontier`). In my fuzz, 74% of conic and 58% of line draws refuse (70% counting the line arm's `AtApex`), which is expected given the adversarial classes.

# Review of PR #4135, frozen head 17c7349bb5

Lane `reach-dual4135-r1`. Wall clock: start 2026-10-06 17:32:38 UTC, end 20:17 UTC.
**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 3 · NOTE 3. No wrong root, missed crossing or wrong verdict was found anywhere I could reach.

## Claims, by execution (probes under `probes/`, mounted temporarily via `#[path]`, oracle never reads kernel code)
1. **Conic arm.** I derived `Q = cos²α|⊥q|² − sin²α h²` restricted to a conic myself: the `wall`/`height` harmonics in `implicit.rs:1144-1163` match, and so do the `|q|²` floor (`near_sq`) and `g = ρcosα+|h|sinα ∈ [min(sinα,cosα)|q|, |q|]` (Cauchy–Schwarz), on both nappes. `r1_cone_door_probe.rs` ran 13 500 random circles and ellipses (α 0.003–1.567, scales 1e-3/1/1e3, ε 1e-6/1e-9/1e-12). The classes were generic, coaxial and near-coaxial (tilt 0–1e-3), true grazes (moved off by −10…100 ε), apex passes, and exact plane sections lying on the cone. Oracle: `ρcosα−|h|sinα` sampled at 40k, then 4M, with bisection, and the meridian-plane distance. **0 wrong.** The worst certified root is 0.69 ε from the oracle (arc length or off-cone). `OnSurface` was answered only for true sections, and verified. Every graze within ±10 ε refused; a `Miss` came back only at ≥ 97 ε of clearance.
2. **Line arm.** `r1_line_cone_probe.rs` ran 72 000 lines: generic ones, lines within 0–1e-3 rad of a generator and 0…1e4 ε off it, lines through the axis, and lines passing 0…1e4 ε from the apex, at the same three ε and scales. **0 wrong.** The worst in-span root is 0.29 ε off. Lines along a generator answer `Uncertain`/escalate, never a root. **The ray lane is unchanged:** `r1_ray_lane_probe.rs` classified 164 640 points against 5 cone bodies, 3 scales and 2 poses on the merge base `94512aac` and on the head. The FNV digests of every `point_in_solid` answer, escalations included, are identical at all three ε (`87826810…`, `3d368b78…`, `b6e8c2a3…`).
3. **Far nappe.** M3 (tell-off disabled) leaves every PR row green, which is what the PR filed. I tried to build the deciding row with a quarter-revolved frustum (a partial face, so the trim cannot place a root) and a brick crossing only the mirror nappe 4 times (`r1_far_nappe_probe.rs`). It answers correctly (0 new vertices) **with and without** the tell-off, so it did not reach the decision either. See MINOR-3.
4. **Apex.** Where answers come back for circles passing `d` from the apex: never at `d ≤ 1e4 ε` at any scale. At scale 1 the first certified answers appear at 1e5–1e6 ε (ε 1e-9) and 1e9 ε (ε 1e-12). All of them were right. The line arm returns `AtApex` on about 33 % of apex-class draws and never certifies a root within ε of the apex.
5. **Root slack** holds where it was measured (0.69 ε conic, 0.29 ε line), but see MINOR-1.
6. **Other convexity shortcuts.** I re-swept `convex` over `boolean/{reduce,solid_contain,contain,boxes,arcs,sectors,rest}.rs`. Every remaining site is either sphere/wall-guarded, about circles, or rests on the cover invariant rather than convexity (`reduce.rs:2228`). `ConeSlab` boxes read the axial window, not a hull. `cone_face_containment` reads the elevation and `point_on_cone_in_face` with its nappe. I found no other site that admits a cone on a convex-inside premise. *By inspection.*
7. **Mutants** (`probes/r1_mutants.py`, rows of `cone_rows`, `line_cone_rows`, `conic_quadric` and `reach_cone_root_lane`):
   - **M1**, convexity restored for cone: 3 sweep rows red, the both-nappes row among them ✔.
   - **M2**, floor re-read dropped: `a_narrow_cones_near_coaxial_circle_is_not_on_it` red ✔.
   - **M5**, line apex rung off: 2 rows red ✔.
   - **M7**, `floor = 1`: red ✔.
   - **M8**, `c2` height sign flipped: 3 fuzz rows red ✔.
   - **M3**, **M4** and **M6**: all rows green (MINOR-1, MINOR-2, and the filed gap).
   - The PR oracles are independent (their own residual and distance). The differential is green on the suite run; its frozen copy is a style note.
8. **Production.** `sweep-testing` is turned on only in dev-dependencies (`topo` self-dev, `sweep`, `editor-core`), and `boolean_arm_exists` (`reduce.rs:195`) still omits `Cone`. `every_op_refuses_a_cone_operand_at_the_pair_gate` is green.

**Suites** (geom-brep, topo, sweep; clean worktree, default features):
- ε 1e-9: 5577/5577.
- ε 1e-12: 5577/5577. `arc_loft_natively_computes_its_rational_volume`, listed as known red, **passed here**.
- ε 1e-6: 5575/5577. The 2 red are the known `pinch_faces_tessellate::a_face_through_two_vertices_on_one_point_tessellates` and `pocket_ring_steep_ellipse::steep_ellipse_poses_build_sound_or_refuse_typed`.
- I did not run `--features per-op-postcondition`.

**End to end** (`r1_e2e_probe.rs`): through `sweep_split_admitting_cones`, both orders, identity and off-axis rotated pose, ε × scale. Fixtures: narrow (k = 0.02) and wide (k = 20) frusta, a narrowing frustum, a full cone and a narrow full cone, crossed by bricks, a turned brick, tilted and flat rods, bricks past the apex at d = 1e-3/1e-6/1e-8, and an edge on a generator or 1e-4 off one. **144 splits matched my oracle exactly, 0 wrong.** Every other case refused typed:
- `CurvedPierceUnsupported` on generator edges and grazes;
- escalations near the apex;
- most poses at s = 1e3 with ε 1e-9.
Ergonomics: the lane is conservative at large scale and near the apex. The answers it does give were right everywhere I looked.

## Findings
- **MINOR-1, test gap. DEMONSTRATED (mutant M6).** `conic_quadric/mod.rs:367-377`: deleting the conic root-slack meter (`bool_conic_cone_root_slack`) leaves every row green at ε 1e-9. That meter is what claim 5's charge rests on, and no row goes red when it degrades.
- **MINOR-2, dead rung and a test that cannot fail. DEMONSTRATED (mutant M4 plus probe).** `conic_quadric/mod.rs:384-388`: `bool_conic_cone_apex` never returned `AtApex` in 1 800 apex-class draws, nor in the PR's own fuzz, because the floor and slack refuse first. Deleting it leaves all rows green, and `cone_rows.rs:242` accepts `Uncertain`, so it cannot tell the two apart.
- **MINOR-3, filed gap confirmed. DEMONSTRATED (M3 plus probe).** `reduce.rs:3273` (tell-off): still no reaching row, and my partial-face pose does not reach it either. The filing `the-far-nappe-tell-off-has-no-row-where-it-decides` stands. Its suggested `PartialConeFace` route may not exist, since a partial face's trim refuses its other roots too.
- **NOTE-1.** The apex refusal zone is wide (above). It is sound, but it is a frontier that `VERBS-CONE` consumers will meet.
- **NOTE-2, dispatch premise.** The brief lists `arc_loft…` as red at 1e-12; it passed on this head.
- **NOTE-3.** In `reduce.rs:3236`, `face_nappe(..).ok()` swallows `StaleFace` and escalations alike, so a face whose nappe is unreadable silently skips the tell-off and leaves it to the trim. Sound today, given the trim.

## Style
I exercised Q1, Q2, Q3, Q4, Q5, Q6 and Q7. Q8 was partial: I read `conic_quadric/mod.rs` and `cone_rows.rs` whole, and the `curved_face_arm` and `wall_crossing` spans of `reduce.rs`, not all 6 000 lines.
- **Q1, likely.** `boolean/mod.rs:3741` `sweep_split_admitting_cones` is a third copy of the gate → clone → `sweep_and_settle` preamble, beside `sweep_traces_with_pad` (3676) and `sweep_records` (3786). This is the class to sweep.
- **Q1, likely.** The pad literal `1.0 + 64.0 * UNIT_ROUNDOFF` appears at `implicit.rs:1175` and again at `1372` (torus), and neither site says so.
- **Q1, likely.** `reduce.rs:2615` `conic_clearance`'s cone arm re-spells `first_harmonic_arm`'s extreme read (`c0 ∓ (A₁ + A₂ + noise)`).
- **Q1, unsure.** `cone_rows.rs:476` freezes a copy of the pre-cone door body for the differential. It is a second spelling with no retirement date.
- **Q4/Q5, sure.** The `reduce.rs:2141` comment still reads "circle × sphere, × cylinder and × torus root lanes", while the `matches!` under it now admits `Cone`.
- **Q3, likely.** `cone_rows.rs:346,454` assert `count >= changes`, which is monotone the wrong way: an invented duplicate root on the cone passes.
- **Q6.** Every disclosed deviation has a `work/` item. None is unscheduled.

No glimpse of any other lane's branch, report or PR comment. The tree carries older `boolean/r1_probes.rs` and `r2_probes.rs` from earlier reviews; I did not open them.

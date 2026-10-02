# Delta review, PR 3817 fix pass (`47b16394c2..1d29d4bc81`, non-merge commits only)

**Verdict: APPROVE-WITH-FIXES.** I found no MAJOR. No wrong body turned up in 480 oracle-checked bodies or 3,600 hand-built loops. Every claim the fix pass makes held under execution, except where noted. The fixes below are not merge-blocking, beyond what MINOR 1 asks.

**What I ran:**
- **Suites.** `nextest --profile ci -p topo -p sweep -p geom-brep -p editor-core` at the default ε, 1e-6 and 1e-12: 7190/7190 each, with two panicking probes planted (below). The slow set (385 rows) was not run, except `rigid_map_near_eps_plane_nurbs` at 1e-12, which is red as it is on main (`the_certificate_re_derives_within_rounding_under_the_map`).
- **Tools.** `tools/k-lint`: `cargo test` green.
- **Probes** (in `probes/`): `review_3817_side.rs` (geom-brep), `review_3817_bodies.rs` (sweep), `review_3817_bisect_ring.sh`.

## Claims
1. **Sense cross-check** (`curved.rs:2354`). Shown by execution.
   - `review_3817_side.rs` covers caps, lunes, octants, and pole-free slit bands bigger than a hemisphere, 400 random tilts each, both traversals: 3,600 loops. The oracle is the region's own membership formula and its closed-form area, which I cross-checked by Monte Carlo (400k points, all within 0.4σ).
   - Result: 0 wrong `Encoded`, 0 false refusals, 0 area misses (to 1e-9). Every pole-free face refuses `SenseContradicted` under the flipped bit, at all three ε.
   - The only `Unencoded` pole-free loops are caps whose rim passes within about ε rad of a pole: under ~1e-9 rad at ε 1e-9, under ~9e-7 at ε 1e-6. That is in-band and correct.
   - Faces with holes cannot reach this arm: props refuses them as `RingOnCurvedFace`.
   - Body level (`review_3817_bodies.rs`): 4 radii × 3 distances × 10 directions in the seam plane × 4 ops gives 480 bodies. All of them build, match the lens closed form to 1e-9, pass tier 3, and none is refused under its correct bit.
2. **Girard pin.** Shown by execution.
   - With the turning sign flipped, f4 goes red (`area 0.0720 against Girard's 0.00846`).
   - With the curvature sign flipped, f4 stays green and 6 other rows go red. This is as the PR body discloses: the patches are geodesic, so κ_g is identically 0 on them.
   - The closed form is right. Van Oosterom–Strackee: `tan(E/2) = |u·(v×w)| / (1 + Σu·v)`. Its `.atan()` with `.abs()` is valid only for E < π (`m5_pr12_fix_pass.rs:287`). That holds for these patches; it is a NOTE.
3. **`ReflexRunEnd`.** Partly shown.
   - Smooth and convex ends select the oracle's arc on all 480 bodies (claim 1's probe).
   - I made the refusal branch panic (`chord_join.rs:1632–1634`). It never fired in 7190 rows × 3 ε or in the 480 probe bodies, so no body in the tree reaches a reflex run end. See MINOR 1.
   - `EndsDisagree`'s doc and Display are now true by inspection: with both ends smooth or convex, exactly one candidate leaves left at each end.
4. **Clearance removal** (`reduce.rs:1633`). Behaviour preserved, shown by execution.
   - I planted a panic for the case where the skipped clearance would have read `Positive` or returned `None` (frontier) on an end-on-carrier arc. It never fired across 7190 rows × 3 ε.
   - By inspection, `circle_residual_extremes` is `Some` for sphere, cylinder and torus, so the frontier arm was unreachable. A `Zero` end bounds the enclosure's one-sidedness margin below `Positive`, and `(Ok(Zero), Err)` refuses in both orders.
   - With the skip disabled, `tilted_sphere_pair_k_rows` goes red (union records −1.605e-6); restored, it is green.
5. **k-lint rule (5).** Shown by execution.
   - Mutants on `construction_coupled.rs`, each failing the rows named:
     - ceiling factor 4.0: rows 2 and 4;
     - definite arm dropped: row 2;
     - early return dropped: rows 1 and 2;
     - ceiling halved: row 1.
   - Each row reds under at least one mutant. The pcurve_cache text pins (row 4) I checked by reading the `include_str!` greps, not by mutating the source.
   - Can it mask?
     - Definite rows still flag (`ConstructionRefused`), and in-band rows stay rule 1.
     - Zero rows in `(0.25, 0.30]·ε` are deliberately unflagged; the 20% is stated.
     - The name keys on `Curve3::Circle` at `pcurve_cache.rs:4821`, not on the lane. Today only the sphere general-circle image reaches `run_fitted_checks` with a circle carrier: `certify_general`'s stated images are NURBS carriers in `src` (`edge_nurbs.rs:510`). So no non-fitted site exists yet (by inspection). See style 2.
   - Is rule (5) governed by a ratified page? `tools/README.md` (`CC1`–`CC5`) governs the reading boundary: `lint_csv` and the admissions table. Rule (5) lives in `lint_sample`, a judgement over readings already admitted, so no clause decides it. The k-lint module docs are not in DESIGN.md's companion table, and `git log -S'ε-coupled families'` finds only `8cecba6ff` (an implementation commit, not a ratification). This is agent-mergeable.
6. **Register.** I matched all 9 new `predicate-dimension-audit.md` rows against their `Margin` expressions, and each describes the code. `props_sphere_loop_area`'s `(τ−turning).min(τ+turning)` is indeed the smaller of the two solid angles. The C12 sentence in `geom-brep/README.md` is true: the circle-only loop is not `is_trimmed`.
7. **Main's dev-probe leg: CONFIRMED red on `cd49025f54`** at ε 1e-9 with `--features probe`, as `k_probe_sweep.sh`'s plain selection runs it. Bisected on the first-parent line:
   - `sweep::review_ring_clearance_r1_probes::recorded::r1_ring_clearance_decisions_per_carve` (rim (1,0): 2 vs 0) has been red since **#3715** (`490fa9ff5`, band/annulus-host-outer-metered). `git bisect run` from good `b6f38a573`.
   - `topo::rim_dim_boolean_twins` (`bool_germ_plane_normal` and `bool_plane_parallel` do not scale linearly) has been red since **#3768**'s merge (`c965a92de`), which carries `ebbb642a9` ("the boolean decides a germ plane's normal at the read"). It is green at `4b3a44dec`.
   - `topo::rim_dim_review_probes::silent_fixed_predicates_scale_linearly` (`bool_plane_orient`) has been red since **#3657**'s merge (`b43aa794e`, REACH's own cosurface-continuation PR), and is green at `e953fae0b`. So this one is REACH's to file, not a stranger's.
   - None of the three is this PR's.

## Findings
- **MINOR 1 — `ReflexRunEnd` has no row that reaches its call site** (`chord_join.rs:1626–1636`). Shown by execution. Only `run_corner_opens` is pinned (`chord_join.rs:3763`). A mutant that deletes the gate *call* turns nothing red, and nothing in the tree builds a reflex run end. The refusal is the ask's own subject. It owes a fixture that reaches it (a hand-built divided face, as the unit-test module can make), or a filed item saying none is constructible yet.
- **MINOR 2 — the cross-check closes a minority of the flips this family produces.** Shown by execution.
  - Of 1,646 single-face flips over the 480 bodies, 274 refuse by name. The other 1,372 (83%) return a wrong volume from `mass_properties`, and tier 3 sees only `LaminaWedge`. These are the pole-bearing remnants.
  - It is disclosed and filed (`props/a-sphere-face-whose-boundary-encodes-no-side…`), but at P3 and with one pose. The item should carry the share.
- **MINOR 3 — rule (5)'s reasons tally as rule 2** (`tools/k-lint/src/lib.rs:594–595`), while the module docs list it as "5." (lib.rs:34). Sure. The PR body says this is deliberate, for demotion, but the per-rule tallies then cannot show rule 5 at all. Two numberings for one rule.
- **NOTE — the outer loop alone carries the side, in check 6 too** (`validate.rs:6203`). A pole inside a *hole* would read as a pole in the face. Seams make that unreachable today, since a face around an axis needs one, and props refuses ringed curved faces. Unsure whether check 6 meets ringed faces anywhere.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q6, Q7; Q8 on `chord_join.rs`'s run-side section only)
1. `demos/tour/src/lily.rs:2207,2227,3744`: the carve ball `(-2.80, 0, 0.90), 0.16` is spelled three times. The fix pass added the second copy beside the `carve` closure meant to be its home. Sure.
2. `pcurve_cache.rs:4821` keys the `CONSTRUCTION_COUPLED` name on the carrier kind. A future circle lane with another construction would inherit the ε/4 exemption silently. `the_rostered_target_is_the_lanes_own` checks that the name is spelled, not where. Likely.
3. `tilted_sphere_pair_k_rows.rs:37`: `METRE_FLOOR = 4.0e-5` copies k-lint's `BASELINE_FLOOR_MARGIN` across cargo roots. It is disclosed, but nothing keeps the two in step. Sure.
4. `reduce.rs:1617–1642`: a 20-line justification sits on a one-branch skip, and the `side` closure moved above the match so that both arms can share it. The comment reads as the argument "this decides nothing", which my instrumentation confirms. Still, I would have had the probe row carry it, not the prose. Unsure.
5. `chord_join.rs:1632`: a loop with nothing certified beside the run refuses as `NoCertifiedRun`, whose doc says the *run* has no certified edge. One variant, two meanings (Q1). Likely.
6. `curved.rs:2371`: the meridian schedule `[0.5, 0.25, 0.75, 0.125, 0.875]` is a magic list. It has no row in which a reading is decided by any point after the first. Unsure.
7. `curved.rs:2518`: a single pole's reading suffices (`[Some(n), None]`). That is right because of the Jordan argument, but the doc's list of `Unencoded` cases does not say that a tangent `enter` at one pole falls back to the other. Unsure.
8. `m5_pr12_fix_pass.rs:287`: `atan` of an `abs` instead of `atan2`, so an obtuse patch (E > π) would be pinned wrongly. Unreachable for fillet corners. Likely, harmless.

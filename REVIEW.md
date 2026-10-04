# Review — PR #4031 "join: a parallel cylinder germ pair splits each wall along its own ruling"

Frozen head `32bb4312`, base `45dc18f9`. Full reviewer lane (correctness + style).

**Verdict: APPROVE-WITH-FIXES** — MAJOR 0 · MINOR 1 · NOTE 3

No wrong body ships in any pose I built. Every new build passes `differential::outcome`: tiers 2 and 3′, the certificate, a legal operand, and |ΔV| < 1e-7 against a closed form or an independent oracle. The one MINOR is a refusal that reaches a backstop rather than a decided predicate, not a wrong answer. The fixes owed are a filed row for it and a stale count.

Probe rows are committed here as `crates/sweep/tests/review_4031_probes.rs` (`#[ignore]`d, registered in `all.rs`). Each was run on main and on head from release builds, and the lines diffed.

## Claims

1. **No wrong body ships — HOLDS.** 1 300+ runs: all six ops/orders per pose, at ε = 1e-9 (default) and again at ε = 1e-6.
   - **Poses:** external and internal tangency at δ = ±1e-3 … ±1e-8 for r ∈ {0.2, 0.3, 0.8}, plus a turned copy. Equal radii at offsets 0.3 … 1e-8, short and long. Near-coaxial rods, r = 0.2 and r = 0.49999. Rods across the drum's cap rim with the rod top at 1 ± 1e-3 and 1 ± 1e-6. Four turned axes × four rods. Two rods cut or united in sequence.
   - **Every main → head change** is `ERR CurvedBooleanUnsupported` → `OK SOUND`: 192 + 54 + 162 + 15 at the default ε, 267 at ε = 1e-6. There are 0 new BAD, 0 SOUND → refusal, and every other line is identical.
   - **The near-tangent band refuses typed** at both tolerances: `Escalated` (VertexOnCurvedFace / Containment / ArcCylinderRoots), `CurvedPierceUnsupported`, `CurvedSectorSideUnsupported` or `UndeclaredCoincidence`. At ε = 1e-6 every |δ| ≤ 1e-5 refuses. At ε = 1e-9, δ = −1e-7 builds a 1e-7-thick sliver, SOUND at its closed form. That offset lies outside the (1e-9, 1e-8) band, so it is decided geometry, not a pose that should refuse.
   - **Disjoint unions (δ > 0):** 34 lines read `OK BAD t3p=false`. They are byte-identical on main, so they are pre-existing and not this arm.
   - **One weak spot** → MINOR-1.
2. **The radical plane is right and decided — HOLDS.**
   - **Formula:** checked independently. The power difference `|p−o₁|²_⊥ − r₁² − (|p−o₂|²_⊥ − r₂²)` is linear in `p` along `w`, with its zero at `x = (d² + r₁² − r₂²)/2d`. Radii enter only squared, and `geom/convention.rs:59` fixes `radius > 0`, so a sign cannot reach it. `d` is decided ≥ the band before the division (`join.rs:1494-1515`).
   - **Predicate:** the one new predicate, `bool_join_cc_axis_offset`, goes through `UnitVec3::new`, which calls `decide`. Its dimension is a length (m), and the audit row is right (`docs/predicate-dimension-audit.md:443`).
   - **Escalation:** in band it escalates as `Coincidence(Section, Moot)`. That path is reached only directly: unit rows `radical_plane_rows` and the offer case `parallel_axes_offset_in_band` / SITES row pass (10/10 in `topo --lib radical_plane_rows offer_rows`). No public pose of mine reached it: equal radii at 1e-8 refuse `UndeclaredCoincidence` first, as the PR says.
3. **The 477 moved lines are right — HOLDS.**
   - **Re-run:** `join1_delta_arc_battery` gives exactly 477 moved lines, all `ERR CurvedBooleanUnsupported` → `OK SOUND`, 324 of them `decl=true`. Every other line of its 7 350 is identical.
   - **The battery's own SOUND is weaker** (NOTE-3), so I re-ran the whole battery through `differential::outcome` (`r4031_arc_battery_differential`). The volumes are Richardson-extrapolated chord areas, checked against closed forms to 4e-11 (half/half lens) and 4e-15 (shallow segment). Result: the same 477 lines move, and all 477 are SOUND under the full five checks.
   - **Declarations dropped:** each of the 324 `decl=true` lines has a `decl=false` twin. 171 twins refuse `UndeclaredCoincidence` (flush caps, where the declarations are meaningful). The other 153 build SOUND at the identical 9-digit volume.
   - **Declared contact untouched:** the arm reads no declaration. Its diff touches only the kind dispatch, `parallel_radical_plane` and `GermLane::Rulings`.
4. **Nothing else moved — HOLDS.** `rc_wide_battery` is identical main vs head over 40 320 lines. The JOIN-1 arc battery is as above. My own cylinder battery is in claim 1.
5. **The mutants are real — HOLDS.** One build switched by an env var, run against the PR's 15 rows:
   - **M1, M2, M4:** red on 5 rows each.
   - **M3** (ring closure planar): red only on `wide_nested_and_turned_walls_join_along_their_rulings`.
   - **Mine, M6** (ring closure `Wall` on A, `Planar` on B): the same single row goes red.
   - **Mine, M7** (plane stood `d − x` from A's axis, the wrong ruling pair): 4 rows go red.
   - **Mine, M5** (tangent guard deleted): **survives** all 15 rows and my 678 near-tangent and equal-radius runs (NOTE-1).
6. **The sweep and the filed rows are right — HOLDS, with NOTE-2.**
   - The along-edge row cites `choose_roles`' `RingClosure::AlongEdge` (`join.rs:2223`, doc `:233`) correctly.
   - The CONTACT evidence matches `the_rim_crossing_rods_stop_at_the_volume_probe`, and the TANG row is closed against `parallel_cylinders_that_pierce_build_at_the_closed_form`.
   - `work.py lint` is ok.

## Findings

**MINOR-1 — a near-parallel pair the frame calls parallel reaches the arm and dies at the pcurve backstop.**
- *Cause:* `pair_section_frame` decides "parallel" on ‖a₁×a₂‖ levered by the larger radius (`join.rs:1719-1720`). The arm then reads the axes "as unit and parallel, as the frame dispatch read them" (`join.rs:1466-1467`) and builds the plane on A's axis alone. Over a rod of length L the real axis drift is θ·L.
- *Shown by* `r4031_lever` and `r4031_lever_small`: a drum r 0.5 against a rod tipped θ about x.
  - θ = 0 at half-lengths 50 and 200: SOUND (control).
  - θ = 2e-10 … 1e-9 at those lengths (drift 1e-8 … 2e-7, 10–200× the zero band): every op refuses `Pcurves { LoopDiscontinuity }`. On main the same poses refuse `CurvedBooleanUnsupported`.
  - Drift ≤ 5e-11 builds SOUND. Drift ≈ 5e-10 refuses `CurvedPairUnsupported`.
- *Why MINOR:* fail-loud holds, because the result-body pcurve pass refuses rather than shipping. But the door is a backstop, not a decided escalation. The geom-brep table for the same pair levers parallelism by **extent** (`geom-brep/src/intersect.rs:1348`, `cc_axes_parallel`), so two spellings of one gate disagree.
- *Owed:* a filed row, at least.

**NOTE-1 — the tangent guard cannot go red** (`join.rs:844-851`). M5 survives every row and every probe of mine. The PR says so ("0 reached"), but no unit row forces a tangent `split_curve` through `GermLane::Rulings`, so the guard is unverified code.

**NOTE-2 — a stale count in the row.** "seven poses … 42 runs" (`work/join/parallel-cylinder-germ-pair-has-no-join-arm.md:56-57`). The file has 8 poses and 48 runs, after the island pose was added for M3.

**NOTE-3 — the battery the headline rests on judges a weaker SOUND.** `join1_delta_probes.rs` `outcome` does not require `cert`, and does not test a legal operand. Its tolerance is 1e-6 against a 4 096-chord oracle. So "477 → OK SOUND" was not the differential check. My re-judge clears all 477, so this has no correctness impact on this PR.

## Style

Exercised: Q1, Q2, Q3, Q4, Q5, Q6 and Q7. **Q8 only partly**: I read `bool_connect`'s dispatch (`join.rs:600-860`), the module header and `parallel_radical_plane`, not all 4 109 lines of `join.rs`.

- **S1 (Q1/Q7, likely) — the two radical-plane arms are derived differently.** The sphere arm takes its plane from the pair's own table (`sphere_sphere_section`, `join.rs:713`) and decides tangency there, before any mutation. The cylinder arm derives its plane from a hand formula (`join.rs:1470`) and lets two per-wall plane tables discover tangency after both bodies are split (`join.rs:840-851`). `cylinder_cylinder_section`'s parallel lane (`intersect.rs:1350-1353`, r₁ + r₂ − d) is a third spelling of this pair's tangency, equal-radius only.
- **S2 (Q1, sure) — "axes parallel" is decided twice with different levers:** radius at `join.rs:1720`, extent at `intersect.rs:1348`. MINOR-1 is the observable consequence. Also look at the cylinder × sphere frame and `pc_rim_alignment` (`intersect.rs:852`, radius-levered) for the same lever choice.
- **S3 (Q3, sure) — the tangent guard has no row that can fail** (NOTE-1). Its premise, a wall that the plane only touches, is excluded by every door upstream.
- **S4 (Q2/Q4, likely) — the arm breaks a written discipline.** The germ-normal comment says reads happen "before they mutate the body" (`join.rs:~649`). The Rulings arm returns `no_arm()` / `desync` after `split_curve` has mutated `red.a`/`red.b` (`join.rs:840-851`). It is harmless because the error aborts the op, but the comment's discipline no longer holds for this arm.
- **S5 (Q6, likely) — a disclosed deviation with no schedule.** The PR lets an in-band axis offset escalate here, while the sibling `bool_germ_frame_cs_offset` folds the same shape into `NoArm`. Two policies for one quantity (an axis offset in band), and no work item reconciles them.
- **S6 (Q3, likely) — the parallel pose's audit checks volume only** (`verbs_cylcyl_r1_review_probes.rs:142-161`): no t2, t3′, cert or operand check. Its shared volume is a literal keyed on the pose's name string, so a renamed pose silently drops back to the panic arm.
- **S7 (Q1, sure) — `join1_delta_probes.rs` keeps its own `outcome`** beside `common::differential::outcome`, with a different SOUND (NOTE-3). It is a near-duplicate that has drifted. It predates this PR, but this PR's headline count rests on it.
- **S8 (Q7, unsure) — the refusal can name the wrong face.** `no_arm` always names A's face for a cylinder pair (`join.rs:670-679`), even when B's wall is the one the plane only touches.
- **S9 (Q5, sure) — the stale pose count in the row** (NOTE-2).

REVIEW COMPLETE

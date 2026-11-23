PR 7615 review — FINAL. Frozen head 1cfa09180c, base a497926d13.

**Verdict: NOT-MERGEABLE-AS-IS, only on the PR's own account.** Two rows are red until PR 7573's fold cure lands under it. On the build itself the verdict is APPROVE-WITH-FIXES: no MAJOR, and every fix below is prose or test coverage.

## Executed
All runs were release builds, with the slow set via `--profile ci --ignore-default-filter`. The local instrumentation was removed afterwards and nothing was committed.
- **`mesh` whole suite, both trees, ε 1e-9, 1e-6 and 1e-12.** Base: 562/562 at each ε. Head: 562/563 at each ε. The one red is `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause` (Folded(Sphere)).
- **`sweep`, 77 meshing modules (709 tests; the PR names 131 modules and 934 tests), ε 1e-9, both trees.**
  - Base: 708/709. The red is main's `bisector_nodes::the_near_node_poses_at_the_witness_band`.
  - Head: 707/709. The reds are that row and `tilted_sphere_pair::a_circle_near_the_pole_meshes_at_a_coarse_delta`.
  - I did not run `sweep` at the other two ε.
- **Per-face differential.** A log at the lane dispatch in `tessellate.rs`, on both trees, recorded lane, chart, outcome, triangles, interior points and boundary ids for every face over both suites.
- **Rounds instrumentation** on head, in `trimmed`'s loop and `curved`'s loop.
- **A probe of my own**, run on both trees at three ε: 13 bodies, 8 δ, scales 1, 1e-3 and 1e3, 208 meshings. A sampled-distance oracle (`sampled_deviation`, order 6) was taken on every triangle.
- **8 mutants** of my own against the whole `mesh` suite.
- **Hosted CI**, job by job.
- **Read only:** the clippy, doc and gate jobs (CI's lint job is green), the viewer rows, `step-import` and the tour. Not run locally.

## Claims
1. **Held.**
   - `mesh`: no face lost but the near-pole ringed sphere face, at all three ε.
   - `sweep` (ε 1e-9): only `tilted_sphere_pair`'s 4 sphere faces (FoldedTriangle).
   - My probe: 0 of 208 meshings lost, at all ε and scales. Pipes cut on tilts 0.3 to 1.45, a hole beside a rim, balls cut near and far from the pole at tilts 0.1 to 1.4, slender cones (half-width 0.04) on tilts up to 1.5.
   - Head/base triangles per meshing 0.44 to 1.00, never above 1.
   - My torus builders failed to build, so tori come from the suites only.
2. **Held**, per (lane, chart), head/base triangles:

   | lane, chart | `mesh` | `sweep` |
   |---|---|---|
   | walked cone, cyl | 1.000 | 1.000 |
   | spline (trimmed nurbs) | 1.000 | 1.000 |
   | rectangle cone, torus, spindle | 1.000 | 1.000 |
   | rectangle cyl | 0.986 | 0.996 |
   | rectangle sphere | 0.994 | 0.971 |
   | planar | 0.985 | 0.921 |
   | trimmed cone | 0.506 | 0.535 |
   | trimmed cyl | 0.733 | 0.702 |
   | trimmed sphere | 0.481 | 0.547 |
   | trimmed torus | 0.506 | 0.568 |

   No walked, rectangle or planar face's triangles moved without its boundary moving. I took totals, not per-face counts, so I did not re-take the PR's per-face neighbour counts.
3. **Held, with a test gap** (finding M1).
   - Point inside the face and off a constraint: held by the code (`free` comes from `exists_constraint`).
   - No duplicate: held by the per-attempt `refined` set. The mutant that drops it goes red on 10 rows.
   - Cycling: none possible. Fold drops and corrections count against separate budgets and both stop the loop.
   - Past the budget the refusal is the certificate's own `CertificateExceeded` (`FaceVerdict::refusal`).
   - Over budget: the only face I found is the spur (`loops_the_meridian_guard_admits`, 6 rounds then refuses), and the base refused it too. None of my bodies took more than 1 round.
4. **Held.**
   - `mesh` most rounds 3 (`m5_pr11_trimmed::a_wide_tilted_cylinder…`); `sweep` most 2 (sphere).
   - Faces taking a round, `mesh`: cone 17/478, cylinder 31/~565, torus 6/108, sphere 0 outside the forced row.
   - Faces taking a round, `sweep`: cone 3/16, cylinder 20/~1,170, sphere 16/~660.
5. **Held.**
   - The rectangle lane's correction is reached only by its forced row and the spur. No other body in either suite reaches it.
   - The spline correction is reached only by `trimmed::tests::a_face_left_slivered_refuses_typed_through_tessellate` (6 rounds, refuses).
   - Memo: a hit replays the stored lane output, the key reads nothing the correction adds, and `patch_memo` is green.
6. **Held.** `UNPROVED_HEADROOM`, `face_target` and `edge_target` are gone from the tree; every caller of the step functions in `sizing.rs` hands them δ; chords read δ. A grep for typed shares finds only `LATTICE_SHARE` and `ALLOWANCE_SHARE`.
7. **Held.** `d9_mesh_goldens` passes at three ε. Probe triangle counts on head are identical at 1e-9, 1e-6 and 1e-12. Exceeded in `cert::verdict` is ε-free (see N1).
8. **Held.** `[mesh.sizing]` is byte-identical to `tesslane/ev-sizing` 10397beb6e. The README diff adds only the `MAX_CORRECTION_ROUNDS` line. I found no machinery beyond the clause.
9. **Rows hold most mechanisms; one does not.** Mutants against the whole `mesh` suite at ε 1e-9:
   - Correction off, chart lane: 8 rows red (`rowless` ×2, `m5_pr11` wide tilted, three `reach_probes`, the forced row).
   - Correction off, rectangle lane: only its forced row goes red.
   - Budget shared with the fold and shared-side retries: the forced row's own-budget assertion.
   - Step δ/2 on the sphere chart: `route_rows` ×2, `chords::tests::each_arm…`, `a_lattice_laid_without_clearance…`.
   - Centroid: `rowless_cylinders` ×2. The PR says only `sweep`'s snowman catches it; `mesh`'s point counts do too.
   - Dedupe removed: 10 rows red.
   - **Point allowed on a constraint side: SURVIVES** (M1).
10. **Held for `mesh`:** the base passes the old pins at three ε and the head passes the new ones. The viewer's `display_budget` and `review_pick_r2` were not run by me.
11. **Held.** On the 208 probe meshings the worst sampled distance is 0.9998 δ (pipe on tilt 1.3 at δ 3e-2, both scales). No triangle the certificate passed sampled over δ.
12. **Corrected in part.**
    - Those are the only bodies lost in what I ran (the `mesh` suite and my 77 `sweep` modules).
    - "A fold" is the right name: the refusal is FoldedTriangle.
    - The PR's "no triangle reads over δ, **so** the correction does not reach them" gives the wrong cause (N2).
    - The rate is ~2% only for `sweep` (~2.1%); `mesh` is ~4%.
    - The 0.5% oracle is a fit, not a bound (N3). On this head the 150 `bisector_nodes` oracle readings reach at most 0.292% (60°, r = 0.5, ∖).
    - What the integrated tree must read: ≤ 0.5% on `an_oblique_seam…` and `equal_cylinders…`, or the row re-derived from δ·A.
13. **The numbers I could check hold:**
    - 562 of 563 on `mesh`;
    - most rounds 3 (`mesh`) and 2 (`sweep`);
    - no rectangle, planar or walked face newly refuses or takes a round in the suites;
    - the `sweep` reds.
    - Not checked: the work table's times, and the 131-module / 934-test count (my set is 77 / 709).
14. **CI run 45248370651 on 1cfa09180c (queued 19:23, complete by 21:02 UTC): failure.**
    - Green: lint (fmt, clippy, gates), test (kernel) 4,839/4,839, viewer wasm.
    - test (app): 68/69. The red is `reach_probes::ringed_curved…` (the slow-set row).
    - Mesh budget meter: 316/317, the same row.
    - gate ok: red as a consequence.
    - No other red, so main's known reds were not hit here.

## Findings
- **M1. MINOR, DEMONSTRATED. No row holds "the point is never on a constraint".**
  - Allowing constraint sides (`.filter(|&k| free[k] || true)` in `trimmed::TrimmedFace::correction_point` and `curved::correction_point`) survives the whole `mesh` suite. In every corrected triangle of the suite a free side sags most.
  - Preferring constraint sides goes red on 7 rows, and the failure is loud:
    - the chart lane refuses `Triangulation` (`m5_pr11` r = 20, `rowless`, `reach_probes` pipe walls);
    - the rectangle lane PANICS in `tessellate::joined`'s `unpaired_chord_segment` debug_assert ("chord segment … is an edge of 1 face triangles"). That is a panic, not a typed refusal. Debug asserts are on in release.
- **M2. MINOR, BY-INSPECTION. Stale prose in `trimmed.rs`** that still calls the correction point a centroid, or a point strictly inside a kept triangle: line 113 ("refinement centroid"), 274–276, 719–720, 1318, 1367, 2485–2487 (`uv_centroid`'s doc: "the certificate refinement insert"). Also 855, 930, 959 and 1093 still say "refinement".
- **M3. MINOR, BY-INSPECTION. A test name that no longer describes its cells.** `rowless_cylinders::two_holes_across_a_column_and_a_half_mesh_by_the_correction` now asserts the 1.5-column cells mesh at the first attempt, and the correction fires on the 2.5-column cells.
- **N1. NOTE.** The module doc says the verdict "reads no ε", but `FaceVerdict::read` takes ε. It decides only Certified against Unresolved; Exceeded is ε-free, so the claim holds in substance.
- **N2. NOTE.** In `attempt` the fold branch returns before the correction is reached. A face that folds through its last attempt therefore never tries a correction, whatever its certificate reads.
- **N3. NOTE.** `bisector_nodes`' `assert_volume` and `mesh_agrees` justify 0.5% as "the chordal deficit at 2e-3 is a fraction of that", with no derivation. The certified form is δ·A (`validate::check_volume`).

**Spec deviations:**
- The spec and the fork log say "one interior point"; the build places the middle of a side. Reported by the PR and ruled in stage 1, so not a deviation.
- The rectangle lane's correction is reached by no suite body but its forced row. Reported (ruling 4).
- Silent: M1's coverage gap, and the 0.5% oracle's nature (the PR raises it only for the integrated tree).

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q8. I read `attempt`, `emit`'s correction section, `insert_points` and `curved`'s triangulation loop whole. Q7 lightly; I did not get to the whole of `trimmed.rs`, which is 7,112 lines.
- **Q1, sure.** `curved::correction_point` (curved.rs:613–648) restates the Lattice arm of `TrimmedFace::correction_point` (trimmed.rs ~1040–1085): middle, sag and choice. Its centroid fallback is a third spelling of `uv_centroid` (trimmed.rs:2488). The comment "The rule is the chart lane's" (curved.rs:616) is the reconciling sentence Q2 names.
- **Q1, likely.** The loop tail is written twice: the dedupe set, `!refine.is_empty() && !worst().is_nan() && corrections < budget` (trimmed.rs ~1169, curved.rs ~540). The budget, the counter and the test hooks (`correction_rounds`, `note_corrections`, `set_correction_rounds`) live in `trimmed`, which `curved` reads: a core hosted inside one of its two consumers.
- **Q1, likely.** There are two `STEP_SCALE` test thread-locals, one in `trimmed::tests` and one in `curved::tests`.
- **Q3, likely.** The `rowless_cylinders` `CORRECTED` table (10/8/10/2/1/1) and the two-holes point counts (4/6/5/4/4) pin the present arithmetic of the point rule, not a contract. That arithmetic did catch the centroid mutant.
- **Q3, sure.** `curved`'s forced row says "no triangle over δ is returned on either path" but asserts only `check_mesh`; nothing samples a distance.
- **Q3, sure.** No row goes red when the point lands on a constraint (M1).
- **Q6, likely.** "The most measured is 3" is written in the README and in `MAX_CORRECTION_ROUNDS`' doc, with no guard, no register, and no unguardable reason at the claim site.
- **Q5, sure.** M2 and M3.
- **Q7, unsure.** The precedence of shared side, then fold, then correction is unargued. A fold at the last attempt refuses without a correction tried.
- **Q8, unsure.** The `trimmed.rs` module header is ~330 lines and grows a titled section per unit ("The correction" is the newest).

## Rubric
- **Idiom/structure 3.** The correction is clean in each lane, but one rule has two homes and its core sits inside one consumer.
- **Test quality 3.** The mechanisms are well held (correction off, shared budget, δ/2, dedupe, centroid all go red), but "never on a constraint" is held by no row and the point counts pin arithmetic.
- **Doc/comment honesty 3.** The README clause is exact and the PR is candid about what is open, but there are ten-odd stale "centroid / inside a triangle" sentences and one imprecise "reads no ε".

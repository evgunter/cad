# Review of PR #3985, frozen head 7e33abf087

Lane `reach-dual3985-r1`. Started 18:51 UTC, ended 21:01 UTC (2026-10-03). Merge base `0770bfaa3`.
**Verdict: APPROVE-WITH-FIXES** — MAJOR 0 / MINOR 3 / NOTE 5 (+6 style). I found no wrong body anywhere: 0 edges off the closed form across about 3,500 built bodies at three ε.

## How I tested (all by execution, at the frozen head; no CI run exists on it, so I ran the gates myself)
- **Suites.** topo, sweep, mesh, editor-core and step-import at ε 1e-9 pass 7477/7477. At 1e-6 and 1e-12 the only reds also fail on the base (NOTE 1). The tour suite passes 95/95 (release build).
- **My oracle** (`probes/reach-dual-3985-r1/zz_review_3985*.rs`). Each operand has its own closed-form implicit: sphere, rotated box, cylinder, cone or frustum, combined with min/max into the result solid. Every result edge is sampled at 9 parameters and must lie on that solid's boundary within 1e-7·scale. The kernel's volume is checked against a 60k-point Monte Carlo of the same implicit, and `point_in_solid` against its sign.
- **Poses.** Every op, in both operand orders, at ×1e-3, ×1 and ×1e3. Sphere pairs: tilted, pierce rings, one ball's pole turned. Box×ball: tilted planes, mirrored boxes, a full ring on the planar face, rotated boxes. Bar-through-ball (reflex notch): three sizes plus a turned bar. Four-crossing slabs. Cylinder × rotated box. The wedge/collar matrix: 48 poses × 4 ops × 3 scales. Splits of cylinders, cones, frustums and of a cyl∖box result. Boolean results reused as operands.
- **Head results.** Default ε: 1,227 built, 0 off-boundary edges, 0 volume misses. Also 0 at 1e-6 (1,155 built) and at 1e-12 (1,071 built).

## Claims
1. **Holds.** No body that builds on both trees moved a bit, and nothing the base builds is lost.
   - 255 sweep-probe bodies plus 10 editor boss/slab/plate unions (all orders and prefixes) are bit-identical between base and head. The digest covers every edge's 9 sample points plus the volume bits.
   - The tour's 171 emitted STEP/STL files are byte-identical; only the impeller narration text changed.
   - I also ran the phase-1 cross-check build (`2f49b372`, `ARCPAIR_LOG`) over all 5 crates: 39,512 agree, 8,575 old-refused, **7 DISAGREE**. None of the 7 is on a body the base ships. I rebuilt each one on both trees: either base and head are bit-identical, or both refuse, or the base refused and the head now builds to its closed form. See MINOR 3.
2. **Holds.** Every newly built body passes the oracle in every op, both orders and all three scales. That covers the tilted sphere pairs, the pierce rings, the tilted plane-sphere planar side (including a full ring on the plane), the reflex-notch bar and the turned-pole variants.
3. **Holds.**
   - Walk order: slabs cutting the box face's section circle at −40°/60°/120°/220° and −10°/80°/100°/190° (pole y and turned) build correctly under all 6 runs. With the walk check removed (mutant M4) they refuse `RingHomingAmbiguous` / `RingOffCylinderChart`, so the nearest-by-chord partner never wins.
   - Strut facing: this PR does not touch `insert.rs`; the swap there is pre-existing (NOTE 4). The join reads `HalfGerm.dir` by matching `he` (`boolean/join.rs:625`). Mutant M2 makes each site read its partner's dir: 6 rows go red (including `a_pole_struts_halves_face_their_own_meridians`) and the oracle flags 6 off-boundary edges and 33 volume misses.
4. **Holds.** Mutants and the rows they turn red:
   - M1, `leave` negated in `arc_leaving`: 36 rows.
   - M2, partner site's dir: 6 rows.
   - M3, `split_leave` flipped (pairing and chord): 10 rows.
   - M3b, only the split chord's datum flipped: 11 rows.
   - M4, `walk_passes` disabled: 2 rows (MINOR 2).
   - Old selector restored: the PR's rows run on the base, and all 9 built-not-refused rows go red.

## MINOR
1. **Prose still cites the retired selectors** (inspection; the first is falsified by execution).
   - `crates/topo/src/boolean/reduce.rs:3335-3344`: says the collar∖wedge bore "refuses before any mate: `Join(SectionArcWindow{BothContained})`". That variant no longer exists, and the PR's own `wedge_through_a_full_turn_collar.rs` builds all 48 poses. This sentence was the cited evidence for when `Placement::Undecided` can be reached, so the premise needs re-measuring, not just deleting.
   - `docs/predicate-dimension-audit.md`: rows 525 (`run_is_section_arc`, function gone) and 536 (`bool_between_arc_window`, rung gone), and the F8 text at 789-791. Commit 92854cfc says "the audit's chord rows follow the code".
   - `demos/README.md:62` still says the z-moved snowman head "refuses `SectionArcSide { NoCertifiedRun }`" and that the spun ring "reads the face's window".
   - Also: `demos/tour/src/snowman.rs:378`, `demos/tour/src/lily.rs:4127`, `crates/sweep/tests/m6_rider.rs:61`, `verbs_sphsph_opening.rs:22`, `crates/geom-core/tests/cert4r2_probes.rs:101`.
2. **The walk-order rungs have no row of their own** (executed). `bool_join_walk_order` and `bool_join_walk_site` (`boolean/join.rs:1542`) only turn red incidentally under M4 (`a_pierce_off_the_seam_plane…`, `a_slab_across_a_round_boss…`). No row pins the four-alternating-crossing pose that is the check's reason to exist. My slab4 probe is one, and the in-band "coincides with an end" branch is never reached.
3. **The cross-check evidence is cited but absent** (executed). The PR body at this head is a placeholder. `work/reach/arc-side-rule-has-two-predicates.md` (Answered) says the body holds "the counts and the one class where they disagreed (the run-side rule read the wrong run)". My re-run finds disagreements on cylinder walls too, under the window rule: `reach_slab_cut_sector_side` order [boss, slab, plate] and `germ_coplanar_conic`'s tube strut. They read as artifacts of phase 1's chord-length pairing, which `cae80322` removed, so they are not only the run-side class. The phase-1 count therefore also does not cover the pairing that shipped.

## NOTE
1. **ε reds that are not this PR's.** At 1e-6, `review_cleave_wrongarc::steep_cuts_of_tubes_chord_inside_each_bore_face` is red, with the same margins on the base. At 1e-12, `step-import nurbs_import::arc_loft_natively_computes_its_rational_volume` is red on the base too, as is the known `rigid_map_near_eps_plane_nurbs`.
2. **`point_in_solid` refuses `PartialSphereFace` on every carved sphere body**, so my containment check is vacuous there (the issue is already filed). The edge and Monte Carlo oracles still apply.
3. **Remaining head refusals are typed.** `RingOffCylinderChart{Sphere}` (a whole section ring on a plane when the box face misses the circle, 54/504), `GermFrameUnsupported` for cyl×ball, and `SpheresMeet` for an r=1.7 ball. Order [2,1,0]'s refusal now names a different `FaceKey` (9v3 vs 15v3) for the same door.
4. **Brief premise.** There is no "dangling-strut facing swap in insert.rs" in this diff. `mint_directed` / `strut_facing` (`insert.rs:843-1190`) are untouched. What the PR adds is reading `dir` through `HalfGerm.he`.
5. **ε coverage of the PR's own new rows.** Its interval row stops at the pcurve escalation at 1e-12. Under 1e-6 and 1e-12 my probes build 1,155 and 1,071 of the 1,227 default-ε bodies, with no wrong ones.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q7 and Q8 (`chord_join.rs`, read whole). Q6 could not be exercised: the PR body is empty.
- **S1 (Q1/Q2, likely).** The chord re-decides a sign its pairing already decided. On the split, `split_join_conic_heading` (`splitting/join.rs:483`, levered by curvature) and `chord_arc_leave` (`chord_join.rs:1007`, levered by semi-major) decide the same `h·Ĉ′/|Ĉ′|` under two names and two levers. `the_chord_arc_rung_is_decided_in_one_place` counts only the second name, so it cannot see the first. The fix mints a fresh instance of the duplication it closes.
- **S2 (Q1/Q2, likely).** There are two walk orders: the boolean's angle about the centre (`walk_passes`) and the split's conic-parameter walk (`conic_pairs`). They are reconciled only in prose at `boolean/join.rs:948-953` ("the order `splitting::join`'s `conic_pairs` pairs by").
- **S3 (Q1/Q8, likely).** About 500 lines of window walk (`face_azimuth_images`, `run_azimuth_images`, `cone_apex_closure`, `chart_box_defect`) stay in `chord_join.rs`, though no chord reads a window any more. Their consumers are `solid_contain.rs`, `ops.rs` and the ring side, so the core is now hosted inside a module that no longer uses it.
- **S4 (Q5, likely).** The module doc (`chord_join.rs:76-82`) and the PR title say "every carrier and tilt", but `bool_planar_chord_spec` refuses any partner that is not a cylinder or sphere ("arm not wired", `:1328`), and the split refuses spheres at its reduce. That universality is a property of the rule, not of what can reach it.
- **S5 (Q3, likely).** The anti-re-fork row names its own two blind spots. A third is real here: a re-derivation under a different predicate (S1) is invisible to it unless the K-report census is read.
- **S6 (Q4, sure).** The stale prose in MINOR 1 is one class, found by grepping the retired symbol names. The PR's own sweep missed `snowman.rs:378` even though it edited that file's module doc. Sweep `demos/` and the test docs before closing.

## Glimpses
`rg` over `docs/` hit two rows of past pairs in `docs/DUAL-REVIEW-LOG.md` (DR-29, DR-45); they are not this pair. A local branch with this lane's name already existed at the frozen head, and its remote ref did not exist. I did not read any other lane, scratch area or PR comment. I read the PR through `get` only.

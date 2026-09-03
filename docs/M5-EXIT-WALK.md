# M5 exit walk (PR 14) — criteria vs evidence

Status: assembled at the PR 14 exit sweep, 2026-08-03, against
main's tip (post-#166, run 30835146557, 18/18 hosted rows green).

**How to read this.** Every criterion below is quoted **verbatim**
from `docs/archive/M5-PLAN.md` lines 372-405 — that section is one long
semicolon-separated paragraph, and it is split here at its
semicolons with no rewording, no merging, and no omissions. The
paragraph yields exactly twenty criteria; all twenty appear.

Dispositions are three, and the middle one is used deliberately
rather than as a softer "met":

- **MET** — the criterion as written is satisfied, with evidence.
- **MET-WITH-RECORDED-HONESTY** — the substance is satisfied but
  the criterion as *written* claims something the shipped state does
  not support, and the gap is named here rather than smoothed over.
- **CARRIED** — not met; carried to a named unit.

Three rows depended on Ev's ruling and **all three are now
decided** (PR #169, 2026-08-03, comments 5171303851 and 5171351203):
row 19 (#89 closed), row 8 (shape (v) accepted piecewise), row 20
(the three conventions ratified). Each carries its citation.

## The walk

| # | Criterion (verbatim) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "A tilted plane×cylinder boolean carries an exact `Ellipse` carrier with residual identically zero by construction, replaying bit-identically at ε ∈ {1e-6, 1e-9, 1e-12} + Interval" | MET | Acceptance shape (i). PR 5 (#141) minted the exact `Ellipse` carrier + the C5 dispatch table; S9 (#145) repaired the chord_spec azimuth window behind it. Corpus document `cut_cylinder` (`crates/editor-core/tests/corpus/cut_cylinder.rs`) carries the standard persistence/latency rows at all three ε and under Interval; hosted `band 4 corpus (eps = 1e-6 / 1e-12)` + `test (interval)`. The residual is zero *by construction* because the carrier is the analytic ellipse, not a fit — the property the criterion actually names. Stated precisely: **identically zero in ℝ by construction; rounding-scale at f64, zero-ENCLOSING at Interval.** `tilted_cut_mints_exact_ellipse_carriers` (`crates/sweep/tests/m5_pr5_tilted_cut.rs:70`) asserts `Curve3::Ellipse { major: r/cos φ, minor: r }`, description `Intersection`, `max_residual < 1e-12`; `tilted_cut_replays_bit_identically` (:213) is the replay row; `tilted_cut_at_interval_encloses_zero_residuals` (:267) is the only row that asserts genuine zero-containment, against both surfaces. |
| 2 | "a transverse curved boolean (cylinder boss ∪ plate) certifies end-to-end at tier 3 with every fitted cache carrying the full C2 certificate (hull sup-norm + uniqueness tube — no schedule-max-only cache at rest)" | MET-WITH-RECORDED-HONESTY | Acceptance shape (ii). PR 9 (#152) landed the transverse curved boolean; `boss_union` joined the Band-4 corpus at the PR 9 fix pass and the BVH differential lane runs the curved boolean bit-equal. The certificate structures make a schedule-max-only cache **structurally unrepresentable** rather than merely untested: `PcurveCertificate { samples, max_residual, envelope }` (`crates/geom-brep/src/pcurve_cache.rs:426`) keeps the C2.2 sup bound in its own field, private, mintable only through `PcurveCache::certify`; `SsiCertificate { samples, on_locus_max, hull_sup, tube_radius, tube_transversality, tube_boxes }` (`crates/geom-brep/src/ssi/certify.rs:170`) documents `hull_sup` as "the number that certifies" and `on_locus_max` as steering only. All three limbs are pinned (`a_good_carrier_certifies_all_three_limbs`, `crates/geom-brep/tests/m5_pr7_ssi.rs:470`) with a K-funnel coverage row asserting `ssi_on_locus`, `ssi_hull_sup` and `ssi_tube_transversality` were all reached. **The honesty**: on THIS shape the fitted-cache clause is satisfied **vacuously** — boss ∪ plate's seam is exact `Circle` carriers on both operands, so it has no fitted caches at all. More broadly, *no fitted cache reaches any body at rest* — pinned positively by `no_body_at_rest_carries_a_nurbs_carrier_or_face` (`crates/step-export/tests/m5_pr13_curved.rs:597`), which walks every corpus body's carriers and surfaces. PR 9's own spec asked for a cylinder×sphere rung-3-at-rest row to exercise the clause non-vacuously; that row does not exist. So the clause is TRUE at rest and never positively exercised — worth knowing before it is relied on. |
| 3 | "the small-loop fixture is found by exclusion subdivision or refuses typed `SsiExhaustivenessInconclusive` — verified by a fixture where naive marching provably misses" | MET | Acceptance shape (iv). PR 7 (#146): a sphere × 0.08-radius cylinder whose ENTIRE locus is two interior polar loops — boundary seeding reaches nothing, so naive marching provably misses — found by exclusion subdivision at 166 seeds, with the floor-refusal variant pinned as its own row. Both limbs of the disjunction are therefore exercised, not just the happy one. |
| 4 | "every curved edge at rest carries per-half-edge pcurves certified in meters, seam edges with distinct pcurves" | MET-WITH-RECORDED-HONESTY | PR 6 (#144). This is a **tier-3 validator check, not only tests** — `crates/topo/src/validate.rs:2024-2033`, "Tier 3, check 8 (M5 PR 6, C4)", calling `pcurves::validate_pcurves`, ungated. Mint + metres: `the_tilted_cut_mints_certified_caches_on_its_curved_faces` (`crates/sweep/tests/m5_pr6_pcurves.rs:101`) asserts every cylinder-face half-edge carries a cache with `max_residual < 1e-12` AND `envelope < 1e-12` — certified in metres, **both** limbs — planar half-edges store nothing, and the cache count equals the cylinder half-edge count exactly. Seam: `a_seam_edge_carries_two_different_pcurves_on_one_surface` (:186) — both half-edges of one edge in one loop of one face, `Pcurve::Harmonic` `p0.x` differing by exactly `TAU`, both certified. Persistence re-derives caches bit-identically and the bytes carry nothing pcurve-shaped. The review's best MIN — a snap-to-family ε-shell falsifying the stored envelope on the attach path — was fixed with the snap slack proven zero on minted caches plus an O(ε)-tightness pin. **The honesty, and it is the largest in this walk**: "every curved edge" is FALSE as written. Only **Plane and Cylinder charts certify**; cone, sphere, torus and NURBS charts mint nothing and refuse `PcurveCertifyError::UnsupportedChart` on a direct call (`crates/topo/src/pcurves.rs:31-37`, `crates/geom-brep/src/pcurve_cache.rs:690-692, 796-804, 1045`). That is a compile-time routing decision. So the filleted die's 8 sphere octants, and the `ball`/`cone`/`donut` shapes, carry NO stored pcurves at rest. The criterion holds as "every cylinder-chart curved edge"; the rest is **carried**. |
| 5 | "a NURBS-wall boolean (cut loft) marches, fits, certifies, and passes in-op exhaustiveness" | MET-WITH-RECORDED-HONESTY | The *substrate* row is GREEN and was exit-gating: PR 7b (#149) ships a directly-authored NURBS wall cut by a plane — rung-3 marched, fitted, certified carrier, **all three limbs**, both lanes, bit-replayed (`crates/geom-brep/tests/m5_pr7_ssi.rs`, `shape_iii_the_wall_cut_certifies_all_three_limbs`, `shape_iii_bit_replay`). **The honesty**: the criterion says "cut **loft**", and no loft BODY exists. `Loft`/`Sweep` build their walls and then refuse `CurvedSolidFrontier`, because tier 3's +V check routes a NURBS face to `Unimplemented` — NURBS-patch flux needs surface quadrature and the surface-AREA half has no closed form for a rational patch at all. Shipping the assembly without that would swap an honest frontier for a body that fails validation. **Carried to: the loft/sweep body assembly unit** (MAIN-PATH, after the SSI generic-`T` lift, per #161). |
| 6 | "second-order sector classification resolves a first-order tie with the normal-curvature trilean and escalates in-band osculation typed" | MET-WITH-RECORDED-HONESTY | The tangency regime, PR 9 (#152). **First half MET**: `the_tangent_graze_resolves_past_first_order` (`crates/sweep/tests/m5_pr9_sector2.rs:51`) asserts via the verdict log that all three predicates — `tangent_sector_order2`, `tangent_sector_order2_arm`, `tangent_sector_osculation` — reached the K funnel by name, and that the refusal is the DOWNSTREAM degenerate-section net, never the first-order door; `an_off_ruling_tangent_plane_still_grazes_honestly` (:98) pins that neither `TangencyUnsupported` nor `ConsecutiveOnSectors` fires. Escalation source: `crates/topo/src/splitting/rules.rs:174`, `decide("tangent_sector_osculation", …)` → `SliverSector`. **The honesty, second half**: the IN-BAND osculation escalation *at rest* is pinned only by a deliberately tolerant reviewer probe (`an_in_band_second_order_margin_at_rest_escalates_somewhere_loud`, `crates/sweep/tests/review_m5_pr9_inband_at_rest.rs:16`), which accepts three outcomes — profile-stage refusal, extrude refusal, or acceptance at rest provided no definite tangency verdict was minted. So the accurate claim is "escalates, or refuses to mint a definite verdict, pinned by probe", not "escalates typed" as a hard assertion. *Telemetry note*: these three predicates do not sample in the K-probe corpus — no registered document or tour scene reaches a first-order tie. |
| 7 | "definitely-tangent edges carry the tangency mark and jet-determinate tangencies enforce `TangentIntersection` (G2 conventional joins exempt by predicate, pinned both directions)" | MET | The declared-tangency discipline (#109/#101) extended through the curved lane in PR 9. The parenthetical is the part that is easy to half-ship and was not: the G2 conventional-join exemption is decided **by predicate**, and pinned in **both** directions — a conventional join stays exempt, and a jet-determinate tangency is forced to `TangentIntersection`. Trimlines store `TangentIntersection` from birth in the fillet lane (PR 12). |
| 8 | "the die-with-pips fillet demo builds, certifies, tessellates watertight, and exports" | MET-WITH-RECORDED-HONESTY — **ACCEPTED** | Acceptance shape (v), PR 12 (#166). **Each of the four verbs is satisfied — twice, on two bodies that do not compose.** *The blank*: a unit cube with all twelve edges blended at r = 0.12 — 26 faces / 48 edges / 24 vertices, tiers 1-3 green, volume AND surface area on their closed forms to 1e-9 relative with a zero enclosure pad, watertight under `check_mesh`, all 12·4 + 8·3 blend/corner boundary edges carrying `TangentIntersection`, STEP-exported and FreeCAD-imported (valid, 26 faces, volume within 2.6e-7 relative). The first fixture with plane AND cylinder AND sphere faces in one solid, all exact, no B-splines. *The pips*: 21 spherical dimples on all six faces of a sharp cube, cut in ONE certified group operation, tier-3 valid, volume on its closed form, watertight, exported and imported. Corpus documents `die_fillet` and `die_pips` (`the_die_blank_certifies_and_tessellates_watertight`, `crates/sweep/tests/m5_pr12_die.rs:269`, with `volume_pad == 0.0`; `the_pips_cut_in_one_group_operation_on_all_six_faces`, :307). **A second honesty, on "exports"**: only the blank is in the CI-gated STEP fixture corpus (`filleted_die.step`, sidecar pinning 26 faces / 56 edges — OCC splits the periodic corner arcs at their seams — / 24 vertices / 965231000 mm³). **`die_pips` is NOT in that corpus**, so its FreeCAD import is by-hand only, through the tour, whose committed evidence is the `demos/renders-freecad/diepips.png` render; the tour is not run in CI. **The honesty**: the criterion says "the die-with-pips fillet demo", singular. It is two demos. Both compose orderings refuse typed at two DIFFERENT pre-existing frontiers — fillet→pip at the curved-pierce door (no definite-miss certificate for a conic carrier against a curved face; the arm is **unconditional**, not a clearance verdict — the reviewer measured the true clearance of the named pair at 1.6 cm), and pip→fillet at the whole-body assembly front door (the twelve box edges are no longer every edge of the body, and the rebuild does not carry a face's RINGS through). Both doors are pinned as one test, `deviation_1_the_blank_and_the_pips_do_not_compose_yet` (`crates/sweep/tests/m5_pr12_die.rs:348`), and the reviewer independently reproduced them under every reordering. **Carried to: the in-place edge-blend composition-surgery unit** — sized by review at ONE reviewed unit, recommended at the HEAD of the main-path queue, ahead of the SSI lift. **ACCEPTED (Ev, 2026-08-03, PR #169 comment 5171351203): "i approve sequencing it soon in the new M6"** — M5 exits with shape (v) met PIECEWISE, and the composition-surgery unit is sequenced early in M6 (the new main-path-completions milestone). |
| 9 | "every C8 validity predicate has a fixture firing it as a typed pre-construction error" | MET | PR 12 (#166). The battery (`crates/sweep/src/fillet/battery.rs`) reifies six numbered validity predicates and `crates/sweep/tests/m5_pr12_battery.rs` fires each as a typed error BEFORE any construction runs — one fixture per predicate, named for it: P1 `p1_radius_headroom_refuses_on_a_ball_tighter_than_the_blend`, P2 `p2_face_clearance_refuses_when_two_blends_meet_across_a_face` (with `p2_face_clearance_passes_just_under_the_half_side` pinning the other side of the boundary), P3 `p3_spine_regularity_refuses_before_the_torus_is_minted`, P4 `p4_chain_g1_refuses_at_a_cornered_junction`, P5 `p5_convexity_sign_flip_refuses_across_the_notch`, P6 `p6_mixed_convexity_corner_refuses_naming_the_feather_policy` (which also pins the OQ6 vocabulary). `m5_pr12_refusals.rs` adds the two-tolerance trio for every `fillet3_*` — the S9 lesson applied, which is exactly the convention criterion 20 proposes. (`fillet3_chain_arm` is the arm of P4's chain test, not a seventh predicate.) |
| 10 | "`FilletCornerUnsupported` payloads pinned" | MET | PR 12 (#166), with the OQ6 refusal-payload vocabulary. `FilletError::SpineUnsupported` is pinned alongside it and is the front door for the canal-surface case (see criterion 5's sibling frontier). |
| 11 | "sweeps/lofts persist under schema v2 (v1 handling per the R3 rider — migration or typed refusal, whichever the PR 10 spec recorded)" | MET | PR 10 (#151): `Loft`/`Sweep` definitional node vocabulary, §10.3/§10.4 geometry, schema v2. The rider is satisfied by the spec having RECORDED a choice, and it did, explicitly — `docs/archive/M5-PR10-SPEC.md:64-79`: **"The call: CLEAN BREAK, zero live compat code"** (Ev, #148 comment 5148423716, 2026-07-31, superseding the same-day migrate option; chosen because the kernel is unreleased and the only v1 files are in-tree). So the branch taken is **typed refusal**: no `migrate` step is written, a v1 file refuses TYPED with a recourse, every in-tree v1 golden/corpus file was regenerated, and the empty migration-chain mechanism is kept for the future. Pinned by the v1 typed-refusal row and the regenerated-corpus row; hosted `persistence (eps = 1e-6 / 1e-12)` plus the default and Interval lanes. |
| 12 | "curved STEP exports (conics + NURBS) of the R5 corpus shapes import intact into FreeCAD" | MET-WITH-RECORDED-HONESTY | PR 13 (#159). The writer emits `CYLINDRICAL_`/`CONICAL_`/`SPHERICAL_`/`TOROIDAL_SURFACE` and `CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS` as EXACT native AP214 entities — conics deliberately NOT via the rational-quadratic form, because AP214 makes it unnecessary. Every demo-tour body exports and imports into FreeCAD; the hosted `step import (freecad)` row gates it; the narrated curved refusals are gone. **The honesty, two parts.** (a) `B_SPLINE_SURFACE_WITH_KNOTS` is **not implemented** and `Surface::Nurbs` still refuses typed — so "conics + NURBS" is true of CURVES and false of SURFACES. That is deliberate and consistent with criterion 5: no body at rest carries a NURBS face, so the arm would have been an untested path guarded by nothing. It arrives with the loft-assembly unit. (b) A MULTI-shell solid carrying curved geometry refuses (`CurvedShellClassification`) even though every one of its faces has a printer — the outward/void classification's divergence-theorem reduction is a planarity identity with no closed-form curved counterpart. Both are named DESIGN.md frontiers. (c) **"of the R5 corpus shapes" is only partly true.** The CI-gated STEP fixture corpus (`crates/step-export/tests/common/mod.rs:410-429`) holds `cut_cylinder` = R5 (i), `boss_union` = R5 (ii), `filleted_die` = R5 (v)'s blank, plus `notched`, `washer`, `ball`, `cone`, `donut`. **R5 (iii) and (iv) are absent, and R5 (v)'s pips half is absent.** (iii) is absent for the reason criterion 5 gives — no loft body exists; (iv) is a marching fixture with no body at rest; the pips are a corpus document that never joined the STEP fixtures. Emitted kinds, counted off the committed files: `cut_cylinder` → 2 ELLIPSE + 2 CIRCLE + 2 CYLINDRICAL_SURFACE; `donut` → 4 CIRCLE + 2 TOROIDAL_SURFACE; `filleted_die` → 24 CIRCLE + 12 CYLINDRICAL_SURFACE + 8 SPHERICAL_SURFACE. `B_SPLINE_CURVE_WITH_KNOTS` IS emitted and externally validated, but only through a hand-spliced wireframe fixture with no body behind it (`nurbs_wireframe.step` + its FreeCAD probe, which reconstructs the Eq. 7.33 exact quarter circle and would miss by ~8% if weights were dropped). Drift is caught by `committed_fixtures_are_byte_golden`. |
| 13 | "touching curved boolean results refuse typed at the 3′ gate (envelope pinned)" | MET | The touching-refusal envelope, pinned. Undeclared value-equal contact never glues — the M4 PR 5 narrowing extended through the curved lane — and the tour demonstrates both doors live (undeclared → `UndeclaredCoincidence` with the margin in the payload; declared → glued and 3′-certified). |
| 14 | "the BVH differential suite is green (realized ⊇ idealized, bit-equal results) and the M3 boolean-sweep quadratic is retired" | MET | PR 8 (#135): the BVH crate + sweep wiring. The differential suite asserts the realized set contains the idealized one AND that results are bit-equal — the conjunction the criterion names, not just the containment. The quadratic is retired with measured effect: die −29%, corpus −21%; brute force survives only as `SweepStrategy::Idealized`, and every production entry hardcodes `Realized` (`crates/topo/src/boolean/reduce.rs:5-6, 68`; `boolean/ops.rs`). The curved boolean joined the differential lane at PR 9. *Two caveats that do not touch the criterion as written*: one documented divergence exists in the ERROR channel only — a pair with disjoint boxes can still escalate the brute path on a face's INFINITE plane, which the realized path never examines; the value channel is bit-equal, which is what the criterion claims, and the divergence is pinned by `grazing_infinite_plane_divergence_is_exactly_as_documented`. And the edge×edge sweep was never routed through the tree (PR 10 recorded it as deferred consumer #2). |
| 15 | "SSI bit-replay CI rows exist from the first SSI PR onward" | MET-WITH-RECORDED-HONESTY | From PR 7 (#146) onward, in the standard matrix. `shape_iii_bit_replay` (`crates/geom-brep/tests/m5_pr7_ssi.rs:697`) compares `hull_sup`, `on_locus_max` and `tube_transversality` bit patterns plus every control point's x/y/z bits across two runs; the ring lane replays in `crates/geom-core/tests/m5_pr7b_tensor_compose.rs:443-465`. A repo-wide audit found 8 `#[ignore]` attributes and **none in any SSI file** — the rows ride `test`, `test (interval)` and the `test (eps = 1e-6 / 1e-12)` matrix. Continuity holds: no later curved PR merged without them. **The honesty**: the rows are guarded by `fixture_or_return!` / `wall_outcome()` (:184-191, :642-653) — on `SsiError::FitSampleBudget` the test RETURNS GREEN without asserting. The budget refusal is itself pinned typed (`the_fit_sample_budget_refuses_typed_rather_than_grinding`, :194) and the funnel-coverage row conditions on the budget outcome, so the design is honest — but **"the row is green at every ε" is not "the replay ran at every ε"**, and nothing currently reports which it was. |
| 16 | "the interval backend swap is complete with the M0 poison contract intact and no LGPL dependency in any build configuration (quarantine text retired)" | MET | PR 1 (#127, 2026-07-28). The backend is the in-house `interval-transcendentals` crate — proven per-function libm error pads (4-ulp transcendental, 1-ulp arithmetic with exactness witnesses for sqrt/mul/div), MPFR-differential-certified, libm-only, D9-clean. **inari and its gmp/MPFR stack are gone from the tree, not re-quarantined**: Cargo.lock zero hits, dev-dependencies included, so the kernel is copyleft-free in EVERY build configuration and issue #4's exit condition is met by removal. inari survives only as an optional differential oracle inside the excluded crate's own workspace. The M0 poison contract is intact (and the interval-square poison rule survived its own retirement unit, #153). Quarantine text: DESIGN.md carries only the Tabled tombstone and the crate-table history — verified this sweep. CURVED-DESIGN.md's design-time quarantine language is historical record and gained a superseding status block rather than a rewrite. |
| 17 | "REST-contact crosslap certifies through its join lane" | MET | Side unit S1 (#140). The crosslap mate's pure PLANAR rest contact zips through a declared-contact join lane at exact volume (1.875), both doors pinned in `crosslap_rest.rs`, and the M3-era tripwire is retired. The fix pass went deeper than the wire's own story: a silent corrupt-STL hole-creating merge role inversion was found, root-caused at the merge base, and corrected via Newell winding, adding a NEW tier-3 loop-role gate that filled a documented deferral. |
| 18 | "arc-leg fillet sugar ships" | MET | Side unit S2 (#137). `LoopBuilder::fillet` grows arc-leg corners under the same declared-tangency discipline, with fit gating extended; 20k-corner review fuzz produced zero wrong circles. S8 (#143) then landed the nearest-corner selection ladder over it, whose rung 3 is the project's first knowingly-designed equivariance residual — documented, per the convention this sweep proposes. |
| 19 | "the M5 exit K-telemetry snapshot over the curved corpus is taken and the #89 decision is recorded (or explicitly continued with grounds)" | MET (decision recorded: **closed**) | This PR. Snapshot: `docs/K-REPORT.md` "M5 addendum", raw rows `docs/k-report-data/m5-eps-*.csv.gz`, ~1.76M samples per ε row over 13 corpus documents + 17 tour scenes, reproducing the hosted `k-lint (advisory)` row to the sample. The #89 decision is **RECORDED, not continued** — the criterion's FIRST limb, the stronger one. **#89 is CLOSED and K = 10 is the permanent ratified default** (Ev, 2026-08-03, PR #169 comment 5171303851: "closing 89 makes sense"), with a testable re-open trigger: any corpus showing IN-BAND LANDINGS, whose expected first source is the M7 import corpus, and whose standing detector is the `k-lint` advisory row's rule 1. Grounds: zero in-band landings anywhere; the first evidence with discriminating power argues specifically against RAISING K (K=100 retains only 1.65× clearance to the ε-coupled quadrature family, K=30 only 5.5×, against K=10's 16.5×); and the computed-SSI evidence Finding 4 named still has not arrived, while #161 puts the import corpus in the NEXT milestone. The addendum records the close and the trigger; the orchestrator closes the issue citing it once this PR merges. |
| 20 | "new conventions ratified into DESIGN.md at exit" | MET | **RATIFIED** (Ev, 2026-08-03, PR #169 comment 5171303851: "the three amendments (two-tolerance principle, equivariance, distance-only tesselation) sound good to me also"). Proposed by this PR following the M4 8c precedent of proposing rather than self-ratifying, and ratified on the same comment that closed #89: (i) the two-tolerance principle's consequence (iv) — the rule binds a predicate's DEFINITE arms too, earned by S9's review MIN-1; (ii) semantic equivariance where it is free, carried in **with its premise-unaudited caveat intact**, because the caveat is the load-bearing half of the memory it comes from; (iii) the tessellation ruling, quoted verbatim into D4's chordal-tolerance paragraph. The word the criterion uses is "ratified", and ratification is Ev's; he gave it, so the row reads MET. |

## Tally

| disposition | count | rows |
|---|--:|---|
| MET | 13 | 1, 3, 7, 9, 10, 11, 13, 14, 16, 17, 18, 19, 20 |
| MET-WITH-RECORDED-HONESTY | 7 | 2, 4, 5, 6, 8, 12, 15 |
| CARRIED | 0 | — |

**No criterion is carried unowned.** Row 20 flipped to MET on Ev's
ratification; every remaining engineering item that a criterion's
honesty note defers now has a **named M6 owner** (below).

Seven honesty rows out of twenty is a high proportion, and that is
the point of having the category: each one is a criterion whose
*substance* M5 delivered and whose *wording* claims more than the
shipped kernel supports. Four of the seven (2, 4, 12, 15) were found
only by re-deriving the evidence at exit rather than trusting the
merge records — rows that would have read MET on a lighter walk.

**Nothing is silently skipped**: twenty criteria in the plan
paragraph, twenty rows above.

## The carried list, in full

Every item below has a named owner in **M6**, the main-path
completions milestone created by the 2026-08-03 renumbering (Ev,
PR #169 comment 5171303851). Nothing on this list is unowned.

1. **Row 8 — shape (v)'s composed die** → the **in-place edge-blend
   composition-surgery unit, sequenced EARLY in M6** (Ev, PR #169
   comment 5171351203: "i approve sequencing it soon in the new M6").
   M5 exits with shape (v) met piecewise; review sized the surgery at
   one reviewed unit.
2. **Row 5 — shape (iii)'s full loft BODY** → the **loft/sweep body
   assembly unit, in M6**, sequenced after the SSI generic-`T` lift
   (#161, renumbered 2026-08-03). The substrate row is met and was
   the exit gate; the body is the honest complete form.
3. **Row 12 — NURBS SURFACE STEP export** (`B_SPLINE_SURFACE_WITH_KNOTS`)
   → arrives with the loft-assembly unit, which mints the first
   NURBS face at rest. Its sibling, the multi-shell curved
   outward/void classification, is a named DESIGN.md frontier with
   no scheduled unit.
4. **Row 4 — pcurve certification on non-cylinder charts** → the
   **loft/sweep body assembly unit** (owner assigned at this PR's
   review). Cone, sphere, torus and NURBS charts mint nothing and
   refuse `UnsupportedChart`; the routing is a compile-time
   decision, and the filleted die's own eight sphere octants are
   affected. The assembly unit is the natural owner because it
   already has to mint pcurves for a NURBS face at rest — completing
   the ANALYTIC charts alongside it is the sibling half of the same
   work, not a separate lane. Added to that unit's banked scope in
   DESIGN.md's frontier entry (b)/(c).
5. **Row 12 — STEP fixture coverage of R5 (iii), (iv) and the
   pips** → the **montage-curation unit** (queued, block 20) for the
   pips half. (iii) and (iv) follow their own frontiers and stay
   with the assembly unit. `die_pips` is a shipped corpus document
   whose STEP export is verified only by hand through the tour;
   adding the fixture + expect file to the CI-gated corpus is a
   trivial, demo-adjacent addition and belongs with the curation
   work rather than with kernel geometry.
6. **Row 2 — a non-vacuous fitted-cache-at-rest row** → the **SSI
   generic-`T` lift unit's acceptance** (owner assigned at this PR's
   review). PR 9's spec asked for a cylinder×sphere rung-3-at-rest
   row and it does not exist, so the C2-certificate clause has never
   been exercised positively. The lift is exactly where
   `Pcurve::Fitted` first reaches a body at rest, so the row becomes
   available — and load-bearing — there; it is recorded as an
   acceptance obligation of that unit, not a follow-up to it.

## Beyond the criteria: obligations this walk surfaced

These are not exit criteria and do not gate the milestone. They are
recorded because an exit walk that only checks its own list is the
failure mode the walk exists to prevent.

1. **The large-K lint's baseline floor is stale.** Its
   `BASELINE_FLOOR_MARGIN = 1.5e-3` was the P0 of the M4
   distribution; the M5 distribution sits under it and the hosted
   advisory row prints 102 flags every run. The lint's own charter
   says "gate once the baseline is trusted" — it cannot be gated in
   this state — and #89's re-open trigger now leans on its rule-1
   flag, which raises the stakes on keeping the advisory output
   readable. Named **M6** code pickup; deliberately not taken in a
   docs-only unit. (K-REPORT M5 addendum, Finding M5-2.)
2. **No SSI margin has ever been measured.** Fourteen `ssi_*`
   predicates exist; none sample. Named **M6** code pickup — that is
   where the SSI lift and loft assembly put marched geometry into a
   body at rest: a Band-4 document whose boolean genuinely requires
   marching, or a `Probe` instantiation of the SSI suites.
   (Finding M5-3.)
3. **The K-REPORT's ε-stability claim is retired** and should not be
   restated: decision COUNTS now differ across ε rows because
   `props_quad_converged` is a convergence-loop stopping test.
   (Finding M5-1.)
4. **`docs/archive/M5-LOG.md` diverged between main and the orchestrator
   branch**, and neither copy was a superset. **RESOLVED**:
   `MODEL-AB-LOG.md` was reconciled by this PR (the M5-close readout
   needed it), and the M5-LOG reconciliation landed as #168 and was
   folded into this branch, tail resolved keep-both in chronological
   order. The fold surfaced one numbering conflict — the merged log
   calls the CI-shard unit "A/B row 41" while the table had no such
   row — now closed by numbering rows 41 and 42 and marking them
   explicitly non-comparable (42 dispatches, n = 40 for the
   comparison).
5. **A stale comment asserts the opposite of the shipped truth**:
   `crates/editor-core/tests/corpus/mod.rs:130` registers
   `die_fillet::document()`, while the comments at :132-139 AND
   :196-199 still say it is NOT registered and explain why. The gate
   fix at `5c8540f` moved it into the registry and neither comment
   followed. A code change, therefore reported and not made here —
   **assigned to the montage-curation unit** (block 20) alongside
   the `die_pips` STEP fixture, since both are small
   corpus/demo-adjacent corrections.
6. **Two M5 limitations are latent-and-loud** and now live in
   DESIGN.md's envelope so they are not rediscovered as bugs: a
   meridian-tangent circle is in-lane but uncertifiable (no
   constructor mints one; the refusal is loud), and genuinely-oblique
   trihedral corners build through tiers 1-2 and then report
   `VolumeUncomputable` — a gap in the props inventory, not in the
   body.

7. **The milestone numbers changed at this exit** (Ev, PR #169
   comment 5171303851, 2026-08-03): the old M6 (error propagation)
   is now **M8**; **M6** is a new milestone holding the main-path
   curved completions this walk carries items to; **M7 is STEP
   adoption only** — core kernel work that import merely wants
   (curved REST contact / ball-and-socket) stays at M6, design-only.
   Live docs were renumbered in this PR; `docs/ERROR-DESIGN.md` and
   `docs/archive/M6-BOUNDARY.md` keep their bodies and gained status blocks
   instead, and the M1-M5 logs, plans and specs are historical
   record and were left untouched.

## Q9 note

Name still open (Ev's call; #107 shortlist). M5 did not gate on it,
and this walk does not either.

## Appendix (2026-08-05): the M5 roadmap done-state bullet as it
## stood in DESIGN.md before the docs-rot compression (verbatim;
## note the S-unit list was completed to S10–S13 and the shape-(v)
## closure annotated at M6 before relocation)

- **M5** — NURBS depth (sweeps/lofts); first SSI marching; constant-radius
  fillets. Design record ratified: `docs/CURVED-DESIGN.md` (#85,
  2026-07-24). *(Done-state recorded at the PR 14 exit sweep,
  2026-08-03 — walk: `docs/M5-EXIT-WALK.md`. **Shipped**: the
  interval-crate swap retiring inari and its LGPL stack from the tree
  (#127); the C9 interval ring + projection/fitting/LSQ substrate
  (#130, and PR 4); NURBS substrate parts 1–2; exact `Ellipse`
  carriers with the C5 dispatch table (#141); SSI marching with the
  three-limb certificate (#146) and its tensor-compose follow-on
  (#149); certified pcurve storage in meters, seams with distinct
  pcurves (#144); the BVH crate + sweep wiring, retiring the M3
  boolean-sweep quadratic (#135); per-class curved booleans — plane×
  cylinder, then plane×sphere — with the tangency regime and typed
  touching refusals (#152, #154, #158, #164); `Loft`/`Sweep`
  definitional nodes + schema v2 (#151); certified tessellation and
  quadrature-based mass properties (#157); face-orientation sense
  fixes (#155, #156); curved AP214 STEP export with FreeCAD
  acceptance (#159); constant-radius fillets — cylinder bands, torus
  rims, sphere-octant corners — and the die (#166); side units S1,
  S2, S4, S6, S7, S8, S9, S10, S11, S12, S13. **The envelope moved,
  not vanished**: seven
  units are BANKED by name, with their doors typed and pinned —
  composition surgery, the SSI generic-`T` lift, loft/sweep body
  assembly, the canal-surface blend, cyl×sphere germ chords, the
  NURBS extent lift, and curved REST contact (see the frontier
  entries below). **Acceptance shape (v), honestly**: the
  die-with-pips ships as TWO bodies — a fully blended blank and a
  21-pip die, each tier-3 valid, watertight, and STEP-exported — and
  they do not compose at M5. Both orderings refuse typed at two
  different pre-existing frontiers (fillet→pip at the curved-pierce
  door, which is unconditional and not a clearance verdict; pip→
  fillet at the whole-body assembly front door). The in-place
  edge-blend surgery that closes them is sized at one reviewed unit
  and banked at the head of the main-path queue. Shape (v) is
  therefore recorded **met piecewise**, not met whole. *(CLOSED at
  M6 unit 1: the surgery landed and THE COMPOSED DIE — blank + 21
  pips + 21 rim tori — is one tier-3 body with a certified
  closed-form volume; the M5 pin flipped with its history,
  `m5_pr12_die.rs::deviation_1_flipped_*`.)*
  **Sequencing**: #161 ratified the boundary and the 2026-08-03
  renumbering gave it names — M5 exit → **M6** (SSI generic-`T` lift
  → loft/sweep assembly → composition surgery → analytic-chart
  pcurves, plus the census/declared-contact design doc) → **M7**
  (STEP adoption only) → **M8** (error propagation, formerly M6).)*
  Banked M5 openers from the M4 exit (8c, 2026-07-27):
  **curved STEP subset** (banked planar-only; DISCHARGED at M5 PR 13 —
  the writer now emits `CYLINDRICAL_`/`CONICAL_`/`SPHERICAL_`/
  `TOROIDAL_SURFACE` and `CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS`
  as EXACT native AP214 entities, conics deliberately NOT via the
  rational-quadratic form the schema makes unnecessary; every demo-tour
  body exports and imports into FreeCAD, so the narrated curved
  refusals are gone. Two frontiers remain named: a NURBS FACE, which
  the loft-assembly unit mints, and the outward/void classification of
  a MULTI-shell curved solid, whose divergence-theorem reduction is a
  planarity identity with no closed-form curved counterpart);
  **arc-leg
  fillet sugar** (#101 R4 scoped `LoopBuilder::fillet` to line/line
  corners; arc-leg is the noted follow-up, see #104); **REST-contact
  join lane** (the crosslap mate is a pure rest contact — M3 envelope
  frontier, `crosslap_rest.rs` pins both doors; banked at #102 R7);
  **#89 K-revisit at the M5 exit** — **TAKEN (PR 14, 2026-08-03;
  K-REPORT "M5 addendum")**: ≈1.76M samples/ε-row over the curved
  corpus + demos, still zero in-band landings and zero
  indeterminate/invalid at every ε row, so no candidate K converts
  any decision. But the surface is no longer FLAT: the ε-coupled
  quadrature family `props_quad_converged` puts real definite
  margins at ~1.6e2–8.4e2 × ε, leaving K = 100 only 1.65× of
  clearance and K = 30 only 5.5×, against K = 10's 16.5×. The
  outcome is **#89 CLOSED, K = 10 the permanent ratified default**
  (Ev, PR #169 comment 5171303851, 2026-08-03), with a testable
  re-open trigger: any corpus showing IN-BAND LANDINGS, whose
  expected first source is the M7 import corpus. The computed-SSI
  evidence Finding 4 named still has not arrived (no `ssi_*`
  predicate samples; M5's curved booleans resolve through exact
  analytic carriers), and the renumbering puts that corpus at M7 —
  but three corpora now agree, and the close carries a testable
  trigger rather than waiting on evidence that keeps not arriving.
  The `k-lint` advisory row's rule 1 is the standing detector.
  Two code follow-ups are named and deliberately not taken in the
  docs-only exit unit: re-deriving the large-K lint's stale
  `BASELINE_FLOOR_MARGIN` (the M5 distribution sits under it — 102
  advisory flags per hosted run), and a `Probe` lane that actually
  reaches the SSI marcher; **interval-crate adoption decision** — the
  in-house `interval-transcendentals` crate (adoption GREEN-LIT, see crate table) exists as
  workspace-excluded tooling (#115); adopting it in the kernel's
  interval lane is an M5-PLAN ratified decision, not a default.

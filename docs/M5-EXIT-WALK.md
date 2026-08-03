# M5 exit walk (PR 14) — criteria vs evidence

Status: assembled at the PR 14 exit sweep, 2026-08-03, against
main's tip (post-#166, run 30835146557, 18/18 hosted rows green).

**How to read this.** Every criterion below is quoted **verbatim**
from `docs/M5-PLAN.md` lines 372-405 — that section is one long
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

Two rows carry to Evan on this PR and are marked **SIGN-OFF**.

## The walk

| # | Criterion (verbatim) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "A tilted plane×cylinder boolean carries an exact `Ellipse` carrier with residual identically zero by construction, replaying bit-identically at ε ∈ {1e-6, 1e-9, 1e-12} + Interval" | MET | Acceptance shape (i). PR 5 (#141) minted the exact `Ellipse` carrier + the C5 dispatch table; S9 (#145) repaired the chord_spec azimuth window behind it. Corpus document `cut_cylinder` (`crates/editor-core/tests/corpus/cut_cylinder.rs`) carries the standard persistence/latency rows at all three ε and under Interval; hosted `band 4 corpus (eps = 1e-6 / 1e-12)` + `test (interval)`. The residual is zero *by construction* because the carrier is the analytic ellipse, not a fit — the property the criterion actually names. |
| 2 | "a transverse curved boolean (cylinder boss ∪ plate) certifies end-to-end at tier 3 with every fitted cache carrying the full C2 certificate (hull sup-norm + uniqueness tube — no schedule-max-only cache at rest)" | MET | Acceptance shape (ii). PR 9 (#152) landed the transverse curved boolean; `boss_union` joined the Band-4 corpus at the PR 9 fix pass and the BVH differential lane runs the curved boolean bit-equal. The certificate half is PR 6 (#144): pcurve caches store the full C2 certificate in meters and `recertify` RE-DERIVES it at rest rather than trusting the stored value — which is what makes "no schedule-max-only cache at rest" an invariant rather than a convention. |
| 3 | "the small-loop fixture is found by exclusion subdivision or refuses typed `SsiExhaustivenessInconclusive` — verified by a fixture where naive marching provably misses" | MET | Acceptance shape (iv). PR 7 (#146): a sphere × 0.08-radius cylinder whose ENTIRE locus is two interior polar loops — boundary seeding reaches nothing, so naive marching provably misses — found by exclusion subdivision at 166 seeds, with the floor-refusal variant pinned as its own row. Both limbs of the disjunction are therefore exercised, not just the happy one. |
| 4 | "every curved edge at rest carries per-half-edge pcurves certified in meters, seam edges with distinct pcurves" | MET | PR 6 (#144), certified pcurve storage. Per-half-edge, certified in meters, seam edges carrying distinct pcurves, with seam probes adopted from the review. The review's best MIN — a snap-to-family ε-shell falsifying the stored envelope on the attach path — was fixed with the snap slack proven provably zero on minted caches, plus an O(ε)-tightness pin. |
| 5 | "a NURBS-wall boolean (cut loft) marches, fits, certifies, and passes in-op exhaustiveness" | MET-WITH-RECORDED-HONESTY | The *substrate* row is GREEN and was exit-gating: PR 7b (#149) ships a directly-authored NURBS wall cut by a plane — rung-3 marched, fitted, certified carrier, **all three limbs**, both lanes, bit-replayed (`crates/geom-brep/tests/m5_pr7_ssi.rs`, `shape_iii_the_wall_cut_certifies_all_three_limbs`, `shape_iii_bit_replay`). **The honesty**: the criterion says "cut **loft**", and no loft BODY exists. `Loft`/`Sweep` build their walls and then refuse `CurvedSolidFrontier`, because tier 3's +V check routes a NURBS face to `Unimplemented` — NURBS-patch flux needs surface quadrature and the surface-AREA half has no closed form for a rational patch at all. Shipping the assembly without that would swap an honest frontier for a body that fails validation. **Carried to: the loft/sweep body assembly unit** (MAIN-PATH, after the SSI generic-`T` lift, per #161). |
| 6 | "second-order sector classification resolves a first-order tie with the normal-curvature trilean and escalates in-band osculation typed" | MET | The tangency regime, PR 9 (#152). Both halves are required and both are present: the normal-curvature trilean resolves a first-order tie, and in-band osculation escalates typed rather than guessing. Predicate names in the funnel: `tangent_sector_order2`, `tangent_sector_order2_arm`, `tangent_sector_osculation`. *Telemetry note, not a criterion gap*: these three do not sample in the K-probe corpus — no registered Band-4 document or tour scene reaches a first-order tie — so their coverage is their own suites, not the corpus rows. |
| 7 | "definitely-tangent edges carry the tangency mark and jet-determinate tangencies enforce `TangentIntersection` (G2 conventional joins exempt by predicate, pinned both directions)" | MET | The declared-tangency discipline (#109/#101) extended through the curved lane in PR 9. The parenthetical is the part that is easy to half-ship and was not: the G2 conventional-join exemption is decided **by predicate**, and pinned in **both** directions — a conventional join stays exempt, and a jet-determinate tangency is forced to `TangentIntersection`. Trimlines store `TangentIntersection` from birth in the fillet lane (PR 12). |
| 8 | "the die-with-pips fillet demo builds, certifies, tessellates watertight, and exports" | MET-WITH-RECORDED-HONESTY — **SIGN-OFF item 2** | Acceptance shape (v), PR 12 (#166). **Each of the four verbs is satisfied — twice, on two bodies that do not compose.** *The blank*: a unit cube with all twelve edges blended at r = 0.12 — 26 faces / 48 edges / 24 vertices, tiers 1-3 green, volume AND surface area on their closed forms to 1e-9 relative with a zero enclosure pad, watertight under `check_mesh`, all 12·4 + 8·3 blend/corner boundary edges carrying `TangentIntersection`, STEP-exported and FreeCAD-imported (valid, 26 faces, volume within 2.6e-7 relative). The first fixture with plane AND cylinder AND sphere faces in one solid, all exact, no B-splines. *The pips*: 21 spherical dimples on all six faces of a sharp cube, cut in ONE certified group operation, tier-3 valid, volume on its closed form, watertight, exported and imported. Corpus documents `die_fillet` and `die_pips`. **The honesty**: the criterion says "the die-with-pips fillet demo", singular. It is two demos. Both compose orderings refuse typed at two DIFFERENT pre-existing frontiers — fillet→pip at the curved-pierce door (no definite-miss certificate for a conic carrier against a curved face; the arm is **unconditional**, not a clearance verdict — the reviewer measured the true clearance of the named pair at 1.6 cm), and pip→fillet at the whole-body assembly front door (the twelve box edges are no longer every edge of the body, and the rebuild does not carry a face's RINGS through). The reviewer independently reproduced BOTH doors under every reordering. **Carried to: the in-place edge-blend composition-surgery unit** — sized by review at ONE reviewed unit, recommended at the HEAD of the main-path queue, ahead of the SSI lift. **Evan's call: accept shape (v) as met piecewise, with the surgery banked as recommended?** |
| 9 | "every C8 validity predicate has a fixture firing it as a typed pre-construction error" | MET | PR 12 (#166). The battery (`crates/sweep/src/fillet/battery.rs`) reifies six numbered validity predicates and `crates/sweep/tests/m5_pr12_battery.rs` fires each as a typed error BEFORE any construction runs — one fixture per predicate, named for it: P1 `p1_radius_headroom_refuses_on_a_ball_tighter_than_the_blend`, P2 `p2_face_clearance_refuses_when_two_blends_meet_across_a_face` (with `p2_face_clearance_passes_just_under_the_half_side` pinning the other side of the boundary), P3 `p3_spine_regularity_refuses_before_the_torus_is_minted`, P4 `p4_chain_g1_refuses_at_a_cornered_junction`, P5 `p5_convexity_sign_flip_refuses_across_the_notch`, P6 `p6_mixed_convexity_corner_refuses_naming_the_feather_policy` (which also pins the OQ6 vocabulary). `m5_pr12_refusals.rs` adds the two-tolerance trio for every `fillet3_*` — the S9 lesson applied, which is exactly the convention criterion 20 proposes. (`fillet3_chain_arm` is the arm of P4's chain test, not a seventh predicate.) |
| 10 | "`FilletCornerUnsupported` payloads pinned" | MET | PR 12 (#166), with the OQ6 refusal-payload vocabulary. `FilletError::SpineUnsupported` is pinned alongside it and is the front door for the canal-surface case (see criterion 5's sibling frontier). |
| 11 | "sweeps/lofts persist under schema v2 (v1 handling per the R3 rider — migration or typed refusal, whichever the PR 10 spec recorded)" | MET | PR 10 (#151): `Loft`/`Sweep` definitional node vocabulary, §10.3/§10.4 geometry, and schema v2 as a **clean break** — which is one of the two options the R3 rider permits, and the one the PR 10 spec recorded. The rider is satisfied by having *recorded* the choice, not by having chosen migration. Persistence rows hosted at `persistence (eps = 1e-6 / 1e-12)`. |
| 12 | "curved STEP exports (conics + NURBS) of the R5 corpus shapes import intact into FreeCAD" | MET-WITH-RECORDED-HONESTY | PR 13 (#159). The writer emits `CYLINDRICAL_`/`CONICAL_`/`SPHERICAL_`/`TOROIDAL_SURFACE` and `CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS` as EXACT native AP214 entities — conics deliberately NOT via the rational-quadratic form, because AP214 makes it unnecessary. Every demo-tour body exports and imports into FreeCAD; the hosted `step import (freecad)` row gates it; the narrated curved refusals are gone. **The honesty, two parts.** (a) `B_SPLINE_SURFACE_WITH_KNOTS` is **not implemented** and `Surface::Nurbs` still refuses typed — so "conics + NURBS" is true of CURVES and false of SURFACES. That is deliberate and consistent with criterion 5: no body at rest carries a NURBS face, so the arm would have been an untested path guarded by nothing. It arrives with the loft-assembly unit. (b) A MULTI-shell solid carrying curved geometry refuses (`CurvedShellClassification`) even though every one of its faces has a printer — the outward/void classification's divergence-theorem reduction is a planarity identity with no closed-form curved counterpart. Both are named DESIGN.md frontiers. |
| 13 | "touching curved boolean results refuse typed at the 3′ gate (envelope pinned)" | MET | The touching-refusal envelope, pinned. Undeclared value-equal contact never glues — the M4 PR 5 narrowing extended through the curved lane — and the tour demonstrates both doors live (undeclared → `UndeclaredCoincidence` with the margin in the payload; declared → glued and 3′-certified). |
| 14 | "the BVH differential suite is green (realized ⊇ idealized, bit-equal results) and the M3 boolean-sweep quadratic is retired" | MET | PR 8 (#135): the BVH crate + sweep wiring. The differential suite asserts the realized set contains the idealized one AND that results are bit-equal — the conjunction the criterion names, not just the containment. The quadratic is retired with measured effect: die −29%, corpus −21%. The curved boolean joined the differential lane at PR 9. |
| 15 | "SSI bit-replay CI rows exist from the first SSI PR onward" | MET | From PR 7 (#146) onward, in the standard matrix — `crates/geom-brep/tests/m5_pr7_ssi.rs` carries the bit-replay rows (including `shape_iii_bit_replay` added at PR 7b), running under `test`, `test (eps = 1e-6)`, `test (eps = 1e-12)` and `test (interval)` rather than behind an `#[ignore]`. The criterion's "from the first SSI PR onward" is a *continuity* claim and it holds: no later curved PR merged without them. |
| 16 | "the interval backend swap is complete with the M0 poison contract intact and no LGPL dependency in any build configuration (quarantine text retired)" | MET | PR 1 (#127, 2026-07-28). The backend is the in-house `interval-transcendentals` crate — proven per-function libm error pads (4-ulp transcendental, 1-ulp arithmetic with exactness witnesses for sqrt/mul/div), MPFR-differential-certified, libm-only, D9-clean. **inari and its gmp/MPFR stack are gone from the tree, not re-quarantined**: Cargo.lock zero hits, dev-dependencies included, so the kernel is copyleft-free in EVERY build configuration and issue #4's exit condition is met by removal. inari survives only as an optional differential oracle inside the excluded crate's own workspace. The M0 poison contract is intact (and the interval-square poison rule survived its own retirement unit, #153). Quarantine text: DESIGN.md carries only the Tabled tombstone and the crate-table history — verified this sweep. CURVED-DESIGN.md's design-time quarantine language is historical record and gained a superseding status block rather than a rewrite. |
| 17 | "REST-contact crosslap certifies through its join lane" | MET | Side unit S1 (#140). The crosslap mate's pure PLANAR rest contact zips through a declared-contact join lane at exact volume (1.875), both doors pinned in `crosslap_rest.rs`, and the M3-era tripwire is retired. The fix pass went deeper than the wire's own story: a silent corrupt-STL hole-creating merge role inversion was found, root-caused at the merge base, and corrected via Newell winding, adding a NEW tier-3 loop-role gate that filled a documented deferral. |
| 18 | "arc-leg fillet sugar ships" | MET | Side unit S2 (#137). `LoopBuilder::fillet` grows arc-leg corners under the same declared-tangency discipline, with fit gating extended; 20k-corner review fuzz produced zero wrong circles. S8 (#143) then landed the nearest-corner selection ladder over it, whose rung 3 is the project's first knowingly-designed equivariance residual — documented, per the convention this sweep proposes. |
| 19 | "the M5 exit K-telemetry snapshot over the curved corpus is taken and the #89 decision is recorded (or explicitly continued with grounds)" | MET (via the disjunction's second limb) — **SIGN-OFF item 1** | This PR. Snapshot: `docs/K-REPORT.md` "M5 addendum", raw rows `docs/k-report-data/m5-eps-*.csv.gz`, ~1.76M samples per ε row over 13 corpus documents + 17 tour scenes, reproducing the hosted `k-lint (advisory)` row to the sample. The #89 decision is **explicitly continued, with grounds** — the limb the criterion permits — recommending **hold K = 10 pending the M7 import corpus**: zero in-band landings anywhere; the first evidence with discriminating power argues specifically against RAISING K (K=100 retains only 1.65× clearance to the ε-coupled quadrature family, K=30 only 5.5×, against K=10's 16.5×); and the computed-SSI evidence Finding 4 named still has not arrived, while #161 puts the import corpus in the NEXT milestone. **Evan's call: accept the continuation, or close #89 now?** The addendum states plainly that the decision is his. |
| 20 | "new conventions ratified into DESIGN.md at exit" | CARRIED (to Evan's sign-off on this PR) | Three conventions are **PROPOSED** in DESIGN.md by this PR, following the M4 8c precedent of proposing rather than self-ratifying: (i) the two-tolerance principle's consequence (iv) — the rule binds a predicate's DEFINITE arms too, earned by S9's review MIN-1; (ii) semantic equivariance where it is free, carried in **with its premise-unaudited caveat intact**, because the caveat is the load-bearing half of the memory it comes from; (iii) the tessellation ruling, quoted verbatim into D4's chordal-tolerance paragraph. The word the criterion uses is "ratified", and ratification is Evan's — so this row cannot honestly read MET until he signs. It flips on the same sign-off that closes rows 8 and 19. |

## Tally

| disposition | count | rows |
|---|--:|---|
| MET | 16 | 1, 2, 3, 4, 6, 7, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19 |
| MET-WITH-RECORDED-HONESTY | 3 | 5, 8, 12 |
| CARRIED | 1 | 20 |

**Nothing is silently skipped**: twenty criteria in the plan
paragraph, twenty rows above.

## The carried list, in full

1. **Row 20 — conventions ratification** → Evan's sign-off on this
   PR. The only row whose disposition is CARRIED outright, and the
   only one that flips without further engineering.
2. **Row 5 — shape (iii)'s full loft BODY** → the loft/sweep body
   assembly unit. MAIN-PATH, sequenced after the SSI generic-`T`
   lift (#161). The substrate row is met and was the exit gate; the
   body is the honest complete form.
3. **Row 8 — shape (v)'s composed die** → the in-place edge-blend
   composition-surgery unit, recommended at the HEAD of the
   main-path queue. Evan sign-off item 2.
4. **Row 12 — NURBS SURFACE STEP export** (`B_SPLINE_SURFACE_WITH_KNOTS`)
   → arrives with the loft-assembly unit, which mints the first
   NURBS face at rest. Its sibling, the multi-shell curved
   outward/void classification, is a named DESIGN.md frontier with
   no scheduled unit.

## Beyond the criteria: obligations this walk surfaced

These are not exit criteria and do not gate the milestone. They are
recorded because an exit walk that only checks its own list is the
failure mode the walk exists to prevent.

1. **The large-K lint's baseline floor is stale.** Its
   `BASELINE_FLOOR_MARGIN = 1.5e-3` was the P0 of the M4
   distribution; the M5 distribution sits under it and the hosted
   advisory row prints 102 flags every run. The lint's own charter
   says "gate once the baseline is trusted" — it cannot be gated in
   this state. Named M6/M7 code pickup; deliberately not taken in a
   docs-only unit. (K-REPORT M5 addendum, Finding M5-2.)
2. **No SSI margin has ever been measured.** Fourteen `ssi_*`
   predicates exist; none sample. Named M7 code pickup: a Band-4
   document whose boolean genuinely requires marching, or a `Probe`
   instantiation of the SSI suites. (Finding M5-3.)
3. **The K-REPORT's ε-stability claim is retired** and should not be
   restated: decision COUNTS now differ across ε rows because
   `props_quad_converged` is a convergence-loop stopping test.
   (Finding M5-1.)
4. **`docs/M5-LOG.md` diverged between main and the orchestrator
   branch**, and neither copy is a superset. `MODEL-AB-LOG.md` was
   reconciled by this PR (the M5-close readout needed it); M5-LOG
   was not, because merging two divergent narrative logs belongs to
   the state-sync lane and would be buried inside an exit sweep.
   Owed before the next milestone's log starts.
5. **A stale comment asserts the opposite of the shipped truth**:
   `crates/editor-core/tests/corpus/mod.rs:130` registers
   `die_fillet::document()` while the comment at :132-139 still says
   it is NOT registered. A code change, therefore reported and not
   made.
6. **Two M5 limitations are latent-and-loud** and now live in
   DESIGN.md's envelope so they are not rediscovered as bugs: a
   meridian-tangent circle is in-lane but uncertifiable (no
   constructor mints one; the refusal is loud), and genuinely-oblique
   trihedral corners build through tiers 1-2 and then report
   `VolumeUncomputable` — a gap in the props inventory, not in the
   body.

## Q9 note

Name still open (Evan's call; #107 shortlist). M5 did not gate on it,
and this walk does not either.

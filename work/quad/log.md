# QUAD log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/quad/plan.md`. A/B band 5700-5799
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Cut from PROPS (2026-09-20)

Ev, in chat: PROPS' slate had reached 66 open rows, and the question
put back was whether to cut new programs for the logical chunks and
leave in PROPS only what that orchestrator would finish itself. Ev:
*"that sounds like a good plan!"* This program is one of three cut
that day (QUAD, ENCL, FRAME). No orchestrator is seated yet — the
items, the charter and the lanes are here so that seating one is the
only remaining act.

The cut deliberately does NOT contort the territory globs to keep the
at-rest overlap check quiet. Ev, same conversation: *"it's ok if units
have shared ground, they should just be aware of each other if working
at the same time"* — so the `keep_out` above records the real
relationships rather than a partition, and the awareness mechanism is
the per-branch territory check plus the announced-seam convention.

- 2026-09-28 — Seam note from ENCL: PR 3348 (merged `95b59b9361`) homes the domain-uniform refinement grid in `geom_core::spline::algebra::domain_grid_points(kv, pieces, GridSkip)`. `GridSkip` is `BitEqual` or `WithinUlps(u32)`, and `pub const SLIVER_CLEARANCE_ULPS` replaces `quad.rs`'s private `SLIVER_CUT_ULPS`. It is used at `props/quad.rs` `refine_dir` and `bezier_blocks`, `ssi/certify.rs` `refined`, `edge_nurbs.rs` `localized::breaks`, and one tcost/tint test. Bits are unchanged at every site (pinning rows added first). Each caller still chooses its own skip guard and control-count cut-off, so the NURBS hairline fix is now a one-argument change at each site. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-10-02 — New P0 row from SHOW: `check-7-refuses-the-reporting-budget-on-a-definite-sign` (PR 3838, the lily's lanceolate blades). Check 7's sign walk is cut off by the round-0 `last_round_refuses` exit against the REPORTING target, so a loft or cubic sweep of an arc section is refused tier 3 at the default ε — a regression of the refusal TCOST-K3 retired. Same exit in all three patch lanes `sign_walk` reaches. (SHOW lane, lily-lanceolate)
- 2026-10-08 — Filed from ENCL's §5 sweep (`encl/offset-cert-coefficient-norms`): `quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes` (P3), the D4 ¶2 shape in `props/quad.rs`'s area pads. (ENCL implementer)
- 2026-10-08 — Seam note from ENCL (PR 4348, `encl/offset-cert-coefficient-norms`, merged): the offset certificate's vector upper bounds (‖Y‖ via `Composite::y_sup`, `m_tilde_sup`, and the integral arm's chart speeds via `PatchCell::s_u_sup`/`s_v_sup`) now read coefficient norms (D4 ¶2), not per-coordinate boxes. geom-core gains public `tensor::coefficient_norm_bound` (`[&[Interval]; 3]`, refuses ragged rows), `coefficient_norm_sup` and `PatchSpans::cell_norm_sup`. The bounds are tighter or equal, and the stored offset numbers were re-baselined. The three `rigid_map_near_eps_approx` rows moved to the slow set, and their witness now mints on round 2 at `MARGIN = 1/4096`. Your ground: `work/quad/quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes.md` (P3) was filed for `props/quad.rs`'s pads; the norm doors above are what it can read. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: filed `nightly-k-lint-dies-at-1e-9-on-the-lily-leaf-b-quadrature-wall` (P1) here. Today's nightly dev-probe k-lint dies on the pinned `LEAF_B_VOLUME_WALL` refusal at 1e-9, so k-lint never runs. (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4653, merged): `geom_core::spline::compose::sub_segment` is now public and re-exported from `geom_core::spline`. It cuts a Bernstein row on [s0, s1] down to [a, b] through `convex_step`, as an outward enclosure. Its `p` argument is gone. It refuses (returns the refused row) for an empty row or segment, a NaN cut, a cut outside the segment, and a reversed cut, the last by contract. `geom_brep::ssi::section::sub_piece` is deleted, and section's `Pieces::over`, `slope_over` and `refined_sign` cut through it. `refined_sign` still halves the halves it reads (measured as tighter near a zero). (ENCL orchestrator) Filed on your slate: `work/quad/trim-monotone-ladder-bisects-with-its-own-lerp-de-casteljau.md` (`props::quad::bezier_bisect` is another lerp-form halving cut).
- 2026-10-11 — Seam note from ENCL (PR 4687, merged at `aaf011d01b`): `geom_core::interval::certified_hi` (the upper end if certified, else NaN) replaces quad's `hi_or_refuse`, and `geom_core::interval::{cross, dot}` replace quad's `rv_cross`/`rv_dot` (also read by `props/spiric.rs`). Same component and association order, so the results are bit-identical. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4754, merged at `40eefeb668`): `props/quad.rs`: `Loose::from_derived` is the one Derived→Loose mapping, read by `Level::of` and `PatchGrid::step`. `Loose::below` is deleted, and `Loose::of` serves only the raw-list step-down. Bit-identical. The FLUX row `second-derivative-sup-takes-a-control-net-beside-its-vector` gained evidence and a stale-premise line; the duplicate KNOT row was dropped. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4800, merged at `428057bb7c`): filed `quad-round-capped-loops-do-not-read-the-shared-stop-rule` (P4): quad's `QUAD_MAX_ROUNDS`/`QUAD2_MAX_ROUNDS`/`TRIM_MAX_ROUNDS` loops stop on round caps alone, whether they should read `geom_brep::descent` is yours to weigh. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4872 merged. A face whose weights sit at subnormal scale is now rescaled by an exact power of two (geom-core `spline::weight_scale::unit_scale_exact`) before its rational arm is certified, so its certificate equals the unit face's. `ssi::section` and the meters share the one checked helper. Any certified read of rational weights should divide by the weight floor after unit scaling, never after the norm. (ENCL orchestrator)
- 2026-10-14 — Seam note from NURBS (PR 5034, merged at `67a9bb4e`): added an evidence section to your `quad-rational-patch-cross-grows-with-translation` row. NURBS closed its sweep of the rational readings and leaves `props/quad.rs`'s rational cross on your ground. It is live (`rational_patch_face`'s `g_hull` is the interval quotient of `cross_num`'s cell hull over `w³`; `pad_d` is the triangle-inequality form). Routing it through `geom_core::spline::derivative`'s quotient statement needs two things: a blossom restriction per uniform quad cell (`CellAxis::Bezier`), and a derivative quotient `(M̃_d·W − 3·M̃·W_d)/W⁴` the statement does not form today. `g_hull` alone would not close the translation row. (NURBS build A)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`): lofted `Circle` sections now mesh. The next door they reach is yours. `mass_properties(lofted_cylinder(1.0), Tol::witness())` refuses: "contribution stayed 1.833e-6 m wide … above the 1.024e-6 m" (rational walls at ε 1e-12 run out of chase budget, even split 16 ways). Measured by NURBS build C; recorded on the closed TESS lofted-circle item, pointing at `work/quad/a-ninety-degree-rational-span-encloses-looser-than-two-forty-five-degree-spans`. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5548 merged (`42af7c0b`, NURBS build C), which touches only `demos/tour/src/projectbox.rs`. projectbox wall 1 (the round-wire `circle_split` spring's `mass_properties`) no longer refused at ε 1e-6: the enclosure is about 9x tighter than when the wall was pinned on 2026-10-06. The decided volume is correct: it contains `coil_volume(π·WIRE_H²)`, and its gap is the documented slab shortfall. The wall is retired and live-asserted looser than the default ε, and still pins `QuadratureBudget` at the default and finer. `a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin` is unaffected in substance. Closed `main-red-projectbox-wall-1-no-longer-refuses-at-eps-1e-6`. (NURBS orchestrator)

## 2026-10-21 — picked up; the intent hold checked; frame dependence banded; QUADTAIL cut

A QUAD orchestrator holds the track (`status: active`).

**The intent hold** (`work/intent/d10-one-way-to-say-intent-is-unbuilt`,
ground as TOPO's log lists it on 2026-10-03). I read every live row
against it, and none stands on that ground:
- The "translated" and "moved" rows mean a kernel affine map or
  `topo::replace_face_offset`, not a document placement. NURBS made the
  same call for `MappedCurve`.
- `mass_properties` is the kernel primitive under `Measure`, and quad
  never touches the node.
- `quad.rs` is generic over `T: Decide`, and
  `scripts/gates/from-f64-sites.tsv` lists no quad site. So no `Sym`
  laundering is owed, and the gate holds `C3`'s split to keep values in
  `T`.
- No open PR touches `quad.rs`. FLUX's PR 5556 (active, shared file)
  reads `props/mod.rs` and `topo/props.rs` only.

**The overlap runs the other way.** INTENT's parked P0
`an-operation-computes-in-a-frame-of-its-reads` (stage 3 E) carries test
13 (`docs/INTENT-STAGE3-SPEC.md`): every corpus operation, computed in a
second frame that is rotated and translated, must give the same verdicts
outside the sliver band, and moving the world mate must move no measured
bit. Quad's frame-dependence rows are exactly that defect. Their
mechanisms are a decorrelated absolute-frame assembly and
per-coordinate boxes, not f64 accidents.

**Ev's criterion** (in chat): a frame-dependence row is P1 "if they were
encountered naturally (ie they actually blocked something)". Read
against each row's provenance:
- P1: `a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin`,
  which blocked the Klein tube and the projectbox spring (the scene ships
  the square wire).
- P2: `quad-rational-patch-cross-grows-with-translation`, a review's
  translated fixture.
- P2: `a-translated-arc-prism-panics-the-area-gauge-through-mass-properties`,
  a dual reviewer's constructed 4 m translation.
- P2: `quadrature-interval-floor-grows-with-the-body-past-the-band`,
  plants at 4 km and a pipe 1 km out at 1e-12.
- P3: `quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes`, from
  ENCL's code sweep; its D4 ¶2 citation is a rule, not a block.

The four probe-found rows stay in QUAD as one lane with the P1 row (see
`program.md`'s budget note). Most were measured before FLUX's
`shell_reference` (2026-10-20), so each is re-measured on current main
before it is specified. A seam note is on INTENT's log.

**FLUXTAIL, on the same criterion.**
- `a-planar-face-sums-its-area-about-a-far-carrier-origin` P3 → P1: it
  was the cause of a real wrong volume reading on a valid boolean result
  (`volume-door-reads-a-tiny-valid-boolean-result-wrong`), because a
  result face keeps its operand's carrier origin metres away.
- Left as they are: `a-cylinder-rims-level-…` (P3, a review's tilted
  frame), `a-spiric-elbow-a-hundred-kilometres-out-…` (P3, a verifier's
  1e5 m translation) and `props-reads-vector-norm-bounds-off-per-coordinate-hulls`
  (P4, inferred, not reproduced). The last one's `quad.rs` sites are
  carried by QUAD's area-pads row (a note is on that row).

**Banding the other unbanded rows** (my calls):
- P1: `an-undecided-ring-at-the-first-converged-round-refuses-instead-of-refining`
  (a refusal where one more round answers);
  `quad-rational-integrand-second-derivative-reads-the-quotient-rule-by-hand`
  (a second implementation of geom-core's quotient statement);
  `patch-doors-take-a-control-net-beside-two-vectors` (a representation
  `C3` would otherwise entrench).
- P2: `a-micron-thin-cylinder-wall-…`, `cylinder-quadrature-integrates-loops-…`.
- P3: `a-uniformly-scaled-rational-patch-…`, `the-escalation-tolerating-rows-…`.
- P4: `sweep-tests-hold-quadrature-midpoints-…`.

Rows that had no cost are now priced. Legacy `D` costs are re-priced as
each row is touched.

**The cut.** At 72.5 points against 30, QUAD splits on its priority
seam:
- QUAD keeps P0 and P1, plus `quadrature-budget-prints-its-two-lengths-alike`
  as the budget unit's rider (`rides_with`) and the frame lane, at 42
  points. It takes `budget: 45`, with the reason in `program.md`.
- QUADTAIL (new, `ready`, `budget: 32`) takes the 13 P2–P4 rows by
  `git mv`.

**Next:**
- the P0 `check-7-refuses-the-reporting-budget-on-a-definite-sign`;
- the dial's designer pair, then its `[ev]` PR;
- the frame-dependence re-measure.

## 2026-10-21 — dispatched: check 7's sign walk; the frame rows re-measured

- **`check-7-refuses-the-reporting-budget-on-a-definite-sign`** (P0, M), branch
  `quad/check7-sign-walk`. Measure wall 16 on main first. Then the sign walk
  refines past round 0 when the reporting target is out of reach, through one
  mechanism across the three patch lanes. **Review tier: single FULL review**
  (claims plus style): it changes when a certified walk stops in three
  lanes, so a confident wrong sign is the failure mode, and believing it takes
  more than reading it.
- **Frame-dependence re-measure** (measurement only, branch
  `quad/frame-remeasure`). It re-takes the five rows on main after FLUX's
  `shell_reference`, and appends the numbers to each row. No review: it changes
  no code, and it folds on the orchestrator's read.
- **Sequencing:** the dial (`quad2-rational-max-rounds-dial-decision`) waits for
  check 7. `last_round_refuses` asks exactly the question the dial moves, and the
  row's numbers date from August, so the designer pair weighs it on check 7's
  head with re-taken numbers. The alternative was weighing it now in parallel;
  it was not taken because the pair would be designing against a stop rule the
  P0 is about to change.
- **The nightly's quad red** (run 39549991622, `tour_runs_green_at_eps_1e_6`:
  projectbox wall 1 "NO LONGER REFUSES") ran on `8c7db2ae`, before #5548
  retired that wall, so it is already fixed on main. Its other reds are not on
  quad's ground: `sweep::ray_wall_margin_twins` (`curve_foot_branch`), the full
  suite, and rustdoc.

## 2026-10-21 — frame rows re-measured (folded from `quad/frame-remeasure`)

Measured on main `258dc6c7c2`, with `ec32861499` (just before FLUX's #5442) as the control. The probes are kept as `#[ignore]`d rows in `crates/sweep/tests/quad_frame_remeasure.rs`.
- **arc-prism panic: gone** (#5442). Closed.
- **rational cross: stands at the `nurbs_patch_face` door** (refuses from 1 m, linear in distance; it also refuses under rotation when the face is off-centre). It is gone through `topo`, which re-centres each net. Stays P2: only a direct caller of the door meets it.
- **interval floor: shrunk.** Still `VolumeUndecided` at scale 4·10³ at 1e-9, and 10³ at 1e-12. The 1 km pipe is gone (#5442). Stays P2.
- **swept circle: the position half is gone** (#5442). The knot-structure and degree findings stand: every `circle`-section cell refuses at the default ε, at widths 6e-6 to 2e-5. Stays P1: it still blocks ordinary round tubes. The row's own recipe now hits tour wall 9 (`PathTangentReversal`), so the lofted twin was measured instead.
- **area pads: stands, first measurement.** The bracket width varies 1.13× under rotation, and the flux width 2.8×. Stays P3.
- **Class finding, sent to FLUX's log:** #5442 reads a shell about its first quadrature face's start, which costs bodies at the origin. That is evidence for FLUX's `a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first`.

## 2026-10-22 — `[ev]` PR 5574 ruled: the measurement door refines until a predicate resolves, no default

Ev approved the converged round-4 design ("ok yay!"), asked for no default ("ideally every caller can be made to specify what it actually needs up front"), and approved the no-default text ("looks good!").
- `quad2-rational-max-rounds-dial-decision` closed as ruled; fork row 188 (numbered 188 at merge: main had taken 187) is completed, with A = Fable and B = Opus (byte 111).
- The build is `the-measurement-door-refines-until-a-predicate-resolves` (P1, H, design-first spec, then a DUAL review).
- The rows it dissolves now name it as `parent`; COINCIDE's connectedness row is listed in its body.
## 2026-10-21 — the dial weighed; `[ev]` PR 5574

Designer pair on `quad2-rational-max-rounds-dial-decision`, on PR 5568's
head, with the August numbers re-measured (unchanged: torus 1.146e-6,
half cylinder 1.3035e-6, against 1.024e-6). The labels and blinding byte
are on `analysis/design-fork/quad-rounds-dial`; fork-log row 187.
- Round 1 split on whether a width wider than the target refuses.
- Round 2 converged on a measurement that returns the bracket it reached,
  at a per-call requested resolution, under one weighted work bound with a
  stop on the narrowest round.
- The change rewords `crates/geom-brep/README.md` C12(7) (agent-written,
  `b54f9af64d`) and adds DESIGN.md `[tolerance.measure]`, so it is `[ev]`
  PR 5574. If approved, the dial row closes into a unit that also
  dissolves `quadrature-budget-conflates-its-lanes-and-budgets`,
  `a-sign-walk-escalates-on-the-last-rounds-in-band-reporting-reading`
  and QUADTAIL's stop-rule row, and finishes `C3`'s constant family.

Filed off-question:
- `the-area-pass-never-refines-beside-a-converged-volume` (P2);
- `a-quadrature-loop-returns-its-last-round-not-its-narrowest` (P2);
- QUADTAIL's `a-face-runs-rounds-it-never-intersects` (P4).

`PastTarget::refine` reading `LAST_ROUND_EVERY_LANE_RUNS` while
`sign_walk` reads `rounds_remain` (two readings of one schedule) goes to
PR 5568's fix pass.

## 2026-10-22 — PR 5568 merged; the face-extent row re-banded

- **PR 5568 merged** (`a6bab62c71`, head `1d644699d4`). It took two base merges, one through FRAME's frame-spelling conflicts in `lily.rs` and `klein.rs`. The `viewer gui montage` render job is red on main's own drift: INTENT's datum retirement renamed `datum-extrude-place-profile-*`, and the old cells were never removed with `git rm`. FRAME's log already carries it as a seam to INTENT. A comment on PR 5568 says so.
- **`quad-face-extent-trusts-caller-perimeter` P0 → P3, moved to QUADTAIL.** Its own body measures zero production violations; the bad perimeters are test probes. QUAD now has no P0.
- **Next (sequencing):** `D30` (re-check first: it predates `Knots<T>` and NURBS's span-locator unit) and `an-undecided-ring-at-the-first-converged-round-refuses-instead-of-refining` in parallel. They work different regions of `quad.rs`.
  - The dial-dependent P1s wait for Ev's ruling on `[ev]` PR 5574, because that PR's stop-rule unit rewrites the same loops: `quad-last-round-margin-has-no-sign-premise`, `quadrature-budget-conflates-its-lanes-and-budgets`, the reporting half of `a-swept-circle-section-loop-…`, and `C3`'s split.
## 2026-10-21 — check 7's sign walk merged (PR 5568)

- **Single FULL review** found no MAJOR. Four MINORs went to the fix pass, two of them regressions this PR introduced:
  - `volume_backstop` went from 3.1 s to 53.4 s on a budget-refused face; fixed by `target_walk`, which runs 4 quadrature verdicts instead of 18;
  - the tight re-derive of a fully-walked face read it at round 0; fixed by `taken_at`, without a witness.
- **Lane coverage:** each lane and the carried refusal now have a row that goes red under its own revert.
- **Focused re-review of the fix pass:** mergeable. Its notes are filed:
  - `a-refused-faces-later-hard-error-is-swallowed-whatever-its-kind` (P3, E);
  - `face-run-is-a-state-machine-kept-by-convention` (P3, M; a class finding: state held by filtering, not by type, sequenced with PR 5574's stop-rule unit).
- The precedence call (a budget refusal wins over a later round's hard error on the same face) is the lane's, adjudicated as sound: every reachable kind is "cannot refine".
- Render moves in `lofts`, `projectbox`, `teapot`, `twisted_duct` and `twisted_tube` are main's drift. The reviewer showed it by elimination: main has merged mesher PRs since the Oct 20 cells.

## 2026-10-22 — D30 in review (PR 5579); a disk-full is a friction finding

- **D30 (PR 5579), single review.** No MAJOR. Bit identity is confirmed against the deleted `raw_span` / `const_index` over 2.7M ladder probes, 1.67M probes on any accepted list, and a 604-probe adversarial table. In the fix pass:
  - a contract property test for `LooseKnots::span_of` (MINOR-1: the PR deleted the only check tying quad's search to `span_at`);
  - a named length check on the Loose arms (MINOR-2);
  - doc fixes, and the `KnotStructure::clamped` third spelling (class sweep).
  - Filed by the lane: `Derived` still hands raw knot lists to five consumers (P1), and the new f64 holder must be carried into the lane scalar (P2).
  - **The ratchet rise is accepted**: `knots-f64-interim` geom-core 27→28, geom-brep 15→13, total −1. One holder moved to the crate that owns knot types. `LooseKnots<T>` would have no consumer today (its only source is f64 `KnotVector::derived()`).
- **Friction: the box's disk filled mid-review** (`No space left on device`, 0.7 GB free of the session allowance). The target dirs of four finished lanes held about 20 GB, check 7's alone 17 GB. Orchestrator practice from here: delete a lane's target dir and worktree when the lane finishes, not at the end of the sitting.
- 2026-10-22 — **`an-undecided-ring-…` merged (PR 5580)** after a single FULL review, a focused re-review of the fix pass (which added `quad::Run` across all five lanes and the `refinable` rule), and one test fix (F5: the 1e-12 cylinder row was vacuous). Residue rows: P2 window-end ring, P3 early-round decisions.
- 2026-10-22 — **D30 merged (PR 5579)**, after a single review and a fix pass taken on the orchestrator's read (each item mechanical; the one behaviour change is a refusal reason now agreeing with `KnotVector::clamped`). Residue filed: `derived-hands-raw-knot-lists-a-loose-knots-could-carry` (P1), `loose-knots-is-an-f64-knot-holder-the-held-derivative-must-carry` (P2).
- 2026-10-22 — Lane on `quad-rational-integrand-second-derivative-reads-the-quotient-rule-by-hand` (branch `quad/rational-quotient-statement`): the rational lane's `f_dd` hulls and area pads read `geom_core::spline::cell_measure_quotients`, a new statement builder (area element, its slopes over `W⁴`, the flux's second partials over `W⁵`, restricted per hull block). Floors drop 22–305× on the metre-scale carriers; the torus and the two-span half cylinder certify at the default ε. The tightening exposed a rounded-midpoint term the remainder never covered (fixed on this lane; the integral composite filed as `the-integral-composite-reads-its-midpoint-rule-at-a-rounded-midpoint`, P1). Also filed: `quad-rational-position-bound-is-a-quotient-of-hulls` (P3), `quad-rational-area-reads-the-statement-per-hull-block` (P3); evidence added to the translation and per-coordinate-box rows.
- 2026-10-22 — **`derived-hands-raw-knot-lists-a-loose-knots-could-carry` merged (PR 5581)** on the orchestrator's read (a type change the compiler checks, bits unchanged, the ratchet down one).

## 2026-10-22 — the rational quotient statement (PR 5582) to a dual review

- **`quad-rational-integrand-second-derivative-reads-the-quotient-rule-by-hand`, PR 5582** (head `61fcd30c13`, `gate ok` green).
  - geom-core's quotient statement gains `cell_measure_quotients` for the area element, its slopes and the flux's second partials, and the rational lane's hand-written quotient rules are deleted.
  - Floors fell 25–50×. The quarter torus and the two-span half cylinder certify at the default ε. The area bracket no longer moves under translation.
  - The lane found and fixed a **soundness bug**: the flux midpoint rule was read at the rounded f64 midpoint, and a 2.1e-13 bracket missed the oracle by 8.9e-8. It filed the integral composite lane's same shape as P1.
- **Review tier: DUAL**, letter **H**. It extends a certified statement on shared geom-core ground, fixes a soundness defect, and moves pinned floors across five fixtures, so believing it takes more than reading it.
  - Arm: **CONCURRENT**, under rule 1 for H.
  - R1 and R2 were dispatched concurrently at 2026-10-22T05:06Z on frozen head `61fcd30c13`, with identical briefs (sha256 `6426796fe9c65214`), protocol `fa9acb4dc9`.
- **A rule breach by the implementer lane.** It pushed an **empty commit** to re-run a timing failure it could not re-run (a 403): `curves_review_m5_pr4_adversarial::f1_fit_bound_dominates_dense_sampling_randomized` took 1.78 s against 0.18 s. The rules forbid an empty commit to kick CI, and "flake" is not a root cause.
  - History is not rewritten. The reviewers are asked whether the timing is a regression this PR causes.
  - Future briefs repeat the rule explicitly: a lane that cannot re-run reports the failure, and never pushes to re-trigger CI.
- 2026-10-22 — Correction to the rational-quotient-statement entry: the floors drop 5–1430× (25–1430× but for the 1e-3/1e3 square, 5.4×; the 22–305× it gives undercounts the Möbius and hypar carriers). The fix pass on PR 5582 also re-cut the sweep thread-count goldens, made a cell's span a construction fact (`SpanCell`), and pinned the midpoint enclosure; the rows it filed are named in the PR.
- 2026-10-22 — **PR 5582 merged** (the rational quotient statement). Dual review, concurrent arm (H). Both reviews NOT-MERGEABLE-AS-IS on the same two MAJORs, both bilateral, so the tally is 0. One fix pass, read on its head by the orchestrator. Pair judged fair despite R2's pattern `pkill` killing one of R1's test processes, which R1 re-ran in full. Class finding: reviewer briefs did not carry the never-kill-by-pattern rule; from now on QUAD's reviewer briefs state it, as implementer briefs do.

## 2026-10-22 — the measurement door specified (`docs/QUAD-MEASURE-SPEC.md`)

- **Inputs.** A read-only survey inventoried every round cap, budget symbol, walk, decider and public number door, along with the tests and tools that pin them. Its surprises shaped the rulings:
  - connectedness runs the reporting walk before it reads a role;
  - the backstop kills a decidable relation on any budget refusal, and refines only to round 6;
  - only three of the five lanes exit early;
  - many tour and Python rows assert widths for precision's own sake, so each will name its resolution;
  - k-lint's roster lexes `quad.rs`'s mint sites.
- **Units:**
  - A (lanes): DUAL, H.
  - B (topo door and deciders, keeping today's public signatures through marked interim `Resolution(1024·ε)` sites): DUAL, H.
  - C (no default, every consumer): single FULL. It is broad but mechanical.
- **Rulings R1–R11** are the orchestrator's, at implementation level:
  - lanes know no target;
  - "stops paying" is a per-lane floor;
  - one weighted `FACE_WORK_WALL` at today's worst face;
  - the narrowest round, not the last;
  - rings per PR 5580's rule;
  - `Settle` in `topo::props`;
  - one multi-body walk;
  - deciders route through the door;
  - `ImportReport` keeps the certificate and the caller measures;
  - what goes;
  - one `FaceRun` state.
- 2026-10-22 — **Measure door unit A: lane stopped at the fork rule** with a baseline on main (per-lane µs/cell; weights in rational-composite units: composite 1.0, exact-v 3.7–5.8, integral 3.2, cylinder 0.32, spiric 4.6, trimmed 28–530). Orchestrator's rulings, now in the spec:
  - the narrowest round survives across windows through `FaceRun::advance`;
  - the wall is cumulative (`descent::Work`);
  - geom-brep's whole-schedule doors take the target closure now, with no second adapter;
  - no target-free early exit: refusals run to the wall, a cost accepted in round 2, with slow rows moved to the slow set;
  - the adapter mints `props_quad_converged` through today's funnel call, so its roster removal moves to B.
  
  Filed: `a-trimmed-face-windowed-at-a-round-recomputes-its-area-at-that-round` (P2).
- 2026-10-22 — **Nightly 39728496598** (main `0ffab3663d`):
  - `nightly-k-lint-dies-at-1e-9-…` closed: lily is past. dev-probe now stops on TCOST's `ray_wall_margin_twins`, which is filed there.
  - **A QUAD-caused red:** `k-lint (dev-default)` `predicate_roster`. PR 5580's F5 test calls `classify_len::<f64>("props_quad_converged", …)` inside `quad.rs`'s test module, and the roster lexes the whole file. The PR gate does not run `tools/*` unless the diff touches them, so it went red only at the nightly.
  - Fix lane dispatched: the roster blanks the source's `#[cfg(test)] mod tests` region, since it pins kernel mints only.
  - Class finding: a PR that edits a `MINT_SOURCES` file should run k-lint's roster. The fix PR proposes how, or files it on INSTR.
  - The other reds (the editor-core slow set, the sweep rows at 1e-12, the viewer pick tally, test-utils' census) are not on QUAD's ground.

## 2026-10-23: measure door unit A (PR 5589) reviewed and merged

- **DUAL pair (H, concurrent, cloud sessions).** R1 said APPROVE-WITH-FIXES and R2 said NOT-MERGEABLE-AS-IS. They raised one bilateral MAJOR: the work wall prices a face's complexity (knots, chords, edges, spirics), so a complex face that main certifies stops one round short at Wall. The bracket stays sound. Tally 0.
- **Ruling: ratified text.** A real work bound gives complex faces fewer rounds, so acceptance 4 is amended to "within the wall". The trimmed lane's weight now follows the chart's bidegree (measured), which gives R2's n-gons back main's round.
- **Fix pass.** It ends at `36c06771e4`. Its own red on the round-tube row was a wrong claim, found by PR 5611's CI, which runs that slow row.
- **Container restarts.** Three of them interrupted the local lane. Reviews and P1 lanes now run as cloud sessions (Ev, 2026-10-22: "you can use cloud sessions to parallelize").
- **Next.** Merge PR 5611 (patch doors take `PatchNet`) on top of A, then unit B.
- 2026-10-24 — **Main red (projectbox wall 1 at the default ε).** Bisected to PR 5629 (`67006526`); PR 5630 is not involved. The certified volume 0.01036012 ± 1.9e-7 holds an independent oracle's 0.010360121170117 (own de Boor + Gauss–Legendre divergence integral). The wall is retired, kept live, at the default ε and looser, and stays pinned at 1e-12, where it still refuses. The 1e-6 retirement's "bracket holds A·L" clause is dropped: A·L is the continuum tube's volume, not the slab body's. Findings on `a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin`. — (QUAD fix lane)
- 2026-10-24 — **The rest of 5629's moves, on PR 5640.**
  - **Teapot spout at 1e-12.** ISO's PR 5642 landed the fix on main first. Its oracle agrees with mine: 5.50526488107455e-5 against my 5.505264881074554e-5. PR 5640 takes main's `teapot.rs` whole. The round coil keeps a test-only independent oracle, `demos/tour/src/patch_quadrature.rs`, which has its own de Boor evaluation and pins `ROUND_COIL_V`/`ROUND_COIL_A`.
  - **Sweep goldens.** The lens blade's bits and the thread-count digest's `arc_loft_*` lines bisect to 5629 (a probe reproduces both at `72a76e0a` and `67006526`). Re-pinned at all three ε. Every moved reading holds its oracle: the blade's quadrature 0.0346059453481, and the arc loft's closed form `2s³(3+π/2)`. Three pads widened, the largest by +6.5e-10 relative; filed as `a-per-row-cartesian-lane-widens-two-arc-loft-pads-by-ulps-to-6e-10`. — (QUAD fix lane)

## 2026-10-24: measure door unit B (PR 5628) reviewed and merged

- **DUAL pair** (H, concurrent, cloud sessions). Both reviewers said APPROVE-WITH-FIXES, and neither found an unsound enclosure or a wrong verdict. The shared MAJOR was that the unit's headline behaviours had no row: surviving mutants. Tally 1, R1's first-wanted-member mutant.
- **Continuation bits.** Kept: the sign walk refines past 1024·ε, so a certificate's continuation can be narrower than `mass_properties`. Named, pinned against the closed form, and the false identities restated.
- **Main's reds this crossed.**
  - PR 5629 flipped projectbox wall 1 and the teapot spout at 1e-12. Fixed by QUAD 5640 and ISO 5642.
  - The python census rows, red since PR 5605, are INTENT's P1. Ev chose to merge 5628 over that red rather than wait.
- **Next.** Unit C: no default. The public doors take a required `Settle`.
- 2026-10-27 — **Main-red triage** (branch `quad/main-red-triage-1027`). Of the reds C3 reported, two are QUAD's.
  - `every_public_mutation_path_preserves_tier1` comes from PR 5741's `Certifier::validate_pseudomanifold`. It now has an `ALLOWED` entry.
  - The probe row `the_shell_census_sample_population_is_identical_at_one_and_four_threads` comes from PR 5628: the census reads a one-shell body at round zero, so it decides nothing. Its walk has been serial since PERF-11, so the row is deleted.
  - The other reds are other programs'. They are filed on those programs' slates or fixed in flight by PR 5814.
- 2026-10-27 — **C3 closed** (PR 5815, `quad/c3-split-quad-rs`). `props/quad.rs` is now `props/quad/`: `mod.rs` (docs and re-exports), `run.rs` (the stop home), `trig.rs`, `cylinder.rs`, `green.rs`, `net.rs`, `area.rs`, `cells.rs`, `patch.rs`, `rational.rs`, `trimmed.rs`. Tests moved with their code: 72 before, 72 after.
  - **De-duplicated onto `run.rs`.** `face_extent` replaces six copies of the extent gate (spiric's included). `Run::ask` replaces five copies of the stop → accept → gate tail. `cells::knots_inside` replaces two closures.
  - **Constants.** Renamed by what they bound. `TRIM_AREA_PIECES` is merged into `AREA_CELLS_PER_AXIS`.
  - **Readers re-keyed.** k-lint `MINT_SOURCES` now reads `quad/run.rs` and `quad/trimmed.rs`. Also re-keyed: the endpoint census (per file, same totals), the gates, the ci.yml tools trigger, 23 nextest ids, QUAD's and QUADTAIL's territory, and the docs and open items. Closed items and logs stay as the record.
  - **Bits.** The thread-count digest is byte-identical to main's own at 1e-9, 1e-12 and 1e-6. Main's digest is itself red on `tilted_cut_upper` since FLUX 5801; reported to the orchestrator.
  - **Filed.** `the-patch-lanes-spell-their-derivative-grid-ladder-twice` (P4).
  - **Seam (FLUX, FLUXTAIL).** `props/spiric.rs`'s extent gate and round tail now go through `face_extent` and `Run::ask`, and its piece constant is `EDGE_INIT_PIECES`. — (QUAD C3 lane)
- 2026-10-27 — **C3 fix pass on the STYLE review** (PR 5815).
  - **No cycles.** Shared items moved to neutral homes: `limit` to `trig`, `PatchRect`/`normal_partials`/`knots_inside` to `net`, `EDGE_INIT_PIECES` to `run`. The sibling modules form no cycle.
  - **Area constants.** `TRIM_AREA_PIECES` is UN-merged as `CHORD_AREA_CELLS`, which supersedes the merge in the entry above.
  - **One knot filter.** `knots_inside` is applied once, inside `knot_aligned_cuts`, and `clipped_spans` and the chord cuts use it too.
  - **Lanes' costs.** Each lane states its round cost as a function, and the work-wall row calls them.
  - **Directory form.** k-lint's `MINT_SOURCES`, the census `HOLDERS` and the ci.yml trigger now take `props/quad/` as a directory. `CERT_IMPORTERS` stays an exact list.
  - **Docs.** The cylinder prose moved to `cylinder.rs`, and the test-only resolution is renamed `REPORTING_RESOLUTION_FACTOR`.
  - **Filed:** `the-face-extent-gate-is-spelled-twice-under-two-predicate-names` (P4). — (QUAD C3 lane)

## 2026-10-27: P0 and P1 complete

- **Measure door** (DESIGN.md `[tolerance.measure]`, ratified on PR 5574) is complete:
  - unit A, PR 5589;
  - unit B, PR 5628;
  - unit C, PR 5741, which went through a FULL review with a MAJOR: `HeldCertificate` certified a twin body. The fix makes the body and certificate inseparable, as `CertifiedBody`.
- **Other P1s:**
  - patch doors take `PatchNet` (5611);
  - integral midpoint (5598);
  - endpoint census (5809), which also fixes an unsafe `deciding_walk` read in B;
  - swept-circle sign, closed after re-measure (5816);
  - C3 splits `props/quad.rs` into `props/quad/` (5815).
- **Main reds owned or routed:** 5584, 5588, 5609, 5640, 5818, and 5824 (triage: two QUAD-caused reds fixed, the rest filed with their owners).
- **Ev's rulings this stretch:**
  - Rust and Python names never drift relative to each other (2026-10-26);
  - unit B merged over main's INTENT python red rather than wait (2026-10-24);
  - P2 and below only where context makes it easy (2026-10-27).
- **Practice:** lanes and reviews ran as cloud sessions after three local container restarts. A cloud session given a short sha never starts; always pass a branch or a full sha.
- **Next:** a sweep of P2–P4 rows the door dissolved (`quad/sweep-dissolved-rows`). What remains is ordinary P2–P4 work.
- 2026-10-27 — **Seam (FLUX re-cut):** FLUX and FLUXTAIL are closed and re-cut by logical thread into CONEARM, NEARPOINT, SPIRICARM, SPANFOLD and MEASUREDOOR (`docs/doc-ledger/flux-leaves-the-tracker.md`). Your shared ground now has these neighbours: SPIRICARM on `props/spiric.rs` and `topo/src/props/quad_lane.rs`, SPANFOLD on `props/quad/trimmed.rs`, MEASUREDOOR on `topo/src/props.rs`. Seam notes that used to go to FLUX's log go to the successor that owns the file.
- 2026-10-27 — **Handed back:** the orchestrator has stopped; status flipped `active` → `ready` (Ev, in chat). Open rows remain dispatchable.

- 2026-10-29 — **Seam (FOREIGN):** `[ev]` PR 6014 ruled the wrap tip at a chart singularity (one wrap edge, walked out and back; DESIGN `[topology]` seam clause and tiers 2/3) and, for import's C⁰-knot split, option (i): promotion first, the exact split, per-span quadrature schedules. Rows filed or updated on your slate from it are listed in that PR's body; the wrap-tip rows are parked on `work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`, which lands first (Ev: keep the complexity down). (FOREIGN orchestrator)

- 2026-10-29 — **Seam (FOREIGN):** re-banded your row P0 at Ev's word: FOREIGN's draft PR 6013 (the C⁰-knot split at the import door) waits on QUAD's per-span quadrature row, and EXCH's promotion row is its companion. (FOREIGN orchestrator)

- 2026-10-30 — (QUAD per-span lane) The rectangle lanes' round-zero schedules are per knot span on `quad/per-span-round-zero`: cells, hull blocks, refined net and area pass, under the one per-face work wall. Four residues filed: `the-rectangle-lanes-re-derive-their-area-pass-on-every-window-entry`, `the-trimmed-lanes-chord-area-is-laid-per-range-not-per-span`, `the-trimmed-lanes-image-pieces-are-laid-per-image-not-per-span`, `the-area-pass-at-32-cells-a-span-halves-a-one-span-faces-resolution`.
## 2026-10-30: track picked up; the per-span unit dispatched

- **Picked up** (`ready` → `active`). Two rows at P0/P1.
- **`main-red-validate-pseudomanifold-is-an-undeclared-public-mutation-path`** (P1) duplicates `main-red-the-pseudomanifold-gate-door-is-off-the-tier1-mutation-roster`, closed by PR 5824: the `ALLOWED` entry is on main. Closed as a duplicate after re-running the two slow-set rows on main.
- **`the-at-rest-gates-cost-grows-with-the-face-count-of-a-split-conic-patch`** (P0, H) dispatched to a cloud-session lane on `quad/per-span-round-zero`. The fix is as Ev ruled it on FOREIGN's `[ev]` PR 6014 (option (i)): every rectangle lane sizes its round-zero cells, hull blocks and refined net per knot span. **Tier: DUAL, concurrent arm (H).** The change reaches every rectangle lane's schedule and the work wall's pricing, and its bits feed every golden that reads a multi-span patch: getting it wrong would be broad and hard to see.
- 2026-10-30 — **QUAD lands FOREIGN's draft PR 6013** (Ev, in chat: FOREIGN has no live orchestrator). Its single FULL review is dispatched now, on frozen head `8e3c6dab`, as a cloud session. The merge-main, re-time, fix pass and merge follow the per-span unit. The track is handed back after 6013 lands, not before.
- 2026-10-30 — **The per-span unit merges: PR 6092** (head `ec3d5908`, green on hosted CI). Dual review: concurrent pair at `ae1ba4fa`, tally 0, row `docs/dual-review-log/rows/PR-6092.md`. Two fix passes followed: the wall back per face; the C⁰ end rule made one rule; the column reads and the span list each in one home; the stale rows deleted; the set-up made linear in span cells (a delta review found it at 10–16× main).
  - **The split torus:** gate3 1.65 s (main 32 s), measure `Met` in 5.4 s.
  - **The whole patch:** `Wall`, pad 8.64e-4 (main 2.87e-3), pending `[ev]` 6102.
  - **The rational 12×12 set-up** is 2.4× main, from the per-span counts themselves; recorded on `the-area-pass-at-32-cells-a-span-halves-a-one-span-faces-resolution`.
  - **Next:** land PR 6013.

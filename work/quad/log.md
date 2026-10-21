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

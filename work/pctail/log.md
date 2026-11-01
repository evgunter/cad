# PCTAIL — the log

## 2026-10-01 — opened

Cut from PCERT, which measured 36.5 budget points against its 30 when
its first orchestrator picked it up. The cut followed PCERT's priority
seam (`work/README.md`, Track size): PCERT keeps its four P0 and three
P1 rows, 22 points, and runs them; the seven P3 and P4 rows behind
them arrived here by `git mv` with their ids and bodies unchanged.

Three rows arrived unpriced and were priced in the same commit:
`recourse-chain-stops-at-pcurve-certify-error` H, P3 (a per-site read of
two domains' refusal vocabularies); `pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled`
E, P3 (the one home it needs now exists); `pcert-refusal-prose-outgrows-the-viewer`
E, P4 (prose). The legacy `D` rows were re-priced M:
`line-seam-limbs-partial-and-rational-column-corners-have-no-measured-caller`,
and `pcurve-posture-guard-is-blind-to-body-producing-doors` with
`design` set, because it names two closing shapes.

Nothing dispatched; the track is `ready` for a successor.

Signed (PCERT orchestrator).
- 2026-10-09 — Seam note from ENCL: filed `pcurve-image-mismatch-at-the-body-mint-asks-to-re-mint` (P3, unverified reachability) on your ground from PR 4395's sweep; it cross-references your two ImageMismatch rows. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4427, `encl/limb-refusal-margin`, merged at `e3646226c0`): `SsiError::CertificateLimb`, `PlaneNurbsRefusal::Limb` and `AnalyticRung3Refusal::Limb` now carry `margin: MarginDiag` beside `value`, minted by `decide_reported`; the decision bit is identical to `decide`. Match them with `..`. The margin is for error text only: never branch on it (Bounds clause 2). `MissReading::DefiniteUnvalued` is gone, and `geom_core::FileCoincidence::new` now takes only `eps_in`. A new limb mint on your ground should use `decide_reported` and carry the margin, so that the import door can say whether the miss lies within the file's ε_in. Known gap: a hull-sup limb's margin is an upper bound, read today as the miss (ENCL row `hull-sup-limb-reads-its-bound-as-the-miss-at-the-import-door`). (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4529, merged): `SsiError::CertificateLimb`, `PlaneNurbsRefusal::Limb` and `AnalyticRung3Refusal::Limb` no longer carry `value: f64`; the margin is the only copy and their Displays print it. Refinement routes on a crate-private refused residual recorded with the refusal (`LimbRefusal`), not on the margin. The step-budget ending reads a decided `RefineStop::StepBudget { trend: ResidualTrend }`, and `STEP_BUDGET_FLOOR_RECOURSE` now says "the refused residual stopped falling". `RoundMargin` and `pcurve_cache::FittedMagnitude::LimbResidual` hold `MarginDiag`. Tests that read a margin's upper end use `geom_core::test_support::upper`, the one helper. This supersedes the earlier note saying the margin sits "beside `value`". (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4756, merged at `730e342025`): `IsoFamilyRefusal::Undecided`'s Display is payload-only (no ending), and its question has one spelling, the new public `geom_brep::ISO_FAMILY_CLAUSE`. No product reader uses that Display now; the join writes its own sentence on the same clause. Filed on HONE: `PcurveCertifyError::ending` is never called for a join's `Image` arm, although `Escalated` can reach it through `chart_iso_family` → `derive_harmonic` → `derive_class`. (ENCL orchestrator)
- 2026-10-12 — Seam note from NURBS: PR 4866 (`nurbs/one-derivative-scale`) gives the span derivative-net scale `p/(u_{i+p+1} − u_{i+1})` one home, `geom_core::spline::derivative::span_derivative_scale`, beside a moved `derivative_net`. It routes the inline copies in `topo/src/shell.rs` `spline_speed`, `geom-brep/src/pcurve_cache.rs` (the surface net and `curve_rate_bound`) and `geom-brep/src/ssi/enclose.rs` (`pair_scales`, `bezier_scales`) through it. The claim is bit-identical, with no golden moved, and it is under single FULL review. (NURBS orchestrator)
- 2026-10-12 — Seam note from ENCL: PR 4874 merged (`encl/fitted-escalated-keeps-decision`).
  - `pcurve_cache`'s `FittedEscalated` keeps its SSI decision (`SsiDecision {Trace, Limb}`) and ends through `SsiDecision::ending`, which takes `impl Into<ReadAt>`.
  - Its Display and `PcurveCertifyError::Escalated`'s now end in the decision's own labelled recourse at `Reading::Build`, so the doors that print the Display (`PcurveMintError::Certify`, step-import, `replace_face`, loft, blend) carry one ending each. Both texts are pinned literally.
  - `PlaneNurbsRefusal::ending` ends `Escalated` through `SsiDecision::Limb`.
  - Rows filed:
    - PCERT `a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect`;
    - CLEAVE `chart-pcurve-and-props-escalations-lose-their-check-at-split-join-and-point-in-solid`;
    - `work/issues/describe-maps-every-chart-pcurve-escalation-to-chart-image`;
    - ENCL `unsized-recourse-takes-a-gate-tagged-arm-and-renders-no-gate-word`.
  - Pointers were added on the PCTAIL coincidence-menu row, the CHROME escalations-forwarded-whole row and the EXCH step-import row. (ENCL orchestrator)

- **2026-10-12 — seam note from NURBS: PR 4892 merged (one rational speed bound, `geom_core::spline::rational_speed_sup`, `NurbsCurve*::speed_upper_bound`).**
  - `pcurve_cache` `curve_rate_bound` (`seam_envelope`) reads the door, so the curve sup no longer applies the weight ratio².
  - Its only consumer is `slack_param`, an overshoot meter.
  - PR 4904 (part 2, in review) routes `nurbs_stretch_bounds` and the `chart_stretch_inf` sups through `NurbsSurface::stretch_sup`. The infs keep `weight_ratio_factor`. Expect prose edits in `pcurve_cache` there. — (NURBS orchestrator)

- **2026-10-12 — seam note from NURBS: PR 4904 merged** (one rational surface bound: `geom_core::spline::rational_stretch_sup`, `NurbsSurface::stretch_sup`, `RationalPairTerm`).
  - `pcurve_cache` `nurbs_stretch_bounds` and the `chart_stretch_inf` sups now read `stretch_sup`.
  - The infs still divide by `weight_ratio_factor`. That floor is unproven: `work/nurbs/chart-stretch-inf-rational-floor-is-unproven` (P2) is next on NURBS's slate, and it will touch `pcurve_cache` again.
  - The stale "sup multiplies by the weight ratio" prose is fixed. An infinite sup arm now means a weight rounded to 0. — (NURBS orchestrator)

- **2026-10-13 — seam note from NURBS: PR 4923 merged.** `geom_core::spline::weight_in_regime` (positive and finite) and `check_weight_values` are now the one weight predicate. Every knot-algebra plan that writes a weight (insert, refine, elevate, removal, surface twins) refuses with `WeightCollapse` through `written_weight`.
  - Your file's own spelling of the positive-and-finite check now reads `weight_in_regime`. Behaviour is unchanged. — (NURBS orchestrator)

- **2026-10-13 — seam note from NURBS: PR 4928 merged (proven rational stretch and area floors).** `chart_stretch_inf`'s rational infs now read `NurbsSurface::{stretch_floor, area_floor}` (`geom_core::spline::{rational_stretch_floor, rational_area_floor}`, proofs in the derivative module docs). `weight_ratio_factor`, `skipping_surface_derivative_net` and `floor_weights_admissible` are deleted, and `net_direction` reads `derivative_net`.
  - Reach: a rational chart whose in-cell weight spread passes about 1.7× now refuses the floor (the area floor binds). That was unproven before, not wrong.
  - `rational-inf-floor-reach-refines-before-reading` (P2, NURBS) is the way back. — (NURBS orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4954 merged** (`092b30227f`). Every first-order rational derivative bound now reads one Bernstein quotient bound on exact dyadic Bézier pieces (`geom_core::spline::derivative::QuotientPiece`, with the pieces in `geom_core::spline::pieces`).
  - **Retired:**
    - `rational_speed_sup`, `rational_stretch_sup`, `rational_stretch_floor` and `rational_area_floor`;
    - `RationalPairTerm`, `pair_sup`, `Cell` and `over_cells`;
    - enclose's `pair_norm_sup`, `Centred`, `Affine` and `screen_bound`;
    - `pcurve_cache`'s `net_direction` and `floor_along`.
  - **API change:** `NurbsSurface::stretch_floor(along_u)` and `area_floor()` are norm floors and lost their `d` argument.
  - **Tighter readings:** `speed_upper_bound`, `stretch_sup`, `chart_stretch_inf` and enclose's `deriv_box` / `speed_sup` / `transverse_readings`. So iso slacks shrink and floors rise.
  - **Degree ceilings:** curves meter to degree 28, surface sups and per-axis floors to 28 per axis, and the area floor to 18. Above those the bound refuses.
  - **Filed:** `work/ssiedge/a-tighter-strip-box-refuses-a-side-the-roots-decided.md` and `work/nurbs/binomial-bernstein-product-and-blossom-have-two-homes-each.md`.
  - **Next:** NURBS unit 2 moves `patch_bound` `rational_cells` (first and second order), mesh chords and `offset_meters` onto the same form. — (NURBS orchestrator)
- 2026-10-26 — (tracker sweep) Every live row re-read against main `bb25fa4f`. All 12 are still real, and none closed.
  - `a-seamless-band-…`: its `LoopWraps` text is gone. `ring_path`'s docs now cite this row for the premise they rest on, and a re-read note says what is still owed.
  - `D305`: `TubeScale` and `rational_arc_chain` are gone. A re-read note lists the sole-bound helpers that stand.
  - The plan's slate table now lists all twelve rows. Three rows carry no band or cost (`pcurve-envelope-escalates-at-interval-on-a-wide-arc`, which has P2 but no cost, `pcurve-certify-error-arms-with-no-ending`, `a-seamless-band-…`).
  - Not folded into PCERT: PCERT is active and at 97.5/30.
- 2026-10-26 — **Moved here from CLEAVE** by `git mv` in CLEAVE's priority-seam cut (Ev, in chat, 2026-10-26: CLEAVE at 270.5 budget points against 30), ids and bodies unchanged, each row with a dated `## Slate triage` note giving its band: `azimuth-period-gate-escalates-between-two-pass-readings`. P2 rather than this slate's P3 tail: an in-band headroom between two passing readings is interval and error propagation (`work/README.md` Priority). PCERT itself is active and over budget, so it comes here. (CLEAVE orchestrator)

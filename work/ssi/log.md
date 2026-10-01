# SSI — the log

## 2026-09-20 — opened

Cut out of CHART, which was carrying 105 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CHART's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

7 rows arrived by `git mv` with their ids, bodies and history
unchanged. CHART keeps its band 6200-6299; band 7500-7599 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

- 2026-09-28 — Seam note from ENCL: PR 3348 (merged `95b59b9361`) homes the domain-uniform refinement grid in `geom_core::spline::algebra::domain_grid_points(kv, pieces, GridSkip)`. `GridSkip` is `BitEqual` or `WithinUlps(u32)`, and `pub const SLIVER_CLEARANCE_ULPS` replaces `quad.rs`'s private `SLIVER_CUT_ULPS`. It is used at `props/quad.rs` `refine_dir` and `bezier_blocks`, `ssi/certify.rs` `refined`, `edge_nurbs.rs` `localized::breaks`, and one tcost/tint test. Bits are unchanged at every site (pinning rows added first). Each caller still chooses its own skip guard and control-count cut-off, so the NURBS hairline fix is now a one-argument change at each site. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3351 (merged `e39a5c4cc4`) routes certification refusals per D4 ¶1 as Ev ruled on PR 3352. Recourse belongs to the decision (`CertCheck::ending()`, one table) and the reading belongs to the door: `geom_brep::certify::recourse(check, RefusedArm, Reading::{Build, AtRest, Adopt})`. `CertifyError`/`PlaneNurbsRefusal` `Display` is now payload-only. Each door appends `ending(reading)`. The `certification: ` prefix is gone. `work/ssi/plane-nurbs-certificate-escalation-does-not-name-its-limb.md` is filed on your slate (`SsiError::Escalated` carries no limb; the poisoned transversality aggregate reaches the last resort). (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". Filed on your slate: `work/ssi/ssi-transversality-death-says-lower-the-tolerance.md`. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3392 (merged `277dcb052b`) splits certify's conflated Zero/Negative verdicts: `IntervalNotForward { verdict }`, `WindingExceeded` routed as sign-certain, the tangent tube as its own `CertifyError::TubeNotSeparated` (`CertCheck::TangentTube`, lever alone at every reading), and `TubeStraddles { verdict: Refused }`. `RefusedArm::ZeroOrNegative` is deleted; `geom_brep::recourse` now holds `Refused` and `Definite`. A zero span stays a defect at every reading (Ev, e1600790f9). `ssi.rs`/`ssi/certify.rs`: `SsiError::TubeStraddles { verdict: Refused, boxes }` via `tube_transversality`, pinned at the site. Your row `plane-nurbs-certificate-escalation-does-not-name-its-limb` still says the tube routes as `ZeroOrNegative`; that variant is gone. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

## 2026-10-01: picked up

An orchestrator holds the track (`status: active`). I re-priced every
row: the four unpriced rows, the two legacy `D` rows, and three `H` rows
whose fix is already written down (`lever-arm` and `stretch` to `M`) or
which are only reproductions (`rigid-map` to `M`). The track went from
35.5 to 29 points, so it is under budget without a split. I gave the
three ENCL-filed rows bands: escalation and transversality-death are P1
(a refusal routed to the wrong ending), and the prose row is P4. Units
and review tiers are in `plan.md`. The first wave is `ssi/lever-arm-fold`,
`ssi/diagnoses`, and a designer pair on the chart speed. I combined the
four refusal rows into one PR because runners are a budget. The
alternative was four PRs on one file.
- 2026-10-01: chart-speed designers, round 1 (labels A/B; the mapping is on `analysis/design-fork/ssi-chart-speed`). They agree that the chart speed is minted once as a per-axis pair, with outward rounding through `norm_sup`, and that round-to-nearest `Box3::speed_sup` goes. The mint refuses a zero or non-finite speed and names the axis. Usability is checked where the derived floor or step is minted, not at the speed. The stepper's `tangent_speed` is a different quantity from the chart sup, which corrects `ssi-chart-speed-usability-boundary`'s premise. They diverge on two points. (1) The certificate: A derives the proved region from the radius by one function; B records a per-kind tube that carries the chart pad, because a metre radius over-states what the chart arm proves. (2) Where NaN is caught: A has a checked `to_param`; B has `new` map poison to the vacuous bound. Round 2 hands each the other's report. The off-question findings are filed or routed once the round closes. The `offset_meters` NaN-dropping `max` folds went to the lever-arm lane's sweep.
- 2026-10-01: chart-speed designers, round 2. They CROSSED on both divergent points: A moved to a per-kind tube that records the chart pad, and to `new` mapping poison to the vacuous bound; B moved to a derived region with corrected docs, and to a checked `to_param` plus a NaN-keeping `SupSpeed::max`. Both now agree that on the chart arm `tube_radius` is a ladder rung, not a proved metre radius, so the current certificate doc is false there. Round 3 per Ev's 2026-09-30 rule: each is shown the other's revision and asked for the question beneath both. The agreed core holds either way: one outward `norm_sup`; one per-axis mint that refuses by axis; a floor door that refuses a floor the domain cannot resolve; the stepper guards its own step; `NurbsBoxes::cells` refuses a NaN window.

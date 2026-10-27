# BIND — the log

## 2026-09-20 — opened

Cut out of LIB, which was carrying 67.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed LIB's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

22 rows arrived by `git mv` with their ids, bodies and history
unchanged. LIB keeps its band 300-399; band 9600-9699 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-09-29 — Seam note from ENCL: PR 3390 (merged `719ef596a1`). The shell volume-sign decision is sized (`geom_brep::recourse::SizedPass::NonZero`, the offer valued at |m|/K). `ShellClassifyError` gains a `Straddles` arm and a `payload()` data view, and its `Display` ends in its decision's one ending. `recourse::UNREADABLE_MARGIN_NOTE` is the shared unreadable-margin sentence. pncad-py `py/checks.rs`: the `straddles` tag. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-10-17 — Seam note from WIRE: `a-split-touching-at-a-notch-apex-mints-a-vertex-twice` moves here from `work/issues/`, retitled to its pncad-py half (the prose gate panics on a `Debug`-rendered refusal); its naming half is WIRE's split-pinch row. (WIRE orchestrator)

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) edits `pncad-py/tests`. `oversized_round_loft` in `test_validate.py` is now 64× (8 km); the 125 m tube certifies once no lane refuses at round 0. The test is kept. Two `test_north_star.py` comments are re-worded (the spout at 1e-12 stops at its work wall). No binding change. (QUAD orchestrator's lane)

- 2026-10-23 — **Seam (QUAD):** PR 5628 (`quad/measure-b-door`, measure door unit B) `pncad-py` is not edited. `topo::Settle` exists from this PR and is public, and Python taking it as data is unit C's. (QUAD unit B lane)

- 2026-10-26 — seam from ATREST (stage 5 C, `the-at-rest-census-is-a-check`, branch `atrest/the-at-rest-census-is-a-check`): pncad-py: `assemble`, `Assembly`, `AssemblyError`, `AtRestFinding` and `ErrorClass::Assembly` go; `CheckId.AtRest` replaces `Separation`; `AtRestEvidence` is new; `CheckFinding` gains `quiet` and `census`; `InterferenceFinding`/`CarriedInterference` are read off the report's AtRest findings.
- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-27 — **Seam (ATREST):** `atrest/an-instance-carries-its-parts-coincidences` touches this program's ground. `py/checks.rs`: new `CarriedCoincidence` pyclass beside `CheckFinding`; `py/assembly.rs`: `route_fields` is `pub(super)` so it is shared.

# OFFSET — the log

## 2026-09-20 — opened

Cut out of SHELL, which was carrying 65.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed SHELL's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

5 rows arrived by `git mv` with their ids, bodies and history
unchanged. SHELL keeps its band 2300-2399; band 8100-8199 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## Announced seam from FIX (2026-09-21) — PR 2948

FIX's `recourse-chain-stops-at-the-second-hop-carriers`, the last row
of its wave-4 slate. An arm whose `Display` renders a carried error
whole contributes no recourse of its own, so *"this message names a
repair"* is a claim about the carrier all the way down. Four carriers
gained repairs and an enforcement row each, every repair grounded in the
module's or the variant's own docs rather than invented, and all of them
**proved red by mutation** (run 35548044980 — twelve `test (…)` jobs
red, failure surface exactly the intended rows).

**Your ground:** `crates/geom-brep/src/offset_meters.rs`, which
territory names as ENCL'"'"'s, OFFSET'"'"'s and SHELL'"'"'s together.
`MeterError::NormalFloor` and `CurvatureHeadroom` gained repairs — the
floor-of-exactly-zero case now says to split the face clear of the
degeneracy, and points at `OFFSET_METER_LADDER` for the regular-patch
case, on that constant'"'"'s own sentence that the refusal names the
numbers "so that consumer will know". `Escalated` is unchanged in shape
and asserted transitively at all three `MarginDiag` arms.

Signed (FIX orchestrator).

- 2026-09-28 — Seam note from ENCL: PR 3332 (`encl/rigid-map-approx-headroom`) edits `crates/topo/src/transform.rs`'s `map_approx`. When a rotated Approx face's limb check refuses and the original face certifies in its own frame, `map_approx` now re-fits the mapped description through `OffsetFitLane::mint` at the same ε instead of refusing. Every map that succeeded before still ships the image bit for bit. Residues filed on encl: the 1e-12 re-fit stall, and the edge/meter refusals a re-fit cannot answer. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3351 (merged `e39a5c4cc4`) routes certification refusals per D4 ¶1 as Ev ruled on PR 3352. Recourse belongs to the decision (`CertCheck::ending()`, one table) and the reading belongs to the door: `geom_brep::certify::recourse(check, RefusedArm, Reading::{Build, AtRest, Adopt})`. `CertifyError`/`PlaneNurbsRefusal` `Display` is now payload-only. Each door appends `ending(reading)`. The `certification: ` prefix is gone. `transform.rs`: `TransformError::Certify` appends the Build-reading ending. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". `offset_meters.rs`: `Meter::{NormalFloor, CurvatureHeadroom}`; `MeterError` carries `Refused::{Zero(Classified), Negative{margin}}` and renders its ending per reading; "or lower the tolerance" and the name routing are gone. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3392 (merged `277dcb052b`) splits certify's conflated Zero/Negative verdicts: `IntervalNotForward { verdict }`, `WindingExceeded` routed as sign-certain, the tangent tube as its own `CertifyError::TubeNotSeparated` (`CertCheck::TangentTube`, lever alone at every reading), and `TubeStraddles { verdict: Refused }`. `RefusedArm::ZeroOrNegative` is deleted; `geom_brep::recourse` now holds `Refused` and `Definite`. A zero span stays a defect at every reading (Ev, e1600790f9). `offset_meters.rs`: `Refused` moved to `geom_brep::recourse` (import path only). (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/offset_axial.rs`. `offset_axial.rs` states each re-charted face's own bit (the door kept it before). (TOPO implementer)
- 2026-10-01: Seam note from SSI. PR 3653 routed `offset_meters` `cell_normal`'s `dn` through `geom_core::interval::norm_sup` (and thinness and the regularity margin through `div_down`), so `offset-meters-cell-normal-midpoint-direction-is-an-f64-fold` looks closed in substance; there is a note on the row. (SSI orchestrator)

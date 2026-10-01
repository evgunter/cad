# STRUT — the log

## 2026-09-20 — opened

Cut out of CARVE, which was carrying 73 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CARVE's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

11 rows arrived by `git mv` with their ids, bodies and history
unchanged. CARVE keeps its band 5600-5699; band 7900-7999 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `sweep/src/extrude.rs`. `extrude.rs`'s walls state `wall_sense` in their spec (the post-mint `set_face_sense` is gone). (TOPO implementer)

- 2026-10-01 — Seam note from AUTHOR: Ev ruled on #3551 that an extrude's distance is a positive depth with a structural `side`, and that a negative depth refuses with a recourse naming `side`. The work is filed on EDIT as `extrude-distance-is-a-depth-and-a-side`. Its kernel half is `sweep::Extrusion`/`ExtrudeError` (CARVE/STRUT) and its eval wiring `wire_extrude` (WIRE). The rule's follow-ons are `carve/revolve-angle-is-a-signed-size-beside-a-directed-axis` and `edit/pattern-spacing-is-a-signed-size-beside-a-direction`. (AUTHOR orchestrator)

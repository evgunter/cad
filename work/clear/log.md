# CLEAR — the log

## 2026-09-20 — opened

Cut out of SHELL, which was carrying 65.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed SHELL's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

9 rows arrived by `git mv` with their ids, bodies and history
unchanged. SHELL keeps its band 2300-2399; band 8000-8099 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-10-02 — Seam note from AUTHOR's exit: `clearance-refusal-names-one-face-twice-across-bodies` has a viewer consumer parked on it, DOORS's `the-gui-has-no-clearance-consumer` (P1 H, moved from `work/author/`). When it closes, set that row `open` and DOORS `ready`. (AUTHOR orchestrator)
- 2026-10-06 — Two rows arrive from SHELL's triage, by `git mv`, ids unchanged: `shell-curved-wall-clearance-window` (parked on SHELL-4, which says it closes it — 0 points) and `violated-witness-can-sit-off-the-trimmed-face` (re-priced P3/E: `verify_witness` now filters through `certified_off_the_face` where `chart_arms` answers, so what remains is a regression row pinning the two r2 fixtures; the cone/sphere/torus residue is CHART's `clearance-window-cone-sphere-torus`). (SHELL orchestrator)

## 2026-10-06 — two rows filed from CARVE's designers

CARVE's designers on the loft's section placement both found that the
clearance engine cannot yet certify a loft's embedding, and that
neither gap was on a slate:
`self-intersection-drops-every-vertex-sharing-face-pair-globally` (P2,
H, design open) and `window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch`
(P2, M). With SHELL-3, they are what a loft-door embedding certificate
would need. CARVE will say here if its design lands on them.

Signed: (CARVE orchestrator)

- 2026-10-24 — Note from CHART: `min-separation-tightening-crosses-the-drive` moved onto this slate by `git mv` at CHART's priority-seam cut. Its fix shape is all on `clearance.rs` and `measure.rs`, and its ruling (may a lane split be invisible to the census) is Ev's. See the row's `## Re-homed` section. (CHART orchestrator)

- 2026-10-25 — **Band rule (work/README.md, Priority):** `SHELL-3` re-banded P1→P0, and `window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch` and `self-intersection-drops-every-vertex-sharing-face-pair-globally` P2→P0. They are the three prerequisites the P0 `self-overlapping-spines-build-and-validate` (CARVE) is parked on, and each one's fix as described is what that loft certificate needs: the engine in `topo`, a window for an iso-bounded loft wall, and the vertex-sharing pairs examined (without that, a triangle-section loft has no examined pair). (band-rule triage)

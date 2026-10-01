# LINALG — the log

## 2026-09-20 — opened

Cut out of PROPS, which was carrying 108.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed PROPS's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

8 rows arrived by `git mv` with their ids, bodies and history
unchanged. PROPS keeps its band 2400-2499; band 9400-9499 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

## 2026-10-01 — picked up; #2468 is landing, not holding

An orchestrator holds the track (status `active`). Ev asked on #2468
"what's up with this?"; the answer, posted there: the hold's cause
cleared 2026-09-22 when DECIDE-3 (#3039) merged into `props/sign-hull`,
and the PR then sat with no owner — this row moved here on 2026-09-20
and LINALG had no orchestrator — while DECIDE used the branch as its
base (SYM-9, DECIDE-4…7 merged into it; DECIDE-8 #3282 open against
it). Last main merge 2026-09-21; 34 files conflict against `main`;
no hosted run since 2026-09-20, so the post-DECIDE-3 green has never
been seen hosted.

Dispatched: a lane merges `main` into `props/sign-hull`, resolves the
34, takes the two seam measurements the merge owes (SYM's
`the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic`;
PATHS's `arc_span` against `turned_span`, `work/decide/log.md`
2026-09-25) and drives hosted CI green. **Review tier: single FULL
review of the merge resolution** — every unit riding the branch was
reviewed at its own merge, and what has not been read by anyone is
the resolution of 34 files against ten days of `main`, where a wrong
side taken compiles fine. Then land; DECIDE-8 re-targets to `main`.

Process finding: a held PR became another program's integration
branch, which turned "unblocked" into a 3 900-commit merge debt with no
owner. Nothing new targets `props/sign-hull`.

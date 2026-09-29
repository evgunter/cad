# PROBE — the log

## 2026-09-20 — opened

Cut out of TOPO, which was carrying 132 budget points in one directory
— about four and a half sittings — when Ev ratified the priority and
track-size conventions in chat the same day. The cut divided TOPO on
its PRIORITY seam rather than on another territory seam, per
`work/README.md` "Track size": five successors plus the Euler-operator
remainder TOPO keeps.

12 rows arrived, each by `git mv` with its id, body and history
unchanged. Band 6700-6799 claimed in this commit
(`docs/MODEL-AB-LOG.md`). Nothing dispatched.

## Announced seam from TOPO (2026-09-24)

TOPO's `kevs-fan-merge-needs-a-re-describing-kill-door` (branch
`topo/kev-describing-door`; PR title "TOPO: kev refuses a merge that
would strand a carrier; kev_describing takes the re-descriptions")
lands Ev's ruling (c) on PR 2527. `Body::kev` stays keys-only and now
refuses, before mutating, a fan merge that would re-base a certified
edge onto the surviving vertex (`EulerOpError::MergeRebasesCarriers`,
naming every such member) or move one end of a null edge
(`RebasedNullEdge`). It carries a merge that moves nothing: an empty
fan, or a killed null edge. `Body::kev_describing(he, &[(EdgeKey,
EdgeCurveSpec<T>)], tol)` is the kill that takes a band and the merged
members' re-descriptions. A listed member is certified at the merged
endpoints. An unlisted one passes the re-basing gate `mev`'s fan site
passes. `kev_describing(he, &[], tol)` is the kill with a band and
nothing re-described.

**Your file, and what changed in it: `crates/topo/src/seqgen.rs`.**

- The walk's `Kev` arm (`apply`) and `teardown`'s kill take
  `kev_describing` with `chord_redescriptions`. That helper states
  every merged member as the chord between its merged endpoints, or as
  the canonical self-loop circle where the merge closes it onto one
  vertex. The walk's fan merges no longer leave a stale carrier behind.
- `roundtrip`'s `SplitEdge` inverse was `kev` followed by
  `set_edge_curve`. It is now one `kev_describing` call, handed the
  parent's captured spec.
- `split_site`'s "Why the re-certification" filter no longer filters.
  No op the walk applies leaves an edge whose carrier its own
  endpoints left, so the filter is now an assertion.
  `assert_run_site_refuses`'s stale-carrier skip became an assertion
  for the same reason. On the 64 x 32 pinned streams, a stale edge was
  present at 734 of 2048 steps on the merge base and at 0 steps at the
  head.
- `selection_is_pinned_over_a_fixed_stream_set` was re-pinned from
  `4_401_414_259_635_546_876` to `8_153_169_425_937_027_252`. The walk
  moved because merged members are now splittable, so the split
  filter's rejections stopped. The module docs' measured numbers were
  re-taken:
  - `Kev` selections went from 136 to 138: 41 strut or segment kills,
    28 mirror, 69 general.
  - `MevFan` selections went from 374 to 370.
  - The run-site refusal was reached 283 times, against 237 before.
  - `SplitEdge` selections went from 169 to 176.
- `chord_redescriptions` now `expect`s a fallible core,
  `try_chord_redescriptions` (`pub(crate)`). It returns `None` where a
  torn arena will not let the chords be read. `review_d18`'s hammer
  and `review_m1_pr4`'s torn-body row use it to drive the describing
  kill where the keys-only one refuses the merge in its plan phase, so
  their attack on the shared mutation phase stays in scope.

`S93` (`work/probe/S93.md`) closes with this unit; the orchestrator
closes it at merge.
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

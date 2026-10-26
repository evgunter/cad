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

## Seam from TOPO, fix pass on PR 3161 (2026-09-29)

`Body::kev_merged_members(he)` is new and public: the merged members
of `kev(he)`, in the dying vertex's orbit order, each with the two
endpoints the merge gives it (`topo::MergedMember { edge, start, end }`,
`he_plus` forward order). It is read from the same plan and endpoint
reading the two kill doors certify against, so a caller no longer
re-derives what the merge will do. `kev`'s plan phase now also refuses
`OrbitBroken` where the dying vertex's orbit reaches a half-edge that
does not start there (two `next` tears could walk it through the killed
half, and the describing door then panicked); on a valid body nothing
changes.

**Your file, and what changed in it: `crates/topo/src/seqgen.rs`.**

- `chord_redescriptions` and `try_chord_redescriptions` map
  `kev_merged_members` through one `chord_of` (a line, or the
  self-loop circle where the two merged endpoints are one point); the
  walk's own orbit and endpoint re-derivation is gone.
- The walk's `Kev` arm now asks two more questions on a clone before
  it kills, so the gates the walk's full list never reaches are fuzzed:
  - `assert_keys_only_kill_answers`: plain `kev` must refuse
    `MergeRebasesCarriers` naming every member where the fan is not
    empty, and must kill where it is.
  - `assert_unlisted_members_answer_to_the_gate`: a subset of the
    members, drawn from the lattice counter (at least one), is left
    unlisted, and the describing kill must name the first unlisted
    member whose stored carrier does not re-certify at its merged
    endpoints, or kill where none fails.
  Both run on clones, so the walk, the selection pin and the module
  docs' counts do not move.
- `split_site`'s docs no longer say it asserts over every generated
  edge: `any_split_edge` stops at the first splittable one.
- `review_m1_pr4`'s torn-body row no longer calls the chords helper; it
  asserts that both kill doors refuse every kill on its tear.

## `S93` closed by TOPO (2026-09-29)

`S93`'s `kev` half closed with PR 3161, and the row is closed there
too, as its brief said it would be. The generator's `split_site`
filter now asserts rather than filters, and `seqgen.rs`'s seam is
noted above (PR 3161's fix pass).
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/review_d18.rs`, `topo/src/review_d18_probes.rs`, `topo/src/seqgen.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)
- 2026-10-11 — From TOPO's second cut: `merge-faces-should-panic-rows-on-a-debug-assert-fail-in-release` (priced P4 E at the move), `kef-and-kfmrh-fuse-roundtrip-skips-are-admitted-per-kind`, `revert-review-residue-test-shape-and-row-text` and `census-face-walks-skip-torn-hops-silently` moved here: guards that cannot go red. (`work/topo/log.md`)
- 2026-10-26 — (tracker sweep) **PIN merged in.** All fifteen PIN rows moved here, ids kept, and PIN's program, plan and log are deleted (`docs/doc-ledger/pin-leaves-the-tracker.md`). The charter and plan now cover both halves; the load is about 23 of 30.
  - **Closed, PROBE's own:**
    - `bits-witness-slice-workaround-outlived-its-defect`, superseded: `bits_witness` and `topo/src/source.rs` are gone;
    - `census-snapshot-arm1-discard-has-no-justification`, already fixed: the census walks loops through the audited `face_cycles`, and `snapshot` states its reason;
    - three fixed by this sweep's edits: `censussubject-eq-answers-false-for-a-new-variant-against-itself` (`validate.rs`, mismatches spelled out), `fused-two-shell-body-doc-predates-movefac` (`euler_ring.rs` doc) and `review-d18-probes-header-miscounts-its-own-rows` (header count).
  - **Closed, PIN's:**
    - `rows-do-not-cross-a-boolean-remap`, superseded by D10's declared-pair retirement;
    - `census-at-rest-two-boolean-lane-premises`, `edge-chord-len-defaults-to-one-metre` and `boolean-smooth-arm-rebuilds-line-carrier`, already fixed on main;
    - `boolean-mod-doc-links-a-feature-gated-variant`, fixed by this sweep (`boolean/mod.rs:33`).
  - **Notes added:** `D284`, `flush-pair-relation-has-no-caller` (its code citation re-pointed to `work/probe/`) and `join-probe-charts-hand-roll-the-across-axis-reference` (half fixed).
- 2026-10-26 — (easy-fixes lane) Closed `merge-faces-should-panic-rows-on-a-debug-assert-fail-in-release`: both rows are gated on `debug_assertions`.
- 2026-10-26 — (easy-fixes lane) Closed `join-probe-charts-hand-roll-the-across-axis-reference`: `cylinder_at` and the coaxial-circles probe read `orthonormal_basis`. Four siblings are filed on HONE.
- 2026-10-26 — `topo-test-frames-hand-roll-the-across-axis-reference` (P4 E) arrives from HONE, where the easy-fixes lane filed it; its sibling `join-probe-charts-hand-roll-the-across-axis-reference` is here.

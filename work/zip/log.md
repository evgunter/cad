# ZIP — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

7 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7200-7299 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

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

**Your files, and what changed: `crates/topo/src/boolean/zip.rs` (`record_kev`) and `crates/topo/src/boolean/rest.rs` (the slit fuse's kill in `zip_folded`), the latter shared with TANG.** Each of these kills merges
two vertices that the section or the fuse put a band apart, across a
certified circle, so the merged fan's carriers still end where they
land. Each kill now takes `kev_describing(he, &[], tol)`, which
re-certifies every member under the run's band. The keys-only kill
takes no band, so it would refuse these merges. Measured by
instrumenting `kev` across `cargo test -p topo` and
`cargo test -p sweep` on the merge base: every member at these sites
re-certifies within band, including the zip's 58 ulp-distinct merges.
The strut-undo kill in `boolean/rest.rs` kills a null edge, whose two
vertices hold one point, so it stays keys-only.

Also `crates/topo/src/merge_faces.rs`, which TOPO and ZIP both claim.
The three new variants go into `OpPlacement::of`'s enum-verdict arm.
The `kev` row of its site table now says the fan-merge refusals cannot
fire at `strut_tip`'s site, whose far vertex has valence one.
## 2026-09-26 — note from CONTACT (CONTACT-2, PR 3250)

CONTACT-2's lane filed or edited three rows on your slate:

- `an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`
  (new). The axis-coincident box lap's remaining refusal; it
  reproduces on an all-planar diamond at the base.
- `role-resolution-interior-tiers-certify-only-planar-region-faces`
  (new). Its review turned the open question into a reproduced
  defect: a curved edge's chord-midpoint anchor reads both loops alike
  and the join refuses `SectionLoopMixed`
  (`crates/sweep/tests/axis_lap.rs`
  `a_chord_midpoint_probe_reads_both_loops_alike`). The lane set it to
  P0/H. Re-band it if you read it differently. It refuses loudly
  today; it is not a wrong answer.
- The top-entry blind D pocket stays on
  `blind-d-pocket-subtract-refuses-with-join-internal-words`. The
  bottom-entry pocket now builds, pinned in `axis_lap.rs`.

CONTACT-2 also touched `boolean/join.rs`, where the anchor tiers are
now named by an `Anchor` enum.

Signed: (CONTACT orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3160 (`half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`, branch `topo/mint-rows-at-the-mint-site`) adds one variant to `EulerOpError`, `PcurveMint` (an Euler operator refusing to add a half-edge to a face whose pcurve rows are complete on a spline chart), and so one arm to `OpPlacement`'s exhaustive match in `crates/topo/src/merge_faces.rs`, beside `PcurveSplit`. Nothing else in the file moved; the path is a double claim (TOPO and ZIP). (TOPO lane)
- 2026-09-29 — Seam note from TOPO: PR 3493 (branch `topo/route-refusal-subjects`) routes the Boolean's escalated and contradicted refusals by closed decision types (D4 ¶1 (i), PR 3352). `join.rs` and `rest.rs` construct their `BooleanError::Escalated` through `BooleanError::coincidence` (the decision `BooleanDecision::Coincidence`); `merge_faces.rs`'s `DeclarationContradicted` carries the `Contradiction` its declared rung set. (TOPO implementer)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/boolean/rest.rs` and `topo/src/merge_faces.rs` (a double claim, TOPO and ZIP). `boolean/rest.rs`'s transient `mfkrh(Inherit)` promotions now mint the parent's bit negated; `merge_faces.rs`'s `OpPlacement` match gains one arm, `SenseContradictsChart`. No row moved. The PR also files `slit-zip-band-run-across-two-loops-is-reached-by-no-row` on this slate: `slit_zip`'s band-run promotion is reached by no row. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: PR 3506 (branch `topo/torus-and-merge-one-story`, not yet merged) edits `boolean/rest.rs` and `merge_faces.rs` (a double claim). The Rest verify's declared pairs route a plane-rung escalation by `PlaneDoor::Declared` (a defect on the unreadable norm, `PLANE_ORIENTATION` on orientation). `MergeCoplanarError::Escalated` carries a `MergeDecision`, and the declared pair's orientation is the merge's own decision (it passes only on a same-facing pair), which `DeclaredOppositeOrientation` now ends in as its definite arm, with no label or face keys. (TOPO, PR 3506 fix pass)
- 2026-09-30 — Seam note from TOPO: PR 3513 (branch `topo/every-escalation-names-its-decision`) names the join's escalations `Coincide::Join`/`Coincide::Section` and routes the germ frame's radius guards to `BooleanDecision::Radius` (`join::frame_refusal`); the join's matching decisions are filed at `work/topo/boolean-coincidence-route-still-holds-join-and-self-check-decisions.md`. (TOPO implementer)

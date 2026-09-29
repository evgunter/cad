# BAND — the log

## 2026-09-20 — opened

Cut out of CARVE, which was carrying 73 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed CARVE's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

9 rows arrived by `git mv` with their ids, bodies and history
unchanged. CARVE keeps its band 5600-5699; band 7800-7899 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
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

**Your files, and what changed in them.**

- `crates/sweep/src/blend/surgery.rs` (BAND's and CARVE's): the two
  closure kills take `kev_describing`. `rim_phase`'s is
  `"rim closure kev"` and `rim_phase_annulus`'s crossing kill is
  `"annulus closure kev"`; the refusal site names are unchanged. Each
  hands its one merged member the chord between the endpoints the
  merge gives it (`EdgeCurveSpec::line_between`, `merged_chord_spec`),
  the scaffolding the surgery's struts and trims carry. That member is
  the upper meridian remnant, or the mate seam's rim-side piece. The
  final pass still states the slit as the band's seam, the meridian
  arc, through `attach_contact`. (The first push handed the arc under
  arc scaffolding; at the certified scalar that scaffold's residual
  enclosed wider than eps = 1e-12's band and the kill escalated, so the
  interval 1e-12 rows went red. The chord encloses at ulps.) Without
  this change these were 118 of the 129 sweep refusals S93 measured.
  At the head, `cargo test -p sweep --lib --test all` shows 1746
  passed and 7 ignored, at default eps and at 1e-12.
## 2026-09-25 — first sitting: taken, wave 1 dispatched

Taken by the BAND orchestrator; `status: active`. The review-posture
question the plan left open is gone: the A/B it inherited is
suspended, so review is per unit by the orchestration tiers.
`in-band-corner-verdicts-…` arrived unpriced and is P1/E (one routed
sentence), which puts the slate at 30/30.

Wave 1, three lanes on disjoint ground, each a FULL single review
(each changes what a refusal or a name says, where believing it takes
more than reading it):

- `ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` — branch
  `band/ruled-d-hole-ring-crease`.
- `blend-slit-name-collides-when-two-rims-share-a-meridian` — branch
  `band/band-slit-discriminator`. Announced seam: EDIT's and EMIT's
  `names/role.rs` and EDIT's `names/emit_blend.rs`.
- `subdivided-profile-side-coplanar-walls-gate` — branch
  `band/subdivided-side-walls`. Announced seam: CARVE/STRUT's
  `extrude.rs`, TOPO's boolean gate if the answer is a refusal.

Decided without Ev (sequencing, with a recommendation):
`sweep-emits-no-contact-record-for-declared-cusps` closes by
`Extruded` carrying the declarations, not by ratifying caller
ownership (reason in the plan), and runs after the subdivided-side
row since both are `extrude.rs`'s lowering. `S90-impl` waits on
SCALAR's LANE-4 (#3194).

Ev, in chat the same day: the review posture is the per-unit tiers
("it is indeed replaced by the per-unit review tiers"), and
`Extruded` carrying the declared cusps is the close ("carrying the
declared cusps sounds great").

## 2026-09-25 — `subdivided-profile-side-coplanar-walls-gate` closed (PR #3244)

Measured, no kernel change; reviewed by the orchestrator's read (a
test suite and three filed rows, nothing a reader has to take on
trust). The item's own premise was backwards: extrude and revolve DO
give a declared continuation's walls one surface key; what refuses is
that nothing merges them before the boolean, and the recourse the
refusal names (`merge_coplanar_faces`) has no door above
`topo::Body`. Filed: `swept-continuation-walls-reach-the-boolean-unmerged`
(P0, the fork — goes to Ev, `needs_ev`), `subdivided-rim-fillet-refuses-at-the-collinear-joint`
(P1), and CARVE's `loft-walls-keyed-per-segment-on-a-declared-carrier`
(P2). Blind spot the lane named: `revolve/tube.rs` unmeasured.

The fork goes to Ev as an `[ev]` PR editing F7. The declared-cusp row
(`sweep-emits-no-contact-record-for-declared-cusps`) was waiting on
this lane's `extrude.rs` ground and is now free to dispatch.

## 2026-09-25 — the continuation-merge fork ruled

Ev ruled option 1 on the `[ev]` PR: sweeps over a declared straight
continuation run the structural merge as a documented final stage
(F7 amended in the same PR). `swept-continuation-walls-reach-the-boolean-unmerged`
becomes the implementation row; it dispatches after the declared-cusp
lane lands, because both change what `Extruded`'s face handles mean.

Ev also pointed at the new orchestrator protocol (PR #3247: a designer
pair weighs every design fork before its `[ev]` PR, and orchestrator
branches keep the program prefix even when the harness pins another).
Applied from here on, not retroactively; this sitting's orchestrator
branch moves to `band/orchestrator`.

## 2026-09-25 — note from GERM: one merge seat for the sweep

GERM found that a FULL revolve of an axis-touching profile emits every planar
wall as two same-key halves (`work/carve/full-revolve-emits-split-planar-walls.md`),
and has put it to Ev. Both designers recommend the same fix: the revolve runs
the structural merge as a final stage. When `swept-continuation-walls-reach-the-boolean-unmerged`
is implemented, please make it ONE final-stage merge call at the end of the
sweep, not a continuation-runs-only targeted merge. Otherwise the full-revolve
row has to widen it again. — (GERM orchestrator)

## 2026-09-25 — `ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` closed (PR #3243)

Built rather than re-refused: `chord_site` cuts in whichever cap cycle
(outer or ring) carries the keyed half-edge, so a ruled crease ending in
a cap's ring carves. Full single review. Its one MAJOR: the lane's claim
that ring creases are concave-only was false — a keyhole gives a convex
one, which this PR turns from a refusal into a carve, and a bore inside
the removed sliver then carves silently wrong. Taken as: land the build
(the outer-cycle route to the same silent-wrong body is already on main)
and close the defect as the NEXT unit; the keyhole is pinned as an
ordinary row. Filed by the lane and its fix pass:
`ruled-cut-off-leaves-a-cap-ring-inside-the-removed-sliver` (P0, now
with both routes), `body-not-intact-renders-every-detail-as-a-reference-that-did-not-resolve`
(P2), ZIP's `blind-d-pocket-subtract-refuses-with-join-internal-words`,
ATREST's `check-9-does-not-check-a-ring-nested-inside-another-ring`, and
evidence on CONTACT's `axis-coincident-lap-trips-the-planar-join-invariant`.

GERM's note above (one final-stage merge call at the end of the sweep)
goes into the continuation-merge unit's brief verbatim.

## 2026-09-26 — `blend-slit-name-collides-when-two-rims-share-a-meridian` closed (PR #3245)

The teapot lid's rims roll in ONE `Node::Fillet`. The kernel record
(`BlendNaming::slits`, `meridian_splits`) carries the slitting band's
source edges, and `RoleSeg::BandSlit`/`BandCross` carry that set as a
discriminator — option (a), because N4 names from birth data alone. The
item undercounted: `BandCross` collided one row later the same way.
Full single review, no MAJOR; its fix pass took the reviewer's rows
(the one that matters: an unrewritten `band` silently retargets a held
slit name, and nothing else in the suite saw it), made `band` a
discriminator in `walk_names` rather than a derivation, and gave the
band identity one home. `annulus-rim-phase-…` gained the note that its
hand-kept retain is now load-bearing for `BandCut` uniqueness. Filed:
INSTR's `tess-budget-baseline-names-predate-piece-naming`.
## 2026-09-26 — `sweep-emits-no-contact-record-for-declared-cusps` closed (PR #3257)

`Extruded`, `Revolved` and `Lofted` carry `declared_contacts`, one
`Tangent` pair per declared cusp joint. Full single review, then a fix
pass that went past the review's recommendation on the orchestrator's
call: the review said land and file the upstream move; instead the
validated profile now keeps the cusp/smooth kind the author declared
(`ValidatedLoop::cusp_joints`, decided once in `judge_joints` through
the path door's own predicate, recorded in the guided replay), so the
sweep reads it and its duplicate predicate and three public error
variants are gone; `blend_arcs` stopped listing an arc between cusps.
Announced seam: PATHS (`crates/profile`). Delta review: mergeable, two
MINORs filed as `declared-joint-kind-zero-margin-reads-smooth`. GATHER's
`product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares` now
has its measured red-first row.
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". Filed on your slate: `work/band/blend-endings-say-lower-the-tolerance-and-route-by-name.md`. `sweep::blend::ClassifiedMargin` is a third spelling of a decided margin beside `recourse::Classified`; convergence is noted on the encl certify-span row. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

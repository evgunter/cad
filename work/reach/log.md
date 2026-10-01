# REACH log

## Opened at CURVED's cut (2026-09-20)

Opened by CURVED's orchestrator on Ev's in-chat direction (break the
two programs into smaller tracks; leave in CURVED and TRIM only a chunk
sized to finish this session; no `[ev]` PR needed, it is mostly moving
issues around). Forty-nine items moved from `work/curved/` — S-BOOL's
residue re-homed onto CURVED at S-BOOL's 2026-09-16 exit, plus
CURVED's own operand-reach, merge-door and census lanes. Band 6000–6099
recorded in the ledger's banding entry in the same commit. No unit
dispatched; the first sitting picks from `work/reach/plan.md` §Lanes.
Two items carry orchestrator context worth knowing: `rest-zip-seam-chord-on-cylinder-wall`
was filed by the merge-door unit's STOP-2 re-scope (#2105 — the
chord pre-exists on the merge base; both reviewers executed the
honesty crux), and `the-chord-dip-charge-has-two-homes` was half-fixed
by the torus arm (#2535; the `geom-brep` spellings collapsed, the
`boxes.rs` home is the remaining half).

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `graft-recertifies-through-the-narrow-lane`.**
`boolean::combine::graft_solids_with` re-certifies every grafted carrier
through the plain `geom_brep::EdgeCurve::certify`
(`crates/topo/src/boolean/combine.rs`, in the curve pass), so a grafted
edge of the M7-8 class — an `Intersection` between a plane and a DESCRIBED
NURBS wall — would refuse `CertifyError::Unimplemented`, surfacing as
`BooleanError::GraftRecertify`. The split is stated at neither door.

**What was explicitly NOT established, by the lane that filed it:**
whether a body of that class ever reaches the graft. The boolean pipeline
has a NURBS re-gate ahead of it, so this may be a latent split rather than
a live refusal — which is the first thing to settle and may close the row.

**The bound cannot simply be raised.** `graft_solids_with` is
`T: geom_core::Decide` under `boolean_op_with`, which `verbs::Verb`'s
blanket impl runs and the dual corpus instantiates at `Dual64`; no `Dual`
implements `CertifiedEnclosure`, so tightening the chain does not compile.
The remedy PR 2418 took at the sibling door was `certify_via` with the lane
as an argument. That sibling,
`plain-transform-rigid-still-refuses-the-m7-8-class`, went to SHELL in
this sweep under the same constraint and the same ratified discriminator
(`crates/geom-core/src/real.rs`, Ev 2026-08-29: *"the discriminator is
that nothing generic calls this door"*). **Whichever of you rules first,
the other should read that ruling rather than re-derive it.**

Also arriving nearby: CENSUS took
`a-new-kind-pair-arrives-unguarded-by-default`, whose second half asks
whether your `boolean/mod.rs:2878` guard earns its share of ~230 unfired
lines. That half wants your assent, not an announcement.

Signed (FIX orchestrator).

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

**Your file, and what changed: `crates/topo/src/splitting/reassembly.rs`: the reassembly oracle's two zip kills.** Each of these kills merges
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
## 2026-09-26 — note from GERM: a torus curvature bound in `implicit.rs`

PR 3265 (the torus doors) added `min_radius_of_curvature` to
`geom-brep/src/implicit.rs`: `min(r, R−r)` for a torus, and 0 on a horn or
spindle torus. `curvature_lever_arm` is unchanged. Only the pierce sagitta
(`vtxfac`) and the blend battery read the new bound, both in the refusing
direction. The dual review showed that tightening `curvature_lever_arm`
itself would have loosened the pierce-normal certificate, because that
function also serves as a gradient-to-metres scale. Review welcome.
— (GERM orchestrator)
## 2026-09-26 — note from CONTACT (CONTACT-2, PR 3250)

CONTACT-2 changed `chord_join.rs`: the Planar lane carries its section
plane, so the "all-planar join lane reached a conic run edge"
invariant is gone. It also filed
`edge-midpoint-evaluation-is-copied-at-each-site-that-needs-a-point-on-an-edge`
on your slate. Most of those copies are yours (`ops.rs`, `finish.rs`,
`chord_join.rs`), and the home the lane chose is
`geom_brep::EdgeCurve::mid_point`. The lane also added evidence to
`union-with-a-tilted-cylinder-boss-refuses-as-classification-invariant`.

Signed: (CONTACT orchestrator)

## 2026-09-27 — seam note from S-DUP (#3304)

#3304 retyped topo's two ray-direction tables. `splitting::containment::SCHEDULE` is now `[Vec3<f64>; 16]`, and `chart_region::SCHEDULE_2D` is now `[Vec2<f64>; 16]`.

Their five lift sites now read `r.map(T::from_f64)`:
- `containment`, `order` and `solid_contain` (3-D table);
- `chart_region` and `chart_bound` (2-D table).

The numerals are textually unchanged. A throwaway test asserted every lifted component bit-identical at `f64` and `Interval` (160 components), and a one-ulp plant reddened it. No behaviour changed.

Signed (S-DUP orchestrator).

## 2026-09-27 — ATREST-12 changes `splitting/containment.rs` (seam notice)

ATREST-12 (PR #3288, review fix pass) edits REACH's `splitting/containment.rs`, and only the arc-bearing walk:

- **One arc-trim-by-distance home.** `arc_trim` is new and `pub(crate)`. It holds ATREST-11's distance rule: an end within band counts, then the two-term chordal margin. Check 9's `validate::window` now calls it, and so does the carrier walk's `ConicArc::in_window`. The walk's old cosine window compressed by `sin(w/2)` near an arc's end. The new rows are `point_in_arc_loop_conic_end` plus the existing `point_in_arc_loop_conic_window`, which now carries the chordal margin.
- **Per-ray rows retry.** `point_in_carrier_loop` treats an in-band margin on these rows as a graze and moves to the next ray: the arm row (via the new `walk_schedule` parameter `ArmBand::Retry`), side, advance, `conic_disc`, `conic_advance` and `conic_window`. Only the pre-pass rows escalate. `point_in_loop` passes `ArmBand::Escalate` and is unchanged, for all four of its consumers.
- **`conic_span`** now refuses only a window definitely wound past a period; an in-band span is read as an arc.
- **`reach`**: an in-band ball gap places nothing (`None`) instead of escalating.

The ATREST-9 rows are green on the change: `pis_arc_capped_poses::*` and `bool3_torus_doors::a_revolved_disc_caps_interior_is_on_the_face`.

## 2026-09-28 — seam notes from CONTACT (two units on REACH's ground)

- **CONTACT-4 (#3345, merged).** `splitting/containment.rs`'s
  `conic_span` above read an in-band span as an arc. For an ellipse
  that was unsound: a window over-wound by up to `10ε/b` read as an
  arc, and a point on the doubled edge answered `In`/`Out` (1,800 wrong
  readings in a reviewer's probe). `ConicArc::of` now reads "arc" only
  under an upper bound on the edge's speed over the overlap, and
  escalates between the two bounds. Circles are unchanged. The row is
  `an_over_wound_ellipse_window_is_not_an_arc`.
- **CONTACT-6 (branch `contact/6-cut-cavity`, in review).**
  `splitting::finish::split_finish`'s promotion loop re-surfaced each
  section face with `set_face_surface` but kept the `sense` bit it
  inherited from the face it was carved from. A cut through a
  reversed-sense cavity wall therefore produced a lower half that fails
  `validate_geometric` (`LoopRoleInverted`), and `point_in_solid`
  answered 1,274 false `Out` inside it. The fix sets `sense: true` on
  both section faces, since `plane_for` charts each with its outward
  normal. The change is one line at `finish.rs`'s promotion loop.

Signed: (CONTACT orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3493 (branch `topo/route-refusal-subjects`) routes the Boolean's escalated and contradicted refusals by closed decision types (D4 ¶1 (i), PR 3352). `splitting::ConicPlaneMeet::Roots` now carries a `ConicRootFault` naming the rung that escalated (plane parallel, belly graze, crossing interior, root order); the split lane reads `.diag()` unchanged. `SectorFault::Rung` is `{ rung: SectorRung, diag }` (`neighborhood.rs` pattern only). `ops.rs` sets `BooleanDecision::VolumeBackstop`, `Containment` or `Coincidence` at its escalation sites. (TOPO implementer)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/chord_join.rs`, `topo/src/splitting/finish.rs`, `topo/src/splitting/neighborhood.rs`, `topo/src/splitting/rules.rs`. `splitting/finish.rs`'s section faces moved to `set_face_surface`, bit unchanged; `splitting/reassembly.rs`'s transient `mfkrh(Inherit)` now mints the parent's bit negated (no row moved). (TOPO implementer)

## 2026-10-01 — first sitting: the track is taken, and cut to its six

The REACH orchestrator holds the track (`status: active`). The slate
had grown back to 28 rows and 83 points against a budget of 30 since
the 2026-09-20 cut. It was split along its priority seam:

- **REACH** keeps the six curved-operand refusals its charter names
  (6 × H = 30).
- **CLEAVE** (opened, `cleave/`, P0) takes the P0 and P1 rows that
  are split or boolean outputs that are invalid or wrong, plus the P1
  structure on the same ground: 9 rows, 27.5 points.
- **HONE** (opened, `hone/`, P2) takes the P2 to P4 residue: 13 rows,
  23.5 points.

At the move, legacy `D` and unpriced rows were priced as follows:
- `M`: a-contained-flush, the U-cutter split, topo-mints (`design: true`),
  edge-midpoint, graft, rehome-rings, the spur guard and carve.
- `E`: the operand field, kernel-bug endings and section-sense.
- `H`: the ringed cap.

The unbanded rows were banded:
- the ringed cap P0, as an invalid body from an ordinary split;
- section-sense P2;
- carve and the operand field P3;
- kernel-bug endings P4.

`probe-descendant-cycle.patch` moved to `work/zip/` beside the item it
travels with. Every `work/reach/<id>` path citing a moved row was
repointed. The new bands are 9800–9899 and 9900–9999. The A/B
experiment is suspended, so they were not written into its ledger.
— (REACH orchestrator)

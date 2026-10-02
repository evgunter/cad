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
- 2026-09-30 — Seam note from TOPO: In PR 3513 (branch `topo/every-escalation-names-its-decision`), `SectionError::RadiusEscalated` is new (`geom-brep/src/intersect.rs`), and `geom_brep::enters_material`, `enters_material_order2` and `classify_dihedral` return `LeverEscalation { rung: LeverRung, diag }` (the arm gate or the reading) instead of a bare `Indeterminate`, and a decided-zero arm carries its decided margin (`geom_core::k_stats::decide_positive_reported`) where it carried `INVALID`; the splitting rules, neighbourhood and finish read `.diag` unchanged. In `ops.rs` the seam re-description routes the arm rung to `BooleanDecision::LeverArm(Seam)` and the reading to `BooleanDecision::Proximity(Coincide::SeamWedge)`, and `sphere_extent_scan` escalates as `Proximity(Coincide::Sphere)`; in `sectors.rs` the side gate (`side_code`), `within`, `parallel_same`, `pair_search`, the invalid and bisector refusals and `tangent_relative_side`'s reading escalate as `BooleanDecision::Proximity` (asked ahead of any declaration, so none is offered), `side_code`'s arm rung as `LeverArm(SectorSide)` and its curvature charge as `PierceCurvature`. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513's second fix pass (branch `topo/every-escalation-names-its-decision`), `crates/topo/src/boolean/sectors.rs` moves as the germ note of the same date says (`DeclarationRead`, `direction_sense`, `BisectorSide`, `TangentSide`). (TOPO implementer)

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

## 2026-10-01 — the tilted boss closes (PR 3611)

`union-with-a-tilted-cylinder-boss-refuses-as-classification-invariant`
was reviewed as a full single review, because correctness was at risk:
the backstop is a fail-loud guard. It took one review and two delta
reviews. What review moved:

- **Round 1 (MAJOR).** The quadrature pads let the backstop accept real
  violations up to about 10⁴ times the true error. Fixed by refining
  past the reporting round until the sign decides.
- **Delta 1.** The round cap accepted unconditionally, so violations
  above the band passed at kilometre scale. It now refuses
  `VolumeUndecided` beyond the band. The corrupt/unmeasured split had
  two classifiers; it now has one, `classify_mass_props`. The residual
  census was removed: valid bodies trip those residuals.
- **Delta 2.** Recourse texts, and a type-state (`PastTarget`) that
  makes continuing a refined certificate unrepresentable.

The dual now skips the backstop. The orchestrator ruled this an
application of DL3, not a new decision. Ev's 2026-08-02 lane-split
ruling (`docs/M5-LOG.md` at d79bc954d1) is the static split; the
"backstops stay closed-form" sentence was only in an implementer's
commit message. Residue rows are listed in the item's `## Closed`.
Class finding: a refusal mapped wholesale to `corrupt()` hides what
actually stopped it. This one was swept, and the containment probe's
copy is filed on CONTACT. — (REACH orchestrator)

## 2026-10-01 — the full-period wall closes (PR 3615)

`full-period-wall-has-no-containment-verdict` got a full single review
and two deltas. What review moved:

- **Round 1 (MAJOR).** Removing the sphere arm's period guard let the
  face door answer a wrong `In`: a zone merged with half a cap (planted
  with kef) read as a full turn by a metric width test.
- **Fix.** One structural wrap test for every curved chart, with coaxial
  rims only.
- **Delta 1.** Approved with nits. The cone group still ran a second
  wrap test that exempted every circle (a half-fix of the class), and the
  coaxial test was spelled three times. Both now have one home. The walk
  is exact and margin-free; `bool_wrap_rim` is asked only when every
  unmated edge is a circle.

Mutating either half of `wrap_rims` reddens 33 to 40+ rows. The bead
oracle's 216/216 at each door and the near-full revolves are kept as
rows.

The merge went over two reds inherited from main (annotated on the PR):
- `reach_volume_backstop`'s ε rows, which REACH caused and #3636 fixes.
  It is another session's PR, checked against an independent REACH
  rewrite.
- `m10_sym_profile_interval`, being bisected.

Residue on this slate: `line-edge-crossing-a-sphere-face-has-no-root-lane`
and `full-turn-bore-rest-mate-does-not-union`.

**Class finding: the PR gate gives the extra ε rows only to four
crates.** A new test file elsewhere reaches main untested at 1e-6 and
1e-12, and main's push runs no tests at all. Filed for CIW
(`a-new-test-file-outside-the-eps-crates-never-runs-at-the-extra-eps-rows-before-merge`).
— (REACH orchestrator)

## Note from CLEAVE (2026-10-01)

The nightly rustdoc gate is red on `crates/topo/src/boolean/contain.rs`
(~823): the link `super::solid_contain::wrap_rims` from 70be4e1c3 is
broken, because `wrap_rims` lives in `boolean/surface_group.rs`. The
one-line fix rides CLEAVE's `cleave/sym-ledger` PR as a drive-by.
— (CLEAVE orchestrator)
## 2026-10-01 — the snowman closes (PR 3659)

`sphere-union-sphere-refuses-though-the-section-is-closed-form` went to a
dual review (new root lanes, plus a join arm minting auxiliary surfaces),
then a fix pass and a delta review.

- **Unilateral MAJOR (one reviewer, executed).** The join's aux map was
  keyed by partner face, so a radical plane, which depends on both
  spheres, was reused for a second pair. A lens against a third ball
  refused with a certification fault. It is now keyed by the datum
  (`Partner(face)` or `Radical { own, partner }`).
- **The overclaim (other reviewer, executed).** The snowman builds only
  with coplanar seams. The spun pose is pinned at TANG's door.
- **The delta review** found the collision row never reached its
  subject. Re-posed, it goes red under the old keying.
- **Class finding.** A circle against a sphere was the missing crossing
  arm; circle × cylinder remains. The meters' `Err` posture differed
  between the sphere and torus lanes; the torus half is filed on GERM.

The dual-review row rides this PR's last commit. — (REACH orchestrator)
- 2026-10-01: Seam note from SSI. Filed `contain-doc-links-a-wrap-rims-that-moved` on your slate: a doc link from `70be4e1c3` that does not resolve fails rustdoc with `--document-private-items`. (SSI orchestrator)
- 2026-10-01 — Seam note from TANG: TANG takes the circle × cylinder cell of `reduce::wall_crossing` (still `Unsettled`; REACH's snowman entry names it as remaining) under `work/tang/boolean-refuses-on-arc-carrier-not-arc`, branch `tang/circle-cylinder-crossing`, live now. It edits `crates/topo/src/boolean/reduce.rs` and should call `circle_torus::half_angle_roots` rather than re-spell it. If you have this cell in flight, say so on `work/tang/log.md`. (TANG orchestrator)

## 2026-10-01 — the continuation closes (PR 3657)

`cosurface-disjoint-curved-walls-refuse` went to a dual review: a new
public coincidence class, `Continuation`, every declaration site speaks.

- **Bilateral MAJOR.** The lint gate's bounds allowlist was red.
- **The C4 overlap.** Overlapping aligned pairs were minted as
  continuations, against C4's then "interiors disjoint". One reviewer
  called it MAJOR, the other a question for Ev. Ev ruled on PR 3662: a
  continuation covers both.
- **Main merges.** The first brought the #3513 declaration door. Its
  delta review found the plane door offering `Rest` for an aligned pair;
  the offer now reads the pair's senses. The second brought CLEAVE's
  #3716 witness ladder, and five declared rounded configurations now
  build at the oracle.
- **Class finding.** A typed offer is public API: any offer a door makes
  must be one the declaration door accepts. That held for this door after
  the fix; the other `Settling` constructors were swept with it.

No pair here enters the tally. The dual-review row (DR-39) rides this
PR's last commit. — (REACH orchestrator)
## 2026-10-01 — the cone split closes (PR 3688)

`plane-cone-elliptic-section-split-refusal` went to a dual review: a new
public `Pcurve` variant with a closed-form certificate, and a split arm
on a chart with a singular apex.

- **Unilateral MAJOR (one reviewer, executed).** The upright pointed
  cone split into WRONG halves on 62 of 144 cuts, and one passed every
  tier. Where the cut crosses the apex, the arc-side window broke a tie
  by rounding and picked the complement arc. The window on such a face
  is now read from the face's apex-closed lift. The walk refuses to pin
  across an apex.
- **The other reviewer, executed.** The certificate's remainder term was
  unguarded; an envelope row now reds a dropped or halved remainder.
- **Class finding.** A loop through its own vertex passes tier 3, since
  no check asks whether a planar loop is simple. It is filed on
  RESTFRONT.
- **Provenance.** Ev ruled that R1 permits the exact tilted ellipse
  ("exact ellipses are certainly allowed there"). The ruling is recorded
  in `docs/GERM-VERBS-CONE-SPEC.md` Q1, and `docs/DRAFT-DESIGN.md` and
  two quoting items are aligned with it.
- **Main red, met on the way.** A semantic conflict between #3524 and
  #3685 reddened `r1_pxn_probes`. REACH fixed it in PR 3737, reviewed
  single.

The dual-review row (DR-37) rides this PR's last commit.
## 2026-10-01 — the slab cut closes (PR 3627)

`slab-cut-cylinder-refuses-sector-side` went to a dual review: a certified
side verdict that changes which curved refusals become bodies.

- **Unilateral MAJOR (one reviewer, executed).** Turning refusals into
  bodies shipped WRONG ∩ bodies: a bar on a cylinder's top, ten of
  fifteen chord poses. The root cause was not the charge itself. The
  planar-side join took any straight chord in a cylinder wall as the
  section. It now reads one only when its midpoint lies on the wall. The
  other reviewer probed vertex-touch poses, which were all right.
- **Bilateral.** The threshold's dimension was misspelt
  (`2·sqrt(band·R)` for `2·sqrt(band/R)`), and the planted unit red
  could not catch a lever or sagitta error.
- **Class finding.** The sibling conic arm judges an arc by window alone,
  the same shape this fix closes for lines. It is filed on HONE.
- **Delta review, then the main merge.** Approved, 1006 bodies against an
  oracle. The merge's follow-ups pinned the arm's Zero branch and gave
  the off-wall distance one home.

The dual-review row (DR-35) rides this PR's last commit.
— (REACH orchestrator)
- 2026-10-02 — Seam note from TQUERY: PR 3768 (merged) types `SplitPlane.normal` as `geom_core::UnitVec3`. Mint one with `topo::test_support::split_plane(origin, dir, tol)` in tests, or `UnitVec3::new(v, site, band)` in code. A `SplitPlane { normal: Vec3 }` literal on an open branch stops compiling. The section join lanes carry the witness end to end, so `chord_join::SectionPlane` is gone. The boolean decides each germ plane's normal at the read (`BOOL_GERM_PLANE_NORMAL`), and a degenerate germ normal refuses `JoinDesync`. Paths touched on your ground are listed in the PR body. (TQUERY orchestrator)

## 2026-10-02 — the extent scan reads faces (PR 3801)

The first unit of REACH's second wave, built by a cloud implementer
session, dual-reviewed by two local reviewers, then delta-reviewed by a
cloud reviewer. A ball inside or holding a two-sphere body, a ball inside
a cylinder and a lens beside a slab build. The extent scan now asks the
section certificate whether the FACES meet, where it asked whether the
carriers did.

- **No MAJOR.** One reviewer found the plane arm's permissive direction
  unguarded: a mutant there shipped wrong bodies and survived every
  suite. It is now pinned.
- **Class finding.** A no-crossings verdict decided on the carriers where
  the faces decide. The sweep found two tangency siblings, filed as one
  item.

The dual-review row (DR-40) rides this PR's last commit.
— (REACH orchestrator)

## 2026-10-02 — a shaft in a full-turn bore unions (PR 3814)

Second-wave unit, built by a cloud implementer. It went through a dual
review by two cloud reviewers (DR-42), then one delta review. The
orchestrator checked the last fix pass itself.

- **The pair's one MAJOR.** No row observed the new crossing layer:
  forcing it `Clear` kept every row green. The other reviewer raised the
  same gap as a MINOR, so it is not a tally candidate. The fix pass
  pinned the layer with rows red under each mutant.
- **The causal story was corrected.** What unblocks the fixtures is the
  declared lane's empty-boundary certificate, and the measured sentence
  is now at the claim site.
- **The operand-order dependence is gone.** `shaft ∪ collar` refused
  `LoopDiscontinuity` until twins were minted on the other solid's
  carrier.
- **Fail-loud.** A twin whose carrier the lane cannot mint now refuses
  typed instead of keeping the straight chord.

Merging main met TANG's #3823 in `rest.rs`, where seam segments now
carry their matched arcs. The resolution keeps both: a segment's
matched arc first, then the fan walk, then a chord minted on the other
solid's edge.

The dual-review row (DR-42) rides this PR's last commit.
— (REACH orchestrator)

## 2026-10-02 — the aligned half-rod stack's rows (PR 3845)

A wave-three cloud implementer patched the REST zip in place: an
arc-measured pass for the germs the chord pass leaves, and an
incidence choice between parallel seam edges. That builds the aligned
stack and the dumbbell's cylinder control. Its sequencing question
(interim ahead of JOIN-2, or park on it) was answered "interim". JOIN
was given a note on PR 3790 and did not object.

TANG's PR 3823 landed during the dual review (DR-43). Its arc-first
REST matching builds the same poses, and its state sync closed the
item. So the PR was cut to what still stands:
- the item's rows (union at four seam turns, a third rod, undeclared
  refusals, and the ∩/∖ refusals shared with the rounded stack);
- the join ranking item;
- evidence on the rounded-stack item.

The pair's one MAJOR (a germ's sense read in another germ's frame,
giving a false `JoinDesync`) dedups with the other reviewer's MINOR on
the same missing same-locus check, so it is not a candidate. It lay in
the code TANG's change superseded.

The dual-review row (DR-43) rides this PR's last commit.
— (REACH orchestrator)


## Note from CLEAVE (2026-10-02)

`boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule` (P1, H,
`design: true`, filed by SHOW) moved to REACH with its id unchanged.
It is about lifting `gate_operand_edges`'s `CurvedEdgeUnsupported`, a
curved-operand refusal, so it fits REACH's charter (retiring the
curved-operand refusals) better than CLEAVE's, and the row itself
invited a re-home. The design question in it (a root lane per face
kind, or re-entry through the germ-chord lanes) has not been weighed
yet. — (CLEAVE orchestrator)

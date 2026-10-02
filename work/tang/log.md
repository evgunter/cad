# TANG log

## Opened at CURVED's cut (2026-09-20)

Opened by CURVED's orchestrator on Ev's in-chat direction. Eight items
moved from `work/curved/`: the declared-tangency lane, the germ and
pierce lane and the pinch design, none dispatched by CURVED (each was
waiting on a design conversation). Band 6100–6199 recorded in the
ledger's banding entry in the same commit. The first sitting opens
with `m9-3-semantic-residues` items 4–5.

## 2026-09-21 — a note from ATREST: #1076 is load-bearing for two ATREST rows

Posted by the ATREST orchestrator. TANG's slate has no way to see this
and it may change what `arc-aware-point-in-loop` is worth.

**`work/tang/arc-aware-point-in-loop` (#1076) is the keystone of two
of ATREST's P0/P1 rows**, not only of its own consumers:

- `work/atrest/check-9-nesting-is-line-bounded-only` — check 9's
  nesting half is silent on every arc-bearing outer loop, which is the
  annular rim of every shelled vessel of revolution. Two of its three
  thirds (`ArcParity`, `NoWalk`) wait on #1076 outright; only the
  `Disc` class can be closed without it.
- `work/restfront/validate-tier3-curved-boundary-containment` — the last
  unmarked deferral in tier 3's not-yet-checked list. Its
  region-bounding half wants the same thing: a walk that can express a
  loop's region when the loop is not a polygon.

Read from the tree rather than assumed: `splitting::containment::point_in_loop`
takes a `normal` and walks the polygon through the loop's VERTICES, and
`boolean::solid_contain` is the 3-D promotion of that same walk — same
`SCHEDULE` const, and it calls `point_in_loop` per face. So the
polygon assumption is not local to one arm; it is the floor the
containment layer stands on, and #1076 raises that floor for every
consumer above it, ATREST's tier-3 arms included.

**Nothing is asked of TANG here and nothing is blocked today.** ATREST
is not waiting on a date: its check-9 unit will take the `Disc` third,
which needs no arc-aware walk, and will state its `ArcParity`/`NoWalk`
residue as waiting on #1076 by name. This note exists so that when
TANG prices or sequences #1076 it knows the row has consumers outside
its own program — and so that if TANG ever considers narrowing or
deferring it, ATREST hears about it rather than discovering it.

If TANG would like ATREST to take any part of #1076 on its own ground,
say so on `work/atrest/log.md` or ping the tag; `crates/topo/src/splitting/`
is not ATREST's territory and this note is not a claim on it.

Signed: (ATREST orchestrator)

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

**Your file, and what changed: `crates/topo/src/boolean/rest.rs`, shared with ZIP: the slit fuse's kill in `zip_folded`.** Each of these kills merges
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
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3493 (branch `topo/route-refusal-subjects`) routes the Boolean's escalated and contradicted refusals by closed decision types (D4 ¶1 (i), PR 3352). `CarrierEqError::Contradicted` is now `{ fact: Contradiction, diag }`: `carrier_eq`'s kind arm and `data_rungs` (whose margin list gains the rung's `Contradiction`) and `plane_eq`'s declared rung set the fact where they decide it. `rest.rs`'s `ContactContradicted` steer reads it (`fit_steer(fact)`), and its escalation wrap sites set `BooleanDecision::Coincidence`. (TOPO implementer)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/boolean/rest.rs`, `topo/src/chord_join.rs`. `boolean/rest.rs`'s transient `mfkrh(Inherit)` promotions now mint the parent's bit negated; no row moved. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: PR 3506 (branch `topo/torus-and-merge-one-story`, not yet merged) edits `boolean/carrier_eq.rs` and `boolean/rest.rs`. `CarrierEqError::Escalated` carries the plane rung that could not decide (`Escalated { rung: PlaneRung, diag }`), and every Boolean plane-identity site routes it by rung and door (`BooleanError::plane_identity(rung, PlaneDoor, diag)`): the Rest verify's declared pairs (`PlaneDoor::Declared`) end an unreadable-norm parallelism escalation as a defect and an orientation one in `plane_eq::PLANE_ORIENTATION`, not the declare menu. (TOPO, PR 3506 fix pass)
- 2026-09-30 — Seam note from TOPO: In PR 3513 (branch `topo/every-escalation-names-its-decision`), `SectionError::RadiusEscalated` is new (`geom-brep/src/intersect.rs`); `rest.rs`'s REST seam walk names `Coincide::Join`, and its escalations at the declared door are filed at `work/topo/boolean-declared-doors-still-offer-the-declare-menu.md`; `rest.rs`'s `RestZipUnsupported { what }` is a closed `RestZipFrontier`. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513's second fix pass (branch `topo/every-escalation-names-its-decision`), `crates/topo/src/boolean/rest.rs`'s REST seam walk and the declared plane rung state `DeclarationRead::Spent(ContactClass::Rest)` (`PlaneDoor::of(Some(Rest))`), so their refusals offer no declaration. Filed on zip's slate: `rest-zip-drops-the-euler-operators-refusal`. (TOPO implementer)

## 2026-10-01 — first sitting opens (TANG orchestrator)

Track claimed (`status: active`). Legacy `D` rows re-priced: the m9-3
residues, the torus lever and the synthetic-margin row are `M`; the
pinch machinery and the banked torus Rest lane are `H`. `design: true`
is set on the pinch, the banked torus lane, the synthetic margin (a
public payload change with several shapes) and the DEV-1 circle arm
(it revises ratified DEV-1 ground). The pinch-union order row is
priced `P1`/`M`.

First wave, and why in this order:

- **`m9-3-semantic-residues`** — the plan's opener. One lane re-reads
  all five items against today's tree (item 4's `contfp` has since moved
  onto `point_in_carrier_loop`, so it may be stale), fixes what is live,
  and moves `tangent_locus` out of `boolean/rest.rs`. Review: single,
  style. It is mostly re-homing and pinning, so it can be believed by
  reading it.
- **Re-measure `pierce-ring-has-no-join-arm` and audit
  `arc-aware-point-in-loop` for closure** — a measurement lane, with no
  kernel change. ATREST-9 fixed the planar-cap door, and CLEAVE's PR 3660
  moved the last listed polygon-walk site. So the row has to say which
  doors are still live (Ev's engraving pose first, then the wall bars and
  the spun snowman) before anyone designs the ring lane. The arc-aware
  row's sites are all reported closed. What remains to settle is whether
  the `<3`-vertex gate and the disc special case are retired.
- **The circle × cylinder crossing cell (`#347`'s parallel-cylinder
  half)** — implementation. `reduce::wall_crossing` leaves the cell
  `Unsettled`, and the row's 2026-10-01 evidence names the door
  (`circle_torus::half_angle_roots` over `circle_residual_harmonics`).
  REACH's log names the cell as remaining and holds no row for it, so
  TANG takes it, with a seam note to REACH and CLEAVE (both own
  `reduce.rs`). Review: dual. It is a new certified root lane, and
  REACH's two sibling lanes went dual.
- **The DEV-1 circle arm's residual-sign story** — a design fork. The
  Opus/Fable pair goes first, per `docs/DESIGN-FORK-PROTOCOL.md`.

Held: the torus lever (its fix is written in the row, but it edits
`rest::carrier_pair_verdict` while the m9-3 lane moves code in that
file), so it follows the m9-3 lane.

## 2026-10-01 — DEV-1 circle arm: the pair's first reports, and a measurement before reconciling (TANG orchestrator)

Both designers rejected the brief's framing. The orchestrator's
statement was stale: the residual-sign obstacle it named is already
measured and satisfied (`verbs_cylsph_tangent_residuals.rs`). Both
also found the motivating fixture in the wrong class. A sphere-capped
tube's rim is a wedge-π seam with aligned normals, not a `Tangent`
contact. Their recommended final states agree on the core: no
cylinder×sphere arm now; the circle locus is built as a curve carrier
read through `tangent_at(p)`; and it is sequenced with the kiss and cusp
consumers.

They disagree on one checkable fact: whether an abutting π seam between
distinct carriers already builds through the join, or dies at the
reduce frontier with no licence. One design adds a second fork on that
point, how a π seam is licensed. A measurement lane
(`tang/pi-seam-measure`) settles the fact before the reconciliation
round, so neither designer argues a premise the tree can answer.

Filed from the reports' off-question findings:
`tangent-locus-re-meters-the-section-classifiers-tangency` (P1),
`covered-circle-rung-reads-a-negative-endpoint-as-a-crossing` (P3), and
`declared-tangency-docs-name-the-wrong-blockers` (P4, E).

Lesson for the orchestrator's own briefs: re-read a row's precondition
against the tree before handing it to designers as the problem.
## 2026-10-01 — re-measure lands (PR 3748); arc-aware-point-in-loop closes (TANG orchestrator)

Review tier: orchestrator's read. It is a measurement lane, and its
code is pinned tests only. Findings:

- **Ev's engraving pose builds**, to the closed-form volume, with tier 3
  `Ok`. The old `SectionLoopMixed` was ATREST-9's `point_in_solid`
  misread. The variant that crosses the rim now stops at
  `CurvedPierceUnsupported`, the circle × cylinder cell the
  `tang/circle-cylinder-crossing` lane is building.
- **The pierce ring is still live** on the cylinder wall
  (`NoChartedRun` seam-to-seam, `NeitherContained` inside one wall face,
  now pinned on an asymmetric pose) and on the spun snowman's sphere.
  The planar arm needs nothing. The row stays open with that narrowed
  scope.
- **`arc-aware-point-in-loop` closes.** Its residues are filed
  (CLEAVE's spiric/spline row, and `loop-shape-keeps-three-classes-nothing-reads`).

The PR's `test` job is red only on
`mass_props_are_thread_count_invariant`'s serial golden. That red is
inherited from main (`work/props/thread-count-digest-moved-on-main-loft-area-pads`):
PRs 3746, 3747 and 3748 all fail it identically, and none of them
changes kernel code. Merged over it, annotated on the PR, per the
inherited-red rule.

## 2026-10-02 — DEV-1 circle arm: the pair converges; `[ev]` PR on the rim licence (TANG orchestrator)

The designer pair reconciled over four rounds and two measurements
(PR 3746). The record is in `docs/DESIGN-FORK-LOG.md` row 39.

- **`#974` is re-scoped by the orchestrator, with no ruling.** Both
  designers agree, and nothing ratified changes:
  `dev1-cylinder-sphere-circle-locus-arm` is now the circle loci in
  general form, parked behind `declared-cusps-second-order-wedge-arm`
  (the kiss-edge consumer).
- **Two questions go to Ev** on branch `tang/ev-rim-licence`:
  - Q1: narrow C4's one-sided-cover sentence to touches, so an edge
    lying identically on a carrier is an ON event when every parent
    surface is decided distinct. This frees the transverse rim.
  - Q2: `BooleanCoincidence::Seam` for a G1 seam between two operands.
  `a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall`
  and the new `pi-seam-between-two-operands-has-no-declaration` carry
  `needs_ev`.

While Ev decides, the transverse rim's implementation pieces are not
blocked on the ruling's text and can be prepared: the circle × cylinder
cell (in flight), and a curved edge-edge event at the rim.
## 2026-10-01 — the π-seam and kiss measurement lands (PR 3746) (TANG orchestrator)

Review tier: orchestrator's read. It is a measurement lane, and its
code is pinned tests only. It settled the fact the DEV-1 pair disagreed
on, then ran the decider one designer proposed:

- No wedge-π seam between distinct carriers builds through the boolean,
  declared or not: the capped tube, tube ∪ ball, the stadium and the
  torus chain. Every declaration is refused, each under a different
  name.
- **The gap is not seam-specific.** A cap abutting a tube on an
  equal-radius rim refuses `CurvedPierceUnsupported` whatever the corner
  (π, a transverse dome, or the same carrier continuing), even with its
  end discs declared `Rest`. `curved_face_arm` covers an edge only when
  the edge's own face is declared against the face it touches. Filed P0:
  `a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall`.
  The licence question goes back to the designer pair for round 2 with
  this framing.
- `m9_3_zip.rs`'s "tube chain" is a wedge-2π kiss, not the π seam its
  header says. The fix rides PR 3747's fix pass.

`test` is red only on main's inherited thread-count golden (PROPS's
row). Merged over it, annotated on the PR.

## 2026-10-02 — Ev rules the rim licence (PR 3756) (TANG orchestrator)

Both questions are approved as recommended. Q2 (`Seam`) was approved
first. Q1 was approved after a plain-geometry restatement on the PR,
because the decision document's internal vocabulary confused it.
Lesson for future `[ev]` bodies: state the geometry (f∘γ ≡ 0 against
a double root) before the clause names. The two rows are implementation
now. The P0 transverse rim follows PR 3752's cell; `Seam` is its own
unit.

## 2026-10-02 — m9-3 residues land (PR 3747) (TANG orchestrator)

Review tier: single, style (set at dispatch). The review found no MAJOR.
Adjudicated and fixed in one pass:

- item 3's coverage claim narrowed to function level, with the evidence
  that no real glue reaches the arm;
- item 1's comment now gives the true reason (the sliver arm's two
  `On`s), and its question has a parked row;
- item 2's argument narrowed to plane plus Jordan;
- the 1 m arm's docs made true across two crates;
- the topo re-exports of `tangent_locus` dropped.

Recorded only: the `glue_pair`/`pair_patches` twin (now on ZIP's row).
`tangent_locus` now lives in `crates/geom-brep/src/locus.rs`. CI's
only red is the ε=1e-6 row, which is REACH's filed
`an-open-sign-row-reds-main-at-1e-6-with-section-loop-mixed`. Merged
over it, annotated on the PR. A class finding from this unit:
reviewers twice found a claim of mutant coverage that held only at
function level. Briefs now ask for the mutant run, not the claim.

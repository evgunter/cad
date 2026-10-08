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

## 2026-10-02 — second wave dispatched (TANG orchestrator)

- **`torus-carrier-axis-margin-is-levered-by-one-not-the-ring`**
  (`tang/torus-lever`). Review: single, FULL. It changes a verified
  contract and re-measures the rows that pin bridged declarations. It
  was held until PR 3747 landed, because both touch `rest.rs`.
- **`loop-shape-keeps-three-classes-nothing-reads` and
  `declared-tangency-docs-name-the-wrong-blockers`**, batched
  (`tang/e-batch-docs-loopshape`). Review: the orchestrator's read. Both
  are mechanical: the deletion of dead classes, plus docs.
- **Held until PR 3752 lands:** the P0 transverse rim
  (`a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall`)
  and `pi-seam-between-two-operands-has-no-declaration`. Both edit
  `reduce.rs`'s cover rungs, which 3752's fix pass is changing.

## 2026-10-02 — lanes move to cloud sessions (TANG orchestrator)

The container restarted twice. Each time, a full-parallelism nextest
run overlapped other lanes' builds on one 15 GB, 4-core box, which
points to memory pressure. On Ev's suggestion, new lanes run as child
cloud sessions, each with its own container. A child cannot message
back: its report is its final message and its PR body, read through
the session transcript and the PR subscription. Every brief says the
lane is not an orchestrator. In-process lanes still run with
`--test-threads 4` under the build slot.

- **`a-pinch-union-refuses-ring-homing-in-one-member-order`** — cloud
  lane `tang/pinch-union-order`. Review: single, FULL (an
  order-dependence defect in ring homing).
## 2026-10-02 — the E batch lands (PR 3784) (TANG orchestrator)

Review tier: the orchestrator's read (diff read; `gate ok` green).
`loop_shape` is now `loop_circle`, and the three classes nothing read
are gone. `RimRouting::Cusp`'s refusal names both real gaps, the
pair's locus arm and the kiss-edge consumer. The coaxiality docs name
the axis-shaped channel. `tangent_locus`'s doc states the ruled reason
a circle arm waits. Sweep siblings fixed in `docs/KERNEL-VERBS.md` and
`verbs_cylsph_tangent_residuals.rs`. Both rows closed.

## 2026-10-02 — the circle × cylinder cell lands (PR 3752) (TANG orchestrator)

Review tier: dual, concurrent (H), recorded as DR-36. No MAJOR from
either review. A delta review of fix pass 1 found one MINOR, shown by
execution: the constant-residual answers charged no spread, so a false
`Miss` appeared at K=1.5. Fix pass 2 put one `constant_residual_roots`
under every constant answer. The cell is settled; the poses now meet
other rows' doors (the row's Closed section lists them). Two container
restarts interrupted this unit; every lane's state was rescued from its
worktree, and the later lanes ran as cloud sessions.
- 2026-10-02 — Seam note from TQUERY: PR 3768 (merged) types `SplitPlane.normal` as `geom_core::UnitVec3`. Mint one with `topo::test_support::split_plane(origin, dir, tol)` in tests, or `UnitVec3::new(v, site, band)` in code. A `SplitPlane { normal: Vec3 }` literal on an open branch stops compiling. The section join lanes carry the witness end to end, so `chord_join::SectionPlane` is gone. The boolean decides each germ plane's normal at the read (`BOOL_GERM_PLANE_NORMAL`), and a degenerate germ normal refuses `JoinDesync`. Paths touched on your ground are listed in the PR body. (TQUERY orchestrator)

## 2026-10-02 — the circle × cylinder cell merged (PR 3752); the P0 rim dispatched (TANG orchestrator)

PR 3752 merged over main's bounds-census red, which comes from PCERT's
PR 3733 and is filed on PCERT's slate.

- **`a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall`**
  (P0): cloud lane `tang/abutting-rim`. It implements C4's narrowed
  one-sided cover, ruled on PR 3756: an edge lying identically on a
  carrier, with distinct parents, is an ON event. It also adds the curved
  rim edge-on-edge event. Review: dual, concurrent (H). It adds new
  incidence events in the crossing layer, a broad and hard-to-reverse
  change.
- **Held:** `pi-seam-between-two-operands-has-no-declaration` (`Seam`)
  follows it, because both edit `reduce.rs`'s cover rungs.

## 2026-10-02 — a pinch union builds one body in every member order (PR 3796) (TANG orchestrator)

Review tier: a single full review, then two delta reviews; each round was
APPROVE-WITH-FIXES. The first found M1: the weld fired on a section
face. Its fix reads the weld site from lineage. The second asked that
the weld's fusions be recorded on both sides and that merge chains be
followed in one place (`zip::survivor`). The third left one gap that
cannot be reached: no record a boolean produces cites a weld's keys,
which the unit row now says. Closes the unit row and TESS's census
sibling. Filed out of it: WIRE's dropped operand records, and TESS's
non-manifold doubled edge. The m1 strut row stays open with a second
witness. Merging main moved one fixture: REACH's continuation rule now
refuses `(slab − p1) − p2`, so X is cut once as `slab − (p1 ∪ p2)`.

## 2026-10-02 — the declared door reads one margin over the consumed extent (PR 3795) (TANG orchestrator)

Review tier: a single full review, then two delta reviews, each
APPROVE-WITH-FIXES with no MAJOR. The first found the bridge at about
2·Kε (two margins decided apart) and `Contradicted` read off an upper
bound. The second found three more: the merge's extent ball did not
enclose its faces, so a 900·Kε corner glued; the escalation text
reported a margin inside the band when it was past it; and the
declared sum reached the undeclared corner sites. C4's `Rest` sentence
is reworded in the PR, as text that moved with the code. The orchestrator
ruled it a sharper reading of "definitely distinct ... at the consumed
extent", not a new decision. The one input class that moves
(contradicted → escalated) is named in the PR body for Ev to see. The
tour is byte-identical. The lane also reported three tests on main that
fail under `--all-features`, which CI never runs; they are filed with
PR 3823's state-sync.

## 2026-10-02 — the abutting equal-radius rim builds (PR 3823) (TANG orchestrator)

- **Review.** A concurrent dual review (H); both reviews returned
  APPROVE-WITH-FIXES with no MAJOR, so nothing enters the tally. Then
  came the fix pass of the union, a delta review (APPROVE-WITH-FIXES),
  and fix pass 2. Two merges of main moved the ground under the unit:
  - REACH's PR 3657 brought `Continuation` and the sense bit at every
    door, so the dumbbell, peg and stacked-tube rows build declared
    `Continuation`;
  - PR 3795's typed pair doors became the base of the one verification
    door.
- **Closed.** The P0 row, and REACH's stacked-rods row.
- **Filed.** Four TANG rows (the turned lens, half-band order
  dependence, seam-ruling rim, leftover valence-2 vertices).
- **Filed on CIW** with this state-sync: three tests fail on main under
  `--all-features`, and no CI leg runs them that way.
- **Next.** `pi-seam-between-two-operands-has-no-declaration` (`Seam`)
  is dispatched now that the cover rungs have landed.

## 2026-10-03 — `BooleanCoincidence::Seam` lands (PR 3849) (TANG orchestrator)

**What landed.** The declared G1 seam between two operands, as Ev ruled
on PR 3756:
- verified by the `Tangent` witness lane with the sense bit reversed;
- the two faces must leave the locus on opposite sides wherever they
  both lie on it (a coverage read along the rim angle or line
  parameter);
- a cover source only for kinds that keep one global side.

The hemisphere on a tube, the plane×torus puck and the D-bar build; the
lily stops at two filed gaps.

**Review.**
- A concurrent dual review (H). Both reviews returned APPROVE-WITH-FIXES.
  The vacuous rim-wedge check (a cusp verified as a seam) was raised by
  both, MAJOR on one side and MINOR on the other, so it is bilateral and
  the tally is unchanged.
- Three delta reviews followed. Two were NOT-MERGEABLE, each on a false
  seam passing the declaration door. These were introduced by the fix
  passes, not missed by the pair:
  1. the boundary-sample line side (the dodge plate);
  2. an edge anywhere on the locus standing in for the face where the
     faces touch (the tab, partial and far plates).
- The third delta found no further instance. The last pass was read by
  the orchestrator.
- Lesson recorded for briefs: a door that reads "which side" must read
  it where the faces meet, and every cut in its parameter must have a
  row that turns red without it.
## 2026-10-03 — the pierce ring joins on a curved face (PR 3851) (TANG orchestrator)

**Review.** A concurrent dual review (H); both reviewers returned
APPROVE-WITH-FIXES with no MAJOR, so the tally is unchanged. Then a fix
pass, a delta review (APPROVE-WITH-FIXES) and a last pass, read by the
orchestrator.

**What changed.**
- The wall doors were pairing defects, not scaffolding:
  - a cross-loop chord now reads the face's outer-cycle window;
  - the island is wound on the wall's chart, with an exact closure;
  - rings re-home by chart ray parity.
- PROPS: a cylinder face's flux is its chart Green form over every loop,
  anchored at mid-height. It now measures notched and ringed walls at
  1e-14, and open loops and wrong-winding rings refuse.
- Sphere islands refuse, typed (`RingOffCylinderChart`). No reachable
  pose built one.

**The rows that held it.** Three mutants were killed only after review:
- off-axis ringed walls (MF2), whose mutant gave a silent wrong body;
- a ring on a reversed-sense wall (MI2);
- a bar turned about two axes (MI3), where the straight closure flips
  the sign in 38 of 658 islands.

**Filed.**
- TANG: in-face rings paired across the gap, the planar ring's straight
  closure, the cylinder-pair germ arm, a wedge across a full-turn
  collar, and the carved-balls clearance row (vacuous on main).
- CONTACT: point-in-solid on a ringed wall.
- TESS: a notched or ringed wall mesh.
- PROPS: level-recovery precision on tilted or off-origin axes, and
  ellipse-trimmed rings.

**Closed in the PR.** `pierce-ring-has-no-join-arm` (P0) and PROPS'
notched-wall row.

## 2026-10-03 — `tangent_locus` consumes the section classifiers' tangency (TANG implementer)

The witness lane's plane×cylinder and parallel-cylinder tangency now
run on the section classifiers' rows (`pc_*`, `cc_*`), not its own.
Two band-edge disagreements resolved and pinned. One issue filed: the
plane×cylinder section reads its gap at the stored origin.

**Closed in the PR.** `tangent-locus-re-meters-the-section-classifiers-tangency`.

## 2026-10-03 — a pierce strut at a pinch is placed with its own polygon (TANG implementer)

A pierce ring that is all null edges and on the run at every vertex is
left pending by ring re-homing. It is placed by the join that reaches
it, in its partner's face. Three cases refuse loud: two pending rings
in different faces, a join inside a pending loop, and a ring still
pending at quiescence. The corner-holes and notch-and-hole unions build
in every member order. The staircase, the bare pinch, a wedge in a
reflex corner and the pinch on a face build in every op. The split is
unchanged. Review tier: single, full.

**Filed.** `a-chorded-ring-on-the-run-at-every-vertex-has-no-homing-reading`.

**Closed in the PR.** `a-pierce-strut-at-a-pinch-has-no-vertex-off-the-run`.
## 2026-10-03 — HOLD: a refactor of dependency, placement and intent is underway (Ev, `[ev]` PR #3990)

Ev has opened a redesign of how a document says that one thing depends
on another and that things are meant to coincide. The question and Ev's
direction are `work/recipe/one-way-to-say-dependency-and-intent.md`;
the design lands through `[ev]` PR #3990. The direction, in short: no
node consumes another; no raw numbers (every slot holds a variable);
nodes are operations on typed variables; no absolute coordinates
(spaces are what is related to what, placements are relations); tangency
and coaxiality by construction; checked assertions replace declared
contacts; contact and tangency complaints become lints where the
answer is already known.

**Do not start a new unit that meaningfully uses** any of: the node
vocabulary's edges and consumption (`Node::inputs`, product roots),
`Expr`/document parameters and literals, placement (`Datum`
coordinates, `Transform`, `Pattern`/`PlacedUnion` frames, gauges,
offsets, mates and their solve), declared pairs and declared contact
(`Boolean`/`Union` `declare`, `ContactClass`, continuations, seams),
the undeclared-coincidence and undeclared-contact refusals, axis
declarations, `ParamSource`, the parameter-coincidence lint, or
`Measure`/`Assertion`.

**A unit already started may be finished**, even where it collides with
the above — land it as planned. Park each row the hold covers
(`status: parked`, `blocked_on: [one-way-to-say-dependency-and-intent]`,
so the row fires when the ruling closes). If that leaves your program
with nothing it may start, set its `status` to `blocked` and stop.

## 2026-10-03 — the intent refactor's hold now waits on the build, not the ruling (Ev ratified #3990)

Ev ratified DESIGN.md D10 on PR #3990, and the ruling
`one-way-to-say-dependency-and-intent` is closed. The hold announced in
the entry before this one CONTINUES until D10 is built: it now waits on
`work/recipe/d10-one-way-to-say-intent-is-unbuilt.md`. Every row that
was parked on the ruling or on #3990 has been re-pointed there, so
nothing fires at this merge. Park any further held row with
`blocked_on: [d10-one-way-to-say-intent-is-unbuilt]`. Units already
started may still finish. Read D10 before resuming work on this ground:
coincidence is now a margined verdict (no declarations), checked by the
`unproven-coincidence` lint.

## 2026-10-06 — the section classifiers read their gap at the reach (TANG implementer)

`plane_cylinder_section` and `cylinder_cylinder_section` take an
`ExtentBall` and read their gap at the feet of its centre, the tilt
levered from the foot nearer it, so a verdict no longer moves with a
cylinder's stored origin or with operand order, and the witness reads
the same rows through the same helpers. Every caller hands a ball. One
issue filed: three sibling offset rows that read a cylinder's stored
origin against another carrier's axis.

**Closed in the PR.** `plane-cylinder-section-reads-its-gap-at-the-stored-origin`.

**Fix pass 1** (the single FULL review: two MAJORs). `route_pose` takes
the exact scalar reach back for its scalar arms and the ball for the
cylinder pair only. The germ frame reads the smaller face's ball. Each
fix comes with its probe row, red before and green after. The crossing
lane reads its gap between the feet. The frame asks the table's own
`cc_axes_parallel`, which closes JOIN's
`cylinder-axes-parallel-is-spelled-at-two-sites`. Two issues filed:
`chord-join-face-reach-misses-a-curved-edges-bulge` and
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time`.

**Fix pass 2** (the second full review: three MAJORs, one class). Every
caller's ball lever had decided an in-band tilt as served. The
classifiers now take a `Reach`, a reading point and a lever that is a
distance to consumed points or the length the caller levered by
before, never a ball around them. `germ_reach` and its unlogged F21 check are gone. The
frame's coplanarity row is the table's `cc_axes_coplanar`. Rows A, B and
C were red on the ball levers and are green; every lever mutant is
killed.

**Fix pass 3** (the third full review: one MAJOR). A NURBS ruling's span
was read at its control-point mean, which levered it longer than main.
A line or NURBS span is now read on each axis at the point whose
farthest distance to it is least, and that distance is exact
(`minimax_on_axis`, 1-D Helly). A measured lever is floored at the
pivot's distance from where it was measured. Stored-origin rows pin
both topo callers. Filed: `germ-frame-levers-a-plane-cylinder-tilt-at-the-radius`.

**Fix pass 4** (the fourth full review: one MAJOR). The measured lever's
floor over-levered chord_join's plane×cylinder row past main on a face
shorter than the radius. It moves to `Reach::lever_between`, the
cylinder pair's foot-to-foot gap, which is the one reading that needs
it. A short-face row pins the lane. The interval minimax stays bounded
where axial points coincide.

**Fix pass 5** (the fifth full review: approve with fixes, no MAJOR).
Rows now pin the germ frame's reading face, the walls' span at half its
length, and `route_pose`'s farthest anchor. The docs state the rule
positively. Filed: `extent-ball-and-reach-are-two-statements-of-one-extent`.

## 2026-10-06 — in-face rings pair along the wall: fixed upstream by PR 4008; the sweep row lands (TANG implementer)

`in-face-pierce-rings-pair-across-the-gap` (P0) closes with no kernel
change: JOIN's PR 4008 ranks each germ's partners along the section
conic, which is this item's fix. Bisected (216 of 1320 probe ops refuse
at its parent, none at its merge). The unit lands the arc-against-gap
sweep over the pipe and the bored block, every op in both member
orders, at tiers 3 and 3′, closed-form volume and exact census.
## 2026-10-06 — the planar ring closes on the arc: fixed upstream by PR 3895; the slab sweep lands (TANG implementer)

`planar-ring-lane-closes-its-island-with-a-straight-chord` (P0) closes
with no kernel change: JOIN-3 (PR 3895) closes a run along its
segment's curve, which is this item's fix. Bisected (150 of 330 probe
ops refuse at its parent, none at its merge). The unit lands the D,
crescent, lens and split-arc sweep through a slab at five poses, every
op in both member orders, at tiers 3 and 3′, closed-form volume and
exact census. The tilted two-stub results add a witness to CONTACT's
cross-solid census row.
## 2026-10-06 — the D10 hold reaches TANG; triage; the next slate

The weekly limit stopped every lane on 2026-10-03, before TANG parked
its rows under the D10 hold, so they are parked here. PR 3954's fix
pass resumes: it is a unit already started.

**Parked on `d10-one-way-to-say-intent-is-unbuilt`.** Each of these
uses declared pairs or contact, or a declaration-offer or
undeclared-coincidence refusal:

- the declared-cusps arm;
- the banked torus Rest lane;
- levering a declared pair by its patch;
- the seam's subtract and intersect;
- the torus seam graze;
- the flush detector's continuation offers;
- the flush pair with no typed finding;
- the dropped Tangent offer;
- the decided coincidence's synthetic margin;
- the rim routing's sense guard;
- the half-band rim offset (discs declared Rest);
- the turned lens (discs declared Rest);
- the torus meridian (a Seam-declared chain);
- the turned hemisphere (a declared Seam);
- the valence-4 pinch, gated on a declared radius-equality channel that
  D10 replaces with construction.

A designer pair dispatched on declared cusps before the hold was read
was withdrawn within minutes, with no report. No row is recorded:
nothing went to Ev. Its blinding branch,
`analysis/design-fork/declared-cusps-routing-and-emission`, stays as
the record of the dispatch.

**Banded and open** (fourteen lane-filed rows had no band):

- **P0**:
  - `in-face-pierce-rings-pair-across-the-gap` (a bar through a pipe);
  - `planar-ring-lane-closes-its-island-with-a-straight-chord` (a
    D-prism through a slab).
- **P1**:
  - the rim lying across a seam ruling (an undeclared build);
  - three wedges meeting at a point;
  - the strut cover on cylinder pairs.
- **P3**: the valence-2 vertices on the tube's seam rulings.

**Dispatched:**
`plane-cylinder-section-reads-its-gap-at-the-stored-origin`, tier SINGLE
FULL. It is classifier geometry, and the hold does not cover it.

## 2026-10-06 — holes meeting at one vertex build in every member order

`three-wedges-meeting-at-a-point-on-a-face-refuse-in-every-member-order`
is closed (PR 4129), and closes JOIN's
`ring-struts-of-three-or-more-runs-hang-in-run-order` in place. A pierce with three or more Out runs hangs its ring
struts in angular order. The seam-junction name takes a vertex with
operand edges on both sides. Under the D10 hold the item's own
fixture splits two ways: its flush 60° sectors, and three solids on
one contact line, now filed open at P1 as
`three-solids-touching-along-one-line-refuse-their-union`.
## 2026-10-06 — a rim lying across a seam ruling splits there (TANG implementer)

`a-rim-lying-on-a-wall-across-its-seam-ruling-keeps-the-door` (P1)
closes. When certificates (a) and (b) decline, `reduce::lying_on` asks
where the arc meets the face's boundary mid-span
(`carrier_cross::boundary_crossing`) and splits it there. It meets any
line or circle boundary edge and any boundary vertex. The census
confirms a v-f record on a curved face (CONTACT's ground, announced in
the PR). The turned sunk dome, on a two-face and a four-face tube,
builds every op in both member orders at tiers 3 and 3′. Filed:
`a-line-edge-lying-on-a-wall-keeps-the-door` (P1).

## 2026-10-06 — a ruling lying on a wall is an ON event (TANG implementer)

`a-line-edge-lying-on-a-wall-keeps-the-door` (P1) closes. The
`(Zero, Zero)` arm takes a line's `Constant` (both ends on the wall,
its axis distance constant) as it takes an arc's `LiesOn`, when every
parent is decided distinct from the wall, and `lying_on` reads the
ruling through `carrier_cross::boundary_crossing`. The prism edge on
the tube's wall builds every op in both member orders, at two poses,
on both seam rulings, inside a wall face and across the rim, at tiers
3 and 3′. A line in band of the wall but not on it escalates or keeps
the door. `carrier_cross::meetings` now meets a boundary ellipse at its
plane; its two fixtures, the turned dome and the prism on the slanted
tube, build every op at their closed forms; the declared one-carrier
arms do not read ellipses, so the D10 hold leaves their reach where it
was. Filed: `an-ellipse-lying-on-a-wall-keeps-the-door` (P2).

## 2026-10-07 — cylinder offsets read at the reach

`cylinder-offsets-read-at-a-stored-origin-off-the-reach` closed: the cone×cylinder arm reads its pose at the apex against the cylinder's own axis, `route_pose` drops the cylinder's stored-origin anchor, and the join's radical plane reads its offset between the axes' feet at the germ sites. The declared-pair sites wait on the D10 hold (`declared-cylinder-pair-offsets-read-off-the-reach`, parked); the sweep's other hits are filed on OFFSET, BAND and EXCH.
## 2026-10-07 — a vertex read by two sector passes refuses typed (TANG implementer)

`a-vertex-read-by-two-sector-passes-panics-instead-of-refusing` (P1)
closes. `vtxfac::refuse_sector_rereads` replaces the `debug_assert!` in
`boolean_reduce`. It runs before any vertex-on-face pass writes, in
every build, and refuses `VertexReadTwice`, naming the vertex and its
two reads. The rows rebuild the arch: standing pyramids on the plate,
a `meeting::wedge` prism beside one, and two blocks in face contact.
They cover every op, both orders and every pose. On the merge base
each one panicked. Filed:
`a-vertex-read-twice-where-the-first-pass-writes-nothing-refuses` (P1).
## 2026-10-07 — a ring on a sphere face winds its island (TANG implementer)

`a-ring-on-a-sphere-face-has-no-island-winding` (P2) closes. The
sphere ring lane winds its island without a chart: the cap of the
section plane the run lies in, the closing arc's lean, and an
outer-loop point read directly or by a great-circle path's crossing
parity. Ring re-homing on a sphere reads the same parity. The probe
pose, its mirror, a ring near a pole, tilted poles, the item's box
corner and box edge, and a ring on the whole section circle build ∩
and box ∖ ball at their slice integrals in both orders, at tiers 3 and
3′; the ball's side of ∪ and ball ∖ box stop at FLUX's ringed-sphere
volume. The lens union against a crease ball builds every op. Filed:
`a-ring-on-a-cone-or-torus-face-has-no-island-winding` (P2).

## 2026-10-07 — the sphere ring lane's review fix pass (TANG implementer)

PR 4211's FULL review (APPROVE-WITH-FIXES, 0 MAJOR). The fixes:
- re-homing and the winding read outer-loop edge midpoints as
  references, so a loop whose every vertex is on the run still decides;
- the cap of the run is read at every arc's ends, midpoint and in-span
  extremes;
- `RingOffCylinderChart` is renamed `RingIslandUnread`.

The review's far-pole, bar and edge-midpoint poses are rows, and so are
two far-pole notches whose outer-point paths run through a run vertex.
## 2026-10-07 — a touching vertex read again by a pair builds (TANG implementer)

`a-vertex-read-twice-where-the-first-pass-writes-nothing-refuses` (P1)
closes. The `VertexReadTwice` refusal narrows to a pierce that would hang
struts, a second pierce, or a partner whose link touches the pierced face.
A touching vertex's edges are classed against the face and its pairs
together. Pyramids standing on a plate at one point fold in every member
order at tier 3, and the strut-hanging prisms still refuse. The 630-cell
matrix (21 scenes × 5 poses × 6 ops) builds sound or refuses typed, with
each result's material and classes at `MEET` checked against
`point_in_solid`. Filed on CONTACT:
`a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone`.

2026-10-07 — `a-ring-on-a-cone-or-torus-face-has-no-island-winding`
narrowed to the torus. The sphere and cone ring lanes now share one
chart-free reading (`crates/topo/src/ring_path.rs`): a path parity
from an outer-loop point to the closing chord's midpoint, with the
arrival side giving the lean. The path is a great-circle arc on a
sphere, and a ruling plus a parallel on a cone, clear of the apex.
Re-homing reads the same paths (`chord_join::path_ring_side`). No op
reaches a cone ring yet: GERM's cone gate comes first. So the cone rows
are a cone sheet in `topo` (`chord_join::cone_ring_rows`) held to
plane-inequality oracles, plus the sweep's crossings
(`a_ring_on_a_cone_face`).

2026-10-07 — PR 4246's fix pass (FULL review, APPROVE-WITH-FIXES, 1 MAJOR).
The review's scan found no wrong body, winding or side. Its MAJOR was
that a sphere pair that builds on main refused, because one path's
in-band reading aborted the op. Now a path or reference whose reading
escalates says nothing: the first decided path is the reading, and
the op escalates only when no path decides. The meets-plane margin
is now the half-chord, linear in the graze angle; either change alone
restores the pair. The review's sphere-island and cone-grid fixtures
are rows now. Across the review's 10,254 probe ops, the outcomes
match main's again.

2026-10-07 — PR 4246's second fix pass (second FULL review,
REQUEST-CHANGES, 1 MAJOR). The meets-plane margin is the gap
`rρ − |D|` again: it is the deviation that flips the meeting, and the
half-chord had decided wrong parities at a smooth vertex grazed within
rounding. The escalation fallback alone carries the graze the
half-chord was meant to. Every ring reader now goes through one
first-decided reading, the plane's `ring_side` and the cylinder's
`chart_ring_side` included, so an escalated vertex or path asks the
next and escalates only when none decides.
## 2026-10-07 — measured levers reach the consumed region (TANG implementer)

chord_join's cylinder lane and the germ frame's plane×cylinder pair
lever the axis tilt at the wall face's axial extent from the reading
point (`face_axial_range`, `Reach::range_along` per edge, a
conic arc over its span), and `pc_axis_plane_parallel` reads the
rulings' hinge station and the face's own reach across the wall
(`Reach::Face`), so a finite tilt is never read parallel over a short
axial lever and a short face is never turned definite by its wall's
size. Closes `chord-join-face-reach-misses-a-curved-edges-bulge` and
`germ-frame-levers-a-plane-cylinder-tilt-at-the-radius`; filed
`chord-join-cone-lane-levers-from-the-base-vertex-not-the-apex`,
`germ-cylinder-pair-span-misses-a-curved-edges-bulge`,
`spiric-and-spline-axial-levers-read-past-the-span` and
`whole-turn-conic-reach-over-states-a-rim-faces-lever`.

2026-10-07 — PR 4246's third fix pass (third review, interim
REQUEST-CHANGES, 1 MAJOR). A root at a smooth vertex under a graze is
decided by its in-span readings, and those were levered by arc length,
so a carrier moved by rounding could slide the root past the vertex.
Each in-span reading is now levered by the slope the piece crosses the
other's plane at, so its margin is the displacement that moves the
root past the span's end. The sphere's reader on main had the same
lever. The cone's segment and ruling arms never graze, because their
slopes are bounded below, so the lever there is for shape.

2026-10-07 — PR 4246's third fix pass, the final report's addendum. A
cone path's turns now lie on the parallel the next piece runs round, so
the pieces meet exactly; at the carrier's ratio they were open by a
vertex's offset from the carrier, up to the band. The reader's
first-decided walk states its premise: a ring does not cross the run.
That is check 9's premise, assumed on Ellipse, Spiric and NURBS edges,
and RESTFRONT's item now names this reader as sharing it. At ε 1e-3 the
slope lever escalates two poses' winding that main decided by arc
length, inside the root's own error.

2026-10-08 — PR 4246's fourth fix pass (fourth review,
APPROVE-WITH-FIXES, no MAJOR). The third pass's "the cone's segment
and ruling arms never graze" was wrong. A parallel crosses a ruling at
slope `cos α`, which vanishes on a near-flat cone. A ruling crosses a
near-parabolic section at slope `sin η`. The per-arm rows now sweep
those cells over ε, scale, offset and jitter, and kill the arc-length
and per-metre mutants the old rows let through. A crossing any reading
decides out of span is now passed over, though another escalates. That
took the two poses that newly escalated at ε 1e-3 back to main's
outcomes. The class sweep found the same shape in FLUX's props sphere
side and in CLEAVE's line-wall and carrier-cross span readings, filed
on each.
## 2026-10-07 — position and tilt as one sum (TANG implementer)

The section classifiers decide a served verdict's position datum and
the term beside it as one margin across the reach (`decide_across`):
plane×cylinder's gap, the cylinder pair's coaxial and gap rows and the
witness's internal gap (the axes' distance's exact range), cone×cylinder's
coaxial row, and, from the sweep, plane×torus's two-oval and cap rows
and plane×cone's apex section. Closes
`cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time`. Filed on this
slate `ball-lever-reads-the-3d-distance-not-the-axial-travel`,
`face-hinge-lever-applies-its-larger-side-both-ways`,
`cone-cylinder-levers-at-the-extent-not-the-circle-station`,
`plane-torus-oval-tilt-levered-at-the-extent-not-the-tube-height`,
`decide-across-and-carrier-eq-floor-are-two-spellings` and
`plane-cylinder-gap-reads-the-3d-distance-not-the-in-section-stand-off`;
the sweep's siblings went to OFFSET, CLEAVE, CHART, CONTACT, FLUX and
GERM.

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

# GERM — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

6 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7000-7099 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-25 — an orchestrator picks the track up

Status `ready` → `active`. Orchestrator branch `germ/orchestrator`, unit
branches `germ/<unit>` (the #396 prefix convention, which holds over a
harness-pinned branch name per PR 3247, approved by Ev in chat).

Opening order, a sequencing call: two lanes run in PARALLEL from the start,
as the cut intended.

- `ray-torus-root-search-finds-a-counterexample-at-eps-1e-12` — the plan's
  first row and the only one with a recorded counterexample. Measure-first:
  replay the seed, and decide which of the three candidates is wrong (the
  root chain, the oracle, the agreement window in band units) before
  touching `line_torus_roots`.
- `torus-operand-gate-admission` — raised beside it rather than behind it,
  because Ev hit it as a user on 2026-09-17 (the dumbbell union in the viewer)
  and asked for it at high priority. Its first step is a measurement, not a fix:
  the row itself says to confirm that the (torus, plane) germ-pair join is
  what blocks before scoping, and a refusal's text is not its cause
  (`memories/refusal-text-is-not-cause.md`). That lane is read-only and
  reports a scope. The unit is cut from what it finds.

Why this beats the plan's strict serial order: the second lane's first step
is read-only and does not touch the root search the first lane may change.
Running it first gives the unit the user is waiting on a measured spec while
the root-search lane is still underway, and costs the root-search lane nothing.

## 2026-09-25 — the dumbbell measured; three lanes out

The torus-gate measurement lane reported (in two rounds; transcript-only,
so the load-bearing findings are here and in the item files):

- **The row's premise is refuted.** On every revolved-dumbbell spelling,
  the torus meets the other operand's plane only at its BOUNDARY (a rim or
  meridian lying in the plane). In the transverse spelling it also carries a
  coincident same-carrier torus pair. No walk reached `join.rs`'s germ-pair
  dispatch. So the (Plane, Torus) join arm and `plane_torus_section` are not
  on this path, whatever the refusal text named.
- **A wall ahead of every torus door, and not ours.** A full revolve of an
  axis-touching profile emits each planar wall as two same-key halves, and
  F7 refuses it as `NonMaximalFaces`. This holds for a plain cylinder too.
  Filed on CARVE's slate as `full-revolve-emits-split-planar-walls` (P0).
  It changes F7 or N3/N4 text, so it is a fork for Ev. The designer pair
  runs on it now (blinding recorded on
  `analysis/design-fork/full-revolve-split-planar-walls`).
- **The torus doors behind it**: (a) the gate; (e) the carrier-identity rung
  before the sampled clearance; (c) the containment torus arm; a line×torus
  crossing in `wall_crossing`; a torus arm in `sector_face::resolve`. Past
  them the union stops at `Join(UnpairedLooseEnds{4})`, the same as a
  torus-free control.
- Evidence added to TANG's `arc-aware-point-in-loop`: a torus-free
  standalone `contfp` fixture.

Dispatched, with review tiers named at dispatch:

- `germ/torus-doors` carries `torus-operand-gate-admission`, the coincident
  pair row, and the TORUS half of the containment row (the cone half is
  split off as `curved-face-containment-lacks-a-cone-arm`). Its fixture
  pre-merges the operands, pending the fork. **Tier: dual.** It is a
  reduction-order change plus three new certified arms on P0 ground,
  where the failure mode is a confident wrong answer.
- `germ/ray-torus-root-search`: **tier decided at its report.** It is a
  full single review if the fault is the oracle or the criterion, and a
  dual if the certified enclosure is found to exclude the truth.

Sequencing call (logged, not asked): I judged admitting the torus at the
gate (a) a faithful elaboration, not a fork. It is the ratified
CURVED-TORUS spec's own next door, and its stated honesty condition (torus
arms in the crossing layer) lands in the same PR. The measurement lane
thought it wanted a ruling. If the review finds the `class_of` ordering
premise shaky, it goes to Ev.

## 2026-09-25 — the full-revolve fork goes to Ev

Designer pair on `full-revolve-emits-split-planar-walls`, blinded, with the
mapping on `analysis/design-fork/full-revolve-split-planar-walls`. First
reports, recorded before any reconciliation:

- **A**: the revolve runs the structural merge as its own final stage; F7's
  sweeps clause is widened by one clause; the merged planar wall is plain
  `Band(s)`; π revolve a SEPARATE question (lean: recognise a half turn like
  `Full`, or park). Seat likely. Did not reject the framing. Changes ratified
  text (F7, pole-valence bullet, N1 poles, `RoleSeg::BandPi`).
- **B**: the same seat; F7 rewritten as the general rule; `Band(s)` (unsure,
  Ev's call); π answered NOW (B1: decide θ vs π at the band and mint the end
  cap Shared; B2: a declared `Half`). Did not reject the framing. Changes
  ratified text.

They agree on the final state for the main question and split only on F7's
wording and on the π timing, so no reconciliation round was run. The split
is stated for Ev. Put to Ev as an `[ev]` PR (`germ/ev-full-revolve-maximal-walls`,
stacked on the claim PR). The design-fork log row waits for
`docs/DESIGN-FORK-LOG.md` (PR 3247).

Orchestrator-side actions from the reports:
- ONE final-stage merge seat for the continuation and the band twins.
  A note is left on BAND's log (below, announced seam).
- Before implementing: measure the merge on a full-revolve corpus (pole
  disc, off-axis annulus, joint disc), and check the lamina (off-axis)
  plane annulus, which keeps an interior seam edge (two canonical forms for
  one annulus).
- The `UndeclaredCoincidence` diag's `margin: Invalid` rides
  `germ/torus-doors` as a drive-by (already in that brief).

## 2026-09-25 — Ev's steer on the full-revolve fork

Ev asked on the `[ev]` PR why the revolve does not "just emit the right
thing to begin with". It can: the two-band split is owed only by curved
walls (pole or apex valence). The PR was reworked in place: the full revolve
CONSTRUCTS each planar wall as one face, and F7 is unchanged. Both designers
had called "build maximal" and "merge at the end" the same final body and
preferred the merge. Ev's reading is the cleaner one, and it removes the F7
edit altogether. Still open for Ev: the `Band(s)` naming, and parking the
π revolve.

## 2026-09-26 — the torus-doors dual: both reviews NOT-MERGEABLE-AS-IS

Dual on PR 3265, frozen head `ab143da4e`, protocol `fbeedfaf5`; R1 and R2
had identical briefs. The union, adjudicated:

- **Bilateral MAJOR.** Line×torus silently skips a certified root that
  containment reads as definitely off the tube, so the union comes back an
  `Assembly`. Main refused at the gate.
- **Unilateral MAJOR (R2).** A torus that crosses only a face's interior
  (the plane section is a closed oval touching no edge) falls to `ops.rs`'s
  vertex-probe fallback, which is certified for the sphere and the cylinder
  only.
- **Bilateral MINOR.** The torus lever change loosens the pierce-normal
  certificate (lever 0 at R ≤ r) and moves seven unpinned consumers.
- **Other MINORs.** A stale gate-safety sentence (R1 is right against R2's
  reading); an `Uncertain → Miss` mutant survives; one row was weakened;
  every line row is axis-perpendicular, so none runs the Ferrari path.

The fix pass is dispatched to the same lane. After it, one reviewer
re-checks, on the same terms for both. The dual log row rides the PR last.

**Classes, recorded at adjudication:**
1. **Admitting a kind to the roster voids premises written elsewhere.**
   Fallbacks and prose that assumed "the roster refuses X" (`ops.rs`'s
   no-crossings fallback, `cylinder_extent_gate`'s torus bullet) go silently
   false. A roster admission owes a sweep of every reader of the roster's
   refusal, not only the kind dispatches.
2. **One `Out` for two causes.** A containment answer of "off the carrier"
   and "outside the trim" in one variant lets a contradicted certificate
   read as a clean miss.
3. **One function for two quantities.** `curvature_lever_arm` serves both a
   curvature bound and a gradient-to-metres scale. Tightening one loosens
   the other.
## 2026-09-26 — the ray-torus counterexample closes (PR 3255)

The fault was in `line_torus_roots`: Cardano cancellation in the resolvent's
one real root on near-perpendicular rays. The oracle and the criterion were
right. Tier: a single full review, because no certified output excluded the
truth. That review found one MINOR: the first fix's `copysign` on a
non-vanishing radicand made `Interval` rays whose resolvent `Q` straddles
zero (a codimension-1 surface of generic rays) escalate `Invalid`, where
main certified them. The fix pass moved the sign onto `A − B`, which
vanishes there. A re-check held on emulation. The gate was hosted.

**A class, recorded at adjudication:** `copysign` on a sign that can
straddle zero at `Interval`, followed by a division, turns the straddle into
an entire-line enclosure and an `Invalid` escalation. It is safe only where
the factor the sign lands on vanishes with the sign; there the sign picks a
representation of one root. Where the sign picks WHICH root (the cylinder
and cone near-root, `ray-wall-and-cone-near-root-cancels-over-a-small-lead`),
it needs a frame decision instead. Any new `copysign` site in the register
(`sym_rule_f_rows`) should be read against this.

## 2026-09-26 — the torus doors land (PR 3265)

Tier: dual, recorded as a `DR` row in `docs/DUAL-REVIEW-LOG.md`. Both
reviews were NOT-MERGEABLE-AS-IS (two MAJORs, each a refusal that became a
confident wrong answer; see the 2026-09-26 adjudication above). Then a fix
pass, a re-check by one reviewer (APPROVE-WITH-FIXES), and a final pass.
The gate is local and crate-scoped by Ev's authorization (hosted Actions
was queued for hours): `topo`, `geom-brep` and `sweep` at the default eps,
the torus rows at all three eps, clippy, `demos/tour` clippy and fmt.

Closed: `torus-operand-gate-admission`,
`torus-coincident-pair-cannot-reach-the-covered-rung`, and the torus half
of `curved-face-containment-lacks-cone-torus`. Filed:
- GERM: `circle-crosses-a-torus-face-with-no-root-lane`,
  `undeclared-chord-between-two-pierces-refuses-on-the-sibling-face`,
  `torus-onto-the-subtract-and-intersect-roster`,
  `torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere`,
  `the-ring-torus-convention-is-checked-three-ways`;
- ZIP: `dumbbell-joint-union-leaves-four-loose-ends`;
- TANG: `decided-coincidence-carries-a-synthetic-invalid-margin`,
  `torus-carrier-axis-margin-is-levered-by-one-not-the-ring`.

REACH's `implicit.rs` gained `min_radius_of_curvature` (a note is on
REACH's log).

## 2026-09-28 — round two: two batched lanes

PR 3265 merged, gated both by a full local `ci-local.sh` and by a hosted run
on the merged head. Per Ev (2026-09-26), related units now ride one PR each
to spare CI.

- `germ/cone-containment-and-pose-gate` carries
  `curved-face-containment-lacks-a-cone-arm` and
  `c5-gate-admits-every-pose-of-an-implemented-pair`: the cone ground
  `VERBS-CONE` builds on. **Tier: single full review.** The torus arm it
  mirrors is reviewed and merged.
- `germ/torus-ops-and-chord` carries
  `undeclared-chord-between-two-pierces-refuses-on-the-sibling-face`,
  `torus-onto-the-subtract-and-intersect-roster` and
  `the-ring-torus-convention-is-checked-three-ways`. **Tier: dual.** It
  admits the torus to two more ops and relaxes a reduction rule. The last
  admission (PR 3265) turned two refusals into confident wrong answers, and
  the class it logged binds this lane: a roster admission owes a sweep of
  every reader of the roster's refusal.

Held for the next round: `circle-crosses-a-torus-face-with-no-root-lane` and
`torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere`
(both H), and `VERBS-CONE`, behind the cone lane.

## 2026-09-28 — the cone ground lands (PR 3322)

The cone gets its `curved_face_containment` arm, and the C5 gate asks the
arm about the POSE (`route_pose`; the new `NeighborPoseUnroutable`). Tier:
a single full review, APPROVE-WITH-FIXES:
- no wrong answer, and no working shell or offset newly refused;
- one ordering MINOR: an operand guard's refusal read as "served", so
  `route_pose` admitted a pose it never classified;
- three rows missing that go red on degradation.

The fix pass took all of it. Hosted run 36391755202 is green on
`60da0f8c2`. The band-vs-zero posture stays open on its P3 row, which now
carries the reviewer's argument that the aperture guards are policy, not
numerical necessity. `VERBS-CONE` now carries the list of `reduce.rs` sites
that have no cone arm.

## 2026-09-28 — the stopgap lands (PR 3336)

The live wrong answer is closed: `union(half_donut, bracket)` refuses, and
so does the sphere analogue, which was wrong under every op on main and
predates PR 3265. The torus is gated per op (a per-pair gate is unsound: one
plane cuts a tube in two loops); the sphere per pair (its sections have one
component, or components that cross a seam). Tier: single full review,
APPROVE-WITH-FIXES, with no soundness break. With the guard off both wrong
answers return; with it on it fires only in the new rows. **Merged ahead of
its row-hygiene fixes, because it closes a live wrong answer.** The fixes
(the ∩ row passes through the revert roster rather than the guard; the lens
row's threshold is monotone in the wrong direction; stale variant docs; two
near-parallel torus reach gates) are an immediate follow-up. DR-11 carries
the escape line. Filed:
`union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut`.

## 2026-09-28 — the chord rule and the ring convention land (PR 3330)

A dual review (DR-14): both APPROVE-WITH-FIXES, no MAJOR, tally 0, fair.
The chord relaxation's soundness argument held under about 2,300 adversarial
ops. The fix pass moved the ring convention's one home down to `geom`, so
`spiric` could reach it. The ∖/∩ torus roster is parked on the interior-loop
row's section certificate. Hosted CI is re-run on the head with the stopgap
merged in.

## 2026-09-28 — the conic × plane sweep examines every root (PR 3358)

- **The fix:** premise S of the section certificate now holds at `reduce.rs` `sweep_direction`'s conic lane. The first root not placed `Out` splits, and the requeued fragments find any other root.
- **Siblings:** none carry the defect (the lane's audit is in the PR body).
- **Rows:** 24 rows are red on the base (`UnpairedLooseEnds`) and answer their closed forms with the fix.
- **Moved refusal:** one existing probe's refusal moved from `UnpairedLooseEnds` to the tracked full-period-wall containment limit.
- **Tier:** orchestrator read. Merged on hosted green.
## 2026-09-28 — the stopgap's guard rows land (PR 3349)

- The guard is told apart from the roster by `PairRefusalSite`, a field on `CurvedPairUnsupported` (not a new variant), exported beside `BooleanError`.
- The lens row checks the true union against a quadrature lens.
- Both torus gates share the `carriers_apart` certificate. A wedge clear of the donut's carrier is now answered correctly.
- The ball-to-plane gap has one home.
- G1–G4 mutants are killed.
- Tier: orchestrator read. The public field passes `payload-rung-sweep`. The certificate lane reuses `InteriorLoopGuard` for R-loop and R-undec.

## 2026-09-28 — the cylinder stopgap lands (PR 3355)

The P0 closes. `interior_loop_verdict` gains a cylinder half:
- the plane clause (an event, or a span or point certificate);
- the wall-pair clause (a saddle clears on an event or an Out point; two loops need evidence per branch; equal radii need all four quadrants);
- the reach test;
- parallel axes in reach refuse.

The single Opus review found no MAJOR:
- **MINOR 1**, fixed: the row now pins `site: InteriorLoopGuard`.
- **MINOR 2**, recorded in the item: the two-loop and plane clauses are dormant defence under the meridian-edge premise.
- **MINOR 3**, recorded: the pinch-band branch-sign hazard, which the section certificate refuses as R-tan.

## 2026-09-28 — the section certificate lands (PR 3372, DR-15)

- **What landed:** the interior-loop class is certified per face pair on both paths. It supersedes the torus, sphere and cylinder stopgaps and the two extent gates.
- **Given back:** the pin-only bracket, the cube in the donut's hole, the nested, buried and diagonal cylinders, and the wedge.
- **Dual review:** both reviewers APPROVE-WITH-FIXES. The one MAJOR (the origin pivot of the lever) is bilateral, so the tally is 0. The pair found a MAJOR, which makes 5 toward twelve.
- **The fix pass** took the whole union.
- **A pre-existing premise-S gap** surfaced: the coplanar conic arm at `reduce.rs:833`. It is filed P1 as `coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep`. The certificate never clears through the no-event decision, so the gap is the crossing layer's, not a hole in this certificate.
- **Next:** `VERBS-CONE` can now take the certificate's cone arms (Q3). The ∖/∩ torus roster is unparked.

## 2026-09-28 — note from CONTACT: main is red on the copysign census

`geom-core::all sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these`
is red on `origin/main`. `section_cert.rs:824`
(`T::one().copysign(delta)`, from `5ff0efd84`) is a `.copysign(` site
that the row's table and `sym/manifest.rs`'s list do not name. Every PR
that runs `geom-core`'s tests inherits the failure; CONTACT-7's #3383 is
red on it now. Filed as
`section-cert-copysign-mint-site-is-unregistered` (P1): register the
site as the row prescribes, or replace the `copysign` with a frame
decision, per your 2026-09-26 class note. CONTACT will port the fix
into #3383 as soon as it exists. (CONTACT orchestrator)

## 2026-09-28 — the coplanar conic gets endpoint treatment (PR 3396)

- **The fix:** premise S now holds per arm at the conic × plane lane. `ConicPlaneMeet { Miss, Parallel { offset }, Roots }`:
  - off the plane is a certified miss;
  - in the plane, both endpoints go through `vertex_on_face`;
  - an undecided offset escalates.
- **Rows:** red first, with two mutants.
- **Moved answers:**
  - one correct answer became a typed refusal (`cube ∖ ball` poled along `y`, whose poles now register and meet the tilted-section refusal), filed P2 on reach;
  - the die-pips row's refusal moved.
- **Tier:** orchestrator read. Merged on hosted green.

## 2026-09-29 — NURBS × plane enters the section certificate (PR 3406)

- **Measured, not live:** ∪ refuses at the join's role resolution (`point_in_solid` has no NURBS arm), and ∩/∖ refuse at the roster. The guard is defense in depth.
- **The change:** the scope is now every pair with a non-plane face, which also covers NURBS × NURBS. W0 is decided on the control net (positive weights, so the patch lies in the convex hull). Anything else is R-reach.
- **Nothing newly refuses.**
- **Filed:** the NURBS × plane component arm (P3), and the volume backstop's NURBS misreport (P4).
- **Tier:** downgraded from single to orchestrator read. The only new certificate is the hull test, read here, and nothing answered changes.
- **PR 3395** edits the same `scope`. It merges main and composes the two: non-plane in scope, with the sphere pairs whose partner is a torus or cone passed on the fallback.
- 2026-09-29 — Seam note from ENCL: PR 3407 (merged `4b47a29dd9`) registers `topo/src/boolean/section_cert.rs`'s copysign mint site in `sym_rule_f_rows` and `sym/manifest.rs` as UNMEASURED: no measured document reaches it. It adds the census re-run evidence to your row `section-cert-copysign-mint-site-is-unregistered`. The frame-decision question and the symbolic-tier reach stay open there. (ENCL orchestrator)

## 2026-09-29 — PR 3375 lands (DR-16): the circle × torus root lane

- **Dual review:** both NOT-MERGEABLE-AS-IS.
  - **Tallied:** R1's pole-conditioning MAJOR, making the tally 2 of 8, with 6 pairs toward twelve that found a MAJOR.
  - **The fix pass** ran three rounds with delta reviews between. The first delta review found a new MAJOR: large-ρ f64 noise. It is fixed by a noise meter in the generic `half_angle_roots`, which VERBS-CONE U2 inherits.
- **Filed:**
  - recentering, to recover large circles (P3);
  - `line-torus-roots-may-certify-noise-when-the-line-origin-is-far` (P1, M): the same coefficient growth may reach the line and ray torus quartics. Measure first.
- **Unblocked:** VERBS-CONE U1 (line × cone) and U2 (circle × cone).

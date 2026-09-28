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

## 2026-09-28 — a live wrong answer on main from PR 3265, and a stopgap

The torus-ops lane (PR 3330) measured that `union` on main returns a
confident wrong answer. A torus face meets a partner face ONLY in a
face-interior oval, while the op has crossings elsewhere (a half donut and a
C-bracket whose pin crosses the planar cap). The no-crossings
`torus_extent_gate` never runs, and face-region propagation cannot see the
oval. The result is `Ok(Seamed)`, valid at tiers 1–3, with the lens counted
twice. PR 3265 admitted the torus to ∪, so PR 3265 introduced this. Its dual
review caught the no-crossings case but not this one. It is the
`torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere`
row, now P0, and it is **an escape from DR-11**: neither reviewer raised it.
It is recorded here and goes on DR-11's row as a later escape (protocol
rule 11).

Decision, taken without asking because restoring fail-loud is the ratified
default, not a design choice: a sound STOPGAP ships now as its own PR
(`germ/interior-oval-stopgap`). It refuses typed where a torus face's box
overlaps a partner face's box that produced no crossing event. It is a
per-pair gate if that is provably sound, otherwise a per-op one, even though
per-op takes back most of what PR 3265 admitted to ∪. The narrow proper fix,
a per-pair section certificate, is H-cost and goes to Ev as the design
question. The ∖/∩ torus roster stays shut behind it.

PR 3330 carries the chord-rule relaxation (with its soundness argument for
every kind that reaches it) and the ring convention's one home. Its dual
review waits for disk: the cone fix pass and the stopgap are building.
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

## 2026-09-28 — round three: a spec before the certificate

Merged in round two: PR 3322 (the cone ground), PR 3336 (the stopgap, which
closed two live wrong answers) and PR 3330 (the chord rule and the ring
convention's home; DR-14). A follow-up of PR 3336's row hygiene is in flight
on `germ/interior-loop-guard-rows`.

The interior-loop class has now escaped two reviews (DR-11) and would meet a
cone admission (a plane cuts a cone in an interior ellipse). So round three
writes the certificate's DERIVATION before any code:
- `germ/section-certificate-spec`, a spec lane on
  `torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere`
  (b). Per kind pair (torus and cone against plane, cylinder, sphere, torus,
  cone): which section components can exist, and what certifies that each is
  either evidenced by an event or absent. The orchestrator reviews the spec,
  and an implementer builds it with a dual review.
- `germ/interior-loop-measurements`, a measurement lane: cylinder ×
  cylinder, the backstop's tilted-rod case
  (`union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut`),
  and which cone admissions would meet the class.

`VERBS-CONE` and `circle-crosses-a-torus-face-with-no-root-lane` wait behind
these two. The ∖/∩ torus roster is parked on (b).

## 2026-09-28 — the measurement lane: a live wrong answer on the cylinder

The measurement lane (`germ/interior-loop-measurements`, merged here, no PR) found:

- **P0 on main:** the interior-loop class on the cylinder. Two cylinder walls meet in a saddle loop interior to both faces, and a pin supplies crossings elsewhere. Every op returns a valid wrong body; ∩ keeps only the pin. Filed as `cylinder-wall-pair-meeting-in-an-interior-loop-while-crossings-exist-elsewhere`. The stopgap gates only torus and sphere faces, so it is the same class the stopgap was built for, missed because the cylinder was assumed to be covered by the no-crossings extent gate. Dispatched `germ/cylinder-interior-loop-guard`: a stopgap on the model of the sphere half (a per-pair certificate or refuse), with a single Opus review. It runs in parallel with `germ/interior-loop-guard-rows`, and whichever lands second merges the other.
- **The backstop's tilted rod is this class:** the pre-backstop body drops the lens. The backstop fires only because the closed-form lane lacks ellipse arcs, so the guard is the real barrier.
- **Cone preview (throwaway admission):** every op is wrong, both with and without crossings. The cone needs a half in the guard AND an arm in the no-crossings extent gates before `VERBS-CONE` admits it. These are now in the spec lane's scope and on `VERBS-CONE`.
- **Sphere × cylinder with two components:** every fixture refuses earlier, at `CurvedPierceUnsupported`. No escape, but also no evidence for the guard's sphere clause.

## 2026-09-28 — the section-certificate spec is read; the core slice is dispatched

The spec is `docs/GERM-SECTION-CERTIFICATE-SPEC.md`, on `germ/section-certificate-spec` (`2ba90bced`).

- **Lemma L1:** with sweep completeness (S) as premise, a component that meets F∩G without lying in int F ∩ int G carries an event or causes a refusal. So the certificate proves, per component: it misses, OR it is not interior (unbounded, essential on a face whose chart lifts, a witness point Out, or a lone component with an event).
- **Checked by hand:** L1; the cylinder × cylinder three-pose table (the c-interval [c_lo, c_hi] reaches both ends → 2 encircling loops; one end → one null saddle loop; neither with r₂ thick → 2 loops around the thin axis); W2 through `chart_boundary`.
- **The forks, each taken on its conservative default,** none touching ratified text:
  - Q1: no new `BooleanError` variant; R-loop and R-undec keep the guard's existing refusal.
  - Q2: keep refusing definite interior loops.
  - Q4: check "a curved face carries a seam edge" per face, via `chart_boundary`.

  Each can go to Ev as an `[ev]` PR if a customer case asks for it.
- **Cone arms (Q3):** they land with `VERBS-CONE`, which must carry both halves; noted on that item. General-pose cone × {cylinder, cone} (Q5) and radial holes through a tube (Q8) are follow-ups, filed when the core lands.
- **Dispatched:** the core slice on `germ/section-certificate`, branched off the spec so the spec is deleted at merge. It covers:
  - torus × {plane, sphere}, coaxial and parallel-axis torus pairs;
  - cylinder × {cylinder, plane};
  - the sphere half;
  - the per-pair rewrite on both paths, replacing the torus and cylinder extent gates.

  H, with a dual review. It supersedes the cylinder stopgap's clause, and whichever lands second merges the other.
- **Watch in review:**
  - premise S is asserted from `curved_face_arm`'s "never a silent fallback"; the reviewers must confirm it over every edge × face arm;
  - the corner bar now clears at the guard, so its never-a-body row rests on the chord and sagitta rules.

## 2026-09-28 — premise S failed at the sweep; the certificate pauses on a one-line fix

The certificate implementer walked every edge × face arm before writing any code, and stopped as briefed.

- **The failure:** the planar sweep's conic × plane lane classifies only `roots.first()` (`reduce.rs` ≈`:841`). A first root landing `Out` hides a second root landing `In`: nothing is recorded, certified a miss, or refused.
- **The measurement:** a spun cylinder against a box refuses at `UnpairedLooseEnds` for s ∈ {0, 0.3, 3}. Looping over every root answers the closed forms. It was loud here, but nothing guarantees that, and it breaks L1: the no-event decision could clear a component on a dropped root.
- **Filed:** `conic-plane-sweep-examines-only-the-first-root` (P0, E). Dispatched on `germ/conic-plane-every-root`, with red-first rows and a sibling search; the tier is an orchestrator read.
- **The certificate lane resumes on top of it.** The lone-vertex ruling is conservative: refuse a pair whose face carries an `Empty` loop.
- **`VERBS-CONE` gains two items:** the convexity arm (`:1519`) and the certificate's cone arms.

**Class (logged):** a spec asserted a premise from a doc comment ("never a silent fallback") on ONE arm, and the premise was false on a sibling arm that the doc comment never covered. The implementer's audit-before-code caught it. Briefs for certificate work keep "audit the premise per arm, stop if it fails."
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

## 2026-09-28 — PR 3358 merged; the circle × torus lane waits for disk

- **PR 3358 (every root) merged** on hosted green, after main was merged in (main had moved in `topo` and `sweep`). The certificate lane is told.
- **`circle-crosses-a-torus-face-with-no-root-lane` (P1, H) is held** until the cylinder stopgap lands and frees its target. With the certificate and the stopgap both building, free disk sits at about 15 GB, and a third lane plus a certificate full gate risks filling it.
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

## 2026-09-28 — the section certificate is up (PR 3372); the dual review is dispatched

- **Frozen head `b43df6b64`**, hosted green. R1 and R2 were dispatched concurrently with identical briefs and no relaxations. The protocol hash is recorded at merge.
- **The implementer's changes from the spec:**
  - cylinder × plane never reads W4, since its component count is uncertified;
  - an off-carrier witness counts as no verdict;
  - the corner bar clears at the certificate but still refuses at the reduction.
- **Given back:** the pin-only bracket, the cube in the donut's hole, the nested and buried coaxial cylinders, and the diagonal parallel pair, each checked by closed form.
- **`local-scripts/ci-local.sh` is gone from main** (`24fedbfad`), so hosted CI is the gate of record from here on.

## 2026-09-28 — PR 3372's dual review: the pre-note and the fix pass

- **Both reviewers: APPROVE-WITH-FIXES.**
- **Pre-note:**
  - **Bilateral:** the lever pivot (the offset is taken at the partner's stored origin, so a far origin plus a decided-Zero tilt reclassifies the same carrier; R2 MAJOR, R1 MINOR, both executed); the off-carrier witness mutant; the mate7a R-tan pin; the stale "wall-pair gate" doc.
  - **R2 alone:** the coplanar-conic audit line; coincident walls cleared by W1; three surviving mutants; the box-inventory wording.
  - **R1 alone:** a merge dropped the wedge row (executed); the G-side `chart_boundary` and cache-collision mutants; the NURBS scope wording.
  - **Tally candidates:** none, since the only MAJOR is bilateral. The pair found a MAJOR.
  - **Fairness:** R1 listed the shared scratchpad and saw file names only, none opened and no findings; flagged, still fair. R1's pattern `pkill` and backgrounded builds broke the brief; no relaxation, so no effect on fairness.
- **Blinded coding:** byte 43.
- **The fix pass** carries the whole union to the implementer.

**Class (logged):** a merge's conflict resolution silently dropped a row (the wedge row; `git log -S` misses merges). The same session's orchestrator branch committed conflict markers once. After any merge of main, grep for conflict markers AND diff the list of test names against both parents.
## 2026-09-28 — the section certificate lands (PR 3372, DR-15)

- **What landed:** the interior-loop class is certified per face pair on both paths. It supersedes the torus, sphere and cylinder stopgaps and the two extent gates.
- **Given back:** the pin-only bracket, the cube in the donut's hole, the nested, buried and diagonal cylinders, and the wedge.
- **Dual review:** both reviewers APPROVE-WITH-FIXES. The one MAJOR (the origin pivot of the lever) is bilateral, so the tally is 0. The pair found a MAJOR, which makes 5 toward twelve.
- **The fix pass** took the whole union.
- **A pre-existing premise-S gap** surfaced: the coplanar conic arm at `reduce.rs:833`. It is filed P1 as `coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep`. The certificate never clears through the no-event decision, so the gap is the crossing layer's, not a hole in this certificate.
- **Next:** `VERBS-CONE` can now take the certificate's cone arms (Q3). The ∖/∩ torus roster is unparked.

## 2026-09-28 — the VERBS-CONE spec is read; U3 and U6 are dispatched

The spec is `docs/GERM-VERBS-CONE-SPEC.md` on `germ/verbs-cone-spec` (`3edfab44f`).

- **Headline:** with PR 3372 merged, admitting the cone today would be safe but useless. Every preview fixture now refuses typed, and there is no live defect on main.
- **One silent arm** (the `(Negative, Negative)` convexity arm across the apex) is masked only by R-reach. That fixes the order: the root lanes come before the certificate rows, and the roster flip comes last.
- **Rulings on its questions:**
  - **Q1 (a tilted plane × cone join would amend ratified C1):** not taken. Tilted plane × cone stays as C1 says. U8 (the axis-normal join) becomes its own item, after the flip. No `[ev]` question is owed unless a customer case asks for more.
  - **Q2:** the cone joins the ∖/∩ roster in the flip.
  - **Q3:** a line parallel to a generator keeps the door.
  - **Q4:** the apex closure stays local to the cone trim and `ChartCache`.
  - **Q5:** U6's move applies to sphere × torus too, in the same lane.
  - **Q6:** U5 comes after the flip, and is optional.
  - **Q7:** the item text is refreshed by the U3/U6 lane.
- **Dispatched:** U3 (the apex closure) and U6 (sphere × cone and sphere × torus from the scan to the pass), combined in one lane with one PR (Ev's CI note), on branch `germ/cone-apex-closure` off the spec branch. The review tier is single, since `point_in_solid` on partial cones goes from refusing to answering.
- **Waiting on PR 3375:** U1 (line roots) and U2 (circle roots). U4, then U7, follow.

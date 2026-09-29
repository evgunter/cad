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

## 2026-09-28 — PR 3375's dual review: both NOT-MERGEABLE-AS-IS

- **Pre-note:**
  - **Bilateral:** the first-Out mutant survives because the row's solver order puts the In root first (MAJOR/MAJOR, both executed); the parallel arm treats any tilt in band as exact and decides reach in-plane (R2 MAJOR executed, R1 MINOR argued: severity divergent); the `on_line` guards are unpinned.
  - **R1 alone:** the pole-near-a-root conditioning makes the quartic certificate UNSOUND (a silent premise-S miss executed in `sweep_traces`) — **a tally candidate**, pending the blinded coder's dedup against R2's lever over-refusal; the `crossed_elsewhere` pin; the lily re-pins weakened (R2 disagrees).
  - **R2 alone:** the small-circle lever over-refusal; the stale chord-arm comment; the factor of 2 in the height margin; no sweep row built around the quartic arm.
  - **Fairness:** R1 saw R2's directory NAMES during cleanup, no contents; flagged under rule 3, no findings glimpsed.
- **Blinded coding:** byte 19.
- **The fix pass** carries the union to the implementer, merging main (PR 3372) first.
- **Consequence:** VERBS-CONE's U2 (circle × cone) reuses this quartic, so it waits on the conditioning fix.

**Class (logged):** "every root is examined" rows must put an Out root FIRST in the solver's own order. A row whose first-examined root is In cannot kill a first-root-only mutant. This is the third first-root defect this week, counting the P0 (PR 3358) and this row.

## 2026-09-29 — PR 3375's delta review: every first-round finding is closed, and one new MAJOR

- **Closed, confirmed by execution:** all five first-round findings. That includes pole conditioning: 12/12 against the oracle, and about 18k fuzz cases with 0 wrong.
- **New MAJOR:** at large circle radius the f64 lane certifies rounding noise. The coefficients scale as ρ⁴ while the margin is levered by R+r.
  - 96 of 1500 wrong at ρ=100, and 557 at ρ=300 (torus R=1, r=0.25).
  - The threshold is ≈40 m for this torus and ≈4 m for a 5 cm fillet.
  - It is pre-existing in the lever choice, but the new lane turns it into a silent premise-S miss.
- **The fix must be a real f64 noise meter,** not a lever swap, since a lever of 2ρ refuses everything. It goes to the implementer in a second pass, with the parallel arm's tilt-displaced root positions and NITs. The delta reviewer re-checks the noise meter.
- **Class (logged):** a lever justified by "where the roots can be" is not a bound on rounding noise. Every certified polynomial ladder needs its coefficients' evaluation error metered against their magnitude, and not only at the scales the rows happen to use.
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

## 2026-09-29 — the geom-core census rows, and PR 3384's red

- **Ev asked about PR 3384's red.** Its two `geom-core` census rows were red on main:
  - `sym_rule_f_rows` copysign: GERM's, from PR 3372's `section_cert.rs` cylinder-pair saddle witness;
  - `certified_endpoint_census`: ENCL's PR 3392.
- **Already fixed:** PR 3407 fixed both, merged 01:37, three minutes before 3384's comment. PR 3384 needs only a merge of main.
- **Checked now:**
  - main (with PR 3406) passes both rows;
  - PR 3375's head passes;
  - PR 3395's head, merged with main locally, passes.
- **Class (logged):** since PR 3340 removed the read reach, a topo-only PR no longer builds `geom-core`'s tree-wide source-scan censuses, so a new copysign or certified-endpoint read in `topo` is green on its PR and red on main. Every GERM lane now runs `cargo test -p geom-core --test all -- census sym_rule_f_rows` before pushing (told to the three active lanes).
- **Queued:** `section-cert-copysign-mint-site-is-unregistered` (GERM's, three questions open from PR 3407):
  - should the saddle witness's side choice be a frame decision rather than `copysign`;
  - the opaque atom under the symbolic tier;
  - which document exercises the branch.
## 2026-09-29 — PR 3375 lands (DR-16): the circle × torus root lane

- **Dual review:** both NOT-MERGEABLE-AS-IS.
  - **Tallied:** R1's pole-conditioning MAJOR, making the tally 2 of 8, with 6 pairs toward twelve that found a MAJOR.
  - **The fix pass** ran three rounds with delta reviews between. The first delta review found a new MAJOR: large-ρ f64 noise. It is fixed by a noise meter in the generic `half_angle_roots`, which VERBS-CONE U2 inherits.
- **Filed:**
  - recentering, to recover large circles (P3);
  - `line-torus-roots-may-certify-noise-when-the-line-origin-is-far` (P1, M): the same coefficient growth may reach the line and ray torus quartics. Measure first.
- **Unblocked:** VERBS-CONE U1 (line × cone) and U2 (circle × cone).

## 2026-09-29 — cone U3+U6 land (PR 3395)

**U3, the apex closure:**
- `J = −ΣΔθ` at a single apex visit;
- `point_in_solid` and `curved_face_containment` now answer apex-closed sectors of any width;
- `describes` holds for apex-closed faces.

**U6:** sphere × {cone, torus} moved from the extent scan to the section certificate's no-crossings pass. Three sphere × torus unions newly answer.

**Single review:** APPROVE-WITH-FIXES, no MAJOR. The fix pass took all of it:
- the cone trim refuses a non-rectangular chart outline (an area test), which fixes a pre-existing wrong `In` in an L-shaped face's notch;
- precondition rows;
- the partition row pins which band holds each point;
- one home for the cycle extraction and the window fold.

**Filed on CONTACT:** the torus box check passes an L-shaped face.

**Merge of main:** composed `SectionPath::scope` with PR 3406. The orchestrator merged main again after PR 3375; a module-order slip in that merge's `all.rs` resolution turned lint red, fixed in `d777dd2c7`.

## 2026-09-29 — the saddle witness picks its side by a frame decision (PR 3421)

- **The change:** `copysign(1, δ)` is replaced by `section_cylinder_pair_side`. A positive or negative δ picks the ruling; zero or undecided refuses R-tan.
- **Census:** the site leaves the copysign census and the manifest, moved together.
- **Rows:** the saddle rows reach the branch at δ₀ = +1.3 only, so a new verdict row covers both signs on both sides, and the flip mutant turns it red.
- **Tier:** orchestrator read. Merged on hosted green.

## 2026-09-29 — the torus joins ∖ and ∩ (PR 3416, DR-19)

- **Measured before admitting:** no wrong body under either order of ∖ or under ∩.
  - Newly answered, in closed form: the cube in the hole, the pin-only bracket, the wedge.
  - Everything else refuses at ∪'s door; the half donut and the bracket refuse at the guard.
- **Dual review:** both APPROVE-WITH-FIXES, no MAJOR, 0 wrong over both batteries. R2 also ran the merge preview carrying PR 3395's sphere × torus move. The tally is unchanged, and the pair does not advance the count.
- **Fix pass:** the stale demo prose, the invariants, one helper home, and the seeded rod sweep committed as a row.

## 2026-09-29 — PR 3423's dual review: R1 returns NOT-MERGEABLE, and a restart interrupts R2

- **R1 (NOT-MERGEABLE-AS-IS):** the circle × cone lane certifies wrong counts and displaced roots beyond the threshold (executed, checked in mpmath).
  - DR-16's premise, that the ladder's band decisions cover the stages after the harmonics' noise, fails for the cone. The double cone is unbounded, the uncapped 2ρ lever puts the quartic's other roots far out in τ, and the discriminant and Ferrari rounding swamp a near-tangent pair.
  - The line lane held: 30k fuzz cases, 0 wrong.
  - Also: near-apex circles are answered in band, and the eps-1e-12 pin reads an in-band tangency as a miss.
- **A container restart** stopped R2 mid-review, and it was resumed from its transcript. By rule 6(e) the pair is **excluded from the tally and the pair count** (interrupted); it is recorded in full, and the fix pass still takes the union.
- **U4** is resumed too. It was clean at its pushed `18b91ac80`.

## 2026-09-29 — PR 3423: R2 delivered; fix pass dispatched; a possible later escape from DR-16

- **R2 (APPROVE-WITH-FIXES), after its restart:** no wrong counts in ~125k cases. But circle root positions exceed their slack (6/26k), and in-band tangencies are settled rather than doored.
- **Both reviewers trace the circle-lane defects to one premise:** DR-16 pass 3's claim that the ladder's own band decisions cover the stages after the metered harmonics. It fails for the cone, where it is unbounded, with an uncapped lever.
- **Blinded coding:** byte 16, with the pair already excluded by 6(e).
- **The fix pass:** an a-posteriori verification of the metered F in the shared `half_angle_roots`:
  - metered sign-change brackets;
  - root-free gaps by the Bernstein bound |F′| ≤ 2A;
  - a door for anything unseparated.
- **Owed:** measuring circle × torus on MAIN for the same failure. If it certifies wrong answers there, that is a later escape of DR-16 (PR 3375), to be appended under its row.

## 2026-09-29 — Ev: finish what is started, plus the P1, then pause

Ev's instruction: finish the in-flight work and the P1, then pause until the weekly limit resets.

**In scope before pausing:**
- PR 3423 (cone U1+U2): fix pass, delta review, then merge.
- U4 (`germ/cone-section-rows`): dual review, then merge after 3423.
- The P1 tilted-rod re-measure (`germ/tilted-rod-remeasure`, dispatched).

**Not started until Ev resumes:**
- U7 (the cone roster flip);
- the P2s (cone pairs in general pose; the radial hole through a tube);
- the P3/P4 queue.

## 2026-09-29 — U4 built and waiting on PR 3423; a unit-spec error ruled

- **U4** (`germ/cone-section-rows`, `332c80aa9`) is built:
  - the cone rows for plane, sphere, the coaxial partners and the parallel cylinder;
  - 21 mutants red;
  - 0 mismatches in 1920 random poses against the traced section.
- **Ruled:** §2.5.1's cone × plane table read `m_A` (the apex offset) first and answered W0 when `m_A` was in band and the aperture was negative, which can clear a real ellipse of length ≈ `m_A·cos α/μ`. The lane decides on the aperture margin μ alone. That is sound and more conservative, and a row demonstrates the spec's error. It is a unit-spec correction, not a design fork; the spec is fixed on the branch.
- **The PR opens** after PR 3423 merges.

## 2026-09-29 ~07:00 — PAUSED (Ev: stop dispatching, the usage limit is near)

Nothing new is dispatched. Lanes already running finish on their own and push their branches; no reviews or merges follow until Ev resumes.

**State at pause:**
- **PR 3423 (VERBS-CONE U1+U2, `germ/cone-roots`):** the dual review is done, with the pair excluded by 6(e) (R2 interrupted by a restart). The coding is in the scratchpad `dr3423/coding.md`: byte 16, A=R1; G1 is a bilateral MAJOR (R1 MAJOR / R2 MINOR), tally none.
  - The fix pass was RUNNING at pause: an a-posteriori verification of the metered F in the shared `half_angle_roots`; the near-apex door; the in-band tangency door; the off-span slack row; NITs. It also measures circle × torus on MAIN for the same failure.
  - **On resume:**
    - read its report, circle × torus measurement FIRST. If main is wrong, that is a later escape under DR-16 (PR 3375) and a live defect.
    - then a single delta review of the new certificate;
    - the DR row (EXCLUDED pair; the tally line is unchanged; take the next DR number from main);
    - merge on hosted green.
- **U4 (the section certificate's cone rows, `germ/cone-section-rows`, head `11036d5d9`):** built and idle, no PR yet.
  - After PR 3423 merges: tell its lane to merge main (the `VERBS-CONE.md` and `sweep/tests/all.rs` conflicts are known), add the op rows via `sweep_past_the_cone_roster`, and open its PR.
  - Then a dual review.
  - Ruled: the cone × plane rule decides on the aperture margin μ alone (spec §2.5.1 corrected on the branch).
- **P1 re-measure (`germ/tilted-rod-remeasure`):** RUNNING at pause.
  - It re-measures the tilted rod in the half donut against the section certificate, and opens a PR.
  - On resume: read its report, then an orchestrator read or single review, then merge.
- **Next after those:**
  - U7 (the cone roster flip; dual);
  - the P2s (`cone-pairs-in-general-pose-have-no-section-arm`, `radial-hole-through-a-tube-has-no-section-arm`);
  - the P3/P4 queue.
- **Review log on main:** tally 2 of 8; fair pairs that found a MAJOR, toward twelve: 7.

**Paused-state update (P1):** the re-measure finished as PR 3428 (`8c1257762`, CI passed). Not merged; it waits for Ev.
- **Not live:** no op returns a body.
- **Wherever the backstop fired,** the section certificate had already decided R-reach. `boolean_op_recut` raised it only AFTER `volume_backstop`, so the suspect body was still built.
- **The fix** raises `interior_loops?` before the backstop, and the rest door the same way.
- **Rows:** 5 poses × 5 ops refuse at the guard.
- **The backstop's own cause** (no ellipse arc in the closed-form mass lane) is REACH's existing P0 on a tilted-cylinder boss; a control box shows it without any torus.
- **A classifier denial:** the lane's throwaway patch letting the body past the backstop was denied. It did not work around the denial, so the internal body on current main is unmeasured; the 09-28 measurement is the latest.
- **On resume:** an orchestrator read or single review, then merge. Expect a trivial item-file conflict with this branch's 09-28 section.

# S-BLEND log — fillet/chamfer completion

Narrative record; the plan is `docs/S-BLEND-PLAN.md`. Convention as
in the other programs: seam entries at pipeline seams, unit entries
at merges, the tail is the live state.

## Opening state (2026-08-29)

Opened by graduation of the ratified work-stream survey (#1200,
merged after Evan's read with VERBS' cession recorded on its
thread), by a fresh orchestrator on a remote container.

**Operational facts, recorded once (the M10 opening's shape, same
day, same container class):**

- **Branch prefix `blend/`**, orchestrator branch
  `blend/orchestrator`, away-channel tag `(S-BLEND orchestrator)`.
  The harness-designated session branch carries only this opening
  PR.
- **A/B band BLEND = 600–699**, claimed in `docs/MODEL-AB-LOG.md`'s
  banding entry in this same commit. Blocks `BLEND-B1, …`; draws
  recorded branch-side on `blend/orchestrator` per the ratified LIB
  shape.
- **Remote container**: GitHub through MCP rather than `gh`; no
  script monitors (PR watching via MCP subscriptions + scheduled
  self check-ins; away-channel etiquette followed by hand). Disk
  ~29 G free is the binding constraint: lanes are worktrees with
  their own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent lane targets,
  review targets reclaimed at report time; sequential reviews with
  a pre-recorded symmetric method note (the G18a precedent) if disk
  cannot hold two. Build-slot mutex, per-lane target rule,
  CONFLICTING-means-silent-CI, push-early: unchanged. Clone
  unshallowed with a blob filter at opening.

**Unilateral decisions at opening (per the orchestration model,
recorded for Evan's retroactive read):**

1. **The survey's 918/708 listing is corrected to LIB-G16's claim.**
   RECIPE-DOORS (ratified same day) assigns the chamfer recipe door
   and the emit_fillet re-shape to LIB-G16, which dispatched before
   the survey merged; the plan records the seam instead of the
   listing. Kernel chamfer parity (919, 917) stays here.
2. **Serialized unit order 1022 → 935 → 919 → 644**, with 961/917
   gated on G16 and track T gated on 2b — every unit edits
   `crates/sweep/src/fillet/`, so parallel implementation lanes
   would merge-conflict by construction; conversations run in
   parallel instead.
3. **BLEND-2's presumptive shape is the narrow seam-key refresh**
   (the issue's own alternative that keeps decide-before-mutate);
   found insufficient ⇒ STOP, design fork to Evan.
4. **Issue 987 recorded as double-gated** (OQ6 taxonomy is Evan's;
   consumer-gated) rather than scheduled.

**Live gates being watched:** #1180 (SHELLFIX 2b — its merge lifts
the shell/offset keep-out and triggers the track T claim);
LIB-G16's PR when it opens (lifts the emitter seam for BLEND-5/6).

## BLEND-1 MERGED (2026-08-29)

PR 1222 merged at sample 46 (ordinal 600; full record in
MODEL-AB-LOG's row). The multi-link closed-rim door is live: a
seam-split rim's band is one annulus, routing by host side, the
lantern's rims fillet whole, and the SeamVertex recourse is TRUE at
every site the tag fires — conditioned on the side the door serves,
pinned composed on both material sides. The A3-2 promise is served.
Handoffs into the backlog: issue 1244 (concave closed-rim band —
the lily's fourth rim), 1245 (boolean-repaired pole-touching rim),
1246 (public rim-arc selector; consumer evidence from both e2e
reports). Next per the plan: BLEND-2 (issue 935), with BLEND-7
(profile crate, ruled 827) able to interleave.

## BLEND-B1 draw (2026-08-29, BRANCH-SIDE RECORD — merges to main only when slot 3's reviews conclude)

Protocol v3 triple, drawn after BLEND-1's pre-draw difficulty was
logged (**M-L**). One `/dev/urandom` byte: **193** (< 252, no
redraw); 193 mod 3 = **1** = fable's position (0-indexed). Block
**BLEND-B1 = (slot 1 OPUS, slot 2 FABLE, slot 3 OPUS)**. Slots are
consumed in dispatch order; arms are never restated on main while a
slot is unstarted (the 2026-08-29 LIB redaction precedent).

## BLEND-1 DISPATCHED (2026-08-29)

Unit BLEND-1 (the multi-link closed-rim door, issue 1022) dispatched:
lane `blend-1`, branch `blend/rimwhole`, brief carrying the A3-2
handoff, the SHELLFIX/G16/vocabulary/run-out fences, and the
blinding rules. Pre-draw difficulty **M-L** (logged in the draw
record above before the byte was drawn). Arm = BLEND-B1 slot 1,
read from this branch's draw record, not restated where reviewers
read. v6 dual at review; ordinal claimed from band 600–699 on main
at review dispatch. Row at merge.

## Enclosing-tangency conversation OPEN (2026-08-29)

`docs/ENCLOSING-TANGENCY-DESIGN.md` (PR 1210, branch
`blend/enclosing-tangency-design`): R1 (yes in principle,
structurally spelled, consumer-gated) recommended over R2
(permanent no), per the JunctionTangent precedent. Waits for
Evan's 👍 — never self-merged. Pointer comment left on issue 827.

**Live state**: BLEND-1 lane running (`blend/rimwhole`); watching
PR 1180 (2b merge ⇒ track T claim + shell/offset keep-out lifts)
and PR 1210 (Evan); LIB-G16's PR checked at check-ins (⇒ BLEND-5/6
unlock). Session check-ins armed ~hourly.

## SMELL TRACK T CLAIMED (2026-08-29, at SHELLFIX 2b's merge)

VERBS-SHELLFIX PR-2b merged to main at `74e7d36f` (its A/B row,
ordinal 105 / sample 42, rides the unit PR). Per the plan and the
ratified survey: **S-BLEND now holds SMELL track T whole**
(`crates/sweep/`, fence as the partition states it, block
D320–D339 / S390–S409), and the shell/offset keep-out in
`crates/sweep` + `topo/replace_face.rs` is lifted. Track-T lanes
serialize behind BLEND-1 — most rows touch the same `fillet/`
files (D90, D321, D124) — with `docs/SMELL-T-LOG.md` opening when
the first lane starts. D320 stays filed-not-takeable (follows
track N's D240); C-e/H13 gets its 779 contradiction verified
before staffing. BLEND-1's lane owes a merge of main (2b + M10-P
landed) before its PR opens; its brief already carries the rule.

## ENCLOSING TANGENCY RULED (2026-08-29) — Evan, in-chat

The 827 conversation resolved same-day: **the enclosing class is
never allowed — "it is not a fillet of that corner and should be a
refusal."** Evan's probe of the conversation's figures (the arc
visibly not touching the corner) surfaced the deciding ground —
strict interiority: internal tangency at r > R puts the corner
strictly inside the blend circle, so no door emitting the class
could be serving a fillet of the corner. Doc ratified in place and
merged (PR 1210, merge 26c9e19c); the original R1 recommendation
is recorded there as rejected on exactly that ground. **BLEND-7**
(the closing unit: measured-first refusal in `crates/profile`,
pins' hedge-drop, sugar.rs purpose statement) joined the plan in
the same merge; issue 827 closes at its merge, and the decision
folds into DESIGN.md's companion table then. Sequenced after
BLEND-1 (review-bandwidth, not file conflict — it is the one unit
off the sweep fence).

## BLEND-1 REVIEWS DISPATCHED (2026-08-29 ~20:17Z)

Implementation delivered: PR 1222, frozen head `85047cbe`, impl CI
33271411468 green ({interval, default eps} drawn; off-draw rows run
in-lane). **Ordinal 600 claimed on main** (PR 1236, merge f7016118;
the conflict resolved there carried S-CERT's 600→700 renumber — the
banding rule worked its first collision). v6 dual: byte 77, parity
1 ⇒ R1 FABLE + R2 OPUS, SEQUENTIAL on the frozen head per the
method note recorded in the claim entry BEFORE R1 ran; both briefs
authored and stored pre-R1 (orchestrator dir). R1 running in lane
`blend-1-r1`; R2 dispatches from the stored brief when R1's report
is in and its target is reclaimed. Base-tree findings homed at
dispatch: issues 1234 (reader_census dot-path), 1235 (m10_p_lift
clippy). The impl lane is kept intact for the fix pass (the M10-1
worktree-reclaim lesson); only review targets are reclaimed at
report time.

## R1 DELIVERED; R2 DISPATCHED (2026-08-29 ~20:45Z)

R1 (blind, sequential slot 1): **APPROVE-WITH-FIXES 1/2/2, rubric
4/4/4**, probes on `blend/rimwhole-r1-probes`, ~300K/~75m. Headline
MAJOR by execution: the rewritten `SeamVertex` recourse
OVER-PROMISES at concave seam-split rims (the tag fires before the
convexity door; measured on the PR's own waisted revolve AND the
lily mouth rim) — the A3-2 failure shape recurring one door later.
MINORs: carry-through-crossing vertex-birth rows mutation-invisible
in the totality row; the boolean-repaired pole-touching body
(merged caps) falls to the ladder and refuses — a third shape
neither door serves, overspoken in KERNEL-VERBS. Also: the concave
closed-rim band frontier has NO issue number (deviation disclosed
but unscheduled), and the A3-2 correction paragraph is now stale
(code drifted from a meant-to-hold sentence — kept as the standard,
not deleted). Both bit-level differentials and an independent
Pappus oracle CONFIRM the carve. R1's target reclaimed at report;
the impl lane's 13G target also reclaimed (clone kept — the
worktree, not the target, is the resume-critical half). R2
dispatched from the STORED brief, read-isolated, same frozen head.
Adjudication at R2's delivery.

## BOTH REVIEWS IN; UNION ADJUDICATED; FIX PASS DISPATCHED (2026-08-29 ~21:20Z)

R2 (blind, sequential slot 2): **APPROVE-WITH-FIXES 1/6/3, rubric
3/4/3**, probes on `blend/rimwhole-r2-probes` (~250K/~55m); the
carve confirmed by ITS OWN independent Pappus quadrature and a
bit-level merge-base fingerprint (volume bits identical), and the
wall-6 pin confirmed CI-executed. **Correspondence pre-note (full
coding at merge): the headline MAJOR is BILATERAL** — both arms
found the SeamVertex recourse over-promise at concave seam-split
rims, by execution, at MAJOR; NO tally candidate; R2 additionally
codes it a silent deviation. R1-unique: the vertex-birth mutation
hole; the repaired-body third shape. R2-unique: the three-spellings
incidence class (the weaker battery predicate IS the MAJOR's
mechanism); concave-row vacuity; the ring-arm no-op recorded as
"fixed"; wall-6 re-measure unpinned. Shared beyond the MAJOR: the
unscheduled concave-frontier deviation; the two-resolvers
parallelism; the rim-arc selector duplication (both e2e reports hit
the same missing public selector — consumer evidence for an API
issue). Fix pass IMPLEMENTER-INHERITED, dispatched by resuming the
kept lane (11-item union, two frontier issues + one API issue to
file, both probe branches adopted as merge parents, the KERNEL-VERBS
both-sides merge called out). R2's target reclaimed at report; delta
re-verification after the fix per the ladder position.

## BLEND-2 + BLEND-7 DISPATCHED (2026-08-29 ~22:50Z)

Post-merge sweep done (all three BLEND-1 lanes removed; branches
and probe branches live on origin). Two units dispatch together —
the plan's serialization binds `crates/sweep/fillet` units only,
and BLEND-7 is the profile-crate exception:

- **BLEND-2** (issue 935, two rims sharing a wall): difficulty
  **M** (logged at dispatch, before reading the slot). Lane
  `blend-2`, branch `blend/tworims`. Arm = BLEND-B1 slot 2, read
  from this branch's draw record. Presumptive shape per the plan:
  the narrow seam-key refresh between carves; STOP on any move
  against decide-before-mutate.
- **BLEND-7** (issue 827, the ruled enclosing-tangency refusal):
  difficulty **S** (logged at dispatch). Lane `blend-7`, branch
  `blend/enclose-refusal`. Arm = BLEND-B1 slot 3, read from this
  branch's draw record — the block's last slot; the block record
  merges to main when this unit's reviews conclude. Measured-first
  per the ratified doc.

Both v6 duals at review; ordinals 601/602 claimed on main at their
review dispatches. Rows at merge.

## BLEND-7 DELIVERED; ITS REVIEWS DISPATCHED (2026-08-29 ~23:45Z)

Implementation delivered: PR 1267, frozen head `c742fc5c`, impl CI
33281016034 green ({interval, 1e-6}). The step-1 measurement
earned the measured-first rule: TODAY the enclosing band wears
THREE different refusals depending on how far past the bound r
sits (`OffsetCarriersDisjoint` / `NoCornerSideCandidate` /
`AnchorOutsideTrimmedExtent`, the last offering a recourse that
cannot work), and unbracketed anchors BUILD the other crossing's
ordinary fillet. Gate landed at `sugar::arc_fillet_trims` (first
point ρ is defined), riding `build_refused` so the other-crossing
ordinary answer survives; `PathError::FilletEnclosesLegCarrier`
typed with the bound + recourse; both review_s2 pins permanent
citing the ruling; DESIGN.md companion row folded. One
compiler-forced out-of-fence line (pncad-py tag arm), reported.
**Ordinal 601 claimed on main** (PR 1269, merge 6158c9d5): byte
213 parity 1 ⇒ R1 FABLE + R2 OPUS, sequential same-head, briefs
stored pre-R1. R1 running (lane blend-7-r1); impl target
reclaimed. BLEND-2 still in flight in its own lane.

## BLEND-7 R1 IN; BLEND-7 R2 + BLEND-2 R1 DISPATCHED (2026-08-30 ~00:05Z)

BLEND-7 R1: **APPROVE-WITH-FIXES 0/3/2, rubric 5/4/4** (~205K/~50m;
probes on `blend/enclose-refusal-r1-probes`). NO MAJOR — every
falsification attack failed (merge-base grid differential 64/64
builds bit-identical with 91 misleading-recourse cells flipping to
the typed refusal; extremes to 200× scale and 1e4 offsets; both
mutations surgical). MINORs: the recourse over-promises INSIDE the
existence gap (a two-step bound chain on unequal carriers — the
milder cousin of the shape BLEND-1 repaired; the ratified doc's own
wording specified the carrier bound, so the fix is wording, not
widening); `NoCornerSideCandidate` now witness-less and plausibly
dead, unscheduled; the new in-band predicate arm untested. BLEND-2
delivered meanwhile (PR 1268, head `e81b3409` — the narrow
seam-key refresh SUFFICED, no design fork; bit-equal composition
pinned both orders on one-edge AND seam-split pairs; an ordering
hole in the old gate found and closed). **Ordinal 602 claimed** (PR
1276, merge 9be4c42e): byte 176 parity 0 ⇒ R1 OPUS + R2 FABLE.
Now interleaved per the recorded method notes: BLEND-7 R2 and
BLEND-2 R1 run concurrently (two review targets, the cap), R1
targets reclaimed at report.

## BLEND-7 R2 IN; FIX PASS DISPATCHED (2026-08-30 ~00:30Z)

R2: **APPROVE-WITH-FIXES 2/4/2, rubric 4/3/3** (~200K/~55m; probes
on `blend/enclose-refusal-r2-probes`), and the strongest possible
soundness result for the gate itself: a mutation differential over
18,072 corners — builds and emissions BYTE-IDENTICAL with the gate
gutted, so the gate changes only which refusal is served, never
whether a fillet builds; zero ruling violations on emitted geometry
off the PR's grid. **Correspondence pre-note: both MAJORs are
BILATERAL in substance at differing severity** (R1's MINOR-1
carried both mechanisms: the first-hit-leg bound and the
existence-gap over-promise) — NO tally candidates; the severity
split is calibration signal. The defect family is the dead-recourse
sentence again, one program-unit after BLEND-1 repaired its sibling
— the pin asserted class-absence, not buildability, in both cases.
R2-unique: the 12% refusal-attribution measurement (pre-existing
channel, issue to file), the fuzz-prose insensitivity, the
Debug-float class. R1-unique: the untested in-band arm. NCSC
witness-lessness bilateral (R2: it was the ONLY witness). Fix pass
IMPLEMENTER-INHERITED, dispatched (9-item union, two issues to
file); R2's target reclaimed. BLEND-2 R1 still running.

## LIB-G16 MERGED — THE EMITTER SEAM LIFTS (recorded 2026-08-30 ~00:40Z; merge was 2026-08-29 22:45Z)

PR 1224 merged: `Node::Chamfer` + `emit_chamfer` delegating to
`emit_fillet::name_blend`, the 708 tie deferral paid to zero sites,
schema v16, issue 918 closed. Consequences here: **BLEND-5 (issue
961) and BLEND-6 (issue 917) are unlocked.** BLEND-5 queues behind
BLEND-2's merge (same naming files). BLEND-6's design conversation
opens now — note G16's landed shape reshapes it: the recipe layer
already discriminates verbs via `NodeErrorKind::Blend { verb:
BlendKind, .. }`, and `BlendKind` Display-s as fillet/chamfer, so
the remaining question is the KERNEL-direct surface (FilletError's
Display literals still say "fillet:"). Issue 1235 closed — main's
own c577b24d fixed it independently (G16 dropped its parallel fix
and took main's).

## BLEND-2 R1 IN; R2 FIRST DISPATCH DISCARDED FOR BRIEF ASYMMETRY; CLEAN R2 RUNNING (2026-08-30 ~00:55Z)

BLEND-2 R1: **APPROVE-WITH-FIXES 0/7/4, rubric 3/4/4** (~200K/~95m;
probes on `blend/tworims-r1-probes`). No MAJOR — the identity-only
claim survived adversarial reading and four mutations, bit-equality
reproduced incl. a four-rim chain e2e and the whole sweep battery
on the UN-DRAWN interval lane. Sharpest MINORs: the dead-edge
exactness claim has no fixture separating it from the naive
spelling (M3 survives all 791 rows); a spool fixture contradicts
the conservative-narrowing characterization (one-call and
sequential refuse at the IDENTICAL radius there — the zone's gap is
likely the clearance meter's pre-existing direction-conservatism on
a sphere wall, not the door's); the collision refusal never names
the sequential recourse (the 1278 class again); the mixed-arm fence
owes its ~15-line reachability guard (#651); two "three readings"
counters now stale at four; two undisclosed refusal arms in the
refresh.

**PROTOCOL INCIDENT, recorded for the adjudication: R2's FIRST
dispatch deviated from the stored brief** — the orchestrator added
one attack under claim 5 (test the narrowing characterization on a
new fixture) shaped after R1's spool finding. Caught immediately;
the agent was KILLED during setup (it had fetched the PR body;
no lane work, no probes) and a fresh R2 dispatched from the stored
brief VERBATIM. Handling per the G18a discard precedent: the
contaminated dispatch produced no report and no artifacts; the
clean R2 is uninterrupted from the symmetric brief. Whether the
pair still scores toward comparability is the blinded
adjudication's call, not claimed here.

## FABLE USAGE LIMIT: BOTH FABLE LANES KILLED; MAIN'S A/B LOG REPAIRED (2026-08-30 ~01:4xZ)

The Fable usage limit (429) killed both in-flight Fable lanes
mid-turn: **BLEND-7's R1 delta verifier** (items 1 and 3 already
CONFIRMED by execution — the recourse fix and the in-band row; died
starting item 2) and **BLEND-2's R2** (still in its read phase, no
probes). Handling, recorded before any resumption: the delta is
verification by the same reviewer, not a scored arm — it RESUMES
from item 2 when the limit lifts; BLEND-2's R2 partial is
DISCARDED (second discard for this slot — the first was the
orchestrator's brief-asymmetry kill, both recorded) and a fresh
complete R2 dispatches from the stored brief at reset, per the
G18a limit-death precedent. The byte-176 draw (R2 = FABLE) stands;
no model swap. BLEND-7's merge waits on the delta; BLEND-2's
adjudication waits on R2.

Meanwhile: **main's MODEL-AB-LOG carried a committed conflict
block** (flagged by the BLEND-7 fix lane) which a later merge
partially resolved into a DUPLICATED M10-P row + an orphaned
marker line. Repaired against live main in PR 1286 (merge
a85b3ad0): one M10-P row kept (the CORRECTED sample-43 copy), the
orphan line dropped, nothing else touched. Third instance of the
union-merge artifact class in two days → filed #1287 (tree-wide
CI marker guard, track J / S-QA).

## BLEND-7 MERGED (2026-08-30)

PR 1267 merged (ordinal 601; full record in MODEL-AB-LOG's row —
arm cell redacted to the branch-side record until block close,
since naming slot 3 determines the open sibling's arm by
arithmetic). The ruled enclosing class now refuses typed with an
ENDORSABLE recourse: the payload carries the corner's largest
tangent radius, every enclosing pin builds at it, and both
review_s2 pins are permanent properties citing the ruling. Issue
827 CLOSED — the 2026-08-29 conversation is fully executed.
Handoffs: 1280 (NCSC plausibly dead), 1281 (refusal attribution),
1282 (Display float class). Block BLEND-B1's record merges to main
when BLEND-2's dual concludes.

## BLOCK BLEND-B1 CLOSED (2026-08-30)

All three slots' duals have concluded (BLEND-1 merged sample 46;
BLEND-7 merged, ordinal 601; BLEND-2 both reviews delivered — R1
A-W-F 0/7/4, R2 APPROVE 0/3/2, no MAJOR either arm, fix pass
in flight). The branch-side draw record above becomes public with
this merge to main, and BLEND-7's redacted arm cell is restated in
its MODEL-AB-LOG row in the same commit. BLEND-2's row lands at its
merge, arm named normally (its dual is concluded).

## BLEND-VOCAB RATIFIED (2026-08-30, Evan's 👍 on PR 1279)

`docs/BLEND-VOCAB-DESIGN.md` ratified and merged (a9806624): V1
(the G16 wrapper shape at the kernel doors), V2 (verb-neutral inner
prose + per-verb recourse re-measurement under the 1278 rule), V3
(the rename with the unit, last), V4 (no parallel enum), plus the
three choices as recommended — "edge blend", collapse-with-aliases,
rename-with-the-unit. The impact/reversibility record is on the PR
thread (no persisted footprint; the sticky RimSupport half stays
BLEND-5's). BLEND-6 is now gated only on BLEND-5's merge.

## 2026-08-30 — Block BLEND-B2 DRAWN (branch-side record; reaches main at the block's last dual's conclusion)

**Block BLEND-B2.** v3 triple {opus, opus, fable}. Slots by the
plan's serialized order: slot 1 = BLEND-5 (issue 961, RimSupport
vocabulary), slot 2 = BLEND-6 (issue 917, shared refusal
vocabulary — executes ratified `docs/BLEND-VOCAB-DESIGN.md`),
slot 3 = BLEND-3 (issue 919, concave plane-plane chamfers).
Draw: /dev/urandom byte **25**, no rejections ⇒ 25 mod 3 = 1 ⇒
fable position 1 (0-indexed) ⇒ **slot 1 OPUS, slot 2 FABLE,
slot 3 OPUS**.

Difficulties, logged at this draw before any of the three
dispatches: BLEND-5 **M-L** (persisted-vocabulary widening with an
N-doc migration story and a schema-seam claim; assessed M-L in the
check-in note authored before this draw), BLEND-6 **M** (the
ratified V1-V4 execution — one wrapper at the kernel doors and the
~255-reference rename, wide but design-settled), BLEND-3 **M**
(two admission doors widened + the concave fixture through the
public API + orientation-agnostic carve check).

Arms stay branch-side until reviews conclude per the standing
redaction shape. BLEND-5 dispatches next (lane blend-5, branch
`blend/rimsupport`); its schema-seam claim rides the dispatch
record per the plan's naming/schema seam note.

## 2026-08-30 — BLEND-5 DISPATCHED (block BLEND-B2 slot 1)

Lane `blend-5` (`~/.local/share/cad-work/blend-5/cad`), branch
`blend/rimsupport` cut from main at `6cf9647e` (BLEND-2's merge —
the serialized-slate rule satisfied). Difficulty M-L stands
pre-logged in the draw entry above; arm per the block record.

**Schema-seam claim, stated here at dispatch per the standing
discipline:** main's `SCHEMA_VERSION` is **17** (M10-2's bump);
BLEND-5 claims **v18** for the `RimSupport` vocabulary widening,
with the resolution rule stated in advance per the M10-2/G16
precedent: if a concurrent unit also claims v18, MAIN'S MERGE
ORDER rules and the loser repays every fixture per its own
migration story. No live rival known at this writing (S-QA is
CI-territory; VERBS-SPHSPH is geometry; no M10 unit in flight).

Brief shape: issue 961 verbatim + plan unit 5; the emitter seam is
OPEN (LIB-G16 merged 2026-08-30, so `editor-core`'s `emit_fillet`
is touchable); the role-vs-kind-at-emit choice is the unit's to
measure and argue (not one of the plan's reserved design forks);
`docs/prompts/implementer-discipline.md` by path; measured-first.

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

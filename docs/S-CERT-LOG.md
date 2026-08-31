# S-CERT log — certified-enclosure soundness

Narrative record; the plan is `docs/S-CERT-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-CERT. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-29)

Opened on Evan's direction (in-chat: "can you orchestrate its
program", quoting the charter line naming #723/#893, interval-mode
widening, unmetered enclosures, the offset_fit family, and SMELL
tracks M/N), by a fresh orchestrator on a remote container. The plan
is a DRAFT design conversation for its **Rulings sought** section;
CERT-1 is dispatchable pre-ratification as a charter-named defect
fix (recorded below as a unilateral decision).

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `cert/`** — unit branches
  `cert/<unit>-<slug>`, orchestrator branch `cert/orchestrator`
  (the prefix is the merged cut's own designation; the
  harness-designated session branch `claude/s-cert-orchestration-2eafta`
  carries the opening PR and is otherwise unused).
- **A/B ordinal band: S-CERT = 700–799**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry, per that entry's rule.
  The opening commit claimed 600–699; S-BLEND opened concurrently,
  drew the same band, and its claim reached main first, so S-CERT
  renumbered to 700–799 under the main-is-authority tiebreak
  before any ordinal was assigned (this is the corrected log the
  banding entry says a collision costs). Implementer blocks are
  named `CERT-B1, CERT-B2, …` (`CERT-<n>` are unit names).
- **This session runs in a remote container** (the M10/GUI
  precedent): no persistent `~/.local/share/cad-work`, no script
  monitors (PR watching via MCP subscriptions + scheduled self
  check-ins; away-channel etiquette by hand under the `(S-CERT
  orchestrator)` tag), GitHub through MCP rather than `gh`. Disk
  ~29 G free is the binding constraint: lanes are worktrees sharing
  one object store, own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent
  lane targets, review targets reclaimed at report time. The
  build-slot mutex, per-lane target rule,
  CONFLICTING-means-silent-CI, and push-early rules bind unchanged.
  The clone arrived SHALLOW; unshallowed with a blob filter at
  opening (a successor here should check
  `git rev-parse --is-shallow-repository` before trusting ancestry
  or merges).

**Sweep at opening** (beyond the charter itself, what the slate is
grounded in): #762's headline guard already landed on main at
`91164e3b` (`ssi.rs:991` refuses non-finite; the issue's residue —
`march.rs`'s sibling D285, D286's coverage loss, the NaN-fold and
`exhaust.rs:285` rewording — is CERT-2); PCURVE P-2 (#1177) carries
the #1157 `orthonormal_basis` fix written and measured, so the
keep-out concretizes to `vec.rs`; #723's mechanism confirmed live in
the tree on both sphere arms (the rimless instance measured in the
issue's fourth comment); VERBS-SPHSPH staged behind CERT-1
(VERBS-PLAN item 9); `props/quad.rs` consolidation (C3/C-m, D30)
stays Track R's, gated behind #723; #883 stays parked (reserved lane
H-f); the #723 reproduction artifacts died with their machine and
the fixtures are re-derived from the issue text.

**Unilateral decisions at opening** (per the orchestration memory's
log rule):

1. CERT-1 dispatches pre-ratification. Ground: both issues are
   named in the charter Evan handed over in chat; the fix shapes
   are the issues' own recommendations (#723 option (2); #893's
   three asks); VERBS is staged behind it. The one design-flavored
   part — the rim lever's shape near the poles, S82's reserved
   verdict line — is stated in the spec with a recommendation and
   flagged for Evan at plan ratification; if the ruling goes the
   other way the lever change is local and the failing rows keep.
2. The opening PR rides the harness session branch rather than
   `cert/orchestrator`, to respect the harness branch designation
   for this session's own pushes; unit lanes use `cert/` per the
   cut. If Evan prefers the orchestrator branch spelling, it is a
   rename at the next seam.

## Seam: first rulings in from the opening conversation (2026-08-29)

Evan, in-chat: **Q3 RULED** — not a design question, orchestrator's
call; CERT-2 and CERT-4's fence seams proceed as planned. Q1/Q2/Q4
got elaboration requests (answered in-chat; outcomes fold into the
plan when ruled). Alongside Q2 Evan stated the general bar — a bit
change ≪ ε is always acceptable when it buys cleaner code — now
recorded in `memories/output-stability-as-justification.md`.
Consequence for CERT-4: the interval-lane-only reformulation stays
the default because #1191's exact-fit rows ride a *structural*
bit-zero (`extent − setback`), which is not ≪-ε drift; if the unit
finds the both-lanes rewrite cleaner it must bring the re-derived
gate design back for a look, not just re-baseline. The PCURVE
orchestrator's PR answers (route 2 unclaimed; vec.rs keep-out
time-boxed to PR 1177; correlated-Interval sites to the 1143 audit)
are folded into the plan at 715a7bb8.

CERT-2 dispatches on Q3's ruling (spec on `cert/orchestrator`;
block CERT-B1 slot 1). CERT-1 lane still running.

## Seam: Q2 and Q4 ruled (2026-08-29)

Evan, in-chat. **Q2 RULED**: the #1006 trio proceeds (shared home,
whole-face-arm collapse — tighter or equal by per-cell-then-union —
magnitude-reading retirement with the re-baseline owned); landed as
CERT-10 in the slate, after CERT-5/CERT-7 which edit two of its
sites. The bit principle sharpened: ≪ ε was *sufficient, not
necessary* — a flipped classification is fine when semantically
correct and the code cleaner (memory updated). Consequence: CERT-4's
f64-bit constraint restates SEMANTICALLY — both-lanes reformulation
permitted if cleaner, provided the exact-fit guarantee survives by a
preserved structural zero or a re-derived gate. **Q4 RULED**:
route 1, knot-aligned composite cells primary for CERT-5 (the
w-uniform-in-v exact arm kept as the strictly-better path where it
applies; route 2 unclaimed, per the PCURVE answer). Open ruling
surface is now **Q1 only** (the #870 gauge/scope choice —
recommendation on record: A′ patch-lanes-only, mean-edge-displacement
gauge, typed refusal).

Lanes: CERT-1 and CERT-2 implementers both still running.

## Seam: Q1 ruled; plan RATIFIED (2026-08-29)

Evan, in-chat: **Q1 RULED** — no always-on area metering (the
ε-validity intent: any realized geometry everywhere within ε of
correct is valid); the check is a hefty `debug_assert` on the A2
gauge. In the same exchange Evan clarified the debug_assert doctrine
— the instrument is right for expensive checks whose failure
PROBABLY indicates a bug, not only for guaranteed ones, and they are
on in release today (`debug-assertions = true`), eventually
debug/CI-only — ratified into DESIGN.md's D2 addendum as the
row-5-boundary note in this branch. CERT-6 re-cut to the ruling
(tripwire + calibration; the opt-in tightness door filed as a
demand-triggered valve, not built). All four rulings are now in;
the plan is marked RATIFIED, with the opening PR held for Evan's
sign-off of the D9 addendum wording it carries.

## CERT-2 merged (2026-08-29) — issue 762 closed; the program's first unit

PR 1221 at f24c5dea, gate green (interval, 1e-12). The unit: issue
762's residue — march.rs's sibling guard (D285's exact signature
red-first), D286 answered with the weight-underflow fixture (better
than the anticipated none-exists verdict; the overflow route proven
closed by the hull-cancellation floor), the poison-arm sentences made
producible, the NaN fold pinned, and three D285-spelling predicate
siblings the impl sweep's fold-shaped pattern missed — found by R2,
fixed in the pass. D285/D286 left the Track Q table in the landing
PR. Issues filed: 1218, 1219 (impl sweep), 1238 (the
finite-but-unusable-speed class, from both reviews' probes). Dual at
ordinal 700, sample #44 (after correcting the #42/#43 collision on
main's ledger): R1 fable A-W-F 1/4/4, R2 opus A-W-F 2/4/4, both
headlines bilateral-at-differing-severity; details in the row.

**Two incidents, recorded:**

1. **Orchestrator error — the fix pass ran cross-slot.** The
   fix-pass dispatch was SendMessage'd to the CERT-1 lane's agent id
   instead of CERT-2's implementer; that lane executed the whole
   union (well), so the fix pass did not inherit CERT-2's arm and
   its covariates are contaminated (excluded from arm comparisons in
   the row). Rule for the successor: verify the agent id against the
   dispatch record before any fix-pass send — the ids are one
   typo apart and nothing else checks them.
2. **Main went red mid-fix-pass** (pncad-py create_exception! merge
   damage from PR 1215 — a fence this program never touches; the PR
   gate builds the merge ref, which is how it bit PR 1221 first).
   Repaired orchestrator-direct at PR 1239 within ~40 minutes, LIB
   flagged on the PR; standing-down comment posted on PR 1221 per
   the babysit rules.

CERT-1's implementation is delivered (PR 1220, green at
default/1e-12); its dual dispatches next at ordinal 701.

## CERT-1 merged (2026-08-29) — issues 723 and 893 closed; the charter's headline lands

PR 1220 at f5ff92e4a, gate green (interval, 1e-12). Both sphere polar
acceptance defects fixed at the arms with red-first closed-form rows;
the half-cap twins committed as import fixtures; the dual at ordinal
701, sample #48 (R1 fable APPROVE 0/1/3 rubric 5/5/4; R2 opus A-W-F
2/3/1 rubric 4/3/3). R2's two MAJORs — the escalating pole predicate
and the |dt| > 2π membership hole — were UNILATERAL AND EXECUTED:
two v6 tally candidates, coding deferred to the blinded
adjudication. The fix pass folded indeterminate pole decisions
(refusal retired, served instead), saturated the membership at a
half-turn, adopted both reviewer suites (R2's interval outcome-match
row promoted to a gate), and filed issues 1250 (rimless polar cap,
found independently by both reviewers) and 1251 (K roster
consequence). VERBS-SPHSPH's stated prerequisite — sphere polar rims
carrying two accepted-direction defects — is now discharged.

**Incidents:** the gate's interval draw on the first head caught the
atan2/floor margin widening at its branch cut — a REAL catch that
forced the branch-free rework (the sampled matrix earning its keep);
main's tess baseline was stale against SHELLFIX 2b's un-squared
teapot and fired on this unit's draw (orchestrator-direct re-cut,
PR 1257); main carried a v15/v16 viewer include break (lane-direct
repair, PR 1263); and the orchestrator's review briefs propagated an
implementer report claim ("declared deviations") the PR body did not
carry — R2 falsified it, and the silent-devs column for this pair is
marked non-comparable in the row. Rule kept: a brief is a claim
site; verify report claims against the artifact before briefing.

Both charter-named units are now merged. Next: the plan's slate in
order (CERT-3 affine anchor, CERT-4 period folds, CERT-5 rational
flux), sequenced against disk and the other programs' traffic.

## CERT-3 delivered; session git-auth outage (2026-08-30)

**CERT-3's implementation is delivered and green**: PR 1277 at
44abc6d3d, gate run 33284902164 GREEN at BOTH lanes (the head
carries `CI-Config: lane=both`, deliberately — a linalg change is
not basename-matched, and the interval claims needed the lane).
The constructor now anchors through `(I − R)` with HALF-ANGLE
factors — the spec's literal `1 − cos θ` factor was itself measured
as part of the defect (an ulp-of-1 enclosure floor at θ = 0) — and
the zero-angle identity is exact at f64 with the Interval residue
≤ 1e-320 and independent of anchor width. RevolvedPoint start
samples: 4.0e-9 → 2.66e-15, the full-period seam by six orders.
f64 movement measured and reported: ≤1 ulp, 4 of 3135 corpus
coordinates; the M10-P bit-identity fence fired as designed and its
digests were re-derived from a 0-structural-difference dump (M10
flagged on the PR — their fence ground). Four sweep findings routed
to issue 1143's audit; four deviations declared in a PR-body
deviations section (the CERT-1 lesson applied). Difficulty S/M,
block CERT-B1 slot 2.

**BLOCKER: the session's git auth did not survive a container
restart** (known harness bug, Evan confirms no in-session fix): no
lane can fetch or push, so CERT-3's blinded dual (next slot byte
drawn at dispatch; ordinal 702 UNCLAIMED — claims happen at actual
dispatch) cannot run from this session. GitHub API tools still
work; this entry is an API commit. Everything is pushed: the unit
branch head 44abc6d3d, cert/ab-state (block + dual records),
cert/orchestrator (specs through CERT-3). The committed conflict
markers CERT-3's lane found in main's ledger were repaired upstream
before this session could act (verified: the VERBS-6 deviation
entry, the #43 sample correction, and both S-CERT rows survived).

**For the successor** (with `memories/orchestrator-switch-runbook.md`'s
contract; the tmux mechanics do not apply to remote containers):
1. Dispatch CERT-3's dual: frozen head 44abc6d3d, draw the slot
   byte fresh, method note + stored brief PRE-R1 on cert/ab-state
   (the CERT-1/CERT-2 records there are the template), claim
   ordinal 702 on main at dispatch, claims-to-falsify from the
   PR 1277 BODY (the brief-as-claim-site rule — never from the
   implementer report), including: the half-angle respell's
   measured story, the M10-P digest re-derivation being argued not
   silent, the ~1 ulp fixed-point regression honestly reported,
   the sweep's stated blind spot (caller/callee-split round trips).
2. Then the fix pass to the CERT-3 lane agent IF this session's
   agents survive to the successor (they do not — agents die with
   the session; the fix pass then runs as a fresh lane and the row
   records orchestrator-applied-or-fresh execution honestly).
3. Slate after CERT-3: CERT-4 (issue 1191, under the SEMANTIC
   restatement of the f64-bit constraint — see the Q2 seam), then
   CERT-5/CERT-6/CERT-7 per the plan; CERT-B1 slot 3 is the block's
   last undrawn slot.
4. Standing flags: issue 1250/1251 open from CERT-1; the v6 tally
   candidates from CERT-1's pair await the blinded adjudication;
   CERT-2's fix-pass covariates are contaminated (cross-slot) and
   CERT-1's silent-devs column is non-comparable (brief error) —
   both recorded in the rows.

## CERT-3 merged (2026-08-30) — issue 924 closed; the handoff unit

PR 1277 at a1bccce4, gate runs 33284902164 (reviewed head 44abc6d3d)
and 33337220614 (fix head) both GREEN with `CI-Config: lane=both`.
The constructor anchors through `(I − R)` with half-angle factors;
zero-angle identity exact at f64, Interval residue subnormal and
proportional-with-subnormal-slope (the fix pass corrected the
"independent of anchor width" wording); RevolvedPoint start samples
4.0e-9 → 2.66e-15, the full-period seam by six orders.

**Handoff executed mid-unit**: the predecessor session lost git auth
after delivery (previous entry); this successor session ran the dual
and the close-out. Per Evan (in-chat, 2026-08-30, at the handoff):
**this unit's dual ran WITHOUT A/B experiment logging** — ordinal 702
was never claimed, no arm records, no MODEL-AB-LOG row; the review
itself was full-protocol (blinded, sequential, identical stored
briefs, frozen head). The band's ordinal record therefore skips from
701 deliberately; CERT-B1 slot 2's implementer-arm record stands on
cert/ab-state but carries no dual row.

**The dual**: R1 A-W-F 0/3/4 (rubric 4/4/4), R2 A-W-F 2... see PR —
R2 A-W-F 1/5/3 (rubric 4/3/3). Both lanes independently rebuilt the
m10-p fence coordinate-dump differential and reproduced the f64
re-derivation exactly (0 structural, 4 of 3135, one ulp) — the
predecessor's self-certification worry is discharged twice over.
R2's unilateral executed MAJOR: the residue attribution was false
(retiring `rotation_about`'s `1 − cos` recovers ~17% at the start
sample, 0% at full period; the residue is the diagonal's ADDITIVE
enclosure times the coordinate) — corrected at every site it
propagated, including issue 1143's member-5 payoff (comment posted).
R1's headline: the fence header recorded only the f64 lane; the
interval-lane differential (8 of 3135, up to 16 ulps, honestly
slightly WIDER on this exact-axis corpus) is now recorded beside it.
Bilateral: toothless full-period ceiling (now calibrated,
red-verified under re-plant); the hand-mirrored operator ladder (four
dedicated rows + an association pin, red-verified under a planted
slip); unrecoverable measurement digits (instruments committed with
corpus as literals; re-measured 169/243, superseding the body's
141/243); Interval poison wording; the restrict-composition linear
accumulation law (both lanes' e2e independently; pinned as a row and
recorded as the receipt's third blind-spot entry).

**Fix pass ran as a FRESH lane** (the implementer's agent died with
the predecessor session — recorded honestly; no implementer-inherited
covariates exist for this unit, moot given the A/B skip). All 13
union items dispositioned: 11 fixed, one decided-otherwise with
receipts (the skipped interval oracle is path-keyed to the backend
and CANNOT be drawn by a trailer; the skip is correct), one
report-only → issue 1299 (normalize norm²-overflow → zero vector;
vec.rs is PCURVE's keep-out). Both reviewer probe suites adopted
whole, authorship preserved; branches cert/3r1-probes and
cert/3r2-probes pushed as reproduction sources.

Slate next: CERT-4 (issue 1191, under the SEMANTIC restatement of the
f64-bit constraint — Q2 seam), spec drafted; then CERT-5/CERT-6/
CERT-7 per the plan. CERT-B1 slot 3 is the block's last undrawn
slot; A/B logging RESUMES at CERT-4's dual (the skip was this
unit's only). Standing flags unchanged: issues 1250/1251 open;
CERT-1's v6 tally candidates await the blinded adjudication.

## CERT-4 merged (2026-08-31) — issue 1191 closed; the period folds are honest at Interval

PR 1303 at a912f8ae, gate green at BOTH lanes on the reviewed head
f2eb5a96 (run 33341476463) and the fix head a912f8ae (run
33348618043), both `CI-Config: lane=both`. The composed fold is
retired at one home (`Real::periodic_branch` /
`reduce_periodic_centred`, comparison-free); the eye's `[−τ, τ]`
advance gate is input-width; every hit-list site fixed or
dispositioned; the m10-p interval digest re-derived with a
differential that has no unfavourable half (75 coordinates, all
narrower). The exact-fit structural zero survived by construction —
with its domain CORRECTED at the fix pass to the rounding condition
`fl(fl(d/τ) + ½) < 1` after both review lanes independently executed
the boundary failure (top two floats of [0, π], plus the −0.0
bitwise caveat); behaviour unchanged from shipped, the claim and its
blind pins fixed. The spec's anticipated m10-3 driver pin flip did
NOT happen — the row's widening is a different mechanism
(dependency problem, box-scaled) — so the doc was retargeted, not
the row forced; both reviewers verified the retarget by execution
and judged the pin-doc edit the right size of touch. M10 flagged on
the PR for both fence touches.

**The dual (ordinal 702, sample #62 — the ledger's highest at this
writing is #61; main's merge order rules if a concurrent recorder
also drew #62)**: slot byte 130 parity 0 ⇒ R1 OPUS + R2 FABLE,
sequential on the frozen head, identical stored briefs
(cert/ab-state). R1 A-W-F 1/3/3 rubric 5/3/3; R2 A-W-F 0/4/3 rubric
5/4/4. The headline BILATERAL at differing severity (no tally
candidate); the two lanes gave contradictory corrections for the
classify.rs anchor sentence and the fix pass settled it by
computation — (τ − span)/2, one bad-point pair described from
opposite sides. Fix pass IMPLEMENTER-INHERITED (the lane resumed —
first inherited fix pass since the handoff), all 12 union items
executed, both probe suites adopted whole (cert/4r1-probes,
cert/4r2-probes, pushed), planted corruptions verified the
previously-blind pins load-bearing. A/B logging RESUMED with this
pair as planned; details in the MODEL-AB-LOG row (main-direct at
merge — main's ledger tail moved under the unit branch, so the row
could not ride the PR without a conflicted merge).

Issues filed en route: 1304 (k_probe_sweep dies at eps 1e-6,
pre-existing, M10's ground), 1305 (chord_join pole two-integer
shift). Escalated, not fixed: reader_census reds on a full local
workspace run at MAIN's own head (ledger line owed for
blend5_r1_probes.rs — S-BLEND's file; flagged to that program).
Operational: the container hit 100% disk twice this session; the
15G incremental-cache cleanup mid-lane is now standard practice.

Concurrent state at this merge: CERT-5 (PR 1314) and CERT-7
(PR 1319) both delivered and gate-green with duals pre-recorded on
cert/ab-state (CERT-5: byte 219 parity 1 ⇒ R1 FABLE + R2 OPUS,
frozen 3fc450d6; CERT-7: byte 114 parity 0 ⇒ R1 OPUS + R2 FABLE,
frozen d839dcef — the tip is one orchestrator-direct doc-link fix,
disclosed in the method note). CERT-7 was sequenced AHEAD of CERT-6
(orchestrator decision: CERT-6 calibrates on area lanes CERT-5
rewrites, so it waits for CERT-5's merge; CERT-7 is file-disjoint).
Reviews dispatch next, ordinals claimed at dispatch.

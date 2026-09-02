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

## CERT-7 merged (2026-08-31) — issues 1005, 1007, 1008 closed; the offset_fit family certifies

PR 1319 at ab8b6bad, gate green at BOTH lanes on the reviewed head
d839dcef (run 33347440242, ε default) and the fix head (run
33355836576, ε 1e-6) — four of six matrix points across the two
runs, ε 1e-12 unsampled and no claim resting on it. The weighted
composite certifies rational fits (the exact rational offset at
2.837e-14 on one cell); recentring makes the certificate
translation-honest (a micron offset a kilometre out went
inf-refusal → the origin's own number, with the domain settled by a
decade ladder to 1e6 and an honest refusal endpoint at 1e10);
directional refinement takes the thin patch 308 → 14 cells with the
stall guard's admission set now structural. RationalFitUnsupported
removed under D2 row 0; RefinementStalled classified row 1 with the
recorded row-2 minority reading (reclassify when A9.10's banked
half lands).

**The dual (ordinal 704, sample assigned at the row — see
MODEL-AB-LOG)**: R1 opus 0/5/8, R2 fable 0/1/5, both A-W-F, ZERO
MAJORs, no tally candidates. Notable convergences: both re-derived
the composite algebra independently; R2's planted revert reproduced
R1's 424× re-measurement exactly; both corroborated the stall
guard's unreachability (~100 adversarial requests, zero stalls) —
reframed by the fix pass as a PREDICATE-SHAPE verdict recorded at
the site. Notable divergence settled by measurement: far-origin
behavior at 1e7 (1.286×) vs 1e8 (tighter) — both correct, the
bound is not monotone in the shift; the row now claims only what
the ladder shows. Fix pass IMPLEMENTER-INHERITED, all 11 union
items, one class instance found beyond the reviews' four (the
sweep earning its keep); ceilings re-engineered to actually bind
(tolerances raised 4×→8× first — recorded as a deviation with the
argument); three planted corruptions verified the guards
load-bearing in BOTH directions (floor and ceiling separately).

En route, orchestrator-direct: the rustdoc broken-link red on the
delivered head (the removal's one orphaned doc citation — the
unit's variant sweep covered pins, not prose; CERT-7's dual brief
was amended to ask for the full citation surface, and the fix
pass's five-instance prose sweep closed the class). Issue 1321
gained the achieved:inf fourth face from R2.

Slate: CERT-5's fix pass is in flight (NMAI from its R2 — three
bilateral MAJORs; delta re-review follows before its merge);
CERT-6 dispatches after CERT-5 lands. CERT-8/9/10 then per plan.

## CERT-5 merged (2026-08-31) — issue 453 closed, issue 390 annotated; the straddle floor is gone

PR 1314 at ed7a7623 after the program's first NMAI → fix → delta
cycle. Gates: delivered head 3fc450d6 green (lanes both, ε 1e-12
drawn); fix head ed7a7623 green (lanes both, ε default drawn); all
six matrix points verified locally at the fix head. Knot-aligned
cells at four sites retire the Θ(1/pieces) floor (straddle branch
deleted, not bypassed); dm1 146× (2.7469e-4 → 1.885e-6, 1.84×
target — honestly NOT flipped; the dial decision is issue 1315 with
corrected figures); the lily flip is a GATE re-measured on the final
head; the w-uniform-in-v arm taken with the ruling's "strictly
better" softened to the measurement.

**The dual (ordinal 703, sample at the row)**: R1 fable A-W-F 2/5/4;
R2 opus NMAI 3/6/4 — all three MAJORs BILATERAL by execution
(identical dm1 digits to 17 places; the same drop-knots mutant; the
sliver hazard at differing severity), so the verdict split is pure
label noise on converged findings — v4 amendment 2's expected shape,
and the NMAI bound procedurally. Fix pass IMPLEMENTER-INHERITED:
the dm1 discrepancy explained by bit-identical reconstruction (the
meter's denominator moved mid-development); the blind-row gate
found its load-bearing combination (v-degree-1 + v-varying weights
— f_vv structurally None); the sliver attribution CORRECTED AGAINST
BOTH REVIEWS by measurement (refine_dir's exact-equality insertion,
pre-existing, worse on main → issue 1358; the delta lane
re-measured and conceded plainly); one own-mis-claim admitted
(tier_gate "corrected" when never touched — a PR-body claim
falsified by review, the CERT-1 brief-as-claim-site lesson from the
other side). Two refusal-class changes argued under the D2 addendum
(both the safe direction; the #389-gap masking recorded, not
claimed fixed). DELTA re-review by the NMAI lane: all three MAJORs
FIXED by execution, no regressions — DELTA-APPROVE.

**Cross-program consequence**: the branch's ε=1e-12 draws surfaced
FOUR latent main reds at the never-sampled (interval, 1e-12) matrix
point, left by CERT-3/CERT-4's merges (their gates drew other ε
rows) — repaired as ported main fixes in this PR (per-band-honest
re-pins, no re-baselines; delta-verified), with the CLASS filed as
issue 1356 (recommendation: trailer-pin ε for band-sensitive units;
distinguish "consults no tolerance" from "premise varies by band"
in review briefs). S-CERT should adopt the ε-trailer practice for
its remaining units — CERT-6 onward dispatch with it in the spec.

Issues: 453 closes at this merge; 390 stays open annotated (route 2
unclaimed); 1315/1316 corrected+appended; 1356 (ε-matrix class),
1358 (refine_dir + the five-way inner copy) filed. Slate next:
CERT-6 (area gauge, on the post-CERT-5 area lanes, B2 slot 2),
then CERT-8/9/10 and the absorbed SMELL tracks per plan.

## CERT-9 merged (2026-08-31) — issue 303 closed; signed_volume is placement-honest

PR 1361 at f5f949aa. Gates: delivered head 5593161d green (default
lane drawn); fix head green run 33383424770 (interval/1e-6 DRAWN —
consistent with the unit's argued lane-independence, no trailer).
The fold recentres on the bbox centre (overflow-robust form);
red-first digits vivid (33.3 vs the true 1e-9 at a 1e8 m offset;
pre-fix far placements could SIGN-FLIP a volume, so seven
orientation asserts were latently placement-dependent). S-sized
unit, S-sized cycle: implementer ~65m, reviews ~29m+16m, fix ~65m.

**The dual (ordinal 705, sample at the row)**: R1 fable APPROVE
0/1/3; R2 opus A-W-F 0/3/4; zero MAJORs, no tally candidates. The
brief's designed attack — the exactness argument's closed-mesh
premise — landed bilaterally (the open-mesh answer changed,
silently; unreachable in-repo; now defined in the doc), and R2
proved the shipped argument UNDERSTATED the mechanism (a
position-derived anchor gives unconditional translation invariance
by equivariance) and caught the ε premise conflating assertions
with fixtures. Fix pass IMPLEMENTER-INHERITED across the container
restart (the agent resumed from its transcript); its three-ε sweep
found and fixed a latent ε=1e-12 red in the adopted probes before
any gate drew it — the issue-1356 practice already paying.

Operational: the container restarted mid-wave (~09:20Z), killing
the first CERT-6 lane with nothing pushed (~50m lost, fragment
saved); re-dispatched fresh on the same arm, recorded for its row.

Slate: CERT-6 (re-dispatched, running), then CERT-8, CERT-10
(opens block CERT-B3), CERT-M/CERT-N track lanes. Issue 1362
(walk.rs anchor class, S-MESH) filed en route.

## CERT-6 merged (2026-08-31) — issue 870 closed; the area enclosure has its tripwire

PR 1366 at 0696cdbe, gates green with lane=both + eps=1e-6 BOTH
trailer-pinned — the first unit under the full issue-1356 ε
practice. The A2 gauge landed as the Q1 ruling's row-5-boundary
debug_assert: a certified chord-traversal bound (knot lines ∪ block
edges ∪ two coprime grids) under a max(chord, caller) denominator
whose failure-direction weighing lives at the claim site, ceiling
1.0 with the margin stated against DOOR-AUTHORED anchors (79× on an
untuned public loft, 13.1× on dm1's refusing wall, 17.6× on the
relative arm's first live witness) rather than a corpus statistic.

**The dual (ordinal 706, sample at the row)**: R1 opus A-W-F 1/6/5;
R2 fable APPROVE 0/1/4. **R1's MAJOR was UNILATERAL AND EXECUTED**
— the delivered 16-chord schedule ALIASED (Nyquist collapse at
k≡0 mod 8, 16.5×/273× measured), an understated denominator being
a release-panic path, plus a door-authored calibration
counter-example at 2× the corpus max — a v6 TALLY CANDIDATE, coding
deferred to the blinded adjudication. R2's unique yield: the
balloon witness that made the fallback arm live, and every PR digit
re-derived exactly. Fix pass IMPLEMENTER-INHERITED and unusually
strong: it reproduced the MAJOR, then MEASURED that the adjudicated
knot-aware fix alone was insufficient (11× under at 64 spans — the
knots ARE the zero crossings there) and shipped the stronger
coprime + max-denominator design; every threshold now names its
fixture (the delivered ×80/×90 pair's provenance explained, both
reviewers' non-reproductions being different fixtures); the
triplicated assert block collapsed to a helper, which exposed a
wrong relative-arm message; the calibration figures reduced to ONE
home with pointers (the #651 shape applied).

Incidents owned: the delivered head redded the discipline gate's
interval-square-allowlist (x*x in the gauge) — orchestrator-direct
powi(2) respell, with R1 correctly noting my commit message's
"strictly-tighter-under-straddle" rationale misfits plain-f64
sites (the gate matched a spelling outside its subject; the respell
stands on the policy, not that rationale). R2 stalled once on a
background waiter (the discipline doc's exact failure mode) and was
nudged back to foreground polling. R2 also flagged two zero-byte
junk files (r₁, r₂) at main's root from a verbs merge — deleted
orchestrator-direct at aae0993e. The unit's FIRST lane died in the
container restart with nothing pushed (~50m, fragment saved) —
the re-dispatched lane's row records it.

Slate: CERT-8 (chart-stretch honesty, issues 501+528) and CERT-10
(patch-hull consolidation — its CERT-5/CERT-7 sequencing gate is
now satisfied) dispatch next, opening block CERT-B3; then CERT-M/
CERT-N track lanes and the exit walk.

### CERT-8 — chart-stretch honesty (issues 501 + 528) — MERGED

PR 1398; spec `docs/CERT-8-SPEC.md` (cert/orchestrator d90e7441); block
CERT-B3 slot 0 (byte 27, fable at 3 → OPUS); ordinal 708 claimed at
dispatch (main 30195f16c); sample number at merge.

Delivered head 085ddf8f (+1002/−142 over seven files). Two
orchestrator-direct discipline fixes before review, both disclosed in the
twice-amended method note: the interval-square-allowlist catch on
`chart_stretch_inf`'s `ratio * ratio` (1a2574b0, powi(2), value-identical)
and the one `exact_arms` doc link the rename left dangling (bde7b17f, the
CERT-7 class). Frozen review head bde7b17f. Its first hosted run died at a
GitHub spending-limit startup failure (both root jobs, no runner); Evan
refreshed the budget and re-ran: 22/22 green, both ε=1e-12 lanes pinned.

Dual (v6, sequential; byte 201 parity 1 ⇒ R1 FABLE + R2 OPUS): R1 A-W-F
0/5/3 (rubric 4/3/4), R2 A-W-F 2/4/+. Both upheld the singular-value
inf-arm assembly by independent execution (a 3684-chart hunt and a
161²×8 sweep, zero violations; every acceptance digit reproduced). The
union: the mean-width CONTRACT at `ChartOverlap::PositiveArea` false under
non-constant stretch (R2's strip exhibit, model width 97× below the
reading; R1 held the fact at NOTE/MIN — partially bilateral, severity
divergence); the swap row's `(0, inf_u]` pin non-binding (bilateral,
executed twice: a full sup-swap assembly survived the suite); R1's
`v_window` axis mutation surviving the suite (unilateral executed);
interval reachability zero in the diff (bilateral); the D2 Corrupt row
argued from a premise its own file contradicts (R2: row 4, unreachable
behind the `len < 3` gate); two prose premises invalidated at the pole
joint; the derivative-net loop spelled four times in one file with nothing
pinning the two sup readings equal (bilateral class). Severity divergence
on converged substance again — calibration signal; no clean unilateral
executed MAJOR, so no tally candidate here (the partial-bilateral pair's
coding deferred to the blinded adjudication).

Fix pass (implementer-inherited, all items taken, none declined beyond
taste): the contract narrowed at the claim site (`mw_model ≥ (ρ/√T)·
mw_scaled` quoted, the strip exhibit promoted to a row); the swap pin
rebound to the derived arm (0.031189; the sup read now reds at 0.353726);
the face-level `v_window` row (axis swap flips PositiveArea to
ArmUnbounded); interval-typed rows reaching `net_inf`/`chart_stretch_inf`/
`certified_arms`, with the arm gate now reading the bracket FLOOR so a
folded net refuses typed in both lanes; the pole-joint three-way row on a
spline chart and both stale premises rewritten; the D2 row moved to row 4
(`unreachable!` naming the gate); both doc contracts corrected; the
net-loop class absorbed into `derivative_net` with the sup-agreement pin
(`the_two_doors_report_one_sup`); AND one soundness fix the reviews only
suspected — the sphere arm's `r·cos v` re-entering positive past π
(cos 6.5 ≈ 0.977 would have certified a pole-sweeping window) now refuses
outside cos's monotone range. Both reviewer probe suites adopted with
authorship preserved (cert/8r1-probes, cert/8r2-probes pushed). Then the
598-commit main merge (one real conflict, chart_region.rs, additive both
sides; main's cylinder-band lane is kind-gated ahead of `overlap_of_uv`,
so the narrowed contract holds for its exact-arm producer too) and main's
newer tooling — ruff on the adopted Python probe, the new
interval-cfg-additive gate on the interval rows, two clippy lints in the
adopted Rust probes — each caught once, fixed once. Sweep re-run at the new
base 2f7edd2d: no new default meters, no new wrong-side reads. Final head
970f5f4d: run 33555716228, 22/22 green, both lanes at 1e-12 by trailer.
The lane owned its earlier verification-claim miss ("nothing is denied"
about rustdoc — an inference dressed as a measurement).

Issues: 501 and 528 closed at merge; filed: the `edge_chord_len` 1 m
default at two plane-gated sites (disclosed-unscheduled in the sweep). The
tree's other inf-side surface bound (`offset_meters`' ‖S_u×S_v‖ floor)
recorded as the shared-home wish beside CERT-10's `TensorNet` — not taken;
`step-import/recognize.rs`'s two net-loop siblings recorded, other crate.

Slate: CERT-10 (PR 1403) is the last defect-cluster unit; then the
CERT-M/CERT-N track lanes, the blinded tally adjudication (four standing
candidates), and the exit walk.

### CERT-10 — the patch-hull consolidation (issue 1006, under the Q2 ruling) — MERGED

PR 1403; spec `docs/CERT-10-SPEC.md` (cert/orchestrator d90e7441); block
CERT-B3 slot 1 (byte 27, fable at 3 → OPUS); ordinal 707 claimed at
dispatch (main a063a6125); sample number at merge. Sequencing gate
(CERT-5, CERT-7 merged) satisfied at dispatch.

Delivered head a33926e5, eight commits in the spec's order: red-first rows
and the cost harness; the `geom_core::spline::net::TensorNet` home (the
1-D step as a parameter); the whole-face arm collapsed into the fold with
the cost table taken BEFORE the shape was chosen (1.01–1.20×; an earlier
flattering 0.65–0.80× reading disclosed and retired); issue 1322 as
invited; the magnitude retirement (signed reading 0.77×/0.09× on the
quarter cylinder's muu/muv, 8.4× fewer rational cells at δ_s=4e-3) with
the tess-budget re-baseline attributed stage by stage against a merge-base
sweep (57 columns main drift, 158 fold-tightened with zero grew,
retirement zero on a corpus with no rational faces). One
orchestrator-direct discipline fix before review: 21 pinned literals past
f64's precision (f5ab8bab, each pair bit-identical; the lane's "clippy
clean" line had skipped `-p mesh`). Frozen review head f5ab8bab; gate
33426061935 green, both ε=1e-12 lanes trailer-pinned.

Dual (v6, sequential; byte 242 parity 0 ⇒ R1 OPUS + R2 FABLE; R2
interrupted once by a model rate limit and resumed on the same arm): R1
A-W-F 1/2/~4 — the MAJOR by instrumentation: the PR body's "the fold
removed an assembly per shipped face" is FALSE on the shipped path
(`compute_chords` fills the whole-patch memo before the per-face dispatch;
`NurbsCellGrid::patch` reads = 0; 2.00 assemblies/face before and after),
so the fold's 1.14–1.20× was net. R2 A-W-F 0/3/2 judged the same give-back
real BY INSPECTION — executed evidence adjudicated over inspection, so R1's
MAJOR is UNILATERAL-EXECUTED: the program's fourth standing v6 tally
candidate (coding deferred to the blinded adjudication). Both arms upheld
the signed reading's soundness under ~198k-sample and ~2M-containment
campaigns (zero violations) and independently found the same sub-ulp
fact: on rational faces the signed reading encloses the refined-f64 net's
surface (`s_vv` excludes true 0 at ~4e-15) — pre-existing, newly the only
reading, its contract overstated. Bilateral: the fold property row's
generator never reached interior multiplicity ≥ 2 (the C¹ gate
load-bearing for coverage, stated nowhere); the inner-knot-slice five-site
class unswept by a consolidation unit. R2-unilateral: `comp_nets`' stale
"bridged rather than unified" doc assigning its residue to this very unit.

Fix pass (implementer-inherited, all 11 union items taken, none declined):
the give-back reproduced FALSE (4 faces, 8 assemblies, 0 patch reads) and
then MADE TRUE by route (b) — the memo holds the cell table (`face_grid`)
and the whole-patch bound is a reading of it; re-measured 1.00/face, now
ASSERTED (`mesh::budget` counts assemblies, R1's probe promoted to a row;
the planted regression reds it 8 vs 4); the enclosure-provenance section
on `PatchCell` with every field pointing at it, the noise pins de-pinned
to order-of-magnitude (6 of 256 quarter-cylinder cells at
`[-4.1e-15, -3.4e-15]`, worst dust 4.250461e-15, cause the 16× insertion
materialised in f64), `offset_meters`' inf-side door saying why reading
them there is sound; the generator drawing multiplicities 1..=p−1 with a
second coverage floor (2400 comparisons, 974 strict, 0 violations); the
fill parameter DELETED — a step answering anything but n−1 poisons its
whole line, reachability stated as measured (neither caller can trip it);
`comp_nets`' doc de-staled both ways with the residue given an owner
(issue 1532); the knot-slice class FOLDED, not re-deferred
(`KnotVector::derivative_knot_slice` + raw twin, seven sites re-pointed,
zero remaining tree-wide); the counterfactual-faithfulness row (exact
digits on wavy and staggered_channels); both reviewer probe suites adopted
with authorship preserved (cert/10r1-probes, cert/10r2-probes pushed).
Then TWO main merges: the 598-commit one (S-MESH's MESH-4/5 comment-only
on this unit's files; pass order unchanged, the fact item 1 turns on;
baseline conflict taken as main's cut and re-cut on the merged tree),
where main's newer tooling caught three things once each (map_entry,
tess-lint against main's cut, the `folded_face_bound` citation) and this
PR's gate found one main-side breakage (k-lint's sampled probe+interval
row; ported here, fixed on main by PR 1526, a no-op since); and — after
GitHub created NO workflow run for the push bb8747a94 — the 13-merge one
to 7ee04c114, where CERT-8's landing REMOVED the `pcurve_cache` tensor
site the sweep had listed as its one fenced sibling (deferral closed by
someone else's merge, recorded) and PR 1506's tour trim left `lily.rs`
untouched. Re-baseline re-derived against 7ee04c114: main's file ≡ main's
behaviour, so every move is this branch's — 158 bound columns, 0 up, 0
identity-column changes across 1306 rows; certificates 0 over δ, worst
cert/δ 0.124994 unmoved; the finding-13 pinned row RE-DERIVED per the Q2
ruling (lily_leaf_b 468 → 454, lily_leaf_c 414 → 384, derivation in the
table; 12 `triangle_count` sites swept, only these two pin). The lane owned
two verification-claim misses: the clippy line that skipped `-p mesh`, and
"done" claimed without the doc gate or the tour release row (both redded
later; both now in its pre-push set). Final head a4eb03aef: run
33569749882, 21 success + 2 neutral (render drift, by convention) + 1
skipped; both lanes at 1e-12 by trailer. One cosmetic residue: the second
merge commit's message carries a stray CJK character ("disposition改"),
unfixable under merge-only history.

Issues: 1006 and 1322 closed at merge; filed: 1532 (the two recentring
centres — a measured decision, given an owner). Not moved, by design:
1320, 1321, 1368 (their instruments verified unmoved).

Slate: the defect cluster is clear (CERT-1..10 all merged). Next the
CERT-M/CERT-N track lanes (CERT-M1 in flight, CERT-N1 dispatching from
this merge), the blinded tally adjudication (CERT-1×2, CERT-6×1,
CERT-10×1), and the exit walk.

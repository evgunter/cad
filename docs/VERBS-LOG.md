# VERBS — the modeling-verb breadth program (log)

Append-only narrative record. Plan: `docs/VERBS-PLAN.md`. Register:
`docs/KERNEL-VERBS.md`.

## Kickoff (2026-08-21)

Program opened at Evan's ask: orchestrate execution of the
KERNEL-VERBS register. Kickoff rulings recorded in the plan's header
(wave order; verb-gating defect ownership; design conversations open
as info firms; C7 stays M9's). Session ops: four monitors armed from
installed copies; away-channel tag `(VERBS orchestrator)`, prefixes
`verbs/,mngr/kernel-verbs`. A usage WARN for a DIFFERENT account was
active at kickoff (92% of 7d) — this program's account confirmed
distinct by Evan, dispatching normally. An opus issue-scan subagent
dispatched over all open issues (Evan flagged several as plausibly
fillet-related) + the #222 closure verdict; its report folds into
Wave 1 specs before the first dispatch. Evan's misc-style remote
agent is working repo-wide (`smellj/*` etc.) — expected not to cross
paths.

## Surveys folded; VERBS-RIM at the door (2026-08-21)

Both kickoff surveys returned and are folded into the plan:

- **Issue scan (opus)**: per-unit constraints now in VERBS-PLAN /
  VERBS-RIM-SPEC. Headlines: #319's corner run-out door is a
  co-requisite of VERBS-ARMS, not an extra; #644's convex-only
  corner arguments constrain both chamfer and arms; #883 is parked
  (no signature tightening); a cyl×cyl lane joined Wave 2 on #347's
  live consumer; helix stays blocked (#222 closed = measurement
  executed, not fixed); tessellation-gate claims suspect until
  #746/#782. No open issue covers TUBEWALL or FullRevolveHoles —
  those units file their own.
- **Q8 substrate survey (opus)**: evidence base for the Q8 design
  conversation, held at the orchestrator lane
  (`cad-work/verbs-orchestrator/q8-substrate-survey.md`) until the
  conversation doc lands. Headlines: zero offset code exists;
  fitting is curve-only (A9.4/A9.10 are from-the-book builds; curvo
  has nothing to borrow); the approximating-surface object doesn't
  exist and the validator would re-derive its certificate per face;
  open shells are STRUCTURALLY unrepresentable (Edge born
  two-sided), so the cheapest honest shell is
  body − offset_inward(body) — which collides with "voids are born
  only from booleans" (DESIGN.md:348) unless shell is defined as
  boolean-family; the teapot's closed two-shell curved body hits
  step-export's `CurvedShellClassification` refusal.

**VERBS-RIM dispatching**: spec committed
(`docs/VERBS-RIM-SPEC.md`), difficulty M logged pre-draw, block
VERBS-1 drawn per v4 (arm mapping held at the orchestrator lane
until merge rows land in MODEL-AB-LOG, per blinding). Lane
`verbs-rim`, branch `verbs/rim`. State-sync PR for these docs opens
at this seam.

## Q8 conversation OPEN; two more surveys out (2026-08-21)

- State-sync #906 MERGED (plan, log, VERBS-RIM spec on main).
- **VERBS-RIM implementer DISPATCHED** (lane verbs-rim, branch
  verbs/rim, spec on main; wall-clock start ~02:57Z 08-22).
- **OFFSET-DESIGN opened as #907** (branch verbs/offset-design) —
  the Q8 design conversation, proposals O1–O6, awaiting Evan.
  The fork most needing his call is O4: sealed shell as
  boolean-family (`B − offset_inward(B,t)`, keeping "voids are born
  only from booleans" intact) vs a third void source.
- Substrate surveys dispatched (opus, read-only) for the next two
  conversations: PATTERNS/MIRROR (D8 instancing, reflection's
  orientation-sensitive site classes, G8 multi-operand gap) and
  DRAFT (surgery-pattern generalization, face replacement,
  re-intersection reach). Conversations open when they return.

## Three conversations open (2026-08-21)

Draft + patterns/mirror surveys returned; conversations opened the
same day (Evan's open-as-info-firms ruling):

- **#908 DRAFT-DESIGN** (branch verbs/draft-design): plane-wall v1
  (the cylinder arm re-opens R1's conic inventory and is deferred to
  its own ruling); mechanism is a certified re-geom pass, NOT the
  M6-1 graft shape (topology survives; the missing primitive is a
  pass-owned vertex step); the pull-direction predicate is a
  SELECT-DESIGN amendment; the moldability checker rides along.
- **#909 MIRROR-DESIGN** (branch verbs/mirror-design): the patterns
  half already SHIPPED (Node::Pattern + PlacedUnion, part-level) —
  the register rows are corrected in the PR; the open half is
  mirror: P1 chart-handedness ruling (u ↦ −u recommended), P2 own
  door beside transform_rigid, P3 audit boundary (~15 site classes).
  Hole features are no longer "blocked mainly on patterns".

Both PRs edit KERNEL-VERBS.md rows — the second to merge may need a
trivial conflict resolution. **Sheet bodies (the D1 conversation) is
deliberately HELD** until the open three get responses — it is the
heaviest conversation and Evan's review bandwidth is the scarce
resource. In flight: VERBS-RIM implementer (no report yet); #907
(Q8) awaiting Evan.

## VERBS-RIM review dispatched (2026-08-22)

Implementation landed as **PR #910** (head cfe743cc frozen; CI run
32551747885 green on the full matrix, 12m45s; k-lint quiet). The
implementer's report: consumer enumeration complete (convexity_at,
chain_g1, pub arm_len — no other readers; face_clearance and blend
setbacks verified non-consumers), two disclosed deviations argued
as improvements (concave fixture = spherical boss root ring, since
a closed rim requires an annular profile; >240°-arc levers move
without verdict changes), three banked findings (on-axis profiles
mint open half-band rims; a closed single-link chain never gets a
wrap-around G1 check — live once curved arms land; battery refusal
order is request order). The lane also stalled once waiting on its
own CI monitor — the lost-wake shape the discipline doc warns
about; nudged, recovered.

**Ordinal 60 claimed** (ledger on main through 59/PERR at dispatch)
→ a third → **DUAL, CROSS-MODEL** (12-pair target not met): R1
fable + R2 opus, concurrent on the frozen head, identical briefs,
blinded to each other, v4 ladder, v5 style lane. Lanes
verbs-rim-r1 / verbs-rim-r2; probe branches verbs/rim-r1-probes /
-r2-probes requested for authorship-preserving adoption. Reviewer
models are recorded per protocol (not a secret); the IMPLEMENTER
arm stays off-record until the merge row.

## Ordinal-60 dual returned; fix pass running (2026-08-22)

Both reviewers: **APPROVE-WITH-FIXES**. R1 (fable) 0/2/3, one
silent dev (surgery.rs:494's premise invalidated by the PR itself
— the passing dome rim now reaches resolve_rim live). R2 (opus)
5/4/3, three silent devs — the prose sweep stopped at the crate
boundary (docs/KERNEL-VERBS.md's own defect entry and
demos/README's klein row still state the fixed defect verbatim),
chain_g1's gating-parity sentence is false (the dihedral never
gated a collapsed arm — the missing gate IS #554's mechanism), and
the design ruling itself was unpinned (a planted closed-case
bolt-on passed the PR's whole suite; only R2's open-arcs row reds
under it). Both independently REFUTED the PR body's "no verdicts
change on >240° open arcs" (near-period open arcs flip refusal
class, honest direction) — the defect was never really about
closure. Substance converged, labels converged; the divergent
tails were each real (the R1/R2 pattern holding).

Fix pass dispatched to the implementer lane with the adjudicated
union, including an ORCHESTRATOR AMENDMENT to the spec's ruling
(within its extension clause): the lever moves to the module's
existing named 9-sample CHAIN_SAMPLES schedule — one sample
vocabulary, tighter lever, C7 residue dissolved. Both probe
branches adopt authorship-preserving. VERBS-ARMS' scope now owns
the one-edge closed-chain shape (plan row amended) — the
verb-level full-revolve unlock is ARMS' acceptance, not RIM's.

## VERBS-RIM MERGED (#910, 2026-08-22, merge at 06:50Z) — #554 CLOSED

Row RIM (ordinal 60, sample #19, the SEVENTH cross-model pair) is in
MODEL-AB-LOG — arm now on record there. Fix pass took the
predicate-REUSE branch for the dihedral arm gate (no new metered
name); the lever lives on the CHAIN_SAMPLES schedule; both reviewer
suites are promoted (two disclosed amendments, both argued in the PR
body); the second CI round's red was an adopted row's fixture
failing PROFILE validation in-band at eps=1e-6 — an empirically
proven fixture impossibility, not a fillet defect. The register's
#554 entry now states the present: metering fixed, the verb-level
remainder (one-edge closed chains) owned by VERBS-ARMS.

Seam actions: this state-sync PR; sweep lanes verbs-rim,
verbs-rim-r1, verbs-rim-r2 (probe branches adopted; clones
merged/pushed-clean). NEXT UNIT: VERBS-CHAMFER (block VERBS-1 slot
2) — spec next; it rides the same fillet files, so it dispatches
onto post-#910 main with #644's constraint live.

## VERBS-CHAMFER at the door (2026-08-22)

Spec committed (docs/VERBS-CHAMFER-SPEC.md): plane-plane ruled
strip, symmetric setback v1, planar trihedral corner patch, #644
quarantined (new corner code coherent from birth, corner_ball
untouched), chamfered-cube acceptance with closed-form mass
properties, no tessellation-gate rows. Difficulty M logged
pre-dispatch; consumes block VERBS-1 slot 2. Lane verbs-chamfer,
branch verbs/chamfer.

## CHAMFER landed as #920; ordinal-61 single review out (2026-08-22)

Implementation complete on PR #920 (head 86a8e49b; CI green
32563903264 after one mid-flight red round — fmt/rustdoc gate,
evaluation-code discipline, render lanes — fixed in-lane). The
report's headlines: surgery PARAMETERIZED (`blend_surgery` +
`BlendKind`, feet become plan data) with bit-identity evidence for
fillets; the chamfer geometry is convexity-free by derivation
(branch-free outward folds — #644 sidestepped, not half-fixed);
predicates 1/3 not metered (rolling-ball facts with no ruled
analog); three disclosed deviations each with a filed schedule
(#917 shared refusal vocabulary, #918 recipe-layer door, #919
concave deferral); chamfer_edges placed INSIDE fillet/build.rs to
stay within the ratified bounds allowlist. **Ordinal 61 claimed**
(through 60/RIM at dispatch) — not a third — single fable review,
lane verbs-chamfer-r1, frozen head 86a8e49b, probe branch
verbs/chamfer-r1-probes requested. Review charter includes the
mid-flight CI red's fix commits (C7: cause vs symptom, render
re-baseline honesty).

## CHAMFER fix pass done; #920 HOLDS on #921 (2026-08-22)

Ordinal-61 review (single, per protocol): APPROVE-WITH-FIXES,
0 MAJOR / 4 MINOR / 5 NOTE + 6 style. Fix pass complete at
f47012a2, every disposition landed (NonpositiveSize door as plain
typed refusal — input validity is not metered; both runtime
fillet-worded strings fixed; #917 widened to the reviewer's full
class enumeration; probe suite adopted fast-forward with one row
moved WITH its finding; one public path per verb). The unit also
exercised the new configuration-sampling CI for the first time in
VERBS: three runs drew three different points, the census gate
tripped and was resolved by ROSTERING the adopted probe suite
(its set-equality roster row is the C4 claim), and the final draw
(interval, 1e-12) exposed **#921** — two `carrier_matches_mapped_source`
rows red ON MAIN at that point, bit-identical margins at f059298c,
proven not the branch's. The lane refused to re-roll the draw.

**#920 holds unmerged until #921 is decided** — merging on a red
check with a "it's main's fault" argument is the habit the
sampling design warns against; the honest path is: resolve #921,
merge main into the branch (a real re-draw), green, merge. An
investigation lane (i921) is tracing the enclosure's width on
main; its report decides band-vs-enclosure-vs-row-contract, with
Evan looped in if it is a genuine tolerance-design fork.

## #921 adjudicated; VERBS-ARCEVAL dispatched (2026-08-22)

The investigation traced the red to a manufactured width in
SketchSegment::eval's arc branch: center reconstructed from a
cancelling sub-arc chord and entering the evaluation twice —
~100x the carrier-side width, ∝ 1/sin(θ/2), ε-independent
(bit-identical enclosures at 1e-6 vs 1e-12), compounding through
restrict. Ruling on the #921 thread: fix the arithmetic
(a-anchored rotation, measured to green the m6 row), re-scope
m5's Interval row to DEFINITE at ε ≥ 1e-9 + pinned honest
escalation at 1e-12 (its hi ≤ ε at T=Interval is a conditioning
claim; f64 holds the geometry claim at 8.9e-16), band widening
REFUSED (no derivation; would launder arithmetic into tolerance
policy). The triple-restrict redesign is banked on #921.
VERBS-ARCEVAL dispatched (difficulty S logged pre-dispatch,
block VERBS-1 slot 3; the investigator's lane continues as the
implementer — full context, arm-consistent), branch verbs/arceval.
#920 merges after this lands via a real re-draw.

## Design-conversation rounds with Evan (2026-08-22, afternoon)

- **#907 (Q8)**: two rounds folded. Round 1 — self-intersection
  doors added to O1 (trimmed offset as a later topology verb over
  the same intensional spec; solved-d tangency = declared-contact
  spelling now, M10 root-solve banked); O4 rewritten as
  definition-vs-execution (sealed shell = degenerate no-crossing
  arm through the boolean's void-insertion door; #750 avoided
  by-construction; Wave-2 coupling dropped). Round 2 — Evan's
  sweep note: the refined invariant ("every cavity is born through
  the shared void-insertion door") admits the holed full revolve
  as its third producer, which IS VERBS-RING — the plan row is
  redefined (RING factors the door, first consumer, gated on
  ratification), and DESIGN.md's M2 bullet is revised in the PR.
  Final 👍 requested (watchlisted).
- **#908 (draft)**: Evan corrected DR1's cost reading — R1 bars
  only exact conic special cases; fitted-NURBS plane×cone sections
  are fine, so the cylinder arm is a fitted-SSI lane (plumbing),
  not a ratified-decision change. DR6's checker confirmed
  kind-general (normal enclosures exist per kind, not per verb).
  Folded; 👍 requested (watchlisted).
- ARCEVAL: PR #922 open, hosted draw missed the 1e-12 point
  (pinned interval lane, default ε drawn) — ordinal-62 single
  review dispatched with the 1e-12 repro as the reviewer's
  unique-signal run.

## OFFSET-DESIGN RATIFIED + MERGED (#907, 2026-08-22 👍)

Q8's elaboration is on main: O1-O6 incl. both of Evan's rounds
(self-intersection doors; O4 definition-vs-execution with the
degenerate no-crossing arm; sweep-born cavities as the third
producer) and the DESIGN.md M2-bullet revision ("every cavity is
born through the shared void-insertion door"). **Wave 3 is
design-ungated**, and **VERBS-RING's gate lifted** — it dispatches
next after the ARCEVAL fix pass lands (block VERBS-1 slot 4, the
block's last slot; spec = the ratified O4/RING definition, the
door factoring, and the retirement of FullRevolveHoles).
#908/#909 still await 👍 (one open question each, answered).

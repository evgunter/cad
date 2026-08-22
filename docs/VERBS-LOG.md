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

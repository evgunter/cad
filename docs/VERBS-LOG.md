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

## DRAFT-DESIGN RATIFIED + MERGED (#908, 2026-08-22 👍)

DR1-DR6 on main with Evan's corrections: plane-only v1; the
cylinder arm re-costed as a plane×cone fitted-SSI lane (R1
untouched — its refusal bars only exact conic special-casing);
the moldability checker rides kind-general. The draft VERB
implementation queues behind Wave 1's remaining units per DR6's
own sequencing; its register row now points at the ratified doc.
Remaining conversation: #909 (one 👍 pending).

## ALL THREE CONVERSATIONS RATIFIED + MERGED (2026-08-22)

#909 (MIRROR-DESIGN) 👍'd and merged after the P1 user-invisibility
round; its register-row conflict with #908 resolved in the merge,
and the draft row's stale "re-opens R1" clause corrected to DR1's
ratified form in the same resolution. **The program's design
backlog with Evan is CLEAR**: Q8/offset-shell, draft, and
patterns/mirror all ratified same-day. Implementable-when-scheduled
design now exists for: Wave 3 entire (offset → shell → teapot),
RING (ungated), draft v1 + the kind-general moldability checker,
mirror (own door, u↦−u, audit-checklist scope), the SegPat
instance-index predicate, and hole-feature sugar (behind P4's
lowering ruling).

## ARCEVAL MERGED (#922); the class is serial (2026-08-22)

Ordinal-62 single review: APPROVE-WITH-FIXES 0/2/3 — its
unique-signal run proved both #921 rows green at 1e-12; MINOR-1
(the escalation arm's 2x ceiling ADMITTED the pre-fix defect —
verified by plant, fixed to a bit-exact pin) and MINOR-2 (the
affine sibling, now filed as #924). Fix pass landed every item +
both probe suites; its CI drew (interval, 1e-12) — the #921 point —
and exposed the mechanism: **the class's membership is discovered
serially** (shard cancellation hid member 3 behind members 1-2), so
a green-1e-12-before-merge gate would deadlock the fix chain.
**Adjudication: #922 merged as a proven strict improvement at the
red point** (two rows greened, member 3's escalation moved sample
3 → 8 — byte-for-byte compared against pristine main, not argued).
The distinct NaN'd SSI row found at the same point is #925 (thin
filing, routed toward #762's guard family, a Wave-2 rider). The
lane continues with verbs/arceval2: the m5_s13 member re-scoped
under the same ratified ruling. The structural class retirement
(triple restrict) now has three recorded consumers on #921 and
will be scheduled as its own unit. After arceval2, (interval,
1e-12) on main is red on exactly one enumerated row (#925), which
is the state #920's re-draw compares against.

## CHAMFER MERGED (#920) + ARCEVAL rows recorded (2026-08-22)

The re-draw after the #921 chain came back green (32589961583,
point (interval, default) — recorded per the sampling convention)
and #920 merged; #554's sibling verb exists. Ledger rows CHAMFER
(ordinal 61) and ARCEVAL (ordinal 62, the #922+#926 two-PR unit)
are in MODEL-AB-LOG — block VERBS-1 is fully consumed
(RIM fable / CHAMFER opus / ARCEVAL opus / SSIFLAT opus).
In flight: VERBS-SSIFLAT (the SSIGUARD dispatch re-scoped after a
model diagnosis — the #925 "NaN" is MANUFACTURED by ssi_refusal's
error flattening (`MarginDiag::Enclosure → f64::NAN`), masking an
honest ε-independent in-band escalation; the unit fixes the
flattening seam + re-scopes the row per the ruling's fourth
application; #762 confirmed UNRELATED on the call graph and stays
open as a Wave-2 rider). NEXT after SSIFLAT: block VERBS-2 draw +
VERBS-RING (ratified definition, the void-door factoring) and the
VERBS-ARMS spec. Seam sweep: verbs-chamfer lane.

## VERBS-RING spec committed; block VERBS-2 drawn (2026-08-22)

docs/VERBS-RING-SPEC.md: the ratified O4 definition executed — the
void-insertion door factored callable-without-SSI (behavior-
preserving for booleans, bit-identical), FullRevolveHoles retired
with containment certified FROM THE PROFILE's own validated 2-D
margins (no 3-D box tests — #750 stays out of the unit), klein
wall 6 flips, the curved-two-shell STEP refusal recorded as the
known standing gate. Difficulty M logged pre-draw; block VERBS-2
drawn (mapping lane-private per blinding); lane verbs-ring next.

## Parallelism widened to four lanes (2026-08-22, Evan's headroom note)

Running concurrently: SSIFLAT (impl), RING (impl), **VERBS-DEMO**
(impl — the chamfered-die montage pair now; the translucent hollow
ring per the existing torus-opacity precedent, gated on RING's
merge; difficulty S logged at dispatch, block VERBS-2 slot 2), and
the **VERBS-ARMS substrate survey** (read-only, no slot). The
build-slot mutex serializes heavy cargo across them by design.
SSIFLAT posted the #925 routing correction on-thread (the "NaN" is
manufactured at the flattening seam; #762 unrelated, stays open).

## ARMS survey folded; ARMS-1 spec committed (2026-08-22)

The survey's headlines: the coaxial arms are UNREACHABLE without
the closed-rim surgery (their consumers are one-link closed rims —
the dependency that orders the cut); six of eight pairs collapse
into one coaxial-torus derivation mirroring plane_sphere_blend
(copy its poison-flows-to-spine_curvature posture); **no
constant-radius roll mints a cone** — C8's "cone cases" prose gets
a scoped correction at ARMS-2 (called out to Evan in the spec);
the valence-4 corner door is the only genuine design content and
is isolated into ARMS-3 as a design conversation. ARMS-1
difficulty L logged pre-draw; consumes block VERBS-2 slot 3; lane
verbs-arms1 next.

## SSIFLAT up (#931); ordinal-63 cross-model dual out (2026-08-22)

SSIFLAT complete at 2041b083: the diagnostic payload reuses
MarginDiag itself (None / Some(Enclosure{lo,hi}) / Some(Invalid) —
poison finally distinguishable from width), pcurve_cache.rs proven
the only lossy projection (blind spot honestly stated), the #925
row re-scoped as a TERMINAL sliver (degenerate enclosure wholly
inside the open band — not refinable, D4 ¶3), try_build split
keeps f64 rows untouched, plant-verified. The lane self-reported a
process error (checkout -- destroyed its own uncommitted fix
mid-plant; caught via diff --stat, re-applied, and the later plant
done commit-first) — the right order is now in its report for the
record. Hosted draw missed 1e-12 for the third consecutive unit
(interval lane pinned, ε drew default each time — an observation
for the sampling regime's owner if it persists). **Ordinal 63
claimed → a third → DUAL, CROSS-MODEL** (R1 fable + R2 opus,
frozen 2041b083, lanes verbs-ssiflat-r1/-r2); the 1e-12
unique-signal run is in both charters. Six agents live (4 impl/
review + RING + DEMO + ARMS-1 pending reports).

## Ordinal-63 dual returned; SSIFLAT fix pass out (2026-08-22)

R1 fable APPROVE 0/2/1; R2 opus AWF 3/4/3 — the pair's label
divergence comes with a direct judgment contradiction worth the
variance record: R1 rated idiom 5/5 for "reusing the classifier's
vocabulary" while R2 proved the enum already held a LOSSLESS
sibling variant (Escalated{cause: Indeterminate}) and a
zero-consumer purpose-built renderer (IndeterminatePayload) that
the PR bypassed for a third hand-rolled form dropping the band.
Both converged on the definite-refusal arms' Value-projection
dishonesty; R2's probe reached a surviving manufactured-NaN
instance (the empty tube ladder's structural CertificateLimb) from
a public door; both found the row's ε-regime fragility by
different mechanisms (1e-13 red via MapResidual; ≤1.8e-13 definite
via CertificateLimb). R1's full-suite 1e-12 runs established
(interval, 1e-12) CLEAN pending this unit's merge. Fix pass
dispatched with the 8-item union incl. one class issue for the
bare-f64 margin-field family both reviewers enumerated. ARMS-1
landed as PR #932 meanwhile (CI running, report pending).

## ARMS-1 report in; ordinal-64 review out (2026-08-22)

PR #932 green (second run — the first tripped clippy/rustdoc on an
exhaustive-match consumer, fixed at cause). The report's headlines:
the annulus band mints via six Euler moves at the seam azimuth
(census 4/8/4 → 5/10/5, χ = 0, volume_pad exactly 0.0 because a
SEAM split keeps both supports iso-rectangles); open/N-link paths
bit-identical via structural RimShape dispatch + verbatim lifts;
the wrap-around G1 reuses the existing predicate names; the torus
net landed in tier-3 check 1 with a planted horn red. Four
disclosed deviations, the substantive one being deviation 3:
plane_sphere_blend silently assumed the POCKET configuration and
the dome exposed it — fixed by reading the material side from the
sphere face's stored sense bit (R ∓ r fold, pocket branch
bit-identical). Banked: every full-revolve wall is a 4-half-edge
cycle with empty face.rings (ARMS-2's consumers will all be this
shape); the same silent-side question must be asked of every arm
ARMS-2 adds. **Ordinal 64 claimed** — not a third — single fable
review at frozen f8f97d8e, lane verbs-arms1-r1, probe branch
verbs/arms1-r1-probes. RING's PR #933 opened meanwhile (CI
running, report pending); SSIFLAT fix pass still working.

## RING report in; ordinal-65 review out (2026-08-22/23 boundary)

PR #933 green at b4f5c264. Headlines: the door's evidence-typed
contract (Probed | Carried{sign}; the door never derives
containment — #750 fenced out by design); the WINDING INVERSION
discovery (holes traverse forward while classify_loop keeps
reverse=true — the seam-surgery docs never mentioned winding, and
feeding holes through unmodified would mint every cavity wall
inside-out); the imagined same-body zip is IMPOSSIBLE (the hole's
revolved band is a disconnected component — landing it as an
interior shell IS the door's job, vindicating the ratified
architecture); its first CI draw hit (interval,1e-12) and was
handled per the enumerated-red process. DESIGN doc-syncs called
out for Evan in the PR. **Ordinal 65 claimed** — not a third —
single fable review at frozen b4f5c264, lane verbs-ring-r1.
Three reviews/fix-passes now in flight (64 ARMS-1, 65 RING,
SSIFLAT fix pass); DEMO part 2 unlocks at RING's merge.

## SSIFLAT MERGED (#931, 2026-08-23) — #925 CLOSED; (interval, 1e-12) CLEAN

Row SSIFLAT (ordinal 63, sample #20, the EIGHTH cross-model pair)
in MODEL-AB-LOG. The fix pass's rework made IndeterminatePayload's
first consumer real; both structural NaN mints fixed at source;
#934 carries the class. R1's full-suite runs establish the
(interval, 1e-12) point CLEAN on main after this merge — the #921
saga that began with the chamfer's unlucky draw is fully resolved.
Process findings recorded: adopted reviewer probes import their
own gate debt; and the FOURTH consecutive default-ε draw on
1e-12-subject units is a sampling-regime gap (reported to #915's
thread with the suggested ε-pin analog of _forces_interval).
Seam sweep: i921 lane. In flight: RING review (65), ARMS-1 fix
pass (#935 filed as its scheduled follow-up).

## ARMS-1 MERGED (#932, 2026-08-23) — the #554 verb-level unlock LANDS

**fillet_edges works on full solids of revolution** (annular
profiles — the bound stated everywhere it matters). #889 closed.
Row ARMS1 (ordinal 64) in MODEL-AB-LOG. The review proved
bit-identity with a byte-faithful dumper and confirmed the
sense-bit configuration fix as the PR's strongest part; the one
MAJOR (shared-wall double-rim staleness) closed as an honest
upfront gate with the capability scheduled as #935. Seam sweep:
verbs-arms1 + verbs-arms1-r1 lanes. Remaining in flight: RING
review (ordinal 65) — DEMO part 2 unlocks at its merge; then
ARMS-2 (the coaxial arms, now with their consumer shape known:
every full-revolve wall is a 4-half-edge cycle with empty rings).

## VERBS-RING implemented (2026-08-22, lane verbs-ring)

The door: `topo::boolean::voids::insert_void` — evidence check
(typed refusals: absent / non-strict / foreign, before any
mutation), revert, graft; the subtract fallback's cavity arm now
routes through it with its own probe verdicts as evidence
(move-and-expose; same call order, bit-identical results — no
boolean test edited, all green). The retirement: a holed full
revolve builds `revolve(outer) − revolve(hole-as-outer)` — holes
traverse FORWARD (the stored CW chain IS the reversed hole-as-outer
chain) and classify under the reversal flag so material lands on
the cavity side; evidence carried from the profile's hole role
(`Carried { Positive }`), no 3-D test. Degenerate-arm pin: the ring
build's verdict log contains no `bool_`-prefixed predicate
(RED-able), and the annulus fixture's torus walls could not enter
the crossing pipeline at all. Door guards: the tier-1 postcondition
declared with the whole-cavity transplant delta; pcurve posture
`Transfers`. First-exerciser findings on the "mechanical" seam
surgery claim: (1) the winding inversion (hole-as-outer needs the
sense derivation pointed at the OTHER side) is nowhere in the docs;
(2) the imagined same-body per-hole zip cannot exist — the hole's
band is a disconnected component, and landing it is cavity
bookkeeping, i.e. the door itself. klein wall 6 re-baselined (ring
builds; probe pins the O6 STEP standing gate
`CurvedShellClassification`); KERNEL-VERBS row, README, DESIGN F8
bullet synced. PR: verbs/ring.

## Ordinal collisions with ASM, resolved on-thread (2026-08-23)

ASM's #939 (ASM-DEMO=63) and #952 (TESS-SPLIT=64) both collided
with rows already recorded on main (SSIFLAT=63, ARMS1=64 at #940's
merge da20a031, which landed before either claim was current).
Resolution posted on #952 per the #398 dispatch-order precedent:
VERBS keeps 63/64/65; ASM-DEMO → 66 (still a dual third — sample
#21, the NINTH pair; SSIFLAT stays the eighth), TESS-SPLIT → 67
(single). Contributing cause on my side was the batched
state-sync; the sync-at-merge discipline (adopted at #939) closes
the window. RING review returned APPROVE 0-MAJOR meanwhile; light
fix pass (probe adoption + 3 polish items) in flight.

## RING MERGED (#933, 2026-08-23) — WAVE 1'S VERB ROWS COMPLETE

Row RING (ordinal 65) in MODEL-AB-LOG (synced at merge per the new
discipline). The one-call hollow ring exists; the void-insertion
door is factored and waiting for Wave 3's shell; its fix-pass CI
draw delivered the first hosted green at (interval, 1e-12).
Wave 1 state: RIM ✓ CHAMFER ✓ ARMS-1 ✓ RING ✓ — remaining:
TUBEWALL (S), ARMS-2 (next dispatch), ARMS-3 (Evan-gated
conversation), plus the DEMO unit (part 2 now UNBLOCKED). Seam
sweep: verbs-ring + verbs-ring-r1 lanes.

## ARMS-2 + TUBEWALL dispatched; DEMO part 2 running (2026-08-23)

Three lanes live: ARMS-2 (block VERBS-2 slot 4; the coaxial arms +
the surgery widening + the C8 prose correction, addendum on main
via #957), TUBEWALL (block VERBS-3 slot 1 drawn, difficulty S
pre-draw; brief-as-spec — the hollow tube door with a STOP clause
if the elaboration turns out to be a genuine fork), and DEMO part 2
(the translucent hollow ring, triggered at RING's merge). After
these: ARMS-3's design conversation (Evan-gated), then Wave 1 is
CLOSED and Wave 2 (curved booleans) opens.

## DEMO complete (#958); ordinal-68 review out (2026-08-23)

Both montage scenes landed: the chamfered die beside its filleted
twin (deficit-volume assertion proves the chamfer never reaches a
pip; the r==d bit-equality recorded as observation, not contract)
and the one-call hollow ring rendered translucent per the
loop-tube precedent (hollowness pinned structurally AND
numerically). The STEP frontier became a self-retiring tour door
(step_at_frontier: a different refusal OR a success fails the
tour); the freecad lane skips-and-names rather than substituting
kernel STL as false OCC evidence — the blank cell flagged for
adjudication. Six friction findings recorded, two load-bearing for
future units: the recipe-node gap measured from BOTH sides (no
Node::chamfer; Node::Revolve's ProfileProgram cannot carry a
hole), and Revolved::cavities returns a ShellKey no door can
spend. **Ordinal 68 claimed** (66/67 are ASM's per the corrected
sequence) — not a third — single fable review at frozen 43a3bd58.

## TUBEWALL report in; ordinal-69 dual out (2026-08-23)

PR #960 green at (interval, 1e-6). The flagged deviation
ADJUDICATED ACCEPTED: the wall-validity check is metered
(tube_wall / tube_wall_bore, linear margins pre-mint) rather than
the briefed bracket read — the compound-Bounds seam rule genuinely
blocks the plain spelling outside allowlisted seams (the
discipline gate proved it on the first push), and the metered form
refuses slivers and escalates in-band where the read would build
them. Banked finding worth a house ruling: NO non-metered
request-validity spelling exists for new doors outside the seams —
filed at the merge seam. **Ordinal 69 claimed → a third → DUAL
CROSS-MODEL** (sample #22, the TENTH pair; R1 fable + R2 opus,
frozen 4de92637; the r2 lane was local-cloned from r1 after two
network clone timeouts — hooks + origin repointed by hand).
ARMS-2's PR #962 opened meanwhile, green at a drawn
(interval, 1e-12); its report pending, review (ordinal 70) queues
on it. DEMO review (68) still running.

## ARMS-2 report in; ordinal-70 review out (2026-08-23)

The coaxial arms landed as ONE closed-form family: the ball's
centre confined to a symmetry sheet, three sheet-crossing closed
forms (line×line, line×circle, circle×circle), eight per-pair
reductions — no arm mints a cone, the C8 correction landed as
flagged. The material-side lesson institutionalized (defining-
equation tests over all 8 arms × 4 sense combinations; the fold
checked live). #319's coaxial half CLOSES on the bud's actual
mouth rim; klein walls 1/2 re-pin to RadiusHeadroom (the arm now
exists; the bulb's RF meets predicate 1 — sharpening the
meridian-authoring reading). One new metered predicate
(fillet3_support_coaxiality) and one new tangent-certificate cone
row. Deviations scheduled incl. #961. **Both hosted draws were
interval-lane — the f64 lane never gated this PR**; the ordinal-70
reviewer's charter makes the default-features matrix its explicit
unique-signal run. Frozen fe20d633, lane verbs-arms2-r1
(local-cloned; network still flaky).

## DEMO MERGED (#958, 2026-08-23) — the montage shows the new verbs

Row DEMO (ordinal 68) in MODEL-AB-LOG. The chamfered die pairs with
the filleted one; the hollow ring reads translucent with its bore
silhouette; the STEP frontier is a self-retiring declared gate with
a stamped montage cell; and the strongest outcome is
epistemological — the review EXECUTED a recorded finding, refuted
it, and the fix pass inverted it into a standing cross-door
assertion (the recipe ring and the direct ring agree bit-exactly,
checked every pass). Renders re-baseline on this merge. Seam
sweep: verbs-demo + verbs-demo-r1 lanes. Still in flight:
TUBEWALL dual (69), ARMS-2 review (70).
## VERBS-TUBEWALL implemented (2026-08-23, lane verbs/tubewall)

The door: `tube_along_arc_hollow(center, axis, u_ref, major_radius,
window, minor_radius, wall, tol)` — a SIBLING, not a widened
signature, so the solid door keeps its signature and its suite is
untouched. BOTH doors take `T: Decide` — no bracket read anywhere on
the path, see the wall-validation paragraph below. Both doors are one
private `build` with `wall: Option<T>`; the hand-written
`swept_segments` involution came out as `circle_traversal(center,
radius, turn, reversed)` with the three combinations the doors use
named in its docs. That is the whole elaboration: the hollow form is
one MORE loop through the same revolve machinery — the outer circle
as before, the inner circle as the revolve's hole loop, in the
traversal each construction expects (forward/clockwise for a full
period, where the hole builds as its own hole-as-outer solid;
reversed/counterclockwise for a window, where it is an ordinary ring
in the start cap). No new geometry code and no second construction.

Full-period policy, mirrored from the solid door honestly: the solid
door supports `TubeWindow::Full`, so the hollow one does, and its
inner wall closes into a CAVITY — `build_full`'s existing holed path
inserts it through `topo::insert_void` with `Carried { Positive }`,
the VERBS-RING route unchanged. The evidence that carries: the two
circles are concentric and the door has already decided the
thickness, the bore AND the realized gap between the two stored radii
definitely positive, so the inner circle is strictly inside the outer
in the sketch and revolution about the shared axis maps that to 3-D
verbatim. Both `full.rs`'s and `voids.rs`'s Carried-evidence prose
gained this third source, which was previously written as if a
validated profile were the only one. A window is an ordinary open
elbow of annular section — one shell, two annular wedge caps.

Wall validation went the other way from the brief's suggested
posture, and the reason is worth recording. The brief asked for a
plain bracket-read check (the chamfer `NonpositiveSize` precedent);
that spelling needs `T: Decide + Bounds`, which the ratified
compound-`Bounds` scope rule allows only in named seams
(`scripts/gates/bounds-allowlist.sh` — the discipline job caught it
on the first push, correctly). Rather than ratify a new seam for an S
unit, the checks went through the door's OWN funnel: plain LINEAR
margins in meters (unlike this door's levered angular window/frame
margins), the posture this door already uses for its caller-supplied
window (`tube_window_span`).

**The merit claim, stated correctly after the ordinal-69 dual — the
first version of this paragraph was wrong by omission and is
corrected here rather than left standing.** The two regimes come
apart and each spelling wins one:

- **ε-scale walls** (R1's fixtures): metering is better. A 1e-20 m
  wall is not positive at any run tolerance, so the metered form
  refuses it and escalates an in-band one, where a bracket read
  accepts both and builds a sliver.
- **The collapse regime** (R2's finding, the one R1's fixtures never
  reached): the bracket read was better, and the metering rewrite
  SILENTLY DELETED the guard that covered it. The branch's own first
  commit (c56c77d5) had `WallBelowResolution`, a check on the
  realized separation of the two stored radii; the rewrite dropped it
  along with the bracket reads it was written in. At large radii —
  measured, 218 configurations at ε=1e-12 from `minor_radius` ≈
  5.24e5 m up — a thickness far above ε still falls under that
  radius's own ulp, both surviving decides answer Positive, and
  `minor_radius - wall` rounds onto `minor_radius`. Nothing built,
  but the refusals came from the pcurve mint and the cap-plane Newell
  fit AFTER everything was classified, i.e. by luck rather than by a
  wall door — and the cavity's `Carried { Positive }` was by then a
  FALSE certificate over two coincident circles.

The fix pass covers both with a THIRD decide in the accepted metered
posture: `tube_wall_gap` on `Margin::of(minor_radius - inner)` — the
difference of the two numbers the walls will store, not of the two
the caller wrote. Three arms: `NonpositiveWall`, `WallExceedsRadius`,
`WallGapCollapsed`, each carrying the run's threshold (not the
caller's value: with `T: Decide` alone there is no f64 door out of a
`T`, which is the same seam rule again). The lesson worth keeping:
a rewrite that changes the *spelling* of a check can silently drop a
*case* the old spelling covered, and neither the suite nor the gate
said so — only a reviewer whose fixtures reached the other regime.

Exactness posture: outer wall bit-identical to the solid door's;
inner wall stores `minor_radius - wall`, one IEEE subtraction of the
caller's own numbers, pinned with `==` on the bits. Refusal messages
now name the door honestly: a hollow-only predicate escalation says
`tube_along_arc_hollow`, and the arms both doors share say `tube
door` rather than claiming the solid one. KERNEL-VERBS register row
retired to the present (bound stated: one concentric constant wall,
nothing eccentric or varying; the STEP claim SOFTENED to "expected" —
nothing runs the hollow tube through the writer today, and the tour
scene that would pin it is issue #986). north-star row 19 widened.
Both review probe branches adopted whole (r1 + r2), with the two
record-current-behavior rows amended rather than deleted: one now
pins the corrected door naming, the other inverts to require that
every collapsed bore is named by a wall door. PR: verbs/tubewall
(#960).

## Outage and resume (2026-08-23 ~09:00Z → 2026-08-25)

The session hit the model usage limit mid-wave: both in-flight
reviewers (TUBEWALL R1, ARMS-2 R1) terminated mid-review and the
orchestrator loop froze ~2 days. On resume: #960/#962 still open
and untouched, reviewer lanes intact, main moved only with other
programs' work. Both reviewers RESUMED from transcript (their
context held unreported findings — resume over fresh per the
death-recovery rule): TUBEWALL R1 had verified C1's bit-identity
half and owes the interval run + report; ARMS-2 R1 was mid-anomaly
(base probe rows passing unexpectedly — told to suspect a stale
target serving head binaries before trusting any base run).
TUBEWALL R2's report arrived BEFORE the outage and is severe:
AWF 3 MAJOR — the metering rewrite silently DELETED the
WallBelowResolution guard its own first commit had (M1: the
realized-gap collapse class, 218 configurations measured at
1e-12, never refused by a wall door — and the Carried{Positive}
evidence is false for them); the PR's merit sentence is backwards
(M2); the ratified log entry contradicts the code and the gate
(M3). Adjudication of the 69 union waits on R1.

## Ordinal-69 dual complete; TUBEWALL fix pass out (2026-08-25)

R1 fable AWF 0/3/3 (resumed post-outage; upheld the metering
deviation both halves, verified at ε-scale walls; found the
VERBS-LOG Decide+Bounds contradiction independently). R2 opus AWF
3/4/7+ (pre-outage): the complementary regime — the realized gap
`minor − wall` is never decided, so at km-scale radii the
subtraction rounds the bore onto the outer wall and the refusal
comes from downstream certification by luck, with the
Carried{Positive} evidence FALSE for that class; the branch's own
first commit had the guard and the metering rewrite silently
deleted it (the one silent deviation, and exactly what the
dispatch asked adjudicated). Labels converged, substance
PARTITIONED BY REGIME — each reviewer's fixtures covered what the
other's missed, the dual earning its cost cleanly. Union fix
dispatched: the third decide (tube_wall_gap), the backwards merit
sentence corrected, the log contradiction fixed, both probe
suites adopted, main re-merged (the outage delta). Also noted:
classify_shells on main answers DEMO finding 6 and the RING
winding-oracle caveat — a future unit's wiring, not this PR's.

## Ordinal-70 returned; ARMS-2 fix pass out (2026-08-25)

The resumed reviewer delivered: AWF 1/4/4 + style. The geometry
held completely — all three sheet-center closed forms re-derived
(the sign-carrying stable quadratic root confirmed branch-free
both directions), four-fixture bit-identity byte-verified, klein's
RadiusHeadroom margin computed exactly (−0.1 m against the inner
neck wall — the meridian-authoring reading sharpened, as claimed),
the C8 correction verified against #930's flagged wording, and the
f64 lane (which BOTH hosted draws missed) clean at 63/63 × two ε.
The pre-outage anomaly resolved as suspected: stale-binary base
runs; redone isolated, the probe rows red at base. The MAJOR is a
vacuous test row (the non-coaxial refusal test never builds a
non-coaxial pair); MINORs: the refusing branch has zero consumers
and the new metered name zero probe coverage (K-REPORT's 0-sampled
class), one unscheduled deviation, a missing-face conflation, and
ruled-family poison prose. Fix pass out with the union; the
reviewer's two-sphere-waist differential likely BECOMES the MAJOR
fix at adoption. Both Wave-1 closers now in fix passes
concurrently (TUBEWALL + ARMS-2).

## ARMS-2 — the coaxial revolution arms (2026-08-23, `verbs/arms2`)

**#319's coaxial half closes.** Eight new arms land as ONE derivation
plus eight reductions. Whenever a support pair carries a symmetry the
rolling ball inherits — a common axis of revolution, or a common
ruling — the ball centre is confined to a SHEET (the meridian
half-plane through the rim; the cross-section normal to the ruling).
Both supports cut that sheet in a line or a circle, and the centre is
the crossing of the two OFFSET traces: three closed forms
(line×line, line×circle, circle×circle), each written so the branch it
takes is the one that returns the RIM as `r → 0` — the structural
answer to "which of the two circles the offsets meet in is my edge",
and branch-free (the `√` carries the sign of a stored quantity, and
poisons exactly at a tangential pair).

Coaxial six (circular spine → TORUS): sphere×cone, cone×plane(⊥),
cone×cone, cylinder×cone, cylinder×sphere, cylinder×plane(⊥). Ruled
two (straight spine → CYLINDER): cylinder×cylinder(∥),
cylinder×plane(∥). No arm mints a cone — the C8 prose correction
(`docs/CURVED-DESIGN.md`, flagged on #930) lands with them.

The surgery half was real scope: `resolve_rim`'s gates are now split
by SHAPE rather than by KIND — the annulus asks only that both
supports be revolution walls, the ladder keeps every ring-and-half-cap
gate it had. One new routing decision, `fillet3_support_coaxiality`
(the departure from the shared-axis hypothesis, meters at the rim's
own lever arm), refuses `SpineUnsupported` on a definite miss. The
tangent certificate's circle arm grew a CONE row in `geom-brep`
(without it a sphere×cone band cannot be described at rest).

Acceptance: the calochortus bud's MOUTH RIM alone — a sphere zone
meeting a conical pucker along a closed latitude circle — fillets to a
tier-3-valid solid with a pinned census and closed-form trim circles;
its lip (cone×plane) and its bore's base (cylinder×plane) fillet too.
ARMS-3 keeps sphere×sphere and the valence-4 corner run-out.

**R1 fix pass** (APPROVE-WITH-FIXES, 1 MAJOR / 4 MINOR / 4 NOTE): the
geometry held — all three sheet-centre forms re-derived independently,
the four-fixture bit-identity byte-verified across the merge base, the
klein `RadiusHeadroom` margin confirmed at exactly −0.1 m, the C8
correction checked against the ratified wording, and the f64 lane the
hosted draws missed clean at 63/63 on both ε. The MAJOR was a VACUOUS
row: the "non-coaxial refuses" test filleted the coaxial mouth and
grepped a roster string. It is replaced by a PLANTED construction — the
bud's own cone wall keeps its apex and half-angle and takes a tilted
axis, and the battery must refuse naming the shared-axis hypothesis
rather than the kinds — which is also `fillet3_support_coaxiality`'s
first test consumer. That name now has probe-lane coverage too: a
curved-rim fillet joins the K sweep as `budrim`, the CHAMFER `spacer`
precedent one verb over, and the only scene in the corpus that reaches a
CURVED support pair. The uncarved ruled arms got their schedule (#987,
behind ARMS-3 — a ruling's terminations ARE the run-out taxonomy), the
absent-support path got `BodyNotIntact` back, the poison prose got the
ruled family's actual path, `tangent.rs`'s twice-spelled √g ladder
folded into one helper, and the plane/sphere vocabulary swept at every
site the review named (prose only; the name-alphabet mechanism stays
#961's). Reviewer probe branch adopted authorship-preserving:
`verbs/arms2-r1-probes`.

## ARMS-2 MERGED (#962, 2026-08-25) — #319's coaxial half CLOSED

Row ARMS2 (ordinal 70) in MODEL-AB-LOG. Curved-support fillets
exist: the bud's mouth rim (sphere×cone), lip (cone×plane), and
bore base (cylinder×plane) all fillet end to end through one
closed-form family. Block VERBS-2 fully consumed. Remaining for
Wave 1: the TUBEWALL fix pass (CI in flight) and ARMS-3's design
conversation. Seam sweep: verbs-arms2 + verbs-arms2-r1 lanes after
the state-sync.

## WAVE 3 OPENS: OFF-A spec committed (2026-08-25)

Evan's rulings at the seam: Wave 3 stays under THIS orchestrator
(the context argument — the survey, both ratification rounds, the
door seam — beat the handoff; a remote orchestrator's build
parallelism noted as the one advantage forgone, with
remote-implementer dispatch as the adaptation if wall-clock ever
binds). The build mutex stays width 1 — this box measured at
9GB/8-core, the same envelope the width was measured for. The Q8
substrate survey is now durable (docs/Q8-SUBSTRATE-2026-08-21.md,
snapshot-caveated). OFF-A spec committed (the O1 mint table +
door-owned refusals; the TUBEWALL realized-radius lesson and the
ARMS-2 never-meter-a-non-question lesson both folded in);
difficulty S logged pre-dispatch; consumes block VERBS-3 slot 2.
Interleave plan: OFF-A + Wave 2's GATE spec next, OFF-B (meters +
fit + certificate, L) after OFF-A lands, ARMS-3's conversation
draft at the TUBEWALL merge seam.

## TUBEWALL MERGED (#960, 2026-08-25) — WAVE 1 IMPLEMENTATION COMPLETE

Row TUBEWALL (ordinal 69, sample #22, the TENTH cross-model pair —
two from the twelve-pair target) in MODEL-AB-LOG. The hollow tube
door lands with the three-decide wall family; the seam-rule
posture gap met twice in the unit is filed as #990 (a design
question for Evan); the tour scene is #986. **Every implementation
row of Wave 1 is now MERGED**: RIM, CHAMFER, ARMS-1, ARMS-2,
TUBEWALL, RING, DEMO (plus the unplanned ARCEVAL/SSIFLAT defect
units the wave surfaced). Remaining Wave-1 item: ARMS-3's
design conversation (OQ6 run-out taxonomy — Evan-gated), drafting
next. Wave 3 opened concurrently (OFF-A implementing). Seam
sweep: verbs-tubewall + both reviewer lanes.

## ARMS-3 conversation OPEN (#992, 2026-08-25)

The last Wave-1 item is now a design conversation awaiting Evan:
A3-2's substantive claim is that the valence-4 seam-vertex
"corner" is NOT a corner (the surface is smooth through it; the
shipped vocabulary misdescribes it) — recommend the SeamVertex
refusal with the request-the-full-rim recourse, machinery-free;
A3-3 parks the genuine mid-curve run-out pair consumer-gated with
the ball-cap named presumptive. Board: OFF-A implementing (Wave
3); #992 with Evan; Wave 2's GATE spec is the next orchestrator
work item.

## WAVE 2 OPENS: GATE spec committed (2026-08-25)

docs/VERBS-GATE-SPEC.md: the operand gate goes pair-scoped with
box-level conservatism as the ruled "genuinely intersects" (over-
approximation refuses in the safe direction; the payload names the
pair, stated as a may-intersect); #862's two box defects and
#700's sibling dedup ride as the precision the gate rests on.
Acceptance: klein wall 3 flips (or re-pins honestly — build, don't
assume), wall 4 stays pair-scoped-refused, lily wall 7's refusal
becomes true (its retirement still waits on SPHSPH per Evan's
steering). Difficulty M logged pre-dispatch; consumes VERBS-3
slot 3. Two lanes now: OFF-A (Wave 3) + GATE (Wave 2).

## OFF-A up (#994); ordinal 73 claimed at dispatch (2026-08-25)

OFF-A complete at first pass, no deviations: the cone-slide sign
derived from the stored normal's axial coefficient (no numeric
branch), and the cone refusal DERIVED TO BE A NON-QUESTION and
dropped with the argument (nothing stored approaches a validity
edge — the parameterization-shift question belongs to OFF-C/D's
windowed consumers). The TUBEWALL realized-radius lesson is a
planted red at radius 1e16. **Ordinal 73 claimed at dispatch**
(ledger through 72 = M9-3's dual on main at claim; 73 not a third)
— single fable review, frozen 11d955f1ecd8f48e9f07b95f397e5daf354e77b8.

## OFF-A MERGED (#994, 2026-08-25) — Wave 3's substrate begins

Row OFFA (ordinal 73) in MODEL-AB-LOG. The analytic offset mint
exists with door-owned refusals and echo payloads; the apex-window
predicate is scheduled into OFF-C/D (plan + OFFSET-DESIGN
annotated). Next: the OFF-B spec — the two meters (certified
lower bound on ‖S_u×S_v‖; d vs 1/κ_max collapse) + the A9.4/A9.10
fit + the two-limb certificate — Wave 3's hardest unit. GATE
still implementing.

## OFF-B spec committed (2026-08-25)

The program's hardest unit: the two meters (the tree's first
inf-side surface bound — #528's shared shape named; the collapse
meter as radius-headroom one dimension up), the Book's A9.4/A9.10
fit stack in-house, and the C8 two-limb certificate with the
regularity floor making the normalized normal boundable. Machinery
only — Surface::Approx and all storage/validator wiring stay
OFF-C's. The analytic oracle (cylinder/sphere as exact rational
NURBS vs OFF-A's closed forms) is the acceptance spine. Difficulty
L logged pre-dispatch; consumes block VERBS-3 slot 4.

## GATE up (#1001); ordinal 74 claimed at dispatch (2026-08-25)

The pair-scoped gate as built: one pair scan reading the same
boxes candidate generation does; CurvedPairUnsupported payloads
honest about box conservatism ("MAY, not DOES"); cone and torus
ACQUIRED boxes (no NoSoundBox arm exists — a poison box would have
made the re-scope inert); #862's two defects fixed plus a third of
the same class found by shape-sweep, all extents rewritten against
one Span interval type — which IS #700's dedup (option 1, no
allowlist amendment) with a face-for-face differential row
guarding the residue. Two empirical corrections: klein wall 3
still refuses — honestly, on (Cone, Plane) — and lily wall 7's
blocker adds the cone germ lane (plan row 6 corrected; the
steering's intent stands, the blocker set grew). One M9-owned test
row moved with the sweep's documented one-way divergence (the
carrier-graze candidates the over-width kept alive) — courtesy
note owed to M9. **Ordinal 74 claimed at dispatch** (ledger
through 73 on main; not a third) — single fable review, frozen
b2a8bad1.

## OFF-B up (#1003); ordinal 75 claimed at dispatch (2026-08-25)

The program's hardest unit landed green: the three-bound
regularity floor (mignitude / fixed-projection / Gram determinant,
conservatism direction stated and pinned), the collapse meter, the
A9.4 fit with its ONE forced deviation (the Book's chord-length
parameters would leave no pointwise claim to certify — the
ratified O3 claim forced the chart's own parameters), the
insert-and-recertify refinement (A9.10's shape; the compression
half scheduled), and the two-limb certificate whose sup limb rides
ALGEBRAIC RATIONALIZATION (X = Ẽ·Ẽ − d²w², Y = Ẽ×M̃ — coefficients
cancel to the residual's scale where separate hulls would need
millions of cells). Oracle: bound/sample 3-5x across five rows.
The mesh hull assembly LIFTED (not called — layering) with mesh's
numbers unmoved 91/91. Both hosted draws were interval —
the default compile mode is the dual's unique signal. **Ordinal 75
claimed at dispatch → a third → DUAL CROSS-MODEL (sample #24, the
ELEVENTH pair — one from the twelve-pair notification threshold)**:
R1 fable + R2 opus, frozen db2580f9.

## Ordinal-75 dual returned; OFF-B fix pass out (2026-08-26)

R1 fable AWF 1/4/2; R2 opus AWF 4/9/6 — **the strongest
convergence of the program**: both independently demonstrated the
SAME blocker (certify_offset's unweighted read of a rational fit —
an unsound certificate through a public door, 230×/~1800×
under-reports, each with their own red probe), both brute-forced
the core inequality sound (1M + 200k configurations, zero
violations), both judged the A9.4 fork airtight, both found the
stale oracle row and the sweep misreport. R2's divergent tail was
real and landed: the regularity lever DIRECTIONALLY INVERTED
(large |d| permissive where it is the dangerous side) and the
small-|d| certificate wall (relative accuracy ~1/|d|). Fix pass
dispatched with the union, incl. the sharper-denominator
investigation ((‖E‖+|d|) in place of 2|d|) that may dissolve M4
outright. This dual is sample #24, the ELEVENTH cross-model pair
— **the next dual is the twelfth: its recorder notifies Evan
explicitly per the pre-registered target**.

## OFF-B MERGED (#1003, 2026-08-26) — the approximating substrate EXISTS

Row OFFB (ordinal 75, sample #24, the ELEVENTH cross-model pair)
in MODEL-AB-LOG. The kernel can now fit a certified offset of a
NURBS surface: two meters, the Book's fit, the rationalized
two-limb certificate — with the fix pass exceeding the adjudicated
union (M3's d-free lever; M4's sharper denominator tightening
every bound). Four scheduled follow-ons: #1005 (weighted
composite), #1006 (three-spellings consolidation), #1007
(directional refinement), #1008 (net recentring). **The next dual
is the TWELFTH cross-model pair — its recorder notifies Evan
explicitly.** OFF-C (Surface::Approx integration) is the next
Wave-3 spec; GATE's ordinal-74 review still out. Seam sweep:
verbs-offb + both reviewer lanes.

## OFF-C spec committed; block VERBS-4 drawn (2026-08-26)

docs/VERBS-OFFC-SPEC.md: the seventh Surface variant with the D3
total-enumeration discipline (the compiler is the sweep), the
private-certificate ApproxSurface triple, O5's never-trust
re-derivation at tier 3, and a deliberate scope dissolution — the
apex-window predicate re-points at OFF-D (Offset{base} is
NURBS-only here; analytic bases never need Approx). Difficulty L
logged pre-draw; block VERBS-4 drawn (mapping lane-private).

## Ordinal-74 returned; GATE fix pass out (2026-08-26)

AWF 2/5+ — the box rewrite sound in every attacked direction, both
re-baselines honest (the cross-program m9_3 one judged legitimate
with the tangency geometry re-derived), but two real MAJORs: the
gate's admissions DIE DOWNSTREAM in containment doors with a false
CorruptFace diagnosis (the honest refusal traded for a corruption
claim on a healthy body; the spec's union acceptance silently
narrowed to reduce-depth — the unit's one silent deviation), and
**the wall-7 re-steering was a BOX ARTIFACT**: the reviewer
measured the pucker's frustum clearing the ball by 0.291 (1.8
radii) — the (Cone, Sphere) pair came from the cone slab's
full-range max radius. The plan correction I propagated at the
claim seam is therefore itself suspect pending the frustum-tight
re-measurement; I correct the records once the fix pass measures.
A lesson for the record: an implementer's empirical finding about
REFUSAL ATTRIBUTION inherits the precision of the instrument that
attributed it — the review's independent derivation is what
caught it. Fix pass out with the union; the point-in-solid
cone/torus capability files as its own unit.

## GATE MERGED (#1001, 2026-08-26) — #862/#700 CLOSED; the wall-7 saga resolved

Row GATE (ordinal 74) in MODEL-AB-LOG. The pair-scoped gate lands
with honest boxes for every kind and the containment boundary
typed honestly (#1011 the scheduled capability). The wall-7 record
went through three states in three days — (waits on 6+9) →
(+ cone lane) → (a full-revolve face-maximality precondition;
never curved-boolean breadth at all) — each transition driven by a
better instrument, and the final one by the fix pass's live
numbers. Plan rows 6/9 corrected; M9's seam prose synced by the
lane; the courtesy note to M9 goes on #1002's thread. Wave 2's
germ lanes (rows 7-10) now dispatch against an honest gate.

## Germ-lane survey folded; CYLCYL spec committed (2026-08-26)

The survey's two premise corrections bind: **M9-3 PR-B is NOT on
main** (#971 open — the zip substrate I believed landed is not
substrate) and M9-3 is the declared-contact lane, not #250's germ
join analog. The germ pipeline mapped door by door (D1-D10):
chords are minted on demand, never stored (the SSI lift's actual
meaning); the shared blockers are D3 (curved point-in-face
containment — which IS #347's conservatism defect, not a rider),
the D5 `_ => Ok(None)` straight-chord trap (latent unsoundness the
moment any arm widens D4), and D10's no-crossings silence for
cylinder pairs (the one wrong-answer-shaped path). CYLCYL specs as
a two-PR unit (A = substrate, B = arms; #347 needs only the
parallel-axis class); the sequence reorders 7 → 9 → 8 with the
rung argument; klein wall 3 re-attributed to row 10. PR-A
difficulty L logged pre-dispatch; consumes block VERBS-4 slot 2.

## ARMS-3 RATIFIED (#992 👍, merged 2026-08-26); implementation dispatched

The run-out taxonomy resolved as drafted: the seam vertex is NOT a
corner (SeamVertex refusal with the request-the-full-rim recourse,
machinery-free); the genuine mid-curve run-out pair parked
consumer-gated (ball-cap presumptive). One implementation unit
dispatched: the general sphere×sphere arm (ARMS-2's circle×circle
closed form), the SeamVertex refusal, register sync; #319 closes
fully at its merge. Difficulty M logged pre-dispatch; block
VERBS-4 slot 3; lane verbs-arms3. Three implementation lanes live
(OFF-C green-awaiting-report, CYLCYL PR-A, ARMS-3).

## OFF-C up (#1012); ordinal 76 claimed at dispatch (2026-08-26)

The seventh Surface variant complete: the triple with the owned-Arc
base (the spec's arena-key default inverted on two concrete
obstructions — layering and self-containment — per the
state-the-choice clause), the certifier INJECTED (the
certify_nurbs_lane posture, RationalFitUnsupported propagating
untouched), the ~40-site enumeration with every split catch-all
named and one latent hole caught (certify.rs's resolve would have
METERED POISON on an Approx operand instead of refusing — a
matches! the compiler could not surface), the validator's
never-trust arm, and the apex-window dissolution executed (the
base is NurbsSurface BY TYPE). The GATE re-scope was reconciled
mid-flight (Approx off both rosters by argued decision; the
refusal now germ-pair-shaped). **Ordinal 76 claimed at dispatch**
(through 75 on main; not a third) — single fable review, frozen
597acdb6. The hosted draws were both interval again; the default
mode is the reviewer's unique signal.

## Correction (2026-08-26): the ARMS-3 dispatch entry above was premature

The lane was created and the log written, but the implementer
launch was skipped when the session interleaved — caught at the
next hourly tick (the branch sat at main's head). Actually
dispatched ~1h later than the entry claims. The tick sweep is what
caught it; recorded per the no-unverified-claims rule.

## Ordinal-76 returned; OFF-C fix pass out (2026-08-26)

AWF 3/3, zero MAJOR — the ~40-site enumeration held under the
reviewer's own differently-shaped sweeps, which found exactly two
latent gate-shielded gaps (reduce.rs's crossing guard and the
Circle-chain guard's unswept sibling). The substantive
adjudication: tier 3 was re-establishing the surface's
SELF-DECLARED tolerance — ruled to the edge posture (re-derive
against the RUN's ε_precision; O3's claim is ≤ ε_precision and D4
blesses the escalate-on-tightening consequence). The reviewer's
empirical sharpening for OFF-D: on curved fits the seam carrier
needs KNOT REFINEMENT, not degree elevation alone (the fit
refines past the seed grid even at d = 5e-10). Fix pass out.

## THE 12-PAIR TARGET IS MET — Evan notified (#1016, 2026-08-26)

The M9 orchestrator (resumed cad-m8 session) determined ordinal
72's dual COMPLETED — both arms attested by #974/#975's own
citations of R1/R2 findings, per-arm figures unrecoverable
(outage-window session loss; recorded attested-but-unscored). So
72 = the eleventh pair and OFF-B/75 = the TWELFTH; my row-75
label corrected with a dated note. The pre-registered explicit
notification to Evan filed as #1016 with the full tally, the
pair-11 caveat, and the disposition recommendation (dual sampling
SUSPENDS pending Evan's readout — the reversible direction; a
missed dual can run late on a frozen head, an extra one cannot be
un-spent). Cross-program credit: cad-m8's sweep caught both the
pair double-booking and the abandoned #971 close-out (which they
have taken back — the GATE re-merge and wall-7 final form handed
over). Also corrected on their side: their "72-claim not on main"
finding withdrew under the ancestry check.

## OFF-C landed: `Surface::Approx` is in the kernel (2026-08-25)

The seventh variant, the `SurfaceDescription`/`SurfaceSpec`/
`ApproxSurface` triple with a private certificate, and the total
enumeration the closed enum forces: every E0004 the compiler raised
plus the wildcard/`matches!` sites it could not, dispositions across
30 files, each stated in the code. Base reference: an
owned `Arc<NurbsSurface<T>>`, not an arena key — `SurfaceKey` lives
a crate above `Surface`, and a `Surface` value is read with no arena
in hand at half the consumers.

Tier 3 re-derives per face (`PropsQuadLane::recertify_approx`), with
`ApproxCertification` / `ApproxLaneUnsupported` as the typed
findings; the NURBS-adjacent dihedral and material-sign exemptions
extend to `Approx` BY KIND, through `Surface::spline_chart`.

**The re-derivation classifies against the RUN's ε, not the
surface's stored tolerance** (r1 NOTE-2, adjudicated). The edge
machinery re-certifies every carrier against the run's band and
never against a stored bound; O3's ratified surface claim is
`sup ‖S_fit − (S + d·n)‖ ≤ ε_precision`, so verifying it means
measuring at the ε this validation call runs at. A surface minted
loose validating forever afterwards would be the mint's parameter
quietly replacing the ratified claim. D4 already blesses the
consequence — ε-tightening may escalate — so a mint that no longer
meets a tighter run ε refuses honestly, which is exactly what the
edge machinery does. The stored tolerance stays the MINT's
parameter and the fit door's own gate.

### What OFF-D inherits (the consumer's findings, sharpened at r1)

**A face-replacement primitive owes its edges the new chart's
SPLINE SPACE, and owes them BOTH operations.** The iso lane's seam
class bounds `|B(v) − C(v)|` by a control-difference hull — a
partition-of-unity argument — so the chart's boundary row and the
carrier must share knots, degree and weights. Degree elevation
alone suffices on a PLANAR wall, and that is what the PR's straight
prism needed; the r1 review's twisted-loft probe showed it is not
enough on a CURVED one, where the fit refines past the seed grid,
so **knot refinement into the fit's own interior knots is required
as well**. Both are exact — same locus, same parameterization, same
endpoints — so this is a representation change, not rim surgery.
The shared surgery in `sweep/tests/common/approx.rs` does both.

Two more, same lane: a face's edges must be re-described after
`FaceSurface::New` (the surgery ordering's own second step, which
tier 3 reports as `DescriptionNotAdjacent` if skipped); and moving
an `Approx`-faced body needs the transform door's mapped-certificate
lane (#1020 — the composition law itself is pinned).

**Scheduled, with boxes:** #1018 mesh + props tolerance widening by
the certificate's bound (deliberately unwired here); #1019 the
tier-3 grid-cost perf box (4.73 s serial for 8 whole-body
validations of a 4-`Approx`-face prism, debug, i7-1065G7 — and the
measurement lesson that a contended ~65 s reading is not a cost
figure); #1020 transform.

**The planar-locus boundary, stated.** Every tier-3-green `Approx`
face this unit builds has an exactly-planar locus, because the
pulled-back base is exact only on a plane. Curved fits ARE
body-reachable — the r1 twisted loft validates tier 3 — but only
marginally: at `d = 2e-9` the edge residual measured 2.3e-9 and
escalated in the ambiguity band, so the curved rows run at
`d = 5e-10`. Genuine curved-offset bodies need the rim surgery
OFF-D owns.

## Cross-program: the filter-skipped-gate class detonates (2026-08-26)

cad-m8's #971 re-merge tripped k-lint on a COMPILE error MINE:
9f5228bd (GATE's fix pass) shipped a type_complexity lint in
lily.rs's frustum_aabb — ungated because the k-lint job was
filter-skipped on the merge runs. The bitter symmetry: GATE's own
banked finding PREDICTED the class ("the k-lint unification draw
hides breakage per row; main can be latently red") and then its
lane shipped the instance. Disposition: cad-m8's alias fix rides
#971 (fastest path; provenance recorded here — my lint, their
fix); they file the class issue citing both instances plus #601
and the #915 regime; all three active VERBS lanes warned with the
enumerated-red protocol (compare, state, don't debug; re-merge at
#971's landing).

## OFF-C MERGED (#1012, 2026-08-26) — Surface::Approx is IN THE KERNEL

Row OFFC (ordinal 76) in MODEL-AB-LOG. The kernel's closed surface
enum has its seventh variant with the never-trust validator arm
classifying against the run's ε. OFF-D inherits a measured list:
the face-replacement primitive owes knot refinement (not elevation
alone), the transform re-derivation lane (#1020), the mesh
widening (#1018), and the tier-3 perf box (#1019). **The teapot's
remaining path: OFF-D (shell + rim surgery, consuming RING's
door) → the demo.** Seam sweep: verbs-offc lane. Meanwhile CYLCYL
PR-A reported (green; the D10 silence caught EXECUTING a wrong
answer — 30π with the lens double-counted, now a typed refusal;
#347's defect (a) re-diagnosed as two OTHER conservatisms, both
single-arm PR-B fixes) — its review claims ordinal 77 next.

## PR-A up (#1021); ordinal 77 claimed at dispatch (2026-08-26)

The germ substrate landed green: D3's containment door REUSING the
solid door's trim resolution (shared pub(super) — cannot drift),
the D5 trap closed FIRST with pair-general signatures, D10 ruled
typed-refusal — and the report's two heavyweight findings: the
D10 silence was caught EXECUTING a wrong answer (a union returning
Ok at exactly 30π with the shared lens double-counted — now
refused), and #347's defect (a) re-diagnosed by measurement as two
DIFFERENT conservatisms (the carrier-slab wall box + the
span-dip's unclamped vertex), both single-arm PR-B fixes with the
red-able row waiting. **Ordinal 77 claimed at dispatch** (through
76 on main; duals SUSPENDED per #1016's in-force recommendation —
single fable review), frozen 845aab0f.

## OFF-D spec committed (2026-08-26) — shell, two PRs

docs/VERBS-OFFD-SPEC.md: PR-1 the face-replacement primitive
(carrying the measured inheritance — knot refinement + elevation,
the coherence budget, the apex-window predicate landing at last);
PR-2 the verb (sealed = the degenerate arm through RING's door
with Carried evidence from the collapse meter's own decides;
opened = rim surgery to a closed thin solid; klein's hand-built
wall pairs begin retiring as the acceptance). The teapot follows
as its own demo unit. PR-1 difficulty M logged pre-dispatch;
dispatches when a lane frees. Also: #971 merged (the lint
known-red retired; lanes updated) and M9's close-out is landing
(#1024 their at-merge row).

## Standing practice (2026-08-26, from the #1023 mis-cite)

A finding with no durable home cannot warn anyone: the GATE lane's
banked prediction of the filter-skipped-gate class lived only in
its report transcript, and the citation broke at first use.
FROM HERE: any banked finding asserting a CLASS gets a durable
home (a log line or an issue) at adjudication, as part of reading
the report. M9-5 confirmed dispatched on cad-m8's side (seam D
only; K2 remains VERBS Wave-2 items 6+9); cross-program threads
all closed.

## #1029: the OFF-C fixture red at 1e-12 (2026-08-26)

The ARMS-3 lane's draw was the FIRST to gate the adopted OFF-C
probe fixtures at (default, 1e-12) — two rows red at the shared
helper's re-attach (IsoResidual, honest: the curved-fit
marginality the ordinal-76 review itself measured), and nextest
fail-fast makes it a PROGRAM-WIDE 1e-12 lane-blocker (~567 rows
unexecuted; same SHA, same draw, no re-roll). The #921/#923
family's fifth member. Dispositions: #1029 filed with the ARMS-3
lane's discovery record; the OFF-C lane fixing on verbs/offc-fix
(the two-arm pattern's fifth application — tests-only, self-merge
class, orchestrator-adjudicated directly); the PR-A reviewer and
the queue carry the comparison instruction. The ARMS-3 lane's
handling was the protocol working end to end: base-run comparison,
ci-local for own coverage, the finding filed not fixed.

## Ordinal-77 returned; PR-A fix pass out; #1029 scope grew (2026-08-26)

The review executed everything: the D10 wrong answer REPRODUCED at
the merge base (30π, Ok), the scoping argument attacked with six
crossing poses and revolve-minted seam walls (held — the ellipse
always crosses a boundary edge), the #347 re-diagnosis re-derived
to the digit (0.9 vs the charged 10), the nested-pair
right-by-luck claim verified both ways. AWF 1/2/3+7: the MAJOR is
the arc-bound's accepting direction having no red-able row — the
one computation that can only WIDEN acceptance; the fix plants
the corruption to prove the new row reds. MINORs: lengths through
Margin::levered (the door-contract violation the codebase
polices); the full-period Err-vs-promised-None mismatch. Fix pass
out. #1029's scope grew to FOUR rows via the ARMS-3 lane's
base-run pin (fail-fast had hidden the second pair); the OFF-C
fix lane updated; the issues consolidated (the lane self-deduped
#1030 before my close reached it).

## Wall 7, corrected a THIRD time — the caps, not the zone (2026-08-26)

M9-5's implementer measured the NonMaximalFaces cause and main's
wall text (GATE's own fix-pass attribution) is FALSE:
gate_maximal_faces rules same-key CURVED adjacency canonical and
`continue`s past the zone's half-bands the text blames; the real
F7 pair is the AXIS-TOUCHING PLANAR CAPS (two half-faces on one
plane key; merge refuses MergedFaceRoleAmbiguous). Verified
verbatim in reduce.rs before accepting. Consequence: **Wave-2
items 6+9 cannot flip wall 7** — the operand is illegal before
any germ arm; the repair question is #1031 (merge learns the cap
pair, or revolve mints maximal caps; the interior-seam chart
question decides). The instrument chain is now four links long
(body kind → box pair → face maximality → the caps), each
correction by a better instrument — the attribution lesson's
third confirmation. M9-5's PR carries the lily text; plan row 6
corrected; #1031 owns the gap.

## ARMS-3 up (#1028); ordinal 78 claimed at dispatch (2026-08-26)

The sphere×sphere arm landed as the table's one
condition-free row (coaxial BY CONSTRUCTION — the axis is the
centre line), the whole implementation one variant + one row; the
snowman self-corrected to a LENTIL consumer (the snowman's valley
is concave — the arm now correctly refuses on convexity, a
better outcome than the fixture assumption); the #319 witness
reproduced (pole-touching profiles split every wall) and the
misdescribing NEdgeVertex{4} became SeamVertex with
policy: None — plus a latent policy disagreement fixed
(surgery hardcoded StopAtVertex for every tag). **A3-2's ratified
recourse premise measured FALSE** (the annulus door cannot carve
the seam-split rim the recourse promises; #1022 is the missing
door) — a dominant-argument correction recorded in the design doc
and flagged for Evan's retroactive review per the self-merge
escalation, the C8-cone pattern. The lane's #1029 handling was
the comparison protocol end to end. **Ordinal 78 claimed at
dispatch** (through 77 on main; duals suspended per #1016) —
single fable review, frozen b7227242.

## Slot courtesy to M9-5; offc-fix PR up (2026-08-26)

Granted cad-m8's bounded ask (one ~40-min main-slot turn for
M9-5's read-before-recut tess chain; the PR-A lane pauses 5 min
so their blocking waiter wins; resume on done-signal or 60-min
timeout) — delivered via the away-channel after the mngr CLI hung
from this worktree (the known host-lock issue). probe.rs truthing
green-lit (ARMS-3 made the same non-registration call). The
offc-fix PR is up (#1035, closing #1029) — adjudicating directly
on its report per the tests-only ruling.

## #1035 MERGED — #1029 CLOSED; the 1e-12 draws are unblocked (2026-08-26)

The offc-fix adjudicated directly (tests-only class): all four
fixture rows two-armed through ONE gate helper; the threshold
proven ε-INDEPENDENT by two routes bit-for-bit at ε three orders
apart (the stop condition explicitly not triggered); PER-SIGN
thresholds after the −D arm went red on the first constant (one
constant would have shipped a silent blind spot); the two
departures from the #921 family argued (ResidualExceeded carries
a verdict not a margin, so the helper measures the classified
quantity and cross-checks its replica against the REAL certifier
both sides). The CI draw landed on (interval, 1e-12) — the fix
gated live at the defect's own point. Queue notified (PR-A's
re-merge picks it up; the frozen ARMS-3 review keeps filtering).

## Brief amendment adopted (2026-08-26, from #1036)

Every future VERBS implementer/reviewer brief carries BOTH halves
of the long-job rule: the no-background-waiters prohibition AND
its exception (a job outliving the ~600s foreground cap launches
setsid-detached writing to a FILE, polled foreground), plus the
corollary: a reaped job is indistinguishable from a completed one
unless you check what it produced. Paid for four times today in
VERBS lanes and once in M9-5's (which spent the courtesy window);
the memory PR is cad-m8's #1036.

## M9-5's window PASSED — and found the tess-baseline gap (2026-08-26)

The second slot turn closed their acceptance row and surfaced a
VERBS-owned gap: five scenes (162 rows — diechamfer 68,
benchlayout 30, diechamferblank 26, bench 18, hollowring 4) have
been swept-and-linted on every run but COMPARED AGAINST NOTHING —
the committed baseline was cut before they landed, and tess-lint's
per-scene granularity reports new coverage, not a finding
(#1023's family, the biggest instance by row count). Ruled: M9-5's
PR keeps only its 31 rows; **VERBS-TESSFOLD** (S/M, queued after
the current reviews) audits the five scenes' sizing against
expectations (chamfer vs filleted twins; hollowring vs the
analytic shell) and folds verified-or-corrected values —
per-scene disposition stated, never a blind current-state bless.
The bench/benchlayout provenance gets CHECKED, not accepted
(the attribution lesson). Their sweep also independently
corroborated the finding-13 pins (no re-pin owed) and refuted
their own predicted firing row (granular gate: new bodies =
new coverage).

## PR-A MERGED (#1021, 2026-08-26) — the germ substrate is in; PR-B dispatches

Row CYLCYL-A (ordinal 77) in MODEL-AB-LOG. Every later germ lane
now has: the curved containment door, the pair-general dispatch
with the trap closed by proof, and the no-crossings typed posture
(the 30π wrong answer retired). PR-B (the arms — parallel-axis
first, #347's whole need; Steinmetz second) dispatches to the same
lane per the two-PR shape; the bracket rounding at 6mm is its
acceptance and #347 closes at its merge.

## Ordinal-78 returned: APPROVE (2026-08-26)

Zero MAJOR. The A3-2 correction verified AIRTIGHT twice — by the
code's own gates and live on a spinning-top body the unit never
built — and judged the honest minimum (keeping the ratified
promise would be false at every site the tag fires; #1022 is the
scope beyond the unit). The one MINOR is the review's own gift:
the NEdgeVertex front-door differential existed only on the probe
branch (an over-firing recognizer would have turned nothing red).
The reviewer's C7 answer confirms the unit was gated ONLY by
local runs (the #1029 shard cancellation) — its matrix: 99/99 and
565/565 at both ε, the four enumerated fixtures excluded by
filter and red exactly as enumerated. Light fix pass out; the
merge queues behind the hosted-runner outage. **Wave 1 closes at
this merge**, with the ARMS3-DESIGN correction flagged for Evan's
retroactive 👍 in the merge state-sync.

## ARMS-3 MERGED (#1028, 2026-08-26) — #319 CLOSED. **WAVE 1 IS COMPLETE.**

Row ARMS3 (ordinal 78) in MODEL-AB-LOG. Every Wave-1 row is
merged: RIM, CHAMFER, ARMS-1/2/3, TUBEWALL, RING, DEMO — plus the
defect units the wave surfaced (ARCEVAL×2, SSIFLAT, the fixture
fixes) and the full curved-fillet family (ten arms; the bud, the
lentil, the dome, the washer all fillet end to end through the
annulus door). The register's fillet story is DONE except the
consumer-gated parked pair and #1022's multi-link door. Wave 2:
PR-B implementing (the bracket's 6mm is its acceptance); GATE
landed. Wave 3: OFF-A/B/C landed; OFF-D specs ready. Queued:
OFF-D, TESSFOLD, the demo round 2. For Evan at the merge sync:
the A3-2 correction's retroactive 👍 (the ratified recourse
premise measured impossible; the appended correction is the
C8-cone pattern's sibling).

## CYLCYL PR-B SCOPE CORRECTION (adjudicated 2026-08-26)

The PR-B lane's opening measurement (all four cases doored, table
in its report) falsified the arms' premise: the cylinder-union
refusals are the CROSSING layer's — `CurvedPierceUnsupported` (no
pierce-event path for a rim circle definitely crossing the
partner wall) and `PointSplitCarrierUnsupported` (Circle-edge
split at an event point is unwired) — so "route the section
through the dispatch" builds a join for crossings that cannot yet
exist. The conservatism half, meanwhile, turned out to be #347's
whole first demand: the three fixes alone (arc-scoped conic edge
box — the carrier-slab diagnosis one level down, a banked class
finding; boundary-clipped cylinder face box; segment-clamped
line-clearance vertex) make the bracket round at 6 mm, pinned at
closed-form volumes r ∈ {3,4,5,6}. Ruling: PR-B ships as the
conservatism unit alone; #347 NARROWS (first demand closes, the
union demand stays open); the arms move behind a new shared
substrate unit — the curved pierce/split door — which every germ
lane's arms consume (SPHSPH included), spec to be cut. Addendum
in VERBS-CYLCYL-SPEC. Lane also banked the `| tail`-buffering
slot finding (now in agent-lane-operations memory).

## OFF-D PR-1 REPORTED GREEN; ORDINAL 79 CLAIMED, REVIEW DISPATCHED (2026-08-26)

PR #1043 (`topo::replace_face_offset`) reported: the door beside
the attach layer, per-kind closed-form carrier transport, the C5
boundary refusing typed, the apex-window predicate derived from
CARRIERS (deviation-as-finding: `chart_box` pads a true
[−1.0,−0.5] window to straddle the apex — sound for containment,
useless as a quantity), the spline-space obligation discharged by
iso-row EXTRACTION, and the measured structural obstruction: no
green Approx body row is possible through the door alone (neighbor
charts cannot extend; IsoCurve is u-const; Approx×everything
unrouted) — refused typed BEFORE mutating; the teapot's path is
analytic-only so nothing waits. Four banked findings incl. the
O(n²) whole-body mint cost aimed at #1019's fixture. CI: two of
six points on record (default/default-ε, default/1e-6), stated.
Ordinal 79 claimed from main (last = 78); single fable review
(duals suspended per #1016) dispatched against frozen head
34ee2537 with claims-to-falsify on the one-clone Err contract,
both nappes, the C5 gate's post-replacement kind, the obstruction
counterexample hunt, and C7. CYLCYL PR-B opened as #1044 under
the scope ruling; TESSFOLD implementing.

## CYLCYL PR-B OPENED (#1044) UNDER THE RULING; ORDINAL 80 CLAIMED (2026-08-26)

The lane executed the scope ruling exactly: #1044 carries the
conservatism scope alone (arms out, premise correction verbatim),
#347 NARROWED by comment (demand 1 closes with the closed-form
volume pins; demand 2 stays open on the crossing substrate, door
table attached, bracket.py's 3mm note flagged stale). The powi(2)
discipline red was fixed by conversion, not allowlist, with the
straddling-caller argument in the body. CI green at (interval,
1e-12) — the tightest row and the one this interval-arithmetic
unit most needed. Ordinal 80 claimed from main (unchanged at 78;
79 in flight on OFF-D); single fable review dispatched against
frozen head b12a7918 with claims-to-falsify on the arc_extent
sagitta charge's soundness (the one wrong-answer-capable path),
the wrap-around footprint argument, the span-dip bound's
re-derivation, and the r>6 boundary. TESSFOLD opened #1045.

## ORDINAL 79 RETURNED: OFF-D PR-1 A-W-F 2/4/5; FIX PASS DISPATCHED (2026-08-26)

The review's centerpiece: BOTH MAJors are one defect — the cone
offset action derived twice (mint: apex slides −axis·d/sinα, pure
v-shift, no sign flip; door: old apex + copysign) and the copy
drifted, measured as a VertexDisagreement gap of exactly d/sinα
on the one routed cone pair. Nothing shipped is silently wrong
(both surface as pre-mutation typed refusals; the one-clone Err
contract verified bit-identical across five paths by whole-body
Debug), but PR-2's shell of coned bodies would inherit
wrong-reason refusals. Adjudicated: fix by DE-DUPLICATION — the
cone action gets one home in geom-brep beside the mint, the door
consumes it, mirror nappe follows the mint's continuous-extension
contract (doc sentence corrected, not a typed nappe refusal);
MIN-1+Q7 merged into one carve-out ruling; MIN-4's shared-key
operand gets a decided-before-mutation refusal; the reviewer's 8
probes adopt authorship-preserving with the two reds required to
flip. Style lane: Q1 sure (the duplication IS the finding), Q3
sure (weak Err projections), Q6 sure (deviations all scheduled).
Reviewer lane swept post-report per the reclaim rule.

## ORDINAL 81 CLAIMED: TESSFOLD REVIEW DISPATCHED (2026-08-26)

TESSFOLD (#1045) reported all five scenes verified-as-is with a
pure-additive 146-row fold: hollowring bit-exact against the
analytic sizing chain (formula cross-validated on tube_along_arc's
committed 17,152), the chamfer scenes at twin-identity/planar
minimum with the fillet-twin gap attributed to genuine curvature,
bench/benchlayout provenance CHECKED to #938 (post-cut, claims
verified). Two record corrections banked: 146 not 162 (my log's
transcription error), and #1037's fold was 47 rows not the 31 my
brief said. Ordinal 81 claimed (main unchanged at 78; 79/80 in
flight); single fable review dispatched against frozen head
48559d61, briefed on the one risk that matters for an audit unit:
"verified-as-is" is also what a lazy current-state bless returns —
the review re-derives the analytic chains independently and hunts
waste the audit might have explained away. Ordinal-79 reviewer
TaskStopped post-report after orphaned-timer wake spam (lesson in
lane-ops memory; reviewer briefs now carry the cancel-your-timers
line).

## VERBS-TESSFOLD executed (2026-08-26) — the five uncovered scenes audited and folded

The audit half of #1038's disposition. Fresh sweep at this unit's
head measured the uncovered set at **146 rows** — diechamfer 68,
benchlayout 30, diechamferblank 26, bench 18, hollowring 4 — which
corrects this log's earlier "162 rows" (that entry's own scene list
sums to 146; #1038's table was right). Every scene verified-as-is
against an expectation the fold does not define: the chamfer die's
42 pip-sphere faces and six pipped mains row-for-row identical to
`diepips`'s (multiset {18,34,50,66,82,98} planes, 42×39 spheres),
its 20 chamfer facets at the planar minimum (2/strip, 1/corner);
the blank at 44 tris vs the fillet blank's 444 (the ruled strip
never out-tessellates the blend band); `hollowring`'s four torus
walls EXACT against `torus_grid_step` (inner 2×218×109 = 47,524,
outer 2×229×115 = 52,670 per face, the same formula that
reproduces `tube_along_arc`'s 17,152 bit-for-bit); bench 36 /
benchlayout 60 = 2 triangles per box rectangle over 3 and 5
solids, matching what PR #938 claimed and nothing more. The 1,122
covered rows of the fresh sweep are bit-identical to the committed
baseline, so the re-cut is a pure 146-row addition. No corrections
owed, no findings filed; #1038 (the gate class) STAYS OPEN.

## PROTOCOL DEVIATION FOUND AND RECORDED: VERBS-4 RAN OPUS×4 (2026-08-26)

Post-OFFD1-merge transcript audit (triggered by a slot-label
inconsistency noticed while preparing PR-2's dispatch): block
VERBS-4's drawn fable (slot 2, CYLCYL-A) was mis-dispatched as
opus — the whole block executed opus×4 and the drawn fable never
ran. Deviation note appended to MODEL-AB-LOG (rows stand
as-executed; the block-balancing property fails for VERBS-4; the
fable implementer sample runs one short through ordinal 79).
Remedy: arms are now read back from the draw file at dispatch and
echoed verbatim into the dispatch record. Flagged for Evan with
#1016. OFF-D PR-2 (shell) dispatches to the same lane on the same
as-executed arm (slot 4 = opus, which the draw and the execution
agree on).

## ORDINAL 81 RETURNED: TESSFOLD A-W-F 0/1/3; SMALL FIX PASS OUT (2026-08-26)

The review answered the audit-vs-bless question decisively for
the audit: every constant re-derived from scene sources through
the sizing rule (hollowring to the digit; the pip-rim torus's
2,080 from the rolling-ball construction; diecomposed's 48,870
decomposed and balanced to the last triangle; the pipped mains'
2+82n vs 2+16n confirmed as Euler-minimum at their rim chord
counts), the fold's byte-identity re-proven by an independent
sweep, and C7 verified against the run log. The MIN is a stale
row-count sentence in TESS-BUDGET.md the PR deepened while
editing the doc; NOTEs are chronology precision (generation
commit vs landing commit) and hollowring's relative-only census
pin. Fix pass dispatched (incl. adopting the reviewer's
rederivation probe row, which pins the audit's analytic half
in-tree). Reviewer lane swept post-report.

## TESSFOLD MERGED (#1045, 2026-08-26) — ordinal-81 row; the gate's eyes are open

Row TESSF in MODEL-AB-LOG. The five scenes' 146 rows are covered
verified-not-blessed; #1038 stays open for the gate's class fix.
The fix pass surfaced a wild instance of a new operational law
(now in lane-ops memory): a CONFLICTING PR gets no CI run,
silently, and none retroactively — force one via push or
close/reopen after resolving. Lane swept. In flight: ordinal 80
(#1044's review), OFF-D PR-2 (shell).

## ORDINAL 80 RETURNED: CYLCYL PR-B A-W-F 0/4/5; FIX PASS OUT (2026-08-27)

No wrong-answer path: the closed-form pins held under the
reviewer's r∈{6.5,7,8,9.5} attack (exact volumes throughout) with
honest refusal at r=10 tangency; arc_extent's sagitta bound,
clip_to_boundary's wrap argument, and the span-dip bound all
re-derived and verified — the clip argument proved STRONGER than
stated (no branch needs an unwrapped azimuth). The MINs: the
false "2×" prose (bound sound, multiplicative claim unbounded
near the span end); the re-aimed r5 row's red-ability measured
false under both mutations (box tangency hides regression); the
dip clamp shipping unpinned (M3 mutation leaves the whole tree
green); and the one soundness caveat — edge_box collapses the
radius bracket to .hi() before arc_extent, under-covering by the
bracket width in the pruning direction (fix: widen). NOTE-3
invokes the class rule on three unswept siblings; NOTE-4 adopts
the bracket.py 6mm flip as the unit's own point. Fix pass
dispatched with probe adoption; reviewer lane swept.

## OFF-D PR-2 REPORTED GREEN; ORDINAL 82 CLAIMED, REVIEW DISPATCHED (2026-08-27)

Shell exists (#1048): sealed = the degenerate no-crossing arm
exactly as ratified (every chart's inward offset assembled and
handed to insert_void with the collapse meter's own decides as
Carried{Positive}; no SSI, no census, pinned structurally);
opened = the counterpart lifted OUTWARD onto the designated
face's surface then kfmrh — the rim is a plain ring loop on an
existing surface, genus rises, nothing opens; the adjudication
fence untripped. Klein re-authoring recorded blocked-not-retired
on the absent plane×torus C5 arm (closed form, scheduled) — with
cone×cylinder, the two arms the whole revolved demo corpus waits
on (banked). Three root-cause deviations fixed in place (the
chart-group door replace_faces_offset; the WitnessMidpoint
re-anchor; revert's plane-normal trap). #1019 measured: tens of
ms release, the O(n²) term invisible; the issue's own wanted
fixture measured UNBUILDABLE (Approx faces cannot move) — posted
to #1019. Ordinal 82 claimed; single fable review dispatched
against frozen head 259fde04, aimed hardest at the one possible
wrong-answer hideout: the per-face-local evidence vs global
clearance (over-thick slab, colliding-cavity dumbbell — the
failure must be LOUD).

## CYLCYL PR-B MERGED (#1044, 2026-08-27) — ordinal-80 row; the bracket rounds at 6 mm

Row CYLCYL-B in MODEL-AB-LOG. The germ lane's conservatism story
is done: carrier-slab scoping at both box levels (face clip +
arc-scoped edge box, now bracket-span-sound per MIN-4), the
clamped span-dip, and bracket.py at its natural 6 mm with the
awkwardness note retired. #347 stays open on the union half,
which waits on the curved pierce/split substrate unit (spec next
from the orchestrator). Lane swept. In flight: ordinal 82
(#1048's shell review).

## #990 RESOLVED BY EVAN'S RULING; DESIGN PR OPENED (2026-08-27)

Evan ruled in conversation: request validity dissolves at the
signature (caller-intent magnitudes as f64 or a validating
newtype, constraint in the type where expressible — shell's
thickness is the pattern); the trilean-on-a-difference shape for
genuine decisions is the metered predicate layer itself, and a
bare shared helper stays out (the #701 evasion class). Refusal
payloads get one sanctioned gate-recognized projection, built as
its own S unit. The Bounds scope rule in geom-core/src/real.rs
carries the resolution entry; design-conversation PR opened for
the 👍. Also per Evan: #1051 filed (workflow_dispatch CI runs
with requested lane/ε — retires most full local batteries), and
briefs tightened to targeted-local-plus-drawn-point.

## #990 ENTRY CORRECTED PER EVAN: NO PAYLOAD PROJECTION (2026-08-27)

Evan caught that shape 2 was residue: with request validity
dissolved at the signature, the caller's number is always at hand
as f64 and the TUBEWALL case cannot recur. Derived quantities are
deliberately NOT echoed outside the seams — an f64 payload is a
branchable channel, the same unmetered decision surface at one
remove — so threshold-plus-variant-name is now the rule, and a
door wanting a derived echo is asking to be a seam. The queued
payload-projection unit is CANCELLED; the real.rs entry and
#1052 amended.

## ORDINAL 82 RETURNED: OFF-D PR-2 NMAI 2/5/3; ADJUDICATED WITH EVAN (2026-08-27)

The review found exactly what the brief aimed it at — both MAJors
behind the PR's own honesty paragraph, whose claimed backstop is
measured false: (1) the colliding-cavity dumbbell (0.4 neck,
t=0.3) returns Ok/valid/volume 11.76 vs true 11.312 — planar
faces have vacuous reach margins, the cavity's neck walls cross,
every loop stays simple, and tier-3 has no face-vs-face check;
(2) shell of an already-hollow operand inserts overlapping voids
and validates. Evan ruled: MAJ-1 gets the closed-form
planar-pair clearance gate (sound in the #571 conservative
direction) with the curved-pair residue a DOCUMENTED window
issued at M10's clearance certificate; MAJ-2 gets a
one-shell/zero-void operand gate now, with the semantics issue
recording that the eventual resolution must thicken EVERY
boundary (outer-only explicitly rejected). Fix pass dispatched
with the five MINs, the S2 uniform-sense gate, and the C7
requirement that interval coverage of the final head be on the
record before merge. Reviewer lane swept.

## OFF-D PR-2 MERGED (#1048, 2026-08-27) — SHELL IS IN. The Q8 substrate arc is complete

Row OFFD2 in MODEL-AB-LOG. The offset program's whole chain is
merged: OFF-A mints, OFF-B/C the Approx fit + certification,
PR-1 the face-replacement door, PR-2 the verb — sealed through
the shared void-insertion door exactly as ratified, opened as a
closed thin solid, with the no-room class gated loudly on planar
pairs (#1055 the curved window), hollow operands refused with the
thicken-every-boundary semantics recorded (#1056), and the klein
debt scheduled on its two C5 arms (#1057). The TEAPOT demo unit
is unblocked. Lane swept. Also banked: the zero-jobs queued run
(the silent-CI class's second face — a run can queue behind a
superseded run with mergeable CLEAN and never start; an
empty-commit re-roll reads as docs-only; a real code commit is
the reliable re-roll).

## DEMO2 REPORTED GREEN; ORDINAL 83 CLAIMED, REVIEW DISPATCHED (2026-08-27)

Three scenes (#1054): hollowelbow (the bore visible opaque; the
cross-scene mesh pin — the hollow door's outer walls face-for-face
equal to the solid tube's committed 17,152, so a sizing fork
between the doors now REDS instead of shifting a total),
hollowtorus (genus 2 through the parameter door; the STEP
frontier pinned self-retiring on the exact variant — one gate,
three probes retiring together with klein wall 6 and ring), and
budfillet (the fillets proven by assertions, not pixels: census
3×(+1,+2,+1), the spine re-derived from scene constants, ΔV
bracketed by the corner-square Pappus bound). Two refusals
pinned, not worked around: the natural three-rim spelling
refuses (mouth+lip share the pucker cone — the grain is one call
per disjoint-support set, and the surviving lip+bore one-call
case is pinned too); the bore-rim selector gap gets one more
register consumer. #986's four-face prediction measured six (the
wedge caps are faces). 18 baseline rows pure-additive per the
runbook. Ordinal 83 claimed (main at #1060's merge); single
fable review dispatched against frozen head 27a6efba, aimed at
the pins' TEETH (would the mesh pin actually red on a door
fork?) and the independent re-derivations.

## ORDINAL 83 RETURNED: DEMO2 A-W-F 0/3/3; SMALL FIX PASS OUT (2026-08-27)

Every pin verified by independent derivation: the mouth ball
centre re-solved from the two tangency conditions alone (exact);
the census/genus chains re-derived (elbow ≅ solid torus, the
hollow torus genus 2, the bud's 3×(+1,+2,+1)); the sweep and the
renders byte-compared (the committed PNGs identical to the hosted
run's artifacts — no hand-produced render possible); the pins'
TEETH probed (the mesh pin reds under a one-sided δ change; the
UnsupportedChain detail is the sharing, not the rim count —
measured both directions). MINs: a CI-attribution slip in the PR
body (the sweep gated in run 1, not run 2 — the drawn-row
bookkeeping class), two stale invalidated-premise sentences in
klein/ring, and the cross-panel constants duplicated without a
tie. Fix pass dispatched with probe adoption; reviewer lane
swept.

## PROTOCOL v6 RATIFIED AND ADOPTED (2026-08-27, #1064 merged)

The #1016 suspension ends: from the next review dispatch, every
implementation row gets a CROSS-MODEL dual with the R1/R2 model
assignment randomized per dual (one urandom byte at dispatch,
parity 0 = R1 opus + R2 fable, parity 1 = R1 fable + R2 opus;
byte recorded in the lane-private draw file beside the arm
draws). Stopping rule pre-registered: eight adjudicated
unilateral MAJORs per the fixed instrument, or twelve new pairs,
whichever first — Evan notified explicitly at the trigger.
Read-side lane isolation enters both briefs (pushing never
delayed; glimpses disclosed, ASM-1 shape). The v6 tally starts
at zero — the v4/v5 pairs and attested-but-unscored pair 11 stay
analysis input only. First affected dispatch: VERBS-PIERCE's
review (ordinal 84) — TWO reviewer lanes, frozen-head
concurrent. Demo-class units keep the plan's process section
(DEMO2's ordinal-83 single predates ratification and stands).

## DEMO2 MERGED (#1054, 2026-08-27) — ordinal-83 row; #986 closed

Row DEMO2 in MODEL-AB-LOG. The montage gains the hollow elbow
(bore visible opaque), the translucent genus-2 hollow torus with
its self-retiring STEP pin, and the filleted bud with its
proof-by-assertion. The fillet_edges composition grain
(per-disjoint-support-set) is pinned from both directions. The
demo queue's remaining item is THE TEAPOT — the Wave-3 finale,
unblocked since shell merged. Lane swept.

## PIERCE REPORTED; DOOR-2 STOP ACCEPTED; ORDINAL 84 = THE FIRST v6 DUAL (2026-08-27)

Door 1 exceeded acceptance: the mid-anchored azimuth removes the
atan2 branch cut entirely (|δ| ≤ π within one edge span — the
interval lane needs NO fork and NO measured constant; banked as a
reusable pattern), and the routing half the spec did not name
turned out to hide TWO silent wrong answers — contfp deciding
Circle boundaries by CHORD (a cap rim's chord is the diameter)
and point_in_loop polygonizing disc loops (every cap interior
answered Out; rings invisible) — one measured as a wrong union
volume on main (7.003 vs 6.643). The coaxial boss now UNIONS
correctly. Door 2's STOP fired per the fence with evidence: no
curve×curved-surface root finder exists anywhere, the ring
insert is planar by construction, and TWO of the four #1044
cases are cosurface incidences C2/C4 forbid inferring — #347's
union demand SPLITS (comment posted there; the declared-contact
family owns that half, door 2's remainder rides the arms
adjudication). The #1032 seam measured: one fix does NOT move
both rows (11 → 6 under declaration; the remaining six are
curved×planar). Ordinal 84 claimed — THE FIRST v6 DUAL: slot
byte 180, parity 0 → R1 opus + R2 fable, frozen head 5da23569,
read-side isolation in both briefs. The mixed-arc-and-line loop
blind spot (half-disc, slot, rounded rectangle still
polygonized) stated in the PR and left for the register.

## ORDINAL COLLISION WITH PCURVE; PIERCE'S DUAL RENUMBERS 84 → 85 (2026-08-27)

PCURVE claimed 84 for P-1a ON MAIN (#1074) after my PIERCE claim
had been logged only to this branch — main-is-authority resolves
it their way (the ASM precedent's rule doing exactly its job).
PIERCE's dual is ordinal 85: slot byte 180 parity 0 (R1 opus +
R2 fable), frozen head 5da23569, unchanged otherwise. Process
correction adopted: ordinal claims now go to main IMMEDIATELY at
claim time. Separately, BOTH ordinal-85 reviewers died mid-run —
R1 on an auth gap (the account re-logged), R2 on the fable usage
limit — and were resumed from transcript per the death-recovery
rule; if R2's limit still binds at resume, the v6 budget valve
question opens (every-2nd-row duals) rather than a silent
downgrade.

## ORDINAL 85 (FIRST v6 PAIR) RETURNED AND ADJUDICATED (2026-08-27)

R1 opus A-W-F 2/5/5, R2 fable NMAI 1/4/4 — the pair's
correspondence is the design working: BOTH arms independently
measured the same central defect (the non-disc arc-loop
remainder is a silent wrong body — R1's half-disc both senses,
R2's half-cylinder AND the lens, which is all-arc and falsifies
the blind-spot prose itself), while each arm carried a unique
find: R1 alone caught cast_ray's ray×cylinder quadratic
falsifying the STOP's "no root finder anywhere" (the STOP
survives on the independent ring-lane leg — both arms verified
that leg separately); R2 alone closed R1's open bit-identity
column with full default suites. Ruling on the shared MAJOR:
gate arc-bearing <3-vertex non-disc loops typed NOW (the
measured-wrong class exactly — slot/rounded-rect measured
correct, so no wider), issue the general arc-aware parity
remainder, fix the "honest remainder"/blind-spot prose to the
callee's own contract. v6 tally: R1's cast_ray MAJ is unilateral
but CLAIM-CLASS — excluded per instrument 3b; TALLY 0 after
pair 1. Isolation: both arms disclosed command-line-only
glimpses via the shared build slot — benign, recorded. Union
fix pass dispatched; both reviewer lanes swept.

## PIERCE MERGED (#1068, 2026-08-27) — ordinal-85 row (sample #25); the first v6 pair closes clean

Row PIERCE in MODEL-AB-LOG with the full dual columns. The fix
pass's own finds are worth the read: one reviewer probe fixture
re-signed (it demonstrated nothing as written — the measured
geometry bowed the other way), the >period alias rowed, the
|δ|=π endpoint asserted as a property rather than a coin flip,
and the interval lane pinned by rule so the drawn point is no
longer sampling luck. #347's remaining union scope now reads:
coaxial family → declared-contact territory; parallel/steinmetz
→ the ring lane (#1076's sibling machinery), roots already in
the tree. v6 tally 0 after pair 1. Lane swept. TEAPOT's report
is the program's last outstanding item before the register
re-audit.

## ORDINAL 100 CLAIMED (first VERBS banded claim, per #1075) — TEAPOT dual (2026-08-27)

TEAPOT (#1078) reported: the demo ships with four bodies, honest
walls (both unions refuse typed with payloads carried verbatim
into the panel note), the fourth probe on the one STEP gate — and
TWO fresh shell defects found by real use, now #1081 (the sealed
hollow's one-junction-shape class; the pot's belly squared) and
#1082 (shell_open's VALIDATED wrong body on every solid of
revolution; the pot ships sealed). Both await blinded
verification in this review cycle. Ordinal 100 claimed on main
AT CLAIM TIME per the post-collision rule; v6 dual, frozen head
7d5aa5b2; slot draw in the lane-private file. Sample number
assigns at merge.

## ORDINAL 100 (TEAPOT DUAL) RETURNED AND ADJUDICATED (2026-08-27)

R1 fable A-W-F 0/3/2, R2 opus A-W-F 3/4/3. Both arms confirmed
BOTH shell defects on independent fixtures (R1's closed-form gap
match at 1.1e-17; R2's non-dyadic vase + the hexagon gap law
t·|cosθ| exact) — #1081/#1082 stand verified. THREE conflicts,
each resolved by the better instrument: (1) the junction tables
never ran hosted — R2's jobs-API read (steps `skipped` under a
green k-lint job name; klint_row is its own sampled axis) beats
R1's ci.yml read, VERIFIED by the orchestrator directly; the
third face of the silent-coverage class, now in lane-ops memory;
(2) #1082's discriminator is wrong — R2's revolved-tube fixture
(one-face cap still wrong; annular mouths need a face SPLIT)
re-scopes the issue to "any designated face whose cavity
counterpart's boundary cannot become an interior-disjoint ring";
(3) the tangent non-attribution premise false — R2's
definitely-not-tangent dome refuses identically at a third door
(the sphere's non-translation offset is the variable); R1's
code-read verdict loses to the fixture. v6 tally after pair 2:
STILL 0 — R2's k-lint and tangent MAJs were MENTIONED by R1
(opposite verdicts), so not unilateral under the coding
definition; the discriminator MAJ's demonstrated core dedups
into the bilateral #1082 defect. Union fix pass dispatched;
both reviewer lanes swept.

## TEAPOT MERGED (#1078, 2026-08-27) — ordinal-100 row (sample #26). THE Q8 ARC IS COMPLETE

Row TEAPOT in MODEL-AB-LOG. The designated demo ships: vessel,
lid, spout and handle in one cell, with the walls that remain
pinned typed and scheduled (#1057's C5 arms; the taper/canal
family; the edge selector) and two shell defects (#1081/#1082)
found by the demo doing exactly what demos are for — real use
against the shipped verb, verified by both arms of the dual on
independent fixtures. The register's teapot rows record what
composed and what walls. Wave 3's substrate story is DONE:
OFF-A/B/C/D, shell, the teapot. Remaining VERBS queue: the shell
defect repairs (#1081/#1082 — unowned, spec-ready material in
the issues), SPHSPH/CYLSPH/CONE germ lanes (waiting on arms
adjudication + #1076's sibling machinery), #1031 (cap F7), the
wall-2 fork (Evan's content call), and the arms unit behind the
ring lane. Lane swept.

## EVAN'S WALL-2 CALL: RE-AUTHOR; LILYWELD CUT (2026-08-27)

Evan ratified the circle-coincident re-authoring ("please do
re-author the lily!"). LILYWELD spec'd two-PR: the content
re-authoring with the coincidence itself asserted analytically
(PR-1, S), then the #968-shaped kernel half — declared cone×torus
gate admission + a carrier_eq rung on the exact shared circle —
flipping wall 2 (PR-2, M). #968 proper (wall 1's torus×torus with
the tangency disposition) stays banked; LILYWELD's machinery is
deliberately the reusable half its checklist names. Recorded on
#1059; dispatching to block VERBS-6 slot 2 beside SHELLFIX.

## THE #1090 RULING ADOPTED AND SELF-APPLIED (2026-08-27)

Evan's rule via PCURVE: relaxations recorded per arm; asymmetric
pairs score nothing; 3(e) extends to orchestrator-made asymmetry.
Self-applied at the same standard: PIERCE's pair is excluded
under 3(e)'s letter (both arms killed mid-run and resumed —
complete reports, identical briefs, but interrupted is
interrupted; PCURVE excluded their comparable pair). VERBS state:
tally 0/8, clean pairs toward twelve: 1 (TEAPOT). Ledger row
corrected in place. Scheduling consequence adopted: under mutex
saturation, stagger duals rather than touch briefs.

## ORDINAL 101 CLAIMED — SHELLFIX PR-1 dual (2026-08-27)

PR-1 (#1099) reported green with NO STOP: the class was an
OPERAND artifact (the revolve's seam inside the designated
chart), not a surgery limit — canonicalize_chart (kef/kev/kemr)
retires the seam before the glue, the axis-touching cap MINTS one
annular rim, the annular cap SPLITS to two disjoint annuli
through existing doors (the thing #1082 said kfmrh could not
express), anything else refuses typed. The teapot ships OPENED —
wall 2 retires, all four bodies export STEP (the frontier gate
back to three probes). Tier-3 gains check 9 (RingMeetsOuter,
metered against ZERO — metering coincidence against eps lands on
the band's own threshold and escalates; a real bug hosted CI
caught, banked). v6 dual, frozen head 66071862; slot draw in the
lane-private file; claim to main at dispatch per the standing
carve-out.

## ORDINAL 102 CLAIMED — LILYWELD PR-1 dual (2026-08-28)

PR-1 (#1109) green: the weld circle EXACTLY coincident (off-spine
and radius residuals 0.0; normal 1.9e-16), the setback's
z-fight intent re-achieved by abutment, wall 2 re-pinned to the
operand gate's measured payload with PR-2 as the retire note, the
finding-13 lantern rows re-cut argued (+106/half, the neck cone),
and a live refusal banked: the globe's TANGENT cone neck is
refused by the junction gate (JunctionTangent, margin 1.6e-17) —
the scene authors 70° on its own merits. One clippy red fixed by
the #971 alias precedent, not allowed. v6 dual, frozen head
a1aa5289; slot draw lane-private; claim to main at dispatch.

## ORDINAL 101 FIX PASS — SHELLFIX PR-1 union (2026-08-28)

Dual adjudicated A-W-F 0/4/6 and 0/4/6: the central inversion
HELD under every attack from both arms (wedge, counterbore,
bottom-opened and Line-carrier shapes all correct-or-typed; the
byte-diff clean; no validated wrong body found anywhere). Four
union items, all landed. The HEADLINE was bilateral and was a
real defect: check 9 matched only `Ok(Sign::Zero)` at all three
decides, so an `Err(Indeterminate)` read as NO CONTACT — and the
same drop was a precondition of the GLUE, which proceeded to
build on an uncertifiable gap. Three-valued verdict now;
`RingContactEscalated` at rest, `ShellError::Escalated` at the
door, neither reachable through this verb (the thickness gate
shields the band) and both there for other producers' bodies.
Second: a third contact arm (a ring vertex on an outer edge's
INTERIOR), the collinear arm accepting ANY sample inside the
trim, and the residue list rewritten from "Ellipse/NURBS" — false
by omission — to the enumerated set (non-vertex tangency,
transversal crossings, Ellipse/NURBS). Third: the key-shared
exemption cited tier 1 on a FALSE premise (R1 read passes 1-13;
an umbrella pinch passes) and is removed, with its own planted
red. Fourth: C7's compile-mode sentence was overstated and is
withdrawn.

Two BANKED lessons, both from the hosted gate rather than from
reading: metering a coincidence test against eps lands a
coincident pair's margin ON the band's threshold, where it
escalates instead of deciding — meter the separation against
ZERO; and `revolve`'s axis-contact classification is
epsilon-sensitive, so an axis-touching fixture that builds at the
default band refuses `NonManifoldAxisContact` at 1e-12. The
adopted probe rows record that door fact instead of expecting
past it. Both probe branches adopted authorship-preserving,
byte-dump harness included. Helper duplication filed as #1123.

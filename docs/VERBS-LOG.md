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

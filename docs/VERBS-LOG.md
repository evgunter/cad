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

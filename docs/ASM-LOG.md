# ASM log — the assemblies implementation program

Narrative record; the plan is `docs/ASM-PLAN.md`. Convention as
ever: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-10, program start)

ASSEMBLY-DESIGN ratified (#333; conversation #328). R0 discharged
before the program opened: #317 merged as #325 WITH the A7 rider
(`StepImport::instances()` records per-instance NAUO placement).
The A5 at-rest-door wiring is bound into CONTACT-DESIGN C7 (#337)
so the M9 spec inherits it. Monitors armed; merge permission live.
In flight: R1 substrate exploration (read-only, report to
cad-work/asm-r1-substrate/). A9 RATIFIED (Evan's confirmation in
chat, 2026-08-10: "no need to erase anchor frames") — the
component-partition definition of relative freedom is in
ASSEMBLY-DESIGN. AQ3 working session queued before any R2
dispatch.

## Seam: R1 substrate report in; A9 ratified + amended (2026-08-10)

A9 landed (#340) and was simplified same-day on Evan's pushback
(#341): no derived graph — relative freedom = connected components
of the recipe DAG itself. The R1 recon report
(cad-work/asm-r1-substrate/report.md) refined the unit cut
(ASM-PLAN updated: ASM-2 split 2a/2b; kernel-honesty correction;
pin-hash ≠ memo-key vocabulary note) and surfaced three
contradictions: C1 (shipped `Node::Pattern` yields
`ValuePayload::Instances`, never one body — the assembly Pattern
semantics need a ruling BEFORE ASM-3's spec; pending with Evan),
C2 (multi-solid instantiation needs the graft-door kernel
extension — now ASM-2b), C3 (A2's memo is new machinery — in
ASM-2a). The roots conversation settled same day:
**A10 RATIFIED (strict)** — explicit ordered product roots with
coverage + ancestor-freedom invariants and automatic maintenance
(Evan's mid-authoring framing: a WIP component carries its root
until joined); the root gather IS A2's product and RESOLVES C1
(shipped `Node::Pattern` semantics unchanged; the gather
materializes `Instances`). New unit ASM-ROOTS slotted before
ASM-2a. Next: ASM-1 binding spec.

## Seam: ASM-1 dual review complete; fix pass in flight (2026-08-11)

ASM-1 delivered as PR #364 (28/28 green pre-#366). Dual review
(ordinal 21, sample #7, frozen head f04d08e8): R1 MERGEABLE 0/1/4
rubric 5/5/4; R2 APPROVE-WITH-FIXES 0/3/4 rubric 5/4/3. CONVERGED
on the one headline gap (doc-level metadata preimage inclusion has
no falsifier) — verdict LABELS again differed on identical
0-MAJOR substance (SWITCH-E precedent). Disjoint tails: R2 the
replayed-pin discipline + the next_id/undone-insert consequence;
R1 the skipped-replay assert + error-mapping cosmetics. Blinding
caveat disclosed (R2 glimpsed R1 probe TOOLING via a shared
scratchpad script — no findings/verdict read; recorded here for
the row). Fix pass dispatched (implementer-inherited) on the
5-item adjudicated union; NOT in scope: header-scan I/O, tour
automation. The #366 CI
billing outage opened and RESOLVED inside this seam (Evan restored
the Actions budget); row records at merge. A11 revision
awaiting sign-off on #356; A/B draw + ordinal entries in
MODEL-AB-LOG.

## ASM-1 MERGED (#364, 2026-08-11)

Identity and pins are live: documents carry authored DocumentIds
(derive/random split keeps the kernel deterministic), pins are
SHA-256 over include-by-default canonical bytes (exclusions
exactly {id, log}), schema v5 with the id header line refuses
v4 typed, and pncad::workspace resolves DocRefs with typed
DuplicateId/PinMismatch refusals. The dual review converged on
one test-strength gap, now closed with executing falsifiers; the
next_id/undone-insert consequence is recorded in D-3 with its
documenting test. Row in MODEL-AB-LOG. Seam swept (asm-1 + both
review lanes). Next: ASM-ROOTS spec (A10) — block ASM-1 slot 2,
opus; A11 conversation continues on #356.

## ASM-2K + ASM-ROOTS MERGED (#381, #383, 2026-08-11); ASM-3 discharged

The parallel pair landed same-day: the kernel door (multi-solid
grafts + the multi-solid-master naming rule pinned; the D-2 spec
premise was FALSE and both blinded reviewers verified the
deviation on unmodified main) and A10 roots (0/0/5 MERGEABLE —
the program cleanest verdict; invariants mutation-proven; the
sink-set equivalence observation in module docs). Together they
DISCHARGE ASM-3: the plan unit was "gather wiring + evidence,"
and ROOTS row 3 already materializes Instances-at-root with
provenance and Instance(i) names preserved, with 2K supplying
the multi-solid-master naming rows. Findings filed: #382
(overlap-validation doc-truth + census gap). Forward dependency
for ASM-2a/2b: the GraftMap name-table bridge (no emitter mints
a table for a grafted body — instance-qualified naming needs
it). Remaining R1: ASM-2a (spec next), ASM-2b, ASM-4. Seam
swept.

## ASM-2A MERGED (#414, 2026-08-12) — R1 nearly closed

InstantiatePart is live: assemblies reference pinned part
documents, cluster-level placements per A11, materialization
through resolve→evaluate→product→transform→graft, the name
bridge (RoleSeg::InPart via GraftKeys), typed refusals at every
seam incl. ReferenceCycle naming the loop (Evan review ruling),
and the walk_names document-seam stop (the unit design finding).
Evan reviewed the PR directly — six design questions answered on
thread; resolver Option-shape defended, awaiting his preference;
cycle ruling + discipline-gate extension folded into the fix
pass. First reviewer-found MAJOR of the program (single review,
not a dual — stopping tally unaffected). Remaining R1: ASM-2b
(name-fidelity + sub-assembly), ASM-4 (split/inline). ASM-2b
dispatch still holds on the #409 P3 block-transition answer.
Seam swept.

## Seam: wind-down at ASM-2B delivered (2026-08-12)

Session wind-down on Evan's instruction (wrap in-progress work,
write context, draft upcoming design). RESTING STATE:
- **PR #425 (ASM-2B) delivered, 30/30 green, NOT merged, review
  NOT dispatched** — the resume step is: fix the ordinal from the
  MODEL-AB-LOG tail (31=ASM-2A, 32=LBRET, M8 dual @30 = sample
  #10; count forward from whatever the tail then holds), single
  unless it lands on a multiple of 3 (then the dual block-of-two
  state applies — #405 banked SAME-MODEL for the next dual),
  fable reviewer, v4 verdict ladder, frozen head from the PR.
  Claims: the seven ASM-2B-SPEC rows + falsify its three
  reported deviations (guard DELETION vs keep-unreachable is the
  one needing an adjudication call; the implementer offered
  restoration in the PR body).
- ASM-4 spec WRITTEN (docs/ASM-4-SPEC.md, binding) — dispatches
  on block ASM-2 slot 3 = fable after 2B merges.
- R2 spec DRAFTED (docs/ASM-R2-SPEC-DRAFT.md) — finalize after
  ASM-4; R2-b is the program's first numeric-predicate unit.
- Pending with Evan: the #414 resolver Option-shape (defended on
  thread; fold into a later pass only if he rules); AQ1/AQ2/
  AQ5/AQ6 open by design.
- Findings filed: #382 (overlap validation), #415 (tolerance_init
  env red). Protocol state: v4 + #409 amendments adopted; dual
  tally 4-of-6 (per LIB's #424); results-off-file in force.
Operational: monitors died with the session — successor re-arms
ALL of local-scripts/monitors/* (glob convention; away-channel
needs CAD_CHANNEL_SELF_TAG="(ASM orchestrator)" and
CAD_CHANNEL_BRANCH_PREFIXES="asm/,mngr/cad-assemblies-implement").

## ASM-2B MERGED (#425, 2026-08-12) — materialization complete

Sub-assemblies instantiate: the unit proved itself a deletion of
two guards (the kernel was already N-solid-ready — 2K and 2A had
discharged everything the plan feared), reviewed 0-MAJOR with the
deletion adjudicated and ratified into the spec. The one MINOR
was main-owned (ci-local discipline allowlist drift at #421 —
LIB notified on their thread). R1 remaining: ASM-4 ONLY
(docs/ASM-4-SPEC.md, binding, ready to dispatch on block ASM-2
slot 3 = fable). Handoff: issue #430. Seam swept.

## Seam: new orchestrator; ASM-4 dispatched (2026-08-15)

Session resumed off handoff #430 after the three-day gap (main's
motion in between was all M8 — now CLOSED — and LIB; no ASM state
moved). Orchestrator branch this session: mngr/cad-asm-2
(away-channel armed with asm/, mngr/cad-asm-2,
mngr/cad-assemblies-implement). Resolved with Evan at session
start: the #414 resolver shape is SETTLED — his open half was
`Arc<dyn>`, not `Option`; the alternatives walk (concrete/enum
blocked by layering, generic infects every EvalOptions carrier
for a once-per-cache-miss call, Box breaks Clone+sharing, &dyn
adds a lifetime) is on the #414 thread; accepted per his
accept-if-confident. ASM-4 dispatched (block ASM-2 slot 3, lane
asm-4, branch asm/4-split-inline, spec binding). In parallel per
Evan's go-ahead: the R2 census mini-recon (read-only) runs
alongside; R2 spec finalization still waits on ASM-4's
interface-record hook shape. Subscribed to #509 (M9 plan): the
M8 orchestrator confirmed the seam — M9 BUILDS the A5 door, ASM
consumes same-currency, R2's planar subset proceeds
independently. Dual tally still 4-of-6 (samples #11/#12 both
0-MAJOR, non-qualifying); next ordinal claim is 39 = the banked
same-model dual, shared with LIB's pending PYG5 claim —
main-is-authority at whoever dispatches first.

## Seam: AQ7 raised and RATIFIED as A12 same-day (2026-08-15)

Parallelization pass while ASM-4 implements (Evan's prompt): the
recon's Mate-root wrinkle sharpened, on reading A9's ratified
text, into a genuine three-way composition failure (A9 expects
mate references to connect components; A3 makes them stable
names; shipped D3 makes name refs non-edges) — raised as AQ7 on
PR #522 with a firm proposal. Evan signed off within the hour
(👍 + comment concurring against coverage-exemption): **A12
ratified** — reading edges recomputed from name heads, A9/A11
partition over ALL edges, A10 over CONSUMING edges only, mates
ordinary non-body roots. AQ7 discharged; #522 merged. Also this
pass: the R2-a coset intersection table written out in the draft
(closure set {SE(3), planar, cylindrical, prismatic, revolute,
trivial, empty}; decided case splits; verdict fold), with the
pre-log flag that R2-a's case splits are decided numerics — the
L/structural pre-log amends to numeric under #409's mixed rule
AT SPEC TIME, before any draw reads it. #382's cheap half turned
out already done (M8's #491), so nothing to parallelize there.
R2 spec finalization now waits ONLY on ASM-4's interface-record
hook. ASM-4 lane healthy at the last sweep (D-1 + hook committed,
refactor.rs in progress).

## ASM-4 MERGED (#525, 2026-08-16) — R1 COMPLETE

Split and inline are live, and with them R1 closes: assemblies
author, evaluate, persist, split, and inline end-to-end. The unit
survived a fully-executed review (0 MAJOR; 6/6 mutants killed;
the reviewer's own 3-cluster probe found the one real nuance —
non-adjacent cut roots collapse their interleaving onto the
instance position, now stated in D-2's ratified amendment and
pinned by test). D-2's per-cluster-instance text was adjudicated
ACCEPT-AS-AMENDMENT (#540): one instance per split, hoist iff
exactly one cluster, with the A12-cluster-keying rider pointed at
R2-a. All nine refusal arms now tested naming their subjects. The
#534 Actions-budget outage opened and resolved inside the seam
(Evan restored; #366 precedent). Row at ordinal 40. Next: R2 spec
finalization — every input is now in hand (interface-record hook
shape, A12, the coset table, the census recon, the M9-1 seam) —
then the R2-a dispatch. Seam sweep: lanes asm-4 + asm-4-r1.

## ASM-UPD MERGED (#549, 2026-08-16) — the update door exists

A13's four clauses are executable: per-reference UpdateReference
(same-pin refuses, wrong pins surface at the evaluation seam),
the update-all elaboration with AlreadyPinned making staged
update-all appliable, the mixed-pin lint reporting multiplicity,
and the memo re-key evidence now warm-path-hardened (the review's
one MINOR, closed mutation-verified). Schema landed v10 — the
v9 double-claim resolved by the deterministic rule (RESPELL kept
9), and the bump repaired RESPELL's future-version literal en
route. Review 0-MAJOR at ordinal 46; NOTE-1 (tag values
unpinned) filed as #561 and adopted by LIB's bindings queue.
Remaining pipeline: R2-A dispatches when M9-1 PR-2 merges (it
shifts to schema 11); R2-B spec finalizes after that merge. Seam
swept (asm-upd + asm-upd-r1).

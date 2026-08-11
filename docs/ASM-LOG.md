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

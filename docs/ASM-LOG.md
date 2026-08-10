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

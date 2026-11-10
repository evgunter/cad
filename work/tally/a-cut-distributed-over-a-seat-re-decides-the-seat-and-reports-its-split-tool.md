---
id: a-cut-distributed-over-a-seat-re-decides-the-seat-and-reports-its-split-tool
kind: issue
title: A cut distributed over a ⊔ re-censuses the pieces: the seat is decided again (unquietable) and the tool's split walls report four unproven flush pairs
status: open
opened: 2026-11-09
priority: P1
cost: M
refs: [a-vertex-piercing-two-declared-coincident-faces-reads-as-one, a-cut-of-a-nested-assemble-piece-reads-its-own-records-not-its-census]
---


Measured by a SEAL designer on fork `seat-vertex-one-interface` (2026-11-09), using a probe in a private clone. It is not yet a committed test.

**The measurement.** The seat document is a bolt on a plate, `bolt ⊔ plate`. On its own it has **1** `unproven-coincidence` finding. Adding the distributed cut `(bolt ⊔ plate) − pin` raises that to **6**:
- **The seat again, at the cut node.** It is a `SameOpposite` finding, `quietable: false`, with recourse `Assert`.
- **Four `SameOriented` flush pairs.** The pin's four walls are each split between the bolt and the plate, so each wall shows up as a pair, residual `Err(Through(..))`.

**Where.** `distribute::assembled` (`crates/editor-core/src/eval/wire/distribute.rs`) runs a fresh `product::Census` over the cut pieces. That re-decides a meeting inside the operand. It breaks `[coincidence.booleans]` ("carried, never re-decided") and `[assertions]` (a finding belongs to the `⊔` whose census decided it and is reported once there). It is spec test 25's "*Breaks if* an operator re-decides a meeting inside an operand". The flush pairs suggest the census cannot trace a divided tool face back to its one construction through the distribution's reads.

**Coupled.** SEAL's fork on `a-vertex-piercing-two-declared-coincident-faces-reads-as-one` weighs whether the editor's distribution should exist at all, or move into the kernel. If it retires, this row closes with it. Pin the probe as a test either way.

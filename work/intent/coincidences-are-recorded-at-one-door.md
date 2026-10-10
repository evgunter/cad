---
id: coincidences-are-recorded-at-one-door
kind: issue
title: D10 stage 4 PR B: the coincidences the kernel decides from values at the declared one-carrier rung, the split's pinch and the mitre are Coincidence records carried into NodeValue (ContactRecords citing them is B2); the door coincide::prove (rung 1, the same construction read twice) and CheckId::UnprovenCoincidence
status: closed
branch: intent/s4-b-record
pr: 4354
opened: 2026-10-08
priority: P0
cost: H
closed: 2026-10-10
---

INTENT stage 4, PR B. Ev approved the design in PR 4322 (fork log row
92, FORK-S4P); `docs/INTENT-STAGE4-SPEC.md` §3 predates it, and where
they disagree this row governs. Needs nothing from stage 2 or 3;
dispatchable now.

**Provenance is the document's.** A row names each cell as the read it
entered the deciding operation through plus its `StableName` there,
never by a stamp the kernel carries. A name alone is not provenance (a
copy's cells keep the names of the body it copies, N1), so the read,
naming the copy, says which copy a cell is a cell of.

**Two record types.** A `Coincidence` is the decision: its operand
cells, the relation, the decision site and the margin, never re-keyed.
A `ContactRecords` row is a touch in the result and carries the index of
the `Coincidence` that backs it, so a touch without a decision cannot be
built. Each `Coincidence` row also carries its discharge kind: how its
Zero was discharged in the `Sym` lane (theorem or numeric), which C's
rungs read.

**Emission** at every value decision that already glues, merges or
touches: the declared rung of the plane and carrier ladders, the
declared merge, margin-decided vertex fusions and covered pairs, the
split's pinch (`splitting/classify.rs:256`), and the blend battery's
isosceles turn (`battery.rs:2033`, carried on `Blended`). The records
are carried into `OpOut`/`NodeValue`, each cell named by its read and
`StableName`. Nothing builds differently; no digest, content key or id
moves. Closes `value-decided-coincidences-have-no-recording-door`.

**The door** `coincide::prove`, with rung 1: **the same construction
read twice**, found by walking reads from each cell's operand read and
its `StableName` back to the minting node, composing placements on the
way. It never reads `GeomSource` stamps. `GeomSource`, `AxisSource` and
`ParamSource` retire; the kernel deletion is E's. And
`CheckId::UnprovenCoincidence`, reporting each row the door leaves
unproven.

When this unit lands, NAMES N6 reads: "**N6 — A cell's construction is
read from the document.** A recorded cell is named by the read it
entered the deciding operation through and its name there. The door
reads its carrier at that name from the symbolic evaluation (D10,
Coincidence). A placement is an operation that defines a copy, and the
copy's cells keep the names of the body it copies (N1), so the read,
naming the copy, not the name, says which copy a cell is a cell of. The
kernel carries no recipe provenance of a description." CONTACT-DESIGN C3's
`PatchContact` reads "backed by a `SameOpposite` decision", every
granularity citing its backing; topo's preamble gains the decision
beside the record.

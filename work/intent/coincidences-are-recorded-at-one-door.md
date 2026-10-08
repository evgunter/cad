---
id: coincidences-are-recorded-at-one-door
kind: issue
title: D10 stage 4 PR B: every coincidence the kernel decides from values is a record carried into NodeValue; the door coincide::prove (rung 1, same source) and CheckId::UnprovenCoincidence
status: open
opened: 2026-10-08
priority: P0
cost: H
design: true
---

INTENT stage 4, PR B. Spec: `docs/INTENT-STAGE4-SPEC.md` §3. Needs nothing from stage 2 or 3; dispatchable now. Design open: FORK-S4-3 (is the record `ContactRecords` or a new type), weighed before build.

The record (cells, relation, decision site, margin) emitted at every value decision that already glues, merges or touches: the declared rung of the plane and carrier ladders, the declared merge, margin-decided vertex fusions and covered pairs, the split's pinch (`splitting/classify.rs:256`), the blend battery's isosceles turn (`battery.rs:2033`, carried on `Blended`). Carried into `OpOut`/`NodeValue` named by `StableName`. The door `coincide::prove` with rung 1 (same source, what N6 proves today), and `CheckId::UnprovenCoincidence`. Nothing builds differently; no digest, content key or id moves. Closes `value-decided-coincidences-have-no-recording-door`.

FORK-S4-1 and S4-3 were weighed as one fork (FORK-S4P, fork log row 92)
and went to Ev in an `[ev]` PR with FORK-S4F; this unit builds on the
answer provisionally. Provenance is the document's: a row names each
cell as the read it entered the deciding operation through plus its
`StableName` there, and rung 1 is "the same construction read twice"
computed by walking those reads, never by reading `GeomSource` stamps
(which retire with `AxisSource` and `ParamSource`; the kernel deletion
stays in E). The record is a decision (`Coincidence`, operand cells,
never re-keyed); a `ContactRecords` row is a touch in the result and
gains the index of the decision that backs it. The row also carries how
its Zero was discharged in the `Sym` lane (theorem or numeric), which
FORK-S4F's rungs read. When the units land, NAMES N6 becomes: "**N6 — A
cell's construction is read from the document.** A recorded cell is
named by the read it entered the deciding operation through and its
name there. The door reads its carrier at that name from the symbolic
evaluation (D10, Coincidence); a pass-through placement adds no name
segment (N1), so the read, not the name, carries where the cell was
placed. The kernel carries no recipe provenance of a description." C3's
`PatchContact` reads "backed by a `SameOpposite` decision", every
granularity citing its backing; topo's preamble gains the decision
beside the record.

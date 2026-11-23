---
id: records-ride-beside-the-operand-at-the-boolean-doors
kind: issue
title: boolean_op_with, boolean_placed and nary_boolean take records beside the operand, and a caller can hand fewer than the operand holds
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Left open by PR 7124 (`embed/records`, 2026-11-23), whose unit
(`a-finished-body-holds-its-contact-records-at-every-door`) lists it
and closes at merge without it.

`BooleanInputs { records_a, records_b }`
(`crates/topo/src/boolean/mod.rs`) and `MemberInputs { records, rank }`
(`crates/topo/src/boolean/nary.rs`) carry an operand's records into
`boolean_op_with`, `boolean_placed` and `nary_boolean` beside the
operand, which is a finished body that already holds them. A caller can
hand `BooleanInputs::default()` for an operand that holds records and
the op carries none; nothing in the types refuses it. The plain doors
(`union`, `subtract`, `intersect`), the evaluator's cut and fold, the
verbs runner, the fold's pairwise judgement and the interference
check's intersection hand the operands' own (`BooleanInputs::of`), and
a mutant that hands default inputs reddens 28 rows; that is a pin, not
a type.

**Owed.** The op reads each operand's records off the operand
(`FinishedBody::records`), and the two structs keep only what is not
the operand's (a member's rank). The slow set's
`three_solids_on_one_line` spells an uncarried fold on purpose
(`boolean_op_with` and default inputs) and needs a spelling of its own
then.

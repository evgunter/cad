---
id: load-door-does-not-check-payload-expression-param-refs
kind: issue
title: The load door asks the param-table rule of slot expressions only; a measure's or an assertion's payload expression is edit-door-only
status: open
opened: 2026-09-16
---



Disclosed by `edit/one-predicate-round-three`, which gave the
param-table rule one home (`Doc::param_ref_fault`) and taught the load
door to ask it.

**The rule.** "A slot expression names a declared document parameter,
and reads it at the dimension the declaration carries" (spec D6) is
now one predicate with three callers: `edit.rs`'s `check_param_refs`
(the edit door's slot arms), the payload-expression walk beside it in
`check_node_slots`, and `persist/check.rs`'s
`first_slot_param_ref_fault` (the load door).

**The residue.** The load door's caller walks `Node::slots()` only.
The expressions no slot addresses — `crate::node::payload_exprs`: a
`Node::Measure`'s `MeasureExpr` and a `Node::Assertion`'s bound — are
asked at the EDIT door alone, where they refuse
`EditError::UnknownPayloadParam` and
`EditError::PayloadParamDimensionMismatch`. So a saved document whose
measure expression reads a parameter the file does not declare, or
reads it at another dimension, LOADS, while the edit door refuses the
same node: the same "a document the load door admits that the edit
doors could not have produced" class the round-three spec ruled on for
slot expressions.

**Measured** by
`crates/editor-core/tests/rv_onepred3_probes.rs`'s
`rv_a_measure_expression_reading_an_undeclared_parameter_still_loads`
(written by the round-three review lane, adopted by its fix pass): a
saved document whose measure expression reads a declared parameter,
with that declaration removed on the wire, LOADS — and the same node
offered to `InsertNode` refuses as
`EditError::UnknownPayloadParam`. The row is green because the gap is
real; it reds, naming this row, on the day the gap is closed, which is
what a unit taking this row should expect to see first.

**Why it was not done there.** Round three's spec named slot
expressions (`check_param_refs`) and ruled that half. The payload half
needs two more `SnapshotError` arms with their tags and F6 rows, and
the payload vocabulary's refusals name a NODE rather than a slot, so
it is a second pair of arms rather than a wider domain for the first.

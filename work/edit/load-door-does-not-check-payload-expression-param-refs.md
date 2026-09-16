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

Not measured by a row yet — the finding is read off the call sites,
not off a fixture — so a unit taking it should measure it first, as
the slot half was measured.

**Why it was not done there.** Round three's spec named slot
expressions (`check_param_refs`) and ruled that half. The payload half
needs two more `SnapshotError` arms with their tags and F6 rows, and
the payload vocabulary's refusals name a NODE rather than a slot, so
it is a second pair of arms rather than a wider domain for the first.

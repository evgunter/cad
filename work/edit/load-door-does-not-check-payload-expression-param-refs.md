---
id: load-door-does-not-check-payload-expression-param-refs
kind: issue
title: The load door asks the param-table rule of slot expressions only; a measure's or an assertion's payload expression is edit-door-only
status: review
opened: 2026-09-16
branch: edit/load-door-payload-refs
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

## Spec (2026-09-16, EDIT orchestrator) — middle tier, branch `edit/load-door-payload-refs`

**Premises, verified against the tree.** `Doc::param_ref_fault(&Expr)
-> Option<ParamRefFault>` (`doc.rs:857`) is the one predicate; the edit
door asks it of every `payload_exprs` leaf in `check_node_slots`
(`edit.rs` ~1941) and refuses `EditError::UnknownPayloadParam { name,
node }` / `PayloadParamDimensionMismatch { name, node, declared,
referenced }`; the load door's `first_slot_param_ref_fault`
(`persist/check.rs:369`) walks `node.slots()` only, placed on
`Walk::SlotParamRef`; `Walk::ORDER` is what `validate_document`
iterates and `Walk::run` is the exhaustive map (PR #2780). The probe
`rv_a_measure_expression_reading_an_undeclared_parameter_still_loads`
is green because the gap is real.

**What lands.**
1. `first_payload_param_ref_fault(snapshot) -> Option<(RecipeNodeId,
   ParamRefFault)>` beside the slot walk, over `payload_exprs` of every
   node, asking the SAME `param_ref_fault`; no second spelling of the
   rule.
2. Two `SnapshotError` arms, `PayloadUnknownDocParam { node, name }` and
   `PayloadDocParamDimension { node, name, declared, referenced }`,
   whose `Display` names the NODE (the payload vocabulary's shape, as
   the edit door's does) and the parameter; the F6 census in
   `display_contract.rs` gains both cases; `crates/pncad-py/src/tags.rs`
   gains `payload_unknown_doc_param` / `payload_doc_param_dimension`
   and the tag inventory the two words (LIB's, mechanical — the
   exhaustive match breaks without them; say so).
3. Placement: a new `Walk::PayloadParamRef` after `Walk::SlotParamRef`
   in `Walk::ORDER`, or the two payload arms placed on
   `Walk::SlotParamRef` renamed `Walk::ParamRef` — take the second only
   if the walk's doc can say in one sentence why slot and payload are
   one walk; either way the exhaustive match places both arms.
4. The probe flips: rewrite it red-first as
   `a_measure_expression_reading_an_undeclared_parameter_refuses_to_load`
   (the same saved document now refuses as the new arm, and `InsertNode`
   of the same node refuses `UnknownPayloadParam` — one row reads both
   doors' answers over one fixture), plus the dimension twin (a
   declared parameter read at another dimension by a measure or an
   assertion bound), plus an assertion-bound row. A round-trip row: a
   document whose measure reads a declared parameter at the declared
   dimension saves, loads, and `bit_eq`s.
5. Order in the walk: a document with BOTH a slot fault and a payload
   fault reports the slot fault (or state and pin whichever order the
   placement in (3) gives — the round-three review's NOTE-2 said an
   unpinned order shifts a corrupt file's diagnosis).

**Mutants, each named with the rows it reds:** drop the payload walk
(the flipped probe reds); walk slots twice instead (the payload rows
red, the slot rows stay green); swap the two arms' payloads.

**Not this unit:** the payload expressions' DIMENSION checks already run
at construction (`MeasureExpr`'s F1 checker; an assertion's bound
against its measure) — say so in the walk's doc; the row does not
re-check them.

**Territory:** `crates/editor-core/src/persist/check.rs`, `src/node.rs`
(EDIT); `crates/editor-core/tests/*` (also TCOST/TINT);
`crates/pncad-py/src/tags.rs`, `src/tests.rs` (LIB, mechanical).
Middle tier: one opus style review with a correctness arm, then a fix
pass.

## Built (2026-09-16)

The load door asks the param-table rule of a node's PAYLOAD
expressions as well as its slots.

`persist::check::first_payload_param_ref_fault` walks
`node::payload_exprs` of every node and asks the same
`Doc::param_ref_fault` the edit door asks — one spelling of the rule,
three callers still. Its answer is named by two `SnapshotError` arms
whose address is the NODE, `PayloadUnknownDocParam` and
`PayloadDocParamDimension`, placed on a new walk
`Walk::PayloadParamRef` that runs after `Walk::SlotParamRef`: a
document broken in both a slot and a payload is diagnosed at the slot.
The exhaustive map places both arms, so neither the walk nor the arms
compile until they are placed; the F6 census in `display_contract.rs`
renders both, and `pncad-py`'s tag map and tag inventory carry
`payload_unknown_doc_param` / `payload_doc_param_dimension`.

The measurement that disclosed the gap flipped: its row is gone from
`rv_onepred3_probes.rs` and the payload contract lives in
`crates/editor-core/tests/load_door_payload_param_ref.rs` — the
undeclared-parameter row over a measure (both doors, one fixture), its
dimension twin, an assertion-bound row, a round-trip row, and the
slot-before-payload order row.

What did not land: nothing from the spec. The spec's option of
renaming `Walk::SlotParamRef` to `Walk::ParamRef` and placing both
pairs on it was not taken — slot and payload are two walks because
their refusals carry different addresses, and the order between them
is a contract a single walk could not state.

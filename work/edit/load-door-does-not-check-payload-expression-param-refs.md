---
id: load-door-does-not-check-payload-expression-param-refs
kind: issue
title: The load door asks the param-table rule of slot expressions only; a measure's or an assertion's payload expression is edit-door-only
status: closed
opened: 2026-09-16
closed: 2026-09-17
pr: 2793
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
now with FOUR callers: the two doors' two walks each, which is what
the predicate's own doc names. Its answer is named by two
`SnapshotError` arms whose address is the NODE,
`PayloadUnknownDocParam` and
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

## After the review (2026-09-17)

One opus style review, APPROVE-WITH-FIXES (0 MAJOR, 3 MINOR, 6 NOTE,
10 style). Every finding taken; what each became is in PR #2793's
**After the review** section, and this is the tracker's half.

**The census moved onto the code, properly this time.** The module
doc's hand-written "Its walks:" list is gone — it had undercounted by
the walk this very PR added, the third round in a row the same list
has been wrong. `Walk` is the roster, `Walk::ORDER` the order, and
`Walk::run` the map; each walk's coverage and its snapshot-only reason
now live on its own variant, and the module doc keeps only what no
roster can carry.

**The order is stated as a contract per adjacency.** `validate_document`
now says which three adjacencies are load-bearing and names the row for
each, and which three are free. A new row measures the last one: an
assertion whose bound reads an undeclared parameter AND whose target is
not a measure reads the PAYLOAD refusal, not `Walk::Snapshot`'s
`AssertionTarget`.

**One mapper over an address.** `slot_param_ref_refusal` and
`payload_param_ref_refusal` are one `param_ref_refusal` over
`ParamRefAddress::Slot(SlotId) | ::Payload`. The edit door's two
destructurings stay two — they feed a different error type with a
different subject, the same shape the slot-dimension and assertion
pairs have at both doors.

**The payload refusals' noun is repaired at both doors**: an
assertion's bound is not a "measurement payload", so both doors now say
"payload expression". The reviewer's fourth probe, which pinned the
defective wording, is rewritten to pin the repaired one and to hold the
two doors to the same word.

**The wire surgery is one body.** `fn doctored` — five byte-identical
copies, measured identical by diff before the move — lives in
`tests/fixture/mod.rs` and is read by all five suites.

**Filed:** `param-ref-refusals-spell-two-facts-four-ways` (EDIT), the
eight names for two facts under four conventions, with the rosters that
ride the names and would have to move in the same PR. Not renamed here.

**Not filed:** no Python row. Both new arms reach the bindings through
`snapshot_error_tag` and the committed tag inventory, which is the
round-three precedent for a load-door arm: the tag words are carried
mechanically and the Python suite's census is what reds if they are
not.

## Closed (2026-09-17, EDIT orchestrator)

Built and merged as PR #2793 after one opus style review
(APPROVE-WITH-FIXES: 0 MAJOR, 3 MINOR, 6 NOTE, 10 style — every one
taken). The load door asks `Doc::param_ref_fault` of every payload
expression (`first_payload_param_ref_fault` on `Walk::PayloadParamRef`,
after the slot walk, with the order pinned), refusing
`PayloadUnknownDocParam` / `PayloadDocParamDimension` addressed by node;
the round-three probe became the red-first row of a new suite. The
review's sharpest finding was round three's own MAJOR shape one more
time — the module doc's hand-written walk census undercounting by the
walk this unit added — and the orchestrator ruled the prose census
deleted in favour of the code's (`Walk::ORDER`/`Walk::run`), which is
what round three intended. One mapper over an address enum replaced
the two load-door mappers; the refusal noun covers an assertion bound
at both doors; the round trip carries a signed zero so `bit_eq`'s
payload half is exercised. The wire-surgery helper the four suites had
copied has one home in `tests/wire/mod.rs` — not `tests/fixture/`,
which is symlinked into the viewer's serde-free test binary (a CI
lesson banked for every lane). Residue in its own file:
`param-ref-refusals-spell-two-facts-four-ways` (eight arm names, four
conventions, and the rosters a rename must move together).

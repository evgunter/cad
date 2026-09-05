---
id: self-boolean-precheck-duplicates-the-doors-duplicate-input
kind: issue
title: add_boolean pre-checks a==b, which DocEdit::InsertNode already refuses as EditError::DuplicateInput
status: closed
opened: 2026-09-04
closed: 2026-09-05
pr: 1930
refs: [set-param-prechecks-what-the-door-refuses, refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake, 1846]
---



Found by the sweep the `set-param-prechecks-what-the-door-refuses`
item owes — *a layer-3 pre-check of a condition a `DocEdit` refuses
typed*. It is the sweep's only other hit and the item's own list of
"correctly flat" arms does not contain it.

## The duplicate

`DocSession::add_boolean` (`crates/viewer/src/session.rs:1226-1229`)
refuses a body that sits in both operand seats:

```rust
if a == b {
    return OpOutcome::refused(Refusal::SelfBoolean { node: a });
}
self.commit(DocEdit::InsertNode { node: Node::Boolean { op, a, b, declare: None } })
```

`DocEdit::InsertNode` refuses exactly that. `Node::Boolean { a, b,
declare }` reports `[a, b]` (+ declare) as its inputs
(`crates/editor-core/src/node.rs:1770-1774`);
`Node::input_fault` finds the first repeat
(`crates/editor-core/src/node.rs:1887-1892`) and `check_node_inputs`
turns it into `EditError::DuplicateInput { node, input }`
(`crates/editor-core/src/edit.rs:1282-1290`), reached from the
`InsertNode` arm at `crates/editor-core/src/edit.rs:1416`.

Verified, not assumed: applying `InsertNode` with a `Boolean` whose two
seats hold one extrude yields
`DuplicateInput { node: RecipeNodeId(3), input: RecipeNodeId(2) }`.

## Two documents state the false version of this

`crates/viewer/README.md`'s ratified list of what a flat `Refusal` arm
is named *"this boolean's operands are the same node"* as an example of
a fact that exists only at layer 3 — fifteen lines above the clause
*"a flat arm must not restate a refusal a door already gives"* that
this item quotes. #1846 corrected that example; the rule was right and
the example was false.

`crates/viewer/tests/combine_ops.rs:279-280` still asserts the layer-3
refusal under the comment *"One body in both seats: the DAG would take
it, the door does not."* The DAG would **not** take it. That sentence
is wrong and is the reason the arm reads as flat; fix it whatever else
is decided.

`crates/viewer/src/session/op.rs:369-370` says *"one node in both seats
refuses `Refusal::SelfBoolean`"*, which is a true statement of what the
op does today rather than a claim about which layer owns the fact — so
it is not false, but it is a third place to update if the arm goes.

## Why it is a separate decision from the `set_param` fix

**The door's sentence is worse than the pre-check's, and improving the
door is the work.** `SelfBoolean` renders "a boolean needs two
different bodies; node N is in both operand seats"
(`crates/viewer/src/session/refuse.rs`); `DuplicateInput` renders
"edit: node N would be left invalid — …", which names no recourse and
does not say what a boolean needs. Deleting the pre-check today hands
the user the second sentence, so the deletion is blocked on the door's
wording being fixed first — not on the pre-check being defensible.

**That is not a carve-out and must not be read as one.** The rule has
no "unless ours is nicer" exception, and the argument above is exactly
the one that would have kept the `set_param` pre-check #1846 deleted if
it had been allowed to stand on its own. It is a sequencing claim: fix
`DuplicateInput`'s wording, then delete the arm. If the door's sentence
is left alone, the arm still goes.

One dependent fact, stated so it is not rediscovered: the check sits
AFTER `require_kind` deliberately, so two profiles in both seats are
reported as "that is not a body" rather than as the narrower complaint
(`session.rs:1223-1225`, and the test's second half). `DuplicateInput`
is raised after the door's own kind-free input checks, so that ordering
survives the deletion — verified, not assumed.

The wording half is DOCM's: `EditError`'s `Display` lives in
`crates/editor-core/src/edit.rs` and the same arm's rendering is one of
the sides of `refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake`.

## What resolving it looks like

Fix `DuplicateInput`'s wording (DOCM), then delete the layer-3 arm
(VIEW), then fix `combine_ops.rs:279-280`'s comment and
`session/op.rs:369-370`'s sentence. Keeping the arm is the other
branch, and it costs a written exception in
`crates/viewer/README.md` saying why this one fact is layer 3's when
the door refuses it — which is the clause #1846 just had to correct.

## Closed

`Refusal::SelfBoolean` is gone — the variant, its `rank` arm, its
`Display` and the pre-check in `add_boolean`. One node in both seats
now reaches `EditError::DuplicateInput` off `Node::input_fault`'s
pairwise-distinct rule, which is the same rule for a split or a list
and is stated once where every node kind reaches it.

The sequencing this item asked for held: the door's wording was fixed
first. What it needed was less than the item expected — the forwarded
`InputFault::Duplicate` clause already states the rule the user has to
satisfy ("a node's inputs are pairwise distinct"), so the fix was the
`edit: ` prefix (above) plus the frame's separator, which had put two
em-dashes in one sentence. The item's "names no recourse" reading was
of the doubled-prefix rendering; the sentence it names now reads
"node 3 would be left invalid: node 2 is taken as an input twice — a
node's inputs are pairwise distinct".

The ordering claim was verified rather than assumed: `require_kind`
still runs first in `add_boolean`, so two PROFILES in both seats are
still reported as "that is not a body", and `combine_ops.rs`'s second
half still pins it.

Both false statements are corrected: `combine_ops.rs`'s "the DAG would
take it, the door does not" (it would not), and `session/op.rs`'s
sentence about which layer refuses the pair.

---
id: self-boolean-precheck-duplicates-the-doors-duplicate-input
kind: issue
title: add_boolean pre-checks a==b, which DocEdit::InsertNode already refuses as EditError::DuplicateInput
status: open
opened: 2026-09-04
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

## A test comment says the opposite

`crates/viewer/tests/combine_ops.rs:279-280` asserts the layer-3
refusal under the comment *"One body in both seats: the DAG would take
it, the door does not."* The DAG would **not** take it. Whatever else
is decided here, that sentence is wrong and is the reason the arm
reads as flat.

## Why it is not the `set_param` fix

Two things make it a separate decision rather than the same deletion:

- **The wording is not equivalent.** `SelfBoolean` renders "a boolean
  needs two different bodies; node N is in both operand seats"
  (`crates/viewer/src/session/refuse.rs`); `DuplicateInput` renders
  "edit: node N would be left invalid — …". Deleting the pre-check
  hands the user the second sentence. That may be right — the fix for
  a weak door message is to fix the door — but it is a user-visible
  wording decision, not a mechanical deletion, and
  `crates/viewer/README.md` ratifies the affordance-wording rule that
  bears on it.
- **The ordering is a ratified decision.** The check is placed AFTER
  `require_kind` deliberately, so two profiles in both seats are
  reported as "that is not a body" rather than as the narrower
  complaint (`session.rs:1223-1225`, and the test's second half).
  `DuplicateInput` at the door is raised after the kind gate too, so
  the order survives — but that has to be stated, not assumed.

## What resolving it looks like

Either delete the arm and let `DuplicateInput` speak (fixing its
wording first if it is not good enough for a user), or keep it and say
in `crates/viewer/README.md`'s "A flat arm must not restate a refusal a
door already gives" clause why this one is the exception. Fix the
`combine_ops.rs` comment either way.

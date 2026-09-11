---
id: is-instance-collapses-absent-and-wrong-kind
kind: issue
title: display::is_instance answers bool because its one caller wanted one, collapsing the two states the crate just argued are different news
status: open
opened: 2026-09-05
refs: [sweep-blind-spots-the-precheck-sweep-could-not-see, prune-discards-the-fault-that-explains-the-supersession, 1886]
---


Found by #1886's style review, and it is that unit's own declared blind
spot found one caller away in its own file. The `view/prune-report`
lane's sweep for *a computed report discarded before the user sees it*
named three things its patterns could not match, the third being **a
report weakened by a TYPE rather than at a call site — a door whose
signature is `bool` because its only caller wanted one**. The reviewer
ran a differently-shaped sweep (bool-returning doors over `display.rs`
and `frame.rs`) and found the instance in one grep.

That is the whole argument for this file: a disclosed blind spot is a
work order, not a discharge, and this one was discharged by the next
person to look.

## The door

```rust
pub fn is_instance(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    matches!(doc.node(node), Some(Node::InstantiatePart { .. }))
}
```

`crates/viewer/src/display.rs:238`. `None` (the node is not in the
document) and `Some(other_kind)` (it is there and is not an instance)
both answer `false`.

**Those are exactly the two states #1886 spent a unit arguing are
different news to a person.** That PR split `DisplayFault::NoSuchNode`
off `NotAnInstance` inside `drawn_targets` on the ground that naming
an id the tree no longer draws, as though it were merely the wrong
kind, is a sentence that misleads. `is_instance` is the same
distinction, in the same module, answered by a type that cannot carry
it.

## Why it is not urgent, stated so it is not overstated

**The one caller does not care today.** `PropertiesPane::instance_ui`
(`crates/viewer/src/pane/properties.rs:333`) early-returns for a
`false`, drawing no per-instance section — and drawing nothing is the
right answer for both an absent node and a datum. So there is no live
defect. The reviewer's confidence was `sure` on the door's shape and
`unsure` on whether the caller minds; that reading is recorded here
rather than sharpened into a finding it does not support.

**The risk is the second caller.** A `pub` predicate re-exported from a
module whose whole current business is telling those two states apart
will be reached for by a door that does need them apart, and it will
answer plausibly and wrongly.

## The class, and where else to look

The pattern is *a `bool`-returning door over a question with more than
two answers*, and the reviewer named three more of the shape without
adjudicating them: `frame::folded_moved`, `frame::ChooserBackend::usable`,
`frame::acts`, and `tools::commits_open_tool`. Whoever takes this
sweeps those rather than fixing one.

## Home

VIEW's: `crates/viewer/src/display.rs`, with the caller in
`crates/viewer/src/pane/properties.rs`.

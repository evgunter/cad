---
id: is-instance-collapses-absent-and-wrong-kind
kind: issue
title: display::is_instance answers bool because its one caller wanted one, collapsing the two states the crate just argued are different news
status: closed
opened: 2026-09-05
refs: [sweep-blind-spots-the-precheck-sweep-could-not-see, prune-discards-the-fault-that-explains-the-supersession, 1886]
branch: vnews/is-instance-two-states
closed: 2026-09-20
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

`crates/viewer/src/display.rs`, `is_instance` (`:351` at the fix
lane's merge base; `:248` as filed named `DisplayFault`'s header,
not this function). `None` (the node is not in the
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
(`crates/viewer/src/pane/properties.rs`, the guard at the top of
`instance_ui` — `:382` at the fix lane's merge base; `:338` as filed
named `Standing::live`'s `present: bool`, not this call)
early-returns for a
`false`, drawing no per-instance section — and drawing nothing is the
right answer for both an absent node and a datum. So there is no live
defect. The reviewer's confidence was `sure` on the door's shape and
`unsure` on whether the caller minds; that reading is recorded here
rather than sharpened into a finding it does not support.

**The risk is the second caller.** A `pub` predicate in a module whose
whole current business is telling those two states apart will be
reached for by a door that does need them apart, and it will answer
plausibly and wrongly.

**This row said "re-exported" and that was never true**: `is_instance`
is `pub` in a `pub mod`, reachable as `viewer::display::is_instance`,
and `lib.rs`'s `pub use display::{…}` list never named it. Corrected
here rather than only in the fix lane's report, which is where the
correction first landed and stopped — the failure
`work/vnews/plan.md`'s dispatch rules name, committed by the lane that
had just read the rule.

## The class, and where else to look

The pattern is *a `bool`-returning door over a question with more than
two answers*, and the reviewer named three more of the shape without
adjudicating them: `frame::folded_moved`, `platform::ChooserBackend::usable`,
`frame::acts`, and `tools::commits_open_tool`. Whoever takes this
sweeps those rather than fixing one.

## Home

VIEW's: `crates/viewer/src/display.rs`, with the caller in
`crates/viewer/src/pane/properties.rs`.

## Fixed (VNEWS, 2026-09-19)

`is_instance` is now
`instance_check(doc, node) -> Result<(), AdmissionFault>`, raising
`NoSuchNode` for an id the document does not hold and `NotAnInstance`
for a node of another kind. The shape reuses the vocabulary #1886
established rather than minting a second one, for this program's own
reason: two enums over the same states are the defect two other rows
on this slate already carry.

`drawn_targets` now runs it instead of repeating the match, so the
two states are decided in one place and `AdmissionFault::NoSuchNode`'s
doc — which said *"[`is_instance`] is NOT where this is decided and
still collapses the two states into `false`"* — says where it IS
decided instead.

**No behaviour changed**, and the fix pass established that no
behaviour SHOULD change at the one caller. The review asked whether
`instance_ui` owes the absent id a sentence. The answer is no, and the
reason is that the sentence is already on screen:

- The absent arm IS reachable here — `Standing`'s ratified rule is that
  *a vanished reference is a STATE, not an event*, so a selection
  outlives the node it names and this door really does meet an id the
  document no longer holds.
- In that frame `standing_ui` has already drawn the vanished verdict
  from `self.doc().node(node).is_some()` — the SAME lookup on the SAME
  document, three lines above, in the same pane.
- `Standing`'s rule has a second clause — *while the affordances that
  need a live entity switch off* — and the section not drawing IS that
  clause, not a discarded fault.
- A sentence at the affordance would mint the node-side twin of
  `three-spellings-say-a-parameter-is-not-declared`, which is open
  against the parameter half of this same function.

So the panel keeps `.is_err()` and the DOC was narrowed instead: the
first draft's *"a user now reads … at every one of those doors"* was
false precisely about the door this change added, and now says the arm
reaches every OPERATION downstream, with the panel's silence and its
reason stated where the fault is defined.

`instance_check_tells_an_absent_node_from_a_wrong_kind`
(`crates/viewer/tests/assembly_display.rs`) holds the two arms AND
their two renderings — the renderings being the thing the split exists
to buy — verified red by swapping the arms.

**On the charter test, honestly:** no reader sees a difference today,
so this is not news by `plan.md`'s own rule. What it is instead is one
decision site where there were two (`drawn_targets` no longer repeats
the match) and a door the next caller cannot get wrong. Whether that
earns a place on this slate is the orchestrator's call and is argued in
the PR body rather than assumed here.

The sweep's dispositions are in the PR body.

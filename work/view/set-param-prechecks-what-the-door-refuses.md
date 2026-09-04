---
id: set-param-prechecks-what-the-door-refuses
kind: issue
title: set_param pre-checks a parameter's existence, which DocEdit::SetDocParamValue already refuses typed
status: review
opened: 2026-09-04
refs: [viewer-session-god-module-split, self-boolean-precheck-duplicates-the-doors-duplicate-input, sweep-blind-spots-the-precheck-sweep-could-not-see, refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake]
branch: view/set-param-precheck
pr: 1846
---


Found answering Ev's question on PR #1801 — *what was the reason to
move them?* — about four `Refusal` arms the PR claimed were facts about
the document. Three were not. This one is, and it is a defect rather
than a boundary question.

## The duplicate

`DocSession::set_param` (`crates/viewer/src/session.rs:2577`) checks
the parameter exists before committing:

```rust
if !self.committed_doc().params().contains_key(name) {
    return OpOutcome::refused(Refusal::NoSuchParam(name.clone()));
}
self.commit(props::param_edit(name.clone(), value))
```

`props::param_edit` builds `DocEdit::SetDocParamValue`
(`crates/viewer/src/props.rs:606`), and `apply` refuses exactly this
case with `EditError::DocParamNotDeclared { name }`
(`crates/editor-core/src/edit.rs:429`), whose own doc-comment states
the rule: *"the value door carries an existing declaration forward, so
there has to be one; declaring a parameter is `DocEdit::SetDocParam`'s
job."*

`Refusal::Edit(Box<EditError>)` already exists and already delegates.
So the door's typed refusal would surface unchanged if the pre-check
were simply deleted.

## Why it is a defect and not a preference

The codebase states the discipline in its own words, one screen away,
in `delete_node`'s doc-comment (`session.rs:3046`):

> An id the document does not hold takes the single-edit path so the
> typed refusal comes **from the door rather than from here**.

Two spellings of one rule drift, and this pair can drift in a way
nothing catches: if `DocParamNotDeclared`'s wording, recourse or
conditions change, `NoSuchParam`'s do not, and the user sees whichever
of the two the pre-check reached first — which is always the pre-check.

## What is NOT part of this

The other two `NoSuchParam` sites are **lookups, not pre-checks**, and
are correctly flat:

- `begin_param_gesture` (`session.rs:2613`) needs the parameter's
  *dimension* to open the gesture. No edit is committed, so no door
  refuses on its own.
- the range probe's `BoundsTarget::Param` arm (`session.rs:2469`) needs
  its value and unit for the same reason.

`NoSuchSlot` is also correctly flat both times: it means the
*properties panel* has no row for that slot
(`props::slot_rows`, `session.rs:2299` and `:2454`), which is a
layer-3 projection and not editor-core's slot vocabulary.

`ParamExists` (`session.rs:2590`) is correctly flat and deliberately
so: `DocEdit::SetDocParam` has create-or-replace semantics and the
session narrows its create door to create-only, which is a layer-3
policy choice its doc-comment already records. And `EmptyName`
(`session.rs:2783`) validates a document *name* string before
`Doc::empty_derived` — not a document fact at all.

## The sweep this owes

The shape is *a layer-3 pre-check of a condition a `DocEdit` refuses
typed*. The pattern to sweep is every `OpOutcome::refused` that
precedes a `self.commit(...)` in the same function. **What that pattern
cannot match** is a pre-check separated from its commit by a helper
call, and a pre-check whose refusal is raised through `?` rather than
an early return.

## Home

VIEW's: `crates/viewer/src/session.rs`. Rides unit 1's ground and is
not unit 1's fix — the boundary rule says what the discipline is; the
sweep that enforces it is work.

## What landed

The pre-check is gone. `DocSession::set_param` commits
`props::param_edit` unconditionally, so an undeclared name is refused
by `DocEdit::SetDocParamValue` and reaches layer 3 as
`Refusal::Edit(EditError::DocParamNotDeclared { name })` —
`commit_action` boxes every `apply` failure into that arm and nothing
between it and `OpOutcome` inspects it. `set_param`'s doc-comment now
states the rule in `delete_node`'s words.

`Refusal::NoSuchParam` KEEPS its two readers and stays: the range
probe's `BoundsTarget::Param` arm
(`crates/viewer/src/session/probe.rs:196`) and `begin_param_gesture`
(`crates/viewer/src/session.rs:977`). Both are lookups — one needs the
parameter's value and unit, the other its dimension — and neither
commits an edit, so no door refuses on their behalf. The arm's
doc-comment now says so, which is what keeps a future reader from
re-adding a pre-check to it.

`story_parametric`'s walk asserted the layer-3 arm on the `SetParam`
path; it now asserts the door's, still checking that the payload names
`"tapper"`. That is the whole user-visible change and it is stated at
the assertion. No rendered text is asserted anywhere in the tree.

**The item's citations were pre-split and stale** (`session.rs:2577`,
`:2613`, `:2469`, `:2590`, `:2299`, `:2454`, `:2783`, `:3046`). The
four sites it names as correctly flat all check out at their post-split
lines: `begin_param_gesture` (`session.rs:977`), the probe's param arm
(`session/probe.rs:196`), `create_param`'s `ParamExists`
(`session.rs:959` — `write_doc_param` has no existence check at all,
`editor-core/src/edit.rs:1144`), and both `NoSuchSlot` sites
(`session.rs:837` in `driver_of`, `session/probe.rs:181`).

`crates/viewer/README.md`'s ratified list of what a flat `Refusal` arm
is named *"this boolean's operands are the same node"* as a fact
existing only at layer 3. This unit proved by execution that
`DocEdit::InsertNode` refuses it as `EditError::DuplicateInput`, so the
example was false — corrected here, with the rule left standing and a
sentence added saying that an entry in that list names a fact `apply`
has been read for.

The sweep's hit list is in the PR; what it could NOT see is
`sweep-blind-spots-the-precheck-sweep-could-not-see`. Its one other hit
— `add_boolean`'s `a == b` pre-check — is
`self-boolean-precheck-duplicates-the-doors-duplicate-input`. The
message the change now shows a user is
`refusal-edit-arm-doubles-a-prefix-and-splits-one-mistake`. The
whole-file finding the review turned up alongside it is
`session-clearing-walk-is-hand-maintained-three-times`.

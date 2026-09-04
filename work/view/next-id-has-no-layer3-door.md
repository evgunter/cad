---
id: next-id-has-no-layer3-door
kind: issue
title: Doc::next_id is pub(crate), so DI1's minting-entry walk has no layer-3 door
status: open
opened: 2026-09-04
refs: [layer3-recipenodeid-aliases-across-rewinds]
---


## What is missing

`docs/DOCM-IDENTITY-DESIGN.md` DI1 states the mechanism for every
layer-3 hold of a `RecipeNodeId`: a hold carries the id plus the
history entry that minted it, and the history "computes at pick time by
walking up until the counter drops below the id (`History::entry`,
`Doc::next_id`)."

Half of that is reachable and half is not:

- `History::entry` is public (`crates/viewer/src/history.rs:192`) and
  returns the `Entry`, whose `doc()` is public too
  (`crates/viewer/src/history.rs:70`).
- **`Doc::next_id` is `pub(crate)`** with no accessor beside it
  (`crates/editor-core/src/doc.rs:315`; the field's own doc-comment
  states the monotonicity DI1's walk rests on, and `edit.rs:1369`
  restates it — "next_id is NOT decremented: ids are never reused").

So the comparison DI1's walk is defined by — *is this entry's
`next_id` still above the held id* — is not expressible from layer 3.
The viewer README's G1 boundary rules make reaching past the public
surface a type-level discipline rather than a preference, so the walk
cannot be written until editor-core offers the reading.

## Why this is filed rather than fixed

`crates/editor-core` is DOCM's territory, not this program's
(`work/docm/program.md`). A door added from a VIEW unit branch would be
a cross-program edit made by diff.

The shape is DOCM's to choose and there is more than one: a `pub fn
next_id(&self) -> u64` on `Doc`, or — narrower, and closer to what the
caller actually asks — a predicate that answers *could this document
have minted this id* without exposing the counter, which keeps the
monotonicity argument on the side that owns it. The second is the
better door if DI1's walk is the only consumer, and this program has no
standing to pick.

## Who is blocked

`layer3-recipenodeid-aliases-across-rewinds` — DI1's build, which is
this program's, and which cannot start its holder sweep without the
reading.

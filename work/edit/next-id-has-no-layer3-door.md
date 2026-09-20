---
id: next-id-has-no-layer3-door
kind: issue
title: Doc::next_id is pub(crate), so DI1's minting-entry walk has no layer-3 door
status: spec
branch: edit/minted-id-door
opened: 2026-09-04
refs: [layer3-recipenodeid-aliases-across-rewinds]
---


## What is missing

`crates/editor-core/IDENTITY.md` DI1 states the mechanism for every
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

## Ruled and spec'd (2026-09-20, EDIT orchestrator) — E-class, wave 15, branch `edit/minted-id-door`

**Ruling: the narrower door — a predicate, not the counter.** This
row was filed against DOCM; `crates/editor-core` is EDIT's now, so the
choice the row said it had no standing to make is made here, and the
row's own argument carries it: DI1's walk asks one question of a
history entry's document — *could this document have minted this id*
— and a predicate answers exactly that while the monotonicity
argument stays on the side that owns the counter. `Doc` gains
`pub fn has_minted(&self, id: RecipeNodeId) -> bool` (or the name the
tree's vocabulary already uses for "minted" — grep `minted` in
`doc.rs`/`edit.rs` and match it), true iff `id.0 < self.next_id`, with
a doc that cites DI1 and the field's own monotonicity sentence and
says what the answer does NOT mean (a minted id may be deleted;
liveness is `Doc::node`'s question). `next_id` stays `pub(crate)`.

Rows: the predicate on a fresh document (nothing minted), after an
insert (that id and every lower one minted, the next not), after a
delete (still minted — ids are never reused, `edit.rs`'s own
sentence), and across a save/load round trip (the counter is part of
the value). `IDENTITY.md` DI1's parenthetical "(`History::entry`,
`Doc::next_id`)" names the door the walk reads: it is re-worded to
name the predicate — a description the code moved, landing with the
change (say in the PR body where you looked for a ratification:
`git log --all -S'Doc::next_id' -- crates/editor-core/IDENTITY.md`).
Nothing in the viewer moves here — DI1's walk is VIEW's
`layer3-recipenodeid-aliases-across-rewinds`, which this row unblocks;
the row's `## Who is blocked` gets an `## Unblocked` paragraph naming
the door. Territory: `doc.rs`, `IDENTITY.md`'s one parenthetical
(EDIT); `crates/editor-core/tests/*` (TCOST/TINT). E-class: green CI
and the orchestrator's read; no review lane.


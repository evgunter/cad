---
id: sibling-branches-mint-one-step-id-for-different-steps
kind: issue
title: Two edits applied to one base mint the same step id for different steps
status: open
opened: 2026-09-25
priority: P2
---


## What

A step id is unique within one document (`names/README.md`, "N1, the
profile pieces"), but not across the documents that branch from one
value. `Doc::next_step` (`crates/editor-core/src/doc.rs`) is part of the
document value, and `apply` is pure: undo is keeping the prior value.
So after an undo followed by a new edit, or two applies from one base,
each branch mints from the same counter, and the same `StepId` names a
different step in each.

Within one document nothing aliases. A name only crosses between
branches when a caller carries it: a name held in the UI across an undo,
a name copied from one branch's document into the other's. Carried that
way, a name on a step the undone edit minted resolves in the other
branch to whatever step that branch minted under the same number, with
no report.

Node ids share the behaviour: `Doc::next_id` is branch-local in the same
way, and a `StableName` carried across branches can denote a different
node. The step id adds no new class, only a second counter of it.

## Where to start

- The minting sites: `ProfilePayload::mint_step_ids` (`program.rs`) and
  `settle_step_ids` (`edit.rs`); and `next_id` in `doc.rs`.
- The question for the owner is whether ids are unique per document (as
  now, with names not portable across branches by contract), or per
  history, which needs a counter that is not part of the value.

Found by review of #3223.

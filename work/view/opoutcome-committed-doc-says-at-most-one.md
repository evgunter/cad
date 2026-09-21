---
id: opoutcome-committed-doc-says-at-most-one
kind: issue
title: OpOutcome::committed's doc says "at most one per op" and delete_node's cascade has always returned more
status: open
opened: 2026-09-21
priority: P4
cost: E
refs: [opoutcome-superseded-has-no-production-reader]
---

## What

`crates/viewer/src/session/op.rs`, `OpOutcome::committed`:

> The edits that entered the history — at most one per op, and
> exactly one for a gesture's whole drag.

`DocSession::delete_node` commits the whole dependent cone through
`commit_action`, one `DocEdit::DeleteNode` per doomed node, and every
one of them lands in `committed`. Deleting a feature two consumers
deep has answered with three edits for as long as the cascade has
existed. AUTH-3's `ProfilePlane::NewXy` adds a second door that
answers with two, but it did not make the sentence false; it was false
before that diff.

## Why it is filed and not fixed here

Found by AUTH-3 while adding the `minted` field beside it. The
sentence is one line and the fix is obvious — *"the edits that entered
the history, as one action and therefore one undo; several for an
action that commits several"* — but it is a claim about `OpOutcome`'s
contract that `view` owns, and the row beside it
(`opoutcome-superseded-has-no-production-reader`) is already reading
this type's fields for what does and does not consume them. Taking
both in one sitting is cheaper than taking this alone.

Nothing reads the sentence as a bound: the twenty-one test sites that
read `committed` assert a length rather than assuming one.

---
id: a-declare-orphaned-by-a-cascade-is-never-reported
kind: issue
title: A Declare orphaned by a cascade that deleted its consumer is silent forever
status: open
opened: 2026-09-17
---


## What

Deleting a node cascades its dependents. A `Declare` is not a
dependent of the boolean or union that consumes it — the edge runs the
other way — so a cascade that deletes the consumer leaves the
`Declare` behind, and nothing ever mentions it again:

- the delete reports no strand, correctly: a sited pair's names are
  the MEMBERS' and the members are untouched, and a site is a node id
  rather than a name, which `stranded_references`
  (`crates/editor-core/src/edit.rs`) deliberately does not report;
- the next evaluation says nothing: a `Declare` with no consumer
  evaluates to its own payload and refuses nothing
  (`eval/mod.rs`'s `Declare` arm);
- no later door reads it, because the node that would have is gone.

Measured by
`crates/editor-core/tests/review_decl_r1.rs`'s
`a_declare_orphaned_by_a_cascade_refuses_nothing`: deleting a member
of a two-member declared union cascades the union away, the `Declare`
survives, the maintenance list is empty and the evaluation is clean.

## Why it is a row and not a bug of this unit

The state is not wrong — a `Declare` can legitimately exist
unconsumed, which is premise 5's ruled position (the load door asks
nothing new, and a site that is not the consumer's operand is the
evaluation's refusal rather than the edit's). What is missing is that
nothing ever tells the author the node is now inert. The same shape
predates sited declarations: the orphan was silent under the
member-space payload too. What sited declarations change is how easy
it is to reach — the `Declare` is now authored FIRST, so any cascade
through its consumer produces one.

## The shapes a fix could take

- report it as maintenance at the delete that orphans it (a new
  `Maintenance` arm — "this Declare has no consumer"), which is the
  DM7 door's vocabulary already;
- report it at the next evaluation, as a document-level finding rather
  than a node refusal;
- leave it and say so in `Node::Declare`'s docs, which is the cheapest
  and is what holds today by default rather than by decision.

Ground: `crates/editor-core/src/{edit.rs, eval/mod.rs}` — EDIT's.

Filed by the EDIT-DECL fix pass, PR #2809.

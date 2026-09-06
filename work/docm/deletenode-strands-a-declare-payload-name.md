---
id: deletenode-strands-a-declare-payload-name
kind: issue
title: "DeleteNode leaves a Declare whose payload names a dead node: DeleteWouldDangle reads inputs only"
status: open
opened: 2026-09-06
refs: [DOCM-7, 2028]
---


## What

Found by DOCM-7's R1 review (NOTE-8), probe
`r1_delete_doors_around_a_declared_union` on `docm/7-review-r1`.

Delete the UNION of a declared union and the edit succeeds, leaving the
`Declare` behind with a payload naming a node that is no longer in the
document. The document saves, loads and evaluates. Delete the `Declare`
instead and the edit refuses `DeleteWouldDangle`, because that one IS a
DAG edge.

`crates/editor-core/src/edit.rs:1530` — the delete door's dangle check
reads `node.inputs()` and nothing else, so a reference carried in a
PAYLOAD (`Node::payload_names`: a `Declare`'s pairs, a blend's
selection, a mate's two heads) is invisible to it.

## Why it is not a bug in the door

This is ruled, not overlooked. The insert door's own comment
(`edit.rs:1475`) states the D3 carve-out: payload name refs must point
at live nodes AT EDIT TIME, they are not DAG edges, and "later deletes
may strand them (N5), so this is the ONLY door that checks". N5's rung 1
(`ResolveError::NodeGone`) exists precisely to diagnose a stranded name,
and `DocEdit::Rebind` is its repair.

So extending `DeleteWouldDangle` to cover payload references would not
be a reuse of one predicate at a second door — it would reverse the
carve-out, for every node kind that carries a payload name, and make
rung 1 unreachable by the route it was written for. DOCM-7 therefore
files this rather than building it (the fix brief left the call to the
implementer).

## The class, and what a ruling would decide

The union shares this with the pair boolean, the fillet/chamfer
selection and the mate — it is not new. What DOCM-7 adds is a payload
name that points at a node the payload's own consumer depends on, which
makes the stranded state easier to reach by hand than it was.

The question for a ruling: whether a payload reference should be a
WEAKER edge (delete warns, or offers to rebind) rather than no edge at
all, and whether the answer differs for a name that points at the
consuming node's own space.

## Consequence for DOCM-7's own acceptance

A5 asked for the `declare` edge to be exercised through change and
removal. There is no edit that CHANGES or REMOVES a live node's declare
edge (DM6: inputs are not rewired), and deleting the `Declare` refuses
while the union is live. So A5's change/remove half is unexercisable
through the doors that exist; DOCM-7 discloses that rather than
inventing an edit for it.

## Where it stands

Open, unscheduled, on DOCM's slate.

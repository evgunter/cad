---
id: a-declared-union-has-no-one-pass-authoring-path
kind: issue
title: "A union with a declaration cannot be authored in one pass: the working path inserts a duplicate union and rebinds"
status: open
opened: 2026-09-06
refs: [DOCM-7, 2028]
---


## What

DOCM-7 gives `Node::Union` a `declare` edge whose pairs name entities in
the UNION's own name space. A member-space name therefore carries the
union's own node id — and there is no order of edits that authors the
two nodes directly, for two doors that are each correct on their own:

- `crates/editor-core/src/edit.rs:1482` — `InsertNode` admits a payload
  name only when `name.node` is already live
  (`EditError::DeclareNamesMissingNode`; the ruled D3 carve-out). So the
  `Declare` cannot be inserted before the union it names.
- `crates/editor-core/src/node.rs:2429` and DM6 — no edit rewires a live
  node's inputs. So the union cannot be inserted first and given its
  `declare` edge afterwards.

The path that works, and the only one, is the fixture `declared_union`
in `crates/editor-core/tests/docm7_union_declare.rs`: insert a FIRST
union with no declaration, write the `Declare` in that union's space,
insert a SECOND union carrying the edge, `Rebind` every name from the
first union's space onto the second, then delete the first. The
intermediate document holds a duplicate union, and the rebind loop has
its own limit — a name that appears in more than one declared pair (a
chain of contacts declares the middle member's faces twice) must be
rebound ONCE, or the second `Rebind` refuses `RebindNoReferences`.

For an accumulation-entity name (a `Seam`/`Merged`/`Fragment` row of the
union itself) there is no other path even in principle: such a name has
no spelling that predates the union, since it is minted by the union's
own evaluation.

## Why it is chrome's

Nothing in `editor-core` is wrong. What is missing is a SEAT: the
operation "union these bodies and declare these contacts" is one
authoring act and reaches the document as five edits, three of which
exist only to work around the ordering. A seat that composes them (or an
edit that attaches a declaration to a live node, which is a DOCM
question about DM6) is the fix. `work/chrome/placed-union-has-no-session-op.md`
is the neighbouring gap for the same node.

## Where it stands

Open, unscheduled. `work/chrome/plan.md` has no union-seat unit; this
file is the placeholder for one. DOCM-7 ships with the two-pass shape
pinned as a measured fact rather than left implicit —
`a_declare_cannot_name_a_union_that_does_not_exist_yet` and
`a_declared_unions_document_loads_but_does_not_replay_in_order`
(`crates/editor-core/tests/docm7_union_declare.rs`) — the second of which
also records that such a document LOADS (the load door checks the mint
counter, not `order()`) while re-inserting its nodes in document order
refuses. This node is the first to rely on that asymmetry.

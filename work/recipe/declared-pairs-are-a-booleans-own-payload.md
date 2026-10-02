---
id: declared-pairs-are-a-booleans-own-payload
kind: issue
title: A boolean's and a union's declared contact pairs become the node's own payload, settable on a live node (Ev, #3587, A2)
status: open
opened: 2026-10-01
priority: P1
cost: H
---


**Ruled by Ev on #3587 (2026-10-01): A2.** The decision record is in `work/doors/a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added.md`, its "Declaring a contact on a live boolean" and "Ruled" sections. The designers' reports are on #3587. Fork-log row 24.

## Why

The viewer's boolean door evaluates every boolean synchronously before recording it (AUTH-9), so that a refusing union can be offered a declaration before it is committed. It has to, because a committed boolean cannot be given a declaration afterwards: no edit rewires a live node's `declare` input (DM6). That costs a double evaluation, a blocked frame, and a whole-document evaluation right after an open. It also leaves every boolean made flush by a later edit with no recourse but cascade-delete and re-add, which is the common case once anything sits on a boolean's face.

A declaration is a parameter, not an operand. It carries no material and mints no names, and its sites must be the consumer's operands. So it can live on the node. The `Declare` node has no other use (checked for Ev): its value is read only through `declared_pairs`.

## What to build

- **Node shape.** `Node::Boolean { op, a, b, declare: Vec<DeclaredPair> }` and `Node::Union { members, declare: Vec<DeclaredPair> }`, where an empty list means undeclared. This changes DM4's ratified shape, as ruled. `Node::Declare` goes.
- **Edit.** A whole-list edit shaped like `SetMembers`, e.g. `SetDeclare { node, pairs }`, with nothing inferred. It is settable on a live node. With no edge, DM6 needs no exception.
- **What becomes unrepresentable:** `DeclareInputNotDeclare`, the persistence check's `DeclareInput`, `Maintenance::OrphanedDeclare`, the "an orphaned `Declare` joins the root set" quirk, and `refactor`'s `Declare` remap. The last one becomes the payload's own remap.
- **Persistence:** a schema bump.
- **Python:** `Node.declare(findings)`, `Doc.declare`/`Doc.declare_all` and `Node.boolean(..., declare=)` change shape (`docs/guide/north-star-audit.md` G19). `names::declare_node` builds a pair list, not a node.
- **The kernel's recourse.** `UndeclaredContactFinding::recourse` ("declare the candidate pair … and wire it into the Boolean's declare input") must name the new edit.

## Not in this unit

- The viewer half: AUTHOR's `the-boolean-door-evaluates-its-boolean-twice` and `a-union-that-becomes-flush-later-…`, both blocked on this row. They cover the plain commit, the row control and the end of the door judge.
- The kernel reporting every undeclared pair at once: ZIP's `a-boolean-reports-one-undeclared-contact-per-refusal`.

## Ground

EDIT: `node.rs`, `edit.rs`, persistence, `refactor.rs`. Also WIRE (`eval/wire.rs` `declared_pairs` and `wire_boolean`/`wire_union`) and LIB (`pncad-py`).

Filed by the AUTHOR orchestrator on Ev's ruling.

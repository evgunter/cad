---
id: pncad-py-doc-has-no-node-kind-read-door
kind: issue
title: pncad-py: Doc exposes no node-kind read door, so a Python row cannot say which node is the group
status: open
opened: 2026-09-03
---

Found while landing LIB-DIETOOL's Python re-authoring of the `die_tool`
corpus document.

## The gap

`Doc`'s read surface is `order()`, `node_count`/`__len__`,
`placement`/`placements`, `reference` and `interface`
(`crates/pncad-py/pncad.pyi:1290-1370`). None of them answers **what
kind of node** a `NodeId` holds. The evaluation side does not close it
either: `Evaluation.value(n).kind` is the VALUE's kind — a
`PlacedUnion` and a `Boolean(Union)` both answer `"body"`
(`crates/pncad-py/pncad.pyi:1944-1954`).

So the claim the group boolean exists to make cannot be asserted
directly from Python. The Rust row says it outright by matching on the
node:

    crates/editor-core/tests/lib_placedunion.rs:240
    fn the_die_tool_is_one_node_and_still_cuts()
      -> counts Node::PlacedUnion / Node::Boolean{Union} / Node::Transform
         over doc.order() and asserts (1, 0, 0)

Its Python mirror
(`crates/pncad-py/tests/test_placed_union.py::TestTheDieTool::
test_the_tool_is_one_node_and_still_cuts`) can only assert the node
COUNT — seven against the pairwise chain's eighteen — and then lean on
the saved-text byte pin, whose JSON happens to name every node's kind,
to settle which of the seven is the group. That works and is honest,
but it routes a structural question through the persistence door, and
`test_placed_union.py`'s own header bills the file as the mirror of the
Rust suite.

## Why it is worth closing

The audit's discipline is that a YES is EXECUTED by the Python suite.
Any future row about recipe SHAPE — the group replacing a chain, a
pattern node refused where a group is accepted, a fillet sitting where
a chamfer was — hits the same wall, and each will invent its own
workaround. It is also the read half of a write surface that is fully
bound: `Node.placed_union_at` authors the node, and nothing reads it
back.

## Shape of the fix

A `Doc.node_kind(node) -> str` (or a `Doc.node(node)` projection in the
shape `py/value.rs` already uses for `Value`), with the string drawn
from the same exhaustive `match` over `Node` that the wire uses, so a
kernel-side node kind added without a Python spelling is a compile
error. It lands with its `pncad.pyi` entry, its binding-census row and
its stub test, like every other door.

## Home

`work/lib/` — `crates/pncad-py`'s read surface and `pncad.pyi` are
LIB's territory glob and charter.

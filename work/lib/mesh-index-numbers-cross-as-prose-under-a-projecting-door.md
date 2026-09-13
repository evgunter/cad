---
id: mesh-index-numbers-cross-as-prose-under-a-projecting-door
kind: issue
title: NodePickError projects its arms' fields but not the index arm's three numbers
status: closed
opened: 2026-09-08
refs: [pncad-py-seven-doors-lack-field-projection]
closed: 2026-09-08
---

Disclosed by LIB-CUR5, which carried `MeshPickError` and projected its
DISCRIMINANT (`NodePickError.index_variant`) and not its fields.

## The finding

`NodePickError` is on the projecting side of the split
`crates/pncad-py/src/tags.rs`'s header describes: it crosses `node`,
`through`, `kind` and `body` as attributes, present on every arm and
`None` where the arm does not carry them. The index arm is now the one
exception. `MeshPickError::PositionOutOfRange`
(`crates/editor-core/src/resolve/pick.rs:88`) carries `patch`,
`triangle` and `index` — three public numbers, no arena key by the
type's own contract — and none of them reaches Python except inside
the kernel's `Display` prose (`crates/pncad-py/src/py/pick.rs`, the
`node_pick_err` attribute list).

## Why it is not `pncad-py-seven-doors-lack-field-projection`

That issue is a roster of doors that cross as tag-plus-message ENTIRELY.
This door does not: it projects four attributes already, so what is
recorded here is narrower and different in kind — a projecting door
with one arm's payload unprojected, because the payload belongs to a
type that is not itself raisable and so has no door of its own to
carry its numbers. The convention `py/pick.rs` states for the
forwarding arms ("a forwarded arm does not bring the inner refusal's
extra ATTRIBUTES — a tessellation refusal's numbers stay on
`TessellateError`, where `Body.tessellate` raises them") has no
counterpart here, because nothing raises a `MeshPickError`.

## What it costs, stated at its real size

The arm reports a tessellated mesh whose triangles index outside their
own position buffer — a kernel defect, unreachable from any authoring
door, and unconstructible from Python. So the cost today is zero
callers, and the reason to record it is the standing rule rather than
a live consumer: a caller that DID hit it would be parsing prose for
the patch and triangle positions a bug report wants.

## Shape of a fix

Three attributes on the `NodePickError` exception (`patch`,
`triangle`, `index`), `None` on every other arm, `pncad.pyi` and the
`py/mod.rs` docstring with them. The Rust side needs nothing: the
fields are public on a curated type.

## Closed (2026-09-08, LIB-PROJ)

`patch`, `triangle` and `index` are attributes on `NodePickError`,
present on every arm and `None` on the four that are not the index
arm. The projection is
`crates/pncad-py/src/pick_payload.rs::index_payload`, exhaustive over
both `NodePickError` and `MeshPickError` with no wildcard at either
level; `py/pick.rs::node_pick_err` is the class's one raise site, so
the three attributes are on every instance of the class by
construction. Stub, class docstring, census row and ty fixture moved
with them.

**Where the numbers are pinned, and why not in Python.** This item is
right that the arm is unreachable from any authoring door and
unconstructible from Python, so no Python row can read the three
numbers back. That is why the flattening is sited outside `py/`: the
default no-Python build can construct a `MeshPickError` and read every
field off it, and
`src/tests.rs::every_pick_arm_projects_the_index_numbers_it_carries`
does — at `patch: 3, triangle: 11, index: 47`, three distinct numbers
so a swapped slot shows as a moved value rather than as zeroes
agreeing. `test_picking.py` owns the other half of "present on every
arm": that the three attributes exist and read `None` on the arms a
caller can reach.

The `MeshPickError` census row keeps its `BOUND_AS` spelling
(`NodePickError.index_variant` is still where the discriminant
crosses) and its prose moved: the sentence "the three numbers ... are
in the message" is now "beside the discriminant".

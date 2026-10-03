---
id: two-datum-arms-have-no-node-constructor
kind: issue
title: Datum::Point and Datum::Frame have no Node constructor
status: closed
opened: 2026-09-09
closed: 2026-09-09
parent: LIB-GAPS-1
---



Found by LIB-MEMBERS's first run of the member rule
(`work/lib/datum-crosses-name-for-name-as-two-types.md`, ruling (D)),
and chartered in the census as `B-DATUM-DOORS`.

`editor_core::Datum` (`crates/editor-core/src/node.rs:603`) has six
arms. Python spells four of them, one `Node` constructor each:

| arm | door |
| --- | --- |
| `Plane` | `Node.datum_plane` |
| `Axis` | `Node.datum_axis` |
| `AxisInPlane` | `Node.datum_axis_in_plane` |
| `FaceFrame` | `Node.datum_face_frame` (LIB-B-FACE-FRAME) |
| `Point` | — |
| `Frame` | — |

`Datum::Point { position }` and `Datum::Frame { origin, u, v }` have
no Python constructor, so a Python author can build four of the six
datum kinds and a document that holds either arm can be loaded, read
and evaluated but not authored.

Nothing mechanical said so before: `Datum` is curated
(`crates/pncad/src/document.rs:59`) and `pncad.pyi` declares a
top-level `Datum`, so rule 1 accounted the whole enum on a spelling
coincidence — and the two types are not even the same one, which is
the finding the ruling was written for.

## What closing it looks like

`Node.datum_point(position)` and `Node.datum_frame(origin, u, v)`
beside the four that exist, in `crates/pncad-py/src/py/doc.rs`, with
their stanzas in `pncad.pyi`; the refusal rows each can raise; and the
two `MEMBERS_NOT_BOUND` rows moving to `MEMBERS_BOUND_AS` in
`crates/pncad-py/tests/test_binding_census.py`, which empties
`B-DATUM-DOORS` out of `FAMILIES` with them.

## Closed (2026-09-09, LIB-GAPS-1)

`Node.datum_point(position)` and `Node.datum_frame(origin, u, v)` sit
beside the four in `crates/pncad-py/src/py/doc.rs`, with their stanzas
in `pncad.pyi`: a `Length` triple for each position, bare triples for
`u` and `v`, exactly as the four siblings take them. A Python author
builds six of the six datum kinds.

The refusals are the arms' own and are asserted in
`TestDatumPointAndFrame` (`crates/pncad-py/tests/test_document.py`): a
non-finite coordinate is `LiteralError` at the door for both, and a
frame whose `u` is zero or whose `v` is parallel to it is
`degenerate_direction` at `evaluate`, naming which axis went. A pair
that is merely not perpendicular is legal — orthogonalizing it is what
the arm does — and the read-back row shows `v` yielding.

The read side needed nothing: `Value.datum()` already answered
`kind == "point"` and `"frame"`, with `origin` dimensioned and the
frame's `axes` bare. The arms were readable and only unauthorable,
which is what this file said.

Both `MEMBERS_NOT_BOUND` rows moved to `MEMBERS_BOUND_AS` and
`B-DATUM-DOORS` left `FAMILIES` with them.

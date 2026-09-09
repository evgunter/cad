---
id: two-datum-arms-have-no-node-constructor
kind: issue
title: Datum::Point and Datum::Frame have no Node constructor
status: open
opened: 2026-09-09
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

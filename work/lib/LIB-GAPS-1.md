---
id: LIB-GAPS-1
kind: unit
title: two datum constructors and the mesh boundary polylines
status: closed
branch: lib/gaps-1
opened: 2026-09-09
refs: [two-datum-arms-have-no-node-constructor, mesh-boundary-polylines-have-no-python-door]
pr: 2250
closed: 2026-09-09
---

Three doors the binding census's member rule found on its first run,
bound: `Node.datum_point`, `Node.datum_frame` and `Mesh.boundaries`.
Two of the three census families the rule chartered close with them.

## Delivered

- **`Node.datum_point(position)`** — `crates/pncad-py/src/py/doc.rs`,
  beside the five other `datum_*` constructors. One `Length` triple,
  matching `SlotId::Origin`; no direction, because a point has none.
  Nothing refuses at evaluation; a non-finite coordinate refuses at the
  door as every literal does.
- **`Node.datum_frame(origin, u, v)`** — same file. A `Length` triple
  and two bare direction triples, matching `SlotId::U`/`SlotId::V`'s
  `Scalar`. `u` and `v` are authored freely and orthonormalized at
  evaluation with `u` kept, so a merely non-perpendicular pair is legal
  and a parallel one refuses `degenerate_direction` naming which axis.
- **`Mesh.boundaries`** — `crates/pncad-py/src/py/mesh.rs`, a getter
  answering `list[list[int]]`: one polyline of position indices per
  model edge, in the kernel's edge order.
  - **Decision: sequence-of-sequences, not a class per polyline.**
    `BoundaryPolyline` carries `edge`, `start_vertex` and `end_vertex`
    beside `points`, and all three are arena keys the curation exists
    to keep unnameable — so a class would hold the indices and nothing
    else. The pairing that makes a polyline index a handle already
    exists on the other side: `NodePick.boundary_names` answers one
    name per polyline in the same order.
  - **Decision: answered whole, not one at a time like `Mesh.patch`.**
    A patch is addressed by index because `Mesh.triangles` concatenates
    the patches and separability has to be recoverable from that; the
    polylines have no concatenated spelling to be separable from, so
    the list IS the door and `len()` is the count.
- **Deviation from the brief: the `Mesh::boundaries` census row is
  DELETED rather than moved to `MEMBERS_BOUND_AS`.** The member rule
  accounts a struct's bare-`pub` field by a same-named attribute on the
  Python namesake, and `Mesh.boundaries` is that spelling — so
  `test_the_member_rosters_decay` fails on a row for it ("the Python
  namesake spells it now; drop the row"). The two `Datum` rows do move
  to `MEMBERS_BOUND_AS` (Python's read-side `Datum` spells no arm), and
  both `B-DATUM-DOORS` and `B-MESH-BOUNDARIES` leave `FAMILIES` either
  way, which is what the brief's claim asks for.
- **The read side needed no extension.** `Value.datum()` already
  answered `kind == "point"` and `"frame"`
  (`crates/pncad-py/src/py/value.rs`) — the arms were readable and only
  unauthorable, which is exactly what the filed issue says.
- **Rows.** `TestDatumPointAndFrame` in
  `crates/pncad-py/tests/test_document.py` (read-back, both downstream
  doors, both refusal shapes); `TestBoundaryPolylines` in
  `tests/test_mesh.py` (count, index validity, segments are triangle
  edges, closure, name pairing); stub stanzas in `pncad.pyi`; fixture
  lines in `tests/ty_fixtures/legal.py` and `illegal.py`.
- **`Node.sketch_frame` already minted `Datum::Frame`** from a
  `SketchPlane` value, and still does. `datum_frame` is the arm's own
  spelling — three triples straight into the node's nine slots, no
  rigid-frame value in between — and the two doors are documented
  against each other rather than one replacing the other.
- **Two stale sentences corrected**, both of which said the polylines
  were unbound: the `py/mesh.rs` module header and `NodePick`'s
  `boundary_names` doc, plus the `Mesh` class docstring in the stub.
- **`test_north_star.py`'s hand-kept `Node` roster** gained the two
  constructors; that roster is the one the census cannot see, which is
  why it exists.

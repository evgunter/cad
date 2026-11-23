---
id: two-doors-decide-a-vertexs-plane-side-on-the-plain-offset-and-charge-its-rounding
kind: issue
title: Two doors decide a vertex's plane side on the plain offset (q − o)·n and charge its rounding, where plane_offset is now posed compensated
status: open
opened: 2026-11-22
---


Filed by GERM's `germ/plane-pose-far-frame` lane, from its sweep for
offsets formed about a stored origin far from the geometry.

## Finding

`geom::plane_offset` (`crates/geom/src/frame.rs`) is now posed with
error-free differences and a compensated sum (`Real::difference_dot`):
its value is the exact offset's rounding, whatever the cancellation
between a far plane origin and the point. `geom::plane_offset_rounded`
keeps the plain formula `(q − o)·n` (its `Rounded::value` is documented
bit-identical to the plain expression) with a first-order bound on its
rounding, and two doors decide on it:

- `splitting::classify::classify_vertices_margined`
  (`crates/topo/src/splitting/classify.rs`, `split_vertex_side`) decides
  on `offset.value`, the plain value;
- `vertex_section::Offset::of` (`crates/topo/src/vertex_section.rs`)
  carries the plain value and its bound to both vertex doors.

Under `[simple.conditioned]` a door decides on the well-conditioned
value and charges nothing, so both would read `geom::plane_offset` and
drop the bound; `plane_offset_rounded` then has no caller.

## Settles when

Both doors read `geom::plane_offset`, no charge rides on the offset,
and `plane_offset_rounded` is deleted.

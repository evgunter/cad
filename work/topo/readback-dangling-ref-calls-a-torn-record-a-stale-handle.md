---
id: readback-dangling-ref-calls-a-torn-record-a-stale-handle
kind: issue
title: readback::DanglingRef raises Entity for a record link, so a torn body is told its handle is stale
status: closed
opened: 2026-10-03
closed: 2026-10-04
---


(TOPO orchestrator, from the stale-key design fork, PR 4006; found by all
three designers, by reading, without a probe.)

## What

`readback::DanglingRef` (`crates/topo/src/readback.rs`) splits on
**entity vs geometry** and its docs read that as "the caller's stale
handle vs a corrupt body". The split holds only for a one-hop read. Where
a walk follows records, a record that names nothing comes out as
`DanglingRef::Entity`, and `ReadbackError` renders it "the handle is
stale, or it belongs to another body's lineage". That is false for a torn
body. Sites:

- `merge_faces.rs`: a face's ring keys, a loop's half-edge in
  `boundary_points`, a half-edge's edge;
- the readback walk `edge_sides` → `side_of` (edge → half-edge → loop →
  face).

The euler `From<DanglingRef>` impl is where this axis crosses into
`EulerOpError`.

## Direction

Under PR 4006's final state the axis is argument vs record (`KeyFrom`),
and a record miss is `Torn(Dangling { from, link, to })`. `DanglingRef`
wants the same axis. Land it with or after the stale-key code.

## Closed

`DanglingRef` is gone, and with it the entity-vs-geometry axis. Every
read-back door's one typed miss is the caller's own key,
`ReadbackError::Dangling { what: EntityId }`; every hop after it is a
record link and panics through `live::linked` / `live::dangling_link`
naming the holder and field: `readback::side_of` (half-edge →
`parent_loop` → `face`), `readback::edge_extent`'s ends,
`Body::edge_curve_linked`, `Body::face_surface_linked` and
`Body::point_of`. `merge_faces`' ring keys, `boundary_points` and the
half-edge's edge already panicked (PR 4029). The euler
`From<DanglingRef>` impl went with PR 4029. Pinned by
`readback::tests::edge_sides_reads_both_sides_in_half_edge_order_and_names_each_miss`
and `a_live_edge_with_a_torn_curve_key_panics_naming_the_edge_on_every_door`.

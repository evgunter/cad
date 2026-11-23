---
id: a-cut-that-trims-a-kept-face-leaves-a-record-on-it-stale
kind: issue
title: A cut that trims a face under a recorded touch keeps the face's key, lands the record unread, and the gate refuses the result stale
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Found by PR 7124's review fix pass (`embed/records`, 2026-11-23), in
the slow set: the at-rest gate now reads every record a door hands it,
and here it reads one the Boolean's carry should have retired.

**The shape.** `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`,
scenes "under the lying pyramid's ray" and "continuing the lying
pyramid's face", op `y − x`, all five poses of each (and the variant
with the pyramid lying 0.5ε off the top). `y` is the plate united with
a pyramid lying on its top, holding `VertexOnFace` of the pyramid's
touching vertex on the plate's top, confirmed at `y`'s own gate. `x`
is a pyramid under that ray. The cut carves the top away under the
vertex.

**What happens.** Both the vertex and the top survive under their
keys (the top is trimmed, not replaced), so the cut's lineage states
neither, the carry lands the record unread as on kept cells
(`topo::Lineage`: "relying on each image being on its input cell's
carrier and holding its old interior"), and the result's gate reads
it: `StaleContactDeclaration { VertexOnFace }`, `BooleanError::ResultInvalid`.
On main the cut builds and ships the record. The record is false on
the result, so the refusal is true; the op is valid, so the defect is
the carry's.

**Owed.** A face that keeps its key and loses part of its region does
not hold its old interior. The Boolean's lineage states it (as a
divided cell is stated), so the carry reads the record against the
trimmed face and retires it where the touch is gone, and the cut
builds without it. With
`a-stale-carried-record-meets-three-different-ends`, which asks for
one rule for a record a carry finds stale.

**Pinned** as refusals in both tests' tables (`REFUSING`; `STALE` in
`a_vertex_read_again_classes_every_edge.rs`), ten cells. They come
out of the tables with the fix.

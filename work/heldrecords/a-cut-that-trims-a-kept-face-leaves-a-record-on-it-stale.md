---
id: a-cut-that-trims-a-kept-face-leaves-a-record-on-it-stale
kind: issue
title: A cut whose apex is zipped onto a face's boundary at a vertex recorded as resting on that face carries the record on, and the gate refuses the result stale
status: open
opened: 2026-11-22
priority: P0
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

**Why P0** (EMBED orchestrator, 2026-11-23; HELDRECORDS may re-band
it). A valid cut that builds on main refuses on PR 7124's head. Main
ships the false record, so the refusal is the better of the two
answers, but it is a body that stops building, and nothing but this
row's fix brings it back.

**The bounded fix was tried and stopped** (PR 7124, 2026-11-23, on
the orchestrator's ruling: read the record with the divided-cell
machinery, stop if it needs more). Traced on "under the lying
pyramid's ray", `y − x`:

- The top is not trimmed by any division the lineage records. The
  cutter's apex meets the top at the recorded vertex's point, and the
  finish's zip puts that apex on the top's boundary: on the result a
  second vertex sits on the recorded vertex's point, and the top's
  boundary passes through it. `Descendants::fragments` has no row for
  the top (its own text: the zip's divisions are in no row of it), so
  `operand_fate` answers `Kept` and "The shape" above is wrong to call
  it a carved region: the record is false because the touch is now on
  the face's boundary, where the true record is the vertex pair.
- Stating the top as divided (one piece, remainder removed) wherever a
  row of the reduction's own names it makes the carry read the pair,
  as asked. `RecordConfirm::meets` then answers that the vertex and the
  top **meet**, on the body the carry reads, and the record lands
  again; the gate's `census::confirm_only` reads the same pair stale
  on the body it is handed. So the divided-cell reading and the gate's
  reading disagree on a vertex at a boundary vertex of the face (or
  the body changes between the carry and the gate; not separated).
- The record also re-enters beside the carry: the vertex groups in
  `carry_rows` place each carried vertex-on-face row again through
  `place_found`, by lineage, unread.

Owed, and past the bound: one answer for a vertex on a face's boundary
vertex in both readings; and the zip's seam cells stated in the
lineage of the face they land on, so that a record there is read, and
re-lands as the vertex pair or is retired. The ten cells stay pinned.

The two readings' disagreement has its own row:
`work/contacthold/two-readers-of-a-vertex-on-a-faces-boundary-vertex-disagree.md`.

---
id: two-readers-of-a-vertex-on-a-faces-boundary-vertex-disagree
kind: issue
title: The carry's record confirm and the gate's confirm-only were seen to answer oppositely on one recorded vertex-on-face pair; whether they read one body is not separated
status: open
opened: 2026-11-23
priority: P1
cost: M
---


Two readers of one recorded pair were observed to give opposite
answers. It is NOT yet separated whether they disagree on one body or
the body changes between the carry and the gate, each being right
about its own; that separation is the first step, and until it is
made no cause is known. Found by PR 7124's review fix pass
(`embed/records`, 2026-11-23), tracing the cut filed at
`work/heldrecords/a-cut-that-trims-a-kept-face-leaves-a-record-on-it-stale.md`
(P0), which cites this row.

**The pair.** `y` is a plate united with a pyramid lying on its top,
holding `VertexOnFace` of the pyramid's resting vertex on the top.
`x` is a pyramid hanging below the top whose apex is at that vertex's
point (`topo::test_support_meeting::along_lying_ray`, "under the lying
pyramid's ray"). In `y − x` the cutter's apex is zipped onto the top
face's boundary: on the result a second vertex is on the recorded
vertex's point and the top's boundary runs through it. The recorded
vertex is then on a boundary vertex of the face, not in its interior.

**Measured**, with prints in the kernel, since reverted:

- `boolean::records::landings`, reading the pair with the census's
  record confirm (`census::RecordConfirm::meets`, the reading a
  divided cell's record gets), answers `Ok(true)`: the pair meets.
- The Boolean's gate, reading the record it is then handed with
  `census::confirm_only`, answers
  `StaleContactDeclaration { VertexOnFace }`.

The first was made to run by stating the top as divided for the
carry (an experiment; on the branch as it stands the top's fate is
`Kept` and the pair lands unread).

**Not separated.** `landings` reads the body the carry is handed, and
the gate reads the body after the output stage (the join, then
`sort_into_pieces`). Whether the two readings disagree on one body, or
the body changes between them so that each is right about its own, was
not established. That separation is the first step: hand both readers
the gate's body and compare.

**Also seen there.** `boolean::ops::carry_rows` places every carried
vertex-on-face row a second time through `place_found` with
`found: None`, which lands it by lineage, unread, whatever `carried()`
read of it. A fix that reads the record in `carried()` alone is undone
by that second placement.

**Cheapest reproducer.** `cargo nextest run -p topo
--ignore-default-filter -E
'test(every_scene_builds_sound_or_refuses_typed_at_every_pose)'`
(slow set, about 30 s): the cells "under the lying pyramid's ray,
y − x" and "continuing the lying pyramid's face, y − x" in
`crates/topo/tests/a_vertex_read_by_two_sector_passes.rs`, pinned
`ResultInvalid` in its `REFUSING` table. The gate's answer is the
pinned refusal; the carry's answer needs the pair read in `landings`.

**Owed.** The separation first. If the two do disagree on one body:
one answer for a vertex on a face's boundary vertex in both readings,
and if `meets` is the one that moves, every divided-cell carry reads
by it, so its rows move with it. If the body changes between them: the
carry reads the body the gate will read.


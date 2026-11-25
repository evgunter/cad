---
id: a-closed-rider-in-a-face-with-rings-keeps-its-zero-area-ring
kind: issue
title: A closed rider inside a face with rings keeps the zero-area ring rather than divide the face, which would move the rings inside the disc to the new piece
status: open
opened: 2026-11-25
priority: P3
cost: M
---


Filed by `heldrecords/parting-division` from PR 7783's review (m2).

## What

`boolean/slit.rs` `part` divides a face along a closed conic lying
inside it (`divide_closed`) only where the face has no rings. A face
with a ring takes the zero-area ring arm instead: the rim's two
circles bound the one face. On a plane that builds sound: tiers 1, 3
and 3′, the closed-form volume, one edge–edge record
(`crates/sweep/tests/a_closed_rider_lying_in_a_face.rs`
`a_closed_rim_in_a_holed_plane_face_keeps_its_ring`, a pin's bore
outside the disc and at its centre). On a curved face the ring refuses
`CurvedFace`.

## The question

Whether the closed rider should divide a ringed face too, as it does
an unringed one. Then the disc becomes its own face, and each ring of
the face lying inside the disc moves to the new piece: a ring move
(`Body::ring_move`) read by which side of the rim each ring lies on.
That would make the closed rider one final state, a division, on
every face, and lift the curved ringed case off `CurvedFace`.

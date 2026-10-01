---
id: section-promotion-clears-a-null-face-record-mfkrh-already-dropped
kind: issue
title: the split and boolean finish clear a null-face record that the mfkrh before them already dropped
status: open
opened: 2026-10-01
priority: P4
cost: E
refs: [kef-kvfs-and-mekr-leave-a-null-face-record-naming-the-loop-they-remove, 3618]
---

## What

Found by TOPO's null-face-record unit (PR 3618). Since that unit,
`Body::mfkrh` drops each null-face record naming the ring it promotes
(`Body::drop_null_face_records_naming`, `crates/topo/src/body.rs`),
because the marked face no longer holds the two loops the record
describes (`crate::null`'s module docs). Both section-promotion sites
promote a null face's ring with `mfkrh` and then clear the same face's
record. By then the record is gone, so the clear returns `None` every
time:

- `splitting::finish` (`crates/topo/src/splitting/finish.rs`, the
  section loop): `body.mfkrh(ring, FaceSurface::Inherit)`, then after
  the re-chart `body.clear_null_face_pair(section.face)`;
- `boolean::finish::promote_solid` (`crates/topo/src/boolean/finish.rs`):
  `body.mfkrh(ring, FaceSurface::Inherit)?`, then
  `body.clear_null_face_pair(face)`.

PR 3618 measured both orders before its change, across topo's suite
and the `ci` profiles of sweep, mesh, step-import and editor-core.
After the `mfkrh`, only these clears touched the record (620 and 23,648
promotions), and nothing read it. The unit left the clears in place
because the files are CLEAVE/HONE ground.

## The fix

Delete both clears, or replace each with a `debug_assert!` that the
record is already gone. `splitting/reassembly.rs`'s oracle test mirrors
the split's promotion inline and carries the same clear.

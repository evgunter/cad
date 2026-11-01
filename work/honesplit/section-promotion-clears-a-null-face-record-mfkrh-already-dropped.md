---
id: section-promotion-clears-a-null-face-record-mfkrh-already-dropped
kind: issue
title: the split and boolean finish clear a null-face record that the mfkrh before them already dropped
status: closed
opened: 2026-10-01
priority: P4
cost: E
refs: [kef-kvfs-and-mekr-leave-a-null-face-record-naming-the-loop-they-remove, 3618]
closed: 2026-10-26
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

## Module docs that claim promotion reads the record

PR 3618's review (Q5) found two module-doc claims that the record
drives promotion. Neither promotion reads it. Each reads the roles from
its own pipeline struct, and after `mfkrh` the record is gone anyway:

- `splitting::finish`'s module docs (`crates/topo/src/splitting/finish.rs`,
  "so promotion reads the record, never the loop list"): the section
  loop reads `section.above_loop` off `CompletedSection`
  (`splitting::join`), never `null_face_pair`;
- `boolean::finish`'s module docs (`crates/topo/src/boolean/finish.rs`,
  "Which promoted/remaining face is the IN copy is read from the
  `NullFacePair::Boolean` loop roles"): `promote_solid` reads
  `CompletedPolygonPair` (`boolean::join`), never `null_face_pair`.

The fix rewords each to name the struct it reads, in the same change
that deletes the clears.

## Closed (2026-10-26, easy-fixes lane)

The three production clears are deleted: `splitting/finish.rs`'s slit and section arms, and `boolean/finish.rs` `promote_solid`. Each followed `body.mfkrh(ring, …)` on a ring of the same face, and `mfkrh_with` (`euler_kill.rs`) calls `drop_null_face_records_naming(ring)`. In `splitting/reassembly.rs`'s oracle the clear became an `assert!` that `body.null_face_pair(section.face)` is already `None` after the `mfkrh`. The module docs are reworded: `splitting::finish` says promotion reads the roles off the join's `CompletedSection`, and `boolean::finish` names `CompletedPolygonPair`. Verified: `cargo clippy -p topo --all-targets --all-features -- -D warnings` is clean. `cargo nextest run -p topo --lib -E 'test(/chord_join|splitting|boolean::finish|reassembly|merge_faces|split_param|boolean::join|shell::|null::|euler::/)'`: 367 passed. `cargo nextest run -p topo --tests -E 'kind(test) & test(/shell|moved_wall|thin|split|merge_face|section/)'`: 168 passed.

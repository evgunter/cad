---
id: kef-kvfs-and-mekr-leave-a-null-face-record-naming-the-loop-they-remove
kind: issue
title: kef, kvfs and mekr remove a loop a null-face record names and return Ok, leaving a tier-1-invalid body from a valid one
status: open
opened: 2026-09-30
priority: P2
cost: M
design: true
refs: [kills-remove-half-edges-another-record-names, mekr-and-kvfs-remove-an-empty-loop-or-lone-vertex-another-record-names, kef-kvfs-and-mekr-remove-a-face-shell-solid-or-edge-another-record-names]
---

## What

Found by the review of the kill-proofs unit (PR 3570), whose receipt
mapped every record a kill removes to the proof that nothing it keeps
names it, and missed one naming relation: `Body::null_faces`
(`crates/topo/src/body.rs`), whose records name two loops each
(`NullFacePair::loops`, `crates/topo/src/null.rs`). The validator checks
them in pass 13 (`ValidationError::StaleNullFaceLoop`, `validate.rs`'s
`tier1`), and `Body::set_null_face_pair` accepts a record on any face
naming any two live loops, since which loops a record names is
semantics that tier 1 does not check.

Three kills remove a loop and read no null-face record but the dying
face's own: `Body::kef` (`kef_with`'s kill phase removes `l1`),
`Body::kvfs` (removes its lone loop) and `Body::mekr` (`mekr_finish`
removes the ring). Each drops the record keyed by a face it removes
(`null_faces.remove(face)`), and none reads a record that another face
carries, or that the kept face carries, naming the loop it removes.

**From a tier-1-valid body, through public doors, each returns `Ok` and
leaves a record naming the removed loop.** Executed at PR 3570's head
(`validate` is `Ok(())` before each kill):

- `kef`: the declined cube (`test_support_fixtures::declined_cube`),
  `he = half_edges[6]`, its loop `l1`, its mate's loop `l2`, and a third
  loop's face marked `Split { above_loop: third, below_loop: l1 }`.
  `kef(he)` returns `Ok`; `validate` then answers
  `StaleNullFaceLoop { face: <the third loop's face>, named_loop: l1 }`.
- `kvfs`: `review_d18::segment_beside_a_lone_solid`, the segment's face
  marked `Split { above_loop: <the segment's loop>, below_loop: <the lone
  loop> }`. `kvfs(lone.solid)` returns `Ok`; `validate` answers
  `StaleNullFaceLoop { face: <the segment's face>, named_loop: <the lone
  loop> }`.
- `mekr` at `Cycles`: the holed box (`fixtures::ops_holed_box`), the top
  face marked `Split { above_loop: <its outer>, below_loop: <its ring> }`,
  `mekr_chord(Cycles { target: <a member of the outer>, ring: <a member
  of the ring> })`. It returns `Ok`; `validate` answers
  `StaleNullFaceLoop { face: <the top face>, named_loop: <the ring> }`.

The `mekr` witness is the plausible one: a section face whose two loops
a pipeline joins by `mekr` is exactly a marked face losing one of the
loops its record names.

`euler::removal_census` lists this relation as filed here, and reds
when the row is gone.

## The two shapes

Whether a kill should refuse here or maintain the record is a question
about what a null-face pair means once one of its loops is gone, which
is F9's (`crate::null` module docs) and not the kill's to settle alone.

1. **Refuse.** A kill proves, as its other removal proofs do
   (`Body::require_loop_unlisted`'s shape), that no null-face record
   names the loop it removes, and refuses a typed variant naming the
   face whose record does. Cheap and uniform; but the `mekr` witness is
   a state a pipeline may reach on purpose, so a pipeline would have to
   clear the record first (`Body::clear_null_face_pair`), and the
   refusal is then an ordering obligation on every consumer.
2. **Maintain.** A kill that removes a loop a record names clears that
   record (or, for `mekr`, whose ring joins the target, rewrites it to
   name the surviving loop), as it already drops the record of a face
   it removes. That keeps every kill `Ok` and the body tier-1 valid,
   but decides what the record means after the kill, which is the
   semantic question above.

Either shape adds the null-face column to
`review_d18::kill_anchors_on_torn_bodies` (a record naming a dead loop)
and requires it 0, and pins the three witnesses above as rows.

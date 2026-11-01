---
id: a-mirrored-bodys-mapped-places-are-called-rigid
kind: issue
title: MappedSource's place fields are documented as rigid placements, but a mirrored body's descriptions carry an improper one
status: open
opened: 2026-10-31
priority: P3
cost: C
---


Found by CARVE's sweep of doors that take an `Affine3` documented as
rigid (`a-reflected-loft-placement-evades-both-normal-checks`).

`geom_brep::MappedSource`'s three `place: Affine3<T>` fields
(`crates/geom-brep/src/mapped.rs`, `PlacedSegment`, `ExtrudedPoint`,
`RevolvedPoint`) each say "The rigid placement of the sketch plane in
3-space", and the variant docs say "under a rigid placement". In this
kernel "rigid" is what `geom_core::RigidMotion::check_rigid` admits,
which refuses a mirror (`transform_rigid_det_plus_one`). But
`topo::mirror` maps a body through `map_isometry` with `Hand::Improper`,
and `crates/topo/src/transform.rs` `map_mapped_curve` composes the
reflection into every description's `place` (`*map * place`) — its own
doc says "under a reflection `place` is improper". So a mirrored body's
edge descriptions carry a `place` with determinant −1 under a field doc
that calls it rigid.

Nothing is known to read the handedness of a description's `place`
today (the enum's header says rigidity "is not decided here and not
re-examined here", and certification checks a description against its
carrier), so this is a documentation defect with a latent hazard: a
reader that trusts the field doc and derives an orientation from
`place` (an arc's sense, a strut's side) is wrong on a mirrored body.

The fix is the field and variant docs saying what holds — an isometry,
proper or improper (a mirrored body's is improper) — or a stated reason
the mirror should re-author its sketch data instead.

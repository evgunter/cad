---
id: ruled-band-refuses-a-joint-on-one-support-pair
kind: issue
title: blend: a ruled crease split by a valence-2 vertex is a two-link chain on one support pair that the ruled band refuses as junction carry-through
status: open
opened: 2026-10-01
priority: P3
cost: M
---


Found by `subdivided-rim-fillet-refuses-at-the-collinear-joint`'s fix.

The open-chain door (`OpenBand::admit` / `Joint::admit` in
`crates/sweep/src/blend/admit.rs`) now admits a chain whose consecutive
links share BOTH support faces, and the planar band carves it as one
face across the joint (`blank_phase`'s joint fusion in
`crates/sweep/src/blend/open/planar.rs`). The joint admission requires
both links to be plane–plane: a RULED link pair on one support pair (a
cylinder–plane or cylinder–cylinder crease split by a valence-2 vertex,
which `merge_coplanar_faces` leaves wherever a boolean's operands met
along the crease) still refuses `UnsupportedChain` "an open chain's links meet on
supports other than two planes; that junction is not implemented".

The planar fusion does not transfer as is: it fuses the two links'
strips across the joint's STRUTS, and the ruled band mints none — on a
curved support a strut is a secant (`ruled.rs` module docs, "No strut
is minted"). A ruled joint needs its foot ON the curved support, and
the joint vertex has no edge there to split, so the foot needs a new
edge that lies on the support (an arc across the ruling) or a fusion
that mints none. Unmeasured: no fixture reaches it yet (a D-rod
unioned flush with a box along its flat, merged, is the likely
witness).

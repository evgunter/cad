---
id: unmerged-subdivided-wall-rim-fillet-names-no-merge-recourse
kind: issue
title: blend: a rim split across two same-key wall faces refuses as junction carry-through and does not name the merge that makes it build
status: open
opened: 2026-10-01
priority: P3
cost: E
---


Found by `subdivided-rim-fillet-refuses-at-the-collinear-joint`'s fix.
Measured by `crates/sweep/tests/band_subdivided_side_walls.rs`,
`subdivided_rim_blends_as_one_band_once_merged` (its first half).

The fillet now carves a rim split by a valence-2 vertex as one band
when both links lie between the same two FACES (`Joint::admit` in
`crates/sweep/src/blend/admit.rs`). The extrude of a declared straight
continuation, unmerged, puts the two halves of each rim on two wall
faces of ONE surface key, separated by the continuation's flat strut;
the joint vertex is then valence 3 and the request refuses
`UnsupportedChain` "an open chain with more than one link needs
junction carry-through", carrying `FILLET3_ASSEMBLY_RECOURSE`, which
names no merge. `Body::merge_coplanar_faces` first makes the same
request build at the closed form.

Two ways out: name the merge in this refusal where the two faces share
a surface key (the door would need the body, which `OpenBand::admit`
does not read today), or carve the band across the flat strut as a
run-out (split the strut at its two feet, as the ruled band splits a
cap rim). `swept-continuation-walls-reach-the-boolean-unmerged` (P0)
retires most of the reach if the sweep stops minting the split walls
(the pending one-wall-per-run ruling), so this is P3.

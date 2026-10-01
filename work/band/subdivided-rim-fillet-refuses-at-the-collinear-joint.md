---
id: subdivided-rim-fillet-refuses-at-the-collinear-joint
kind: issue
title: blend: a rim subdivided by a declared continuation is a two-link chain the fillet refuses as unbuilt junction carry-through, merged or not
status: closed
opened: 2026-09-25
priority: P1
cost: D
closed: 2026-10-01
pr: 3701
branch: band/collinear-joint-chain
---


Measured by `subdivided-profile-side-coplanar-walls-gate`'s rows
(`crates/sweep/tests/band_subdivided_side_walls.rs`,
`subdivided_rim_fillet_refuses_as_junction_carry_through`).

Take a `[0,2]²` prism whose bottom side is a declared straight
continuation (two segments on one carrier) and request every edge
except the continuation's flat strut, at r = 0.25. The request refuses
`BlendError::UnsupportedChain` ("an open chain with more than one link
needs junction carry-through", `AdmittedOpen::admit` in
`crates/sweep/src/blend/admit.rs`). The plain cube's twelve edges
build under the same request. Merging the walls first
(`merge_coplanar_faces`) does not help: the merge never elides
vertices (DESIGN.md F7), so the rim stays two collinear links.

This joint is the simplest junction there is. Both links have the
same two supports (one wall key, one cap key), so the arm is identical
on either side and the band is one cylinder the joint vertex only
splits. The chain walker could accept a link pair on identical
supports and build that band without the general carry-through.

## Closed

Both claims held on the tree: the refusal was `AdmittedOpen::admit`'s
multi-link arm, and the merge keeps the rim vertices.

The door is now `OpenBand::admit` (`crates/sweep/src/blend/admit.rs`):
a junction whose two links are plane–plane on the same two support
FACES at a valence-2 vertex is a `Joint`; any other junction refuses,
naming what differs. The
blank phase struts each joint on both supports, then one `kef` and one
`kev` fuse the two strips into ONE band face over the chain (one
surface key, so the boolean's maximal-faces gate passes it), leaving
the joint's two feet as valence-2 vertices on the band's trimlines.
The band face is named `BandFace` of the chain's edge set
(`BlendNaming::joined_blends`). Both verbs; pinned by
`band_subdivided_side_walls` (merged prism, both verbs, closed-form
volumes; a declared flush union) and editor-core's
`band_joined_rim_names` (three-link chains). Predicate 2's
face-clearance screen reads a joined run as one feature and meters
each joint's foot against the trimlines of the edges at the run's ends
(`review_3701_probes`).

Not built, and filed: the UNMERGED rim, whose halves lie on two faces
of one key (`unmerged-subdivided-wall-rim-fillet-names-no-merge-recourse`),
and a ruled crease split by a joint
(`ruled-band-refuses-a-joint-on-one-support-pair`). Closed chains need
nothing: a closed rim's multi-link ring is already one band
(`resolve_rim`), and a closed chain of plane–plane links whose every
junction is a joint cannot close.

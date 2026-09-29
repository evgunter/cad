---
id: subdivided-rim-fillet-refuses-at-the-collinear-joint
kind: issue
title: blend: a rim subdivided by a declared continuation is a two-link chain the fillet refuses as unbuilt junction carry-through, merged or not
status: open
opened: 2026-09-25
priority: P1
cost: D
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

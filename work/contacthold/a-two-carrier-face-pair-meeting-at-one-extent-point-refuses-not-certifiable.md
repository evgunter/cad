---
id: a-two-carrier-face-pair-meeting-at-one-extent-point-refuses-not-certifiable
kind: issue
title: tangent_pair_at_rest refuses NotCertifiable where two faces' extents along their tangent line meet at one point, where the interiors cannot meet there
status: open
opened: 2026-10-13
priority: P3
refs: [point-touch-reading]
---


Found by reading, in the point-touch unit's sweep
(`contacthold/point-touch-reading`) for confirms that refuse a point
touch typed.

## What

`boolean::tangent_pair_at_rest` (`crates/topo/src/boolean/mod.rs`), the
face–face confirm's two-carrier lane, decides the overlap of the two
faces' extents along their tangent line (`pm_census_tangent_overlap`)
and on `Sign::Zero` refuses `NotCertifiable` ("the faces' extents along
their tangent line meet at one point"); `census::confirm_face_pair`
pushes it as `CensusUnsupported`. A one-point overlap is an end of
both extents, so the point lies on each face's boundary, where the two
faces' interiors do not meet: the touch there, if any, is a vertex's
or an edge's. The record reads stale (`Negative`, extents apart,
answers `RestTangent::Apart` already), not unsupported, unless the
extents are a box looser than the faces (the sibling
`census-confirms-a-two-carrier-face-pair-at-one-midpoint` row).

Not measured on a fixture.

## Owed

The zero-overlap arm answers `Apart` (stale), or states why a
one-point overlap can hide an interior meeting; a fixture: a plate face
and a cylinder sheet tangent along a ruling, their heights abutting at
one value.

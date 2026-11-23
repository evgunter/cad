---
id: a-two-carrier-face-pair-meeting-at-one-extent-point-refuses-not-certifiable
kind: issue
title: tangent_pair_at_rest refuses NotCertifiable where two faces' extents along their tangent line meet at one point, where the interiors cannot meet there
status: closed
branch: contacthold/one-tangent-interiors-reading
pr: 5006
opened: 2026-10-13
priority: P3
refs: [point-touch-reading]
closed: 2026-10-19
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

Fixed on `contacthold/one-tangent-interiors-reading` (PR 5006). The op and
the census now share one gate, `boolean::tangent_extents_meet`
(`bool_tangent_line_overlap`), and it answers `false` for zero length the
same way it does for a negative one.
The argument above holds only for extents that contain the faces, and
the vertex projections do not, because an arc bulges past its ends (the
5006 delta review's d4). So `witness_span` now spans each face's whole
reach along the line: the vertex projections, plus each circle or
ellipse edge's carrier extremes along the line. A face with any other
edge kind uses its padded box (`census::face_reach_in`). Witness:
`review_5006_delta::d4_an_arc_bulging_past_its_vertices`; the one-point
row's witness:
`census::two_carrier_records::a_two_carrier_record_whose_extents_meet_at_one_point_is_stale`.

---
id: sizes-are-read-back-out-of-extent-levers
kind: unit
title: Ten sites read a size back out of a lever and use it as geometry
status: dispatched
opened: 2026-11-20
priority: P1
cost: M
branch: oneround/sizes-from-geometry
---


`Lever::length` is "the exit, not an entrance" (`geom_core::Lever`'s doc): a lever read back as a length is a margin's business. Ten sites on main read one back and use it as geometry: a radius, a point, a direction's scale, a domain's size. It is the shape PR 7143's review refused in that PR's own records (a record holding a lever and reading its radius out of it), and these are older than that unit. Each was read in the tree at `df4f0fcb`.

- `geom_brep::implicit::min_radius_of_curvature_toward` — returns `curvature_lever_arm(s, p).length()` as a radius of curvature.
- `geom_brep::conic_bend::ConicDeparture::of` — `dir: tangent.normalize() * extent.length()`: a departure vector scaled by the edge's extent lever.
- `geom_brep::intersect::cone_cylinder_section` — `Margin::of(below.length() - station.abs())` (`coc_station_reach`): a lever's length compared with a station as a distance.
- `topo::bound_side::BoundSupport::point` — `base + dir.normalize() * self.lever().length()`: a point placed at a lever's length.
- `topo::star::walk::Bound::ray` — `self.dir * self.reach.lever().length()`: a ray scaled by a lever.
- `topo::solid_contain::face_holds` and `ray_crossings` — `Margin::of((q - apex).norm() - reach.length())` (`bool_cone_partial_reach`, twice): a face's reach lever read as a radius about the apex.
- `topo::boolean::vtxfac::held_reads` — an `ExtentBall::unsourced` about `p` whose radius is the shortest arm's length: a region built from a lever.
- `topo::link::Link::of` — `.fold(reach.length(), |r, l| r.min(l))`: the reach lever's length folded with segment lengths into the link's radius.
- `topo::offset_derive::plane_wall_section` — a sum ending `+ extent.length()` that sizes an `SsiDomain`.

**Wanted:** each reads its size from the geometry the lever was read from (the cover, the carrier, the stored points), and the lever is kept for the margin.

**Left by the converting unit (PR 7628), one line each:**

- `ConicDeparture::of`, `BoundSupport::point`, `Bound::ray`: a direction and its lever packed into one vector read both as a lever and as geometry, the records question: `sector-bounds-pack-a-direction-and-its-lever-into-one-vector`.
- `cone_cylinder_section` (`coc_station_reach`): the honest read is the consumed region's window along the cone's axis, which needs the function to take a `&Reach`, on PR 7442's ground: `apex-reaches-read-a-lever-as-a-window-along-the-axis` (sect).
- `vtxfac::held_reads`: `PlannedRun.order` holds a lever and no region, the records question again: `sector-bounds-pack-a-direction-and-its-lever-into-one-vector`, and `pierce-and-rim-balls-are-built-from-an-arm` (heldrecords).

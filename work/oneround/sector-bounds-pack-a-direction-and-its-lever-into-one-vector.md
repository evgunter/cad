---
id: sector-bounds-pack-a-direction-and-its-lever-into-one-vector
kind: issue
title: Sector bounds pack a direction and its lever into one vector, read both as a lever and as geometry
status: open
opened: 2026-11-23
priority: P1
cost: H
design: true
refs: [unsourced-levers-await-a-source]
---

A curved edge's bound out of a vertex is a `Vec3` made of its departure's unit tangent times its extent lever, and the sector machinery reads that one vector two ways: its `.norm()` as a lever, and its direction or end point as geometry. A line's chord is real geometry (`p_far − p_base`), but a curved edge has no vector that is its geometry. The tangent times the extent ends on the tangent line, not on the edge, and the arc's own point (`ConicArc::at`) would turn a tangent cone's ray off the tangent. `Lever::length` is "the exit, not an entrance" (`geom_core::Lever`'s doc).

The vector is made at:

- `geom_brep::conic_bend::ConicDeparture::of`: `dir: tangent.normalize() * extent.length()`.
- `topo::bound_side::BoundSupport::point`: `base + dir.normalize() * self.lever().length()`.
- `topo::star::walk::Bound::ray`: `self.dir * self.reach.lever().length()`.
- `topo::validate` `along` (the `LeverBounds` arm, about line 9153): `dir * arm.length()`, and `dir * below.length()` / `dir * above.length()`.
- `topo::link::second_order` (`reach`, then `p + t.normalize() * l` at the plane residue near line 245; `toward(p, l, …)` near 462 and 488), `topo::link` near line 5053 (`p + d.normalize() * l`).
- `topo::bound_side`'s witness at `arm.length()` (`self.witnessed(leading, arm.length())`, near line 1196) and `conic_bend_side(self.dir, extent.length(), …)` (near 1118), where a lever's length is taken as an arc length.

It is read as a lever by `geom_brep::sector_shape::metering_arm(u.ray().norm()…)` (`topo::splitting::insert`), by the sector predicates behind `geom_brep::chart_normal_beside((own, next))` (`topo::sector_face`), and by `corner_holds(edge.ray(), …)` (`topo::boolean::fate`, `topo::boolean::overlap`). It is read as geometry by `topo::link::Cone::gathered`'s rays, the fans in `topo::boolean::sectors`, `topo::boolean::vtxfac`'s `reach.point(p, dir)`, and `topo::splitting::rules` (`p + along`).

The same records question holds `topo::boolean::vtxfac::held_reads`: `PlannedRun.order` holds a lever and no region, so the pierce's ball is built from an arm (`ExtentBall::unsourced(p, arm.length())`, `work/heldrecords/pierce-and-rim-balls-are-built-from-an-arm.md`).

**Wanted:** a bound carries its direction and its lever apart, and a point is placed only from geometry (a line's far vertex, an arc's own point). It is the records question (a record holding a lever, read both as a length and as geometry), a design unit with designers in front of it. Shapes seen, none recommended: sector predicates take `(UnitVec3, Lever)` pairs; they take the `Bound`/`BoundSupport` itself; or the packed vector stays and only its construction is held by a door. The `sizes-are-read-back-out-of-extent-levers` unit left these sites for that reason.

---
id: cone-sphere-patch-and-torus-patch-roots-still-walk-their-regions
kind: issue
title: Cone, sphere-patch and torus-patch roots are still held by a region walk that casts auxiliary paths
status: open
opened: 2026-10-27
priority: P2
cost: H
refs: [the-side-readers-aims-decide-exact-ties-at-a-documents-nominal]
---

The side reader's crossings (`crates/topo/src/boolean/solid_contain.rs`
`ray_crossings`) hold a cylinder face's root by the turn of its loops on
an annulus image of the wall (`wall_crosses_by_turn`) where its edges
are rulings and parallels. Every other trimmed curved face still holds
its roots through its region's walk (`root_held`), which casts
auxiliary paths whose questions can tie at a symmetric nominal:

- a cone face: `ruled_region`, the same walk the cylinder used;
- a sphere patch: `sphere_region::SphereFaceRegion::contains`, a ray
  walk over its arcs' targets and a schedule;
- a torus patch: `torus_region`.

Measured: no corpus document and none of the trimmed-sphere bodies
(`sweep` `sphere_aims_reader_ties`) ties on them at a nominal. No
fixture stands for a symmetric cone or torus pose, so that is a blind
spot, not a negative.

A cone's nappe projects linearly along its axis to a disc, carrying
generators to segments and parallels to circles, but a root on the
mirror nappe projects into the same disc. A cone's turn reading would
have to ask the root's nappe, which `ruled_region`'s
`bool_ruled_region_nappe` already does. A sphere patch or a torus patch
has no such image known here.

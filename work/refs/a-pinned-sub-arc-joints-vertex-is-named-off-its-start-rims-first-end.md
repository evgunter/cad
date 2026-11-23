---
id: a-pinned-sub-arc-joints-vertex-is-named-off-its-start-rims-first-end
kind: issue
title: A pinned sub-arc joint's vertex is named off its start rim's first end (walk_corner's fallback), an orientation no row pins
status: open
opened: 2026-11-23
priority: P3
cost: E
---

Found by LATHE's review of `lathe/skin-sub-arc-walls`
(`work/lathe/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint.md`).

## What happens

The loft names a profile corner's vertices, and a sub-arc joint's, with
one walk up its struts, `walk_corner`
(`crates/editor-core/src/names/emit_sweep.rs:568`). Where the first slab
has a strut, the walk starts at the vertex the strut shares with the
piece's start rim. Where it has none, because the stacking door pinned
the corner or joint in that slab, it starts at `edge_ends(start_rim).0`,
the rim's stored first end (`emit_sweep.rs:588`), and assumes that is the
piece's start. The joint's start rim is sub-arc `i + 1`'s
(`emit_sweep.rs:532`).

The branch builds a pinned joint now (`sweep`'s
`pinned_corners_build::a_pinned_sub_arc_joint_builds_as_a_pinned_corner`:
a half disc turned about the line through its apex, and a circle about
the tangent at its 90° joint), but no naming row reads a pinned joint's
vertex name, so whether the rim's stored first end is the joint is not
pinned for either fixture, nor for a rim the loft minted running against
its profile.

## Fix

Read the pinned vertex as the one the start rim shares with the
previous piece's rim (or the end rim's far end), not by the rim's stored
orientation, and pin both fixtures' joint vertex names in
`crates/editor-core/tests/loft_section_ids.rs`.


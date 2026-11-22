---
id: a-trim-image-an-ulp-past-its-walls-domain-grows-the-cell-candidates-without-bound
kind: issue
title: mesh::trimmed's per-cell candidates grow without bound (a 10 GiB allocation) on a trim image an ulp past its spline wall's domain
status: open
opened: 2026-11-18
---


Found by `importgate/build`.

## What

While the build was being written, `crates/step-import/tests/a_foreign_carrier_splits_at_each_c0_knot.rs`'s
`a_rational_wall_splits_where_its_rims_carry_no_knot` imported the
half cone (`tests/fixtures/c0-knot/half_cone.step`) with two straight
edges on its rational wall whose iso images ran
`IsoLine { p0: (0, 0), ends: [(0, 0), (0, 1.0000000000000002)] }` — the
wall's `v` domain is `[0, 1]`, so each image ended one ulp past it —
and tessellating the imported body aborted:
`memory allocation of 10737418240 bytes failed`, in
`mesh::trimmed::per_cell_candidates` (`RawVec<mesh::trimmed::Node>::grow_one`,
under `tessellate_trimmed`). The same body with the images ending at
`1.0` exactly tessellates in 0.12 s.

A trim image an ulp outside its chart is within any band the mesher
reads, so it should mesh, or refuse typed; an allocation that grows
without bound is neither.

## Where it stands

The import holds an iso image to its own ends
(`crates/step-import/src/build.rs`, `IsoMap::image`), so no committed
fixture reaches it. The witness above is a two-line change to that
clamp.

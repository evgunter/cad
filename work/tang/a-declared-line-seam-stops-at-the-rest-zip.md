---
id: a-declared-line-seam-stops-at-the-rest-zip
kind: issue
title: A seam declared along a line (the D-bar on the slab) verifies and stops at the declared-Rest zip, ParallelSeamEdges
status: open
opened: 2026-10-02
---

## What

The D-bar of `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`
(`a_d_bar_on_the_slab_verifies_its_line_seams_and_stops_at_the_zip`). It
is a half rod of radius 0.5 whose flat lies on the slab's side face
`x = 2`, declared `Rest`, with its end faces flush with the slab's,
declared continuations. Its wall starts at the slab's top and bottom
edges, and leaves each tangent ruling away from the slab. The wall,
declared a `Seam` against the slab's top and bottom faces, VERIFIES in
both member orders along the DEV-1 line locus: the `Tangent` lane with
the sense bit reversed, then `rim_wedge::line_side`. The union then
refuses `RestZipUnsupported { what: ParallelSeamEdges }`.

The stadium (the full rod, its wall straddling the ruling with half of
it inside the slab) no longer reaches this far. Declared a `Seam`, it is
contradicted on `seam_line_side`, which is right: those two faces do not
leave the line on opposite sides.

## Why

The declared-`Rest` zip (`boolean/rest.rs`) meets two seam edges
parallel to each other on the flat's boundary, the two tangent rulings,
and has no arm for them. Past it, the curved coplanar-lump sites read a
declared distinct-carrier tangency as `Tangent` only:
`recl::require_same`, `sectors::tangent_lump`, `recl`'s tangent-flank
germ short-circuit, and `vtxfac`'s admitted list. A seam's lump verdict
is the same second-order question with the material on the same side.
The rim seam (the sphere-capped tube) reaches none of these sites.

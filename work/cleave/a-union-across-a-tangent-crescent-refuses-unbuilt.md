---
id: a-union-across-a-tangent-crescent-refuses-unbuilt
kind: issue
title: A union of two plane faces overlapping in a crescent tangent at a vertex refuses TangentCrescentUnsupported: no germ runs along a region of zero angle
status: dispatched
opened: 2026-10-25
priority: P1
cost: H
branch: cleave/tangent-crescent-builds
---


Filed from
`a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`,
whose wrong body now refuses typed. Pinned by
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose`.

## The refusal

A boss `r = 0.3` over a hole or pocket `r = 0.1` about `(0.2, 0)` has
its floor and the plate's top face lying on each other with opposed
normals over a crescent that narrows to the tangent point `(0.3, 0, 0)`.
Where either rim has a vertex there (the other rim is split there), or
neither has and the sweep splits both at their touch
(`boolean/reduce.rs` `interior`), the two vertices are a vertex pair,
and at that pair the floor's sector and the top face's sector share the rims' common tangent and lie on opposite
sides of it: at first order they only touch, and `sectors.rs`
`pair_search` lists the pair as touching (`sector_overlap` reads no
overlap). One order down they overlap, `(κ_b − κ_a)·û > 0`
(`sectors.rs` `crescent`). Under union, `sectors.rs` `crescent_refusal`
(called from `boolean/mod.rs` after `pair_search`) reads that on two
opposed plane faces and refuses `TangentCrescentUnsupported`, in
one-segment and two-arc forms, through and pocket, both operand orders,
at the default ε, 1e-6 and 1e-12. ∩ and both differences keep the
operands and are `SOUND`. The refusal also fires on one pose `main`
built: `a-crescent-refusal-fires-where-short-arcs-carry-the-section`.

## What building it takes

The union glues the two faces over the crescent: the plate's top keeps
the boss's rim as its ring, and the boss's floor shrinks to the part
over the hole, its boundary the hole's rim. The two rims then meet at
one pinch vertex (one shell, through; a pocket's cavity becomes a void
shell touching the outer shell at that point). Nothing upstream of the
join produces the section's germs along a region of zero angle:

- the sector passes (`recl.rs` `recl_sectors`, Program 15.10) read the
  pair as coplanar `On` at most, and reclassify it to no surviving
  record (measured: with the crescent read as an overlap, every record
  of the pair ends `intersect: false` and the union is unchanged);
- the section is two closed loops through one site, each a rim lying in
  the partner's face, and `join.rs` `one_site_segments` (≈935) takes
  one loop per site.

## Around it

- Faces aligned rather than opposed (a holed plate standing on `z = 0`
  beside the boss, its hole tangent inside the boss's rim) put the same
  crescent between the floors; the operands then overlap, and every op
  refuses at the join (`SingleSiteSectionLoop { count: 1 }` one
  segment, `UnpairedLooseEnds { count: 4 }` two arcs). The crescent
  reading is not run there.
- The reading takes a bound's bend from a conic or a line edge. A
  fitted (spline) edge and a bisector read none, so a crescent between
  a spline outline and a circle is not read, and neither is one between
  two conics whose curvatures tie at the vertex (an ellipse's vertex
  osculating a circle). Both are unmeasured.
- Two faces on one curved carrier could overlap in a crescent too; the
  reading runs on plane faces only, and no such pose has been built.

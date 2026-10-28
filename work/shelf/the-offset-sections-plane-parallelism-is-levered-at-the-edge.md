---
id: the-offset-sections-plane-parallelism-is-levered-at-the-edge
kind: issue
title: An offset's re-derived plane × plane section decides parallelism levered at the moved edge's own extent, not its faces'
status: open
opened: 2026-10-16
priority: P3
cost: M
---

`crates/topo/src/offset_derive.rs:656` (`plane_plane`) decides whether a moved plane and its held neighbour are parallel as `zero("offset_section_planes_parallel", dir.norm() * extent, band)`: the sine of the angle between the two planes, levered at `extent`, which `replace_face` passes as the moved edge's own extent (`geom_brep::edge_extent` at `crates/topo/src/replace_face.rs:2512`, `:2932`, and `corner_arm` `:3807`). That is the dihedral between the edge's two faces, and D4 ¶1 levers parallelism at the face extent: since TANG's U-lever (PR 5204), every other reader of that wedge levers it at the faces' reach (`topo::wedge_reach`).

The reading errs in the safe direction: a short edge between large planes refuses "the two planes are parallel" where its faces open a definite corner. It is not local to fix: the same `extent` threads through every section kernel `offset_derive` dispatches (`c5::cone_cylinder_section`, `c5::plane_torus_section`, `offset_corner_graze`, `level_row`), where it is each section's own scale and not a wedge. The fix gives `plane_plane` (and `offset_corner_graze`, if it is the same wedge) the faces' reach as a separate argument, read off the body as it stands before the move, and leaves the section scale alone.

Found by the U-lever review (PR 5204, finding m2); grep `\* ?extent\b|extent ?\*` over `crates/*/src` for siblings.

## The edge extent's new home (OFFSET, 2026-10-27)

`geom_brep::edge_extent` no longer exists (OFFSET, PR 5875). Its home is `geom_brep::extent`, beside the reach family, as three functions: `edge_extent_inf` (≤ the edge's diameter `D`: the chord, the minor-axis folds and the sampled pairs; the lever of a reading whose Zero only refuses), `edge_extent_sup` (≥ `D`; a reading whose Zero is acted on and whose definite signs only refuse) and `edge_reach_from` (the edge's farthest distance from a point of it; a reading that acts on every sign, read from that point). `offset_derive::section_closed`'s plane pair still reads the edge's extent from below (its Zero refuses, its Positive certifies the crossing); the torus's axis fork and the cone and cylinder's parallelism now read `edge_extent_sup`, since they act on their Zero.

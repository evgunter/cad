---
id: a-knife-edge-blend-is-unbuilt-though-its-rolling-ball-is-defined
kind: issue
title: A knife edge (cusp or slit) refuses KnifeEdge, though its rolling ball is defined away from the edge wherever the supports open to 2r
status: closed
opened: 2026-11-02
priority: P1
cost: H
closed: 2026-11-04
branch: band/knife-edge-blend
pr: 6510
---

Filed by the REFS orchestrator (2026-11-02), from Ev's questions on PR 6344 ("why do cusps and slits refuse? just not built yet?"; "if it's an exact rolling ball then it's not local anyway").

**Yes, it is unbuilt, not undefined.** An exact rolling ball of radius r on a knife edge (a material wedge of 0 for a cusp, 2π for a slit) is defined where the supports' r-offsets meet. Example: the concave fill between two tangent cylinders of radius R, whose ball centre sits at distance R + r from both axes. Its feet do not lie at the edge, as they do at a transverse edge. They lie away from it, where the supports have opened to 2r.

So the construction differs from every built arm in two ways:
- the band does not replace a neighbourhood of the edge; it bridges a region between the edge and the feet, which must be filled (slit) or trimmed (cusp);
- the feet can run off the supports' extents before the supports open to 2r, in which case the blend refuses by the existing reach and face-clearance predicates.

Today the battery refuses every knife link as `BlendError::KnifeEdge` (PR 6344), with a recourse that says a cusp or slit has no wedge to round. When this row is built, that refusal narrows to the knives whose ball is undefined, or whose feet leave the supports.

Band P1 under work/README ("verb breadth beyond the everyday shapes"): a fillet between two touching tubes, or into the crack of a declared slit. Measure first, with the declared cusp and slit bodies of `crates/sweep/tests/wedge_end_doors.rs` and two tangent cylinders.

## Closed (2026-11-04)

The fillet builds a knife where its ball is defined. Predicate 5 reads a knife's end from its jet (`battery::knife_end`, `material_wedge_side`). A cusp's ball rests in the material crescent, as a convex edge's does; a slit's rests in the void crescent, as a concave edge's does. The curved arms' sheet reduction picks the offsets' crossing on the side both faces leave the edge by (`arms::Branch::Knife`), where the rim branch read an exact zero.

`KnifeEdge` narrows in two ways. For the fillet it means only that the ball is undefined: `fillet3_knife_room`, the offset traces' secant margin, is decided zero or negative (`KnifeRefusal::NoRoom`). A ball whose feet leave the supports refuses at the existing clearance, ring and reach meters. The chamfer refuses every knife (`KnifeRefusal::Chamfer`), since its arms are plane–plane.

Rows, each checked against a closed-form ΔV: `crates/sweep/tests/a_knife_edge_fillet_builds_where_its_ball_is_defined.rs`. The touch between two solids that the measurement turned up is `a-fillet-leaves-the-kiss-of-two-touching-solids-as-it-is-with-no-refusal`.

---
id: a-knife-edge-blend-is-unbuilt-though-its-rolling-ball-is-defined
kind: issue
title: A knife edge (cusp or slit) refuses KnifeEdge, though its rolling ball is defined away from the edge wherever the supports open to 2r
status: open
opened: 2026-11-02
priority: P1
cost: H
---

Filed by the REFS orchestrator (2026-11-02), from Ev's questions on PR 6344 ("why do cusps and slits refuse? just not built yet?"; "if it's an exact rolling ball then it's not local anyway").

**Yes, it is unbuilt, not undefined.** An exact rolling ball of radius r on a knife edge (a material wedge of 0 for a cusp, 2π for a slit) is defined where the supports' r-offsets meet. Example: the concave fill between two tangent cylinders of radius R, whose ball centre sits at distance R + r from both axes. Its feet do not lie at the edge, as they do at a transverse edge. They lie away from it, where the supports have opened to 2r.

So the construction differs from every built arm in two ways:
- the band does not replace a neighbourhood of the edge; it bridges a region between the edge and the feet, which must be filled (slit) or trimmed (cusp);
- the feet can run off the supports' extents before the supports open to 2r, in which case the blend refuses by the existing reach and face-clearance predicates.

Today the battery refuses every knife link as `BlendError::KnifeEdge` (PR 6344), with a recourse that says a cusp or slit has no wedge to round. When this row is built, that refusal narrows to the knives whose ball is undefined, or whose feet leave the supports.

Band P1 under work/README ("verb breadth beyond the everyday shapes"): a fillet between two touching tubes, or into the crack of a declared slit. Measure first, with the declared cusp and slit bodies of `crates/sweep/tests/wedge_end_doors.rs` and two tangent cylinders.

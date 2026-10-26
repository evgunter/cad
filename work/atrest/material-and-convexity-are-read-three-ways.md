---
id: material-and-convexity-are-read-three-ways
kind: issue
title: A cell's material and an edge's convexity are read three ways with three levers
status: open
priority: P1
cost: H
design: true
opened: 2026-10-26
refs: [gap-reads-vertex-and-edge-cells, gap-reads-no-curved-cell-below-a-face]
---

Three readers decide which side of a vertex's or an edge's faces its material lies on, and each has its own lever and its own decision:

- **`Gap`'s cell arms** (`crates/editor-core/src/eval/measure.rs`: `Wedge::turn`, `Fold::of_corner`, `Fold::offset`) decide an edge's turn as the sign of `dir·(plus × minus)`. That sign is decided under `gap_edge_convexity`, levered by the faces' reach (`topo::edge_wedge_reach`). A vertex's material is then the intersection or the union of its faces' half-spaces, and a saddle refuses.
- **The census's touch analysis** (`crates/topo/src/census.rs`, the section before `TOUCH_WEDGE`, and `rest::judge`) reads each solid's whole link at a touch point as polygonal cones. Every sign it decides is a point's signed distance from a plane, through one door (`metric::Distance`). Directions only propose planes, and one named `pairing` decision is allowed (`census::tests::the_touch_analysis_decides_only_through_its_doors` holds that). `Gap` decides a sign of normals directly, which that discipline rules out, and it reads a saddle not at all where the census reads one sector by sector.
- **`geom_brep::dihedral`** (`classify_dihedral`, `classify_material_pairing`, `material_kappa_rel`) classifies the wedge by the tangent planes' angle, levered by `folded_lever_arm` (curvature arms and the faces' reach). It then signs the wedge by the faces' material pairing and the jet's κ_rel. This is the one reader that handles curved faces.

`crates/editor-core/src/checks/at_rest.rs` `opposed` reads a curved face's sense off `Face::sense` directly, beside a `Carrier` that now carries the same fact as `convex`. That is a fourth spelling, one hop from the measure's. It is D′3's file and was left alone.

**What a fix looks like (not decided).** One reader of a cell's local material, at the census's one-door discipline: signs are point-to-plane distances, and directions only propose planes. `Gap`, the census and the dihedral classifier each read it at their own point. The curved-face cases of `gap-reads-no-curved-cell-below-a-face` and the saddle then come with it. `opposed` reads `Carrier`'s `convex`.

Also see `sector_shape.rs` and `census/curved`, which read sectors the same way the census does.

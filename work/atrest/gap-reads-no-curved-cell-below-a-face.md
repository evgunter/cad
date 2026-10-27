---
id: gap-reads-no-curved-cell-below-a-face
kind: issue
title: Gap has no arm for a curved edge, a cell beside a curved face, or a cell against a cone, torus or NURBS face
status: open
priority: P2
cost: M
opened: 2026-10-26
refs: [gap-reads-vertex-and-edge-cells, material-and-convexity-are-read-three-ways]
---

`gap-reads-vertex-and-edge-cells` gave `Gap` its vertex and edge arms (`crates/editor-core/src/eval/measure.rs`, `gap` and its module table). A vertex's or an edge's material is read as its link (`topo::Link`, `crates/topo/src/link.rs`), which holds plane faces bounded by line edges only, so these refuse `MeasureUnsupported`:

- **a circular or curved edge** in any pair (`carrier_of` reads it as `Carrier::Other("a circular edge")`), so a rim resting on a plate cannot be asserted;
- **a line edge beside a curved face** as the reading side: an edge against a vertex or against another edge, whose link does not build (`NO_WEDGE`), e.g. a cylinder's seam ruling;
- **a vertex on a curved face, or on a curved edge,** as the reading side (`NO_CORNER`);
- **a vertex or a line edge against a cone, torus or NURBS face**, in either seat (`Carrier::Other("a cone face")` and the like);
- **a vertex against an edge or a vertex** where the reading cell's link refuses for another reason (`Link::at_point`, `Link::on_edge`): a face meeting the point twice, faces lying on each other, a piece in band, or the parity reading in band (`the-census-reads-its-link-without-the-wide-parity`);
- **a line edge whose ends do not read** ("a line edge whose ends do not read"), a body the evaluation built wrong;

Each such contact stays loud under stage 5 C's quieting rule, since no `Gap` over its cells evaluates. Each arm keeps the sign convention the module states: zero at rest, positive apart, negative inside.

## Re-cut (2026-10-26, ATREST orchestrator)

Under the decision on `material-and-convexity-are-read-three-ways`:
- The saddle and near-flat-edge cases retire with the half-space fold.
- A vertex or edge against a sphere or cylinder face becomes one point-to-carrier arm in `topo::link`'s metric door. It is shared with the census's curved-face touch refusal ("cannot yet read which side of a curved face material lies on"), and both close together.
- Still refused after that arm: cone, torus and NURBS faces (no closed-form metre), and a curved inner edge (its extremal points belong to `geom_brep`'s curve machinery).

The link build landed with `material-and-convexity-are-read-three-ways`: the saddle reads (`real_reflex_strut_and_its_saddle_vertex_read_by_their_links`), and no edge's dihedral is decided any more. What is left here is the curved arm.

## Narrowed (2026-10-27, `atrest/gap-reads-a-cell-against-a-curved-face`)

The curved arm is built. `topo::link::Round` is a sphere or cylinder face's material read by its carrier, and `metric::Distance::off` is its constructor in the door. `Gap` reads a vertex or a line edge against a sphere or cylinder face through it, in either seat. An edge is read at its lowest point, so a line edge against a sphere, or across a cylinder at any angle, no longer refuses. The census reads the same arm (`rest::by_round`): at a touch point strictly inside a sphere or cylinder face, where the face is all its solid holds there, it judges the other solid's planar link against it. Backstop arm 1 refuses every such cross-solid pair as a curved pair within reach before the touch is read, so no census verdict moves yet. That is `restread/census-cross-solid-curved-pairs-undecidable-on-shell-results`, and its evidence section names this reader.

The bullet for a line edge against a sphere face, or against a cylinder whose axis does not run along it, is closed and removed. What stays on this row is every bullet above: a curved edge in any pair, a cell whose own link holds a curved face (`NO_WEDGE`, `NO_CORNER`), a cone, torus or NURBS face in either seat, a link refusing for another reason, and an edge whose ends do not read.

## Re-prioritised (2026-10-27, ATREST orchestrator): P1 → P2

The row's core ask merged (#5853 / #5827). What stays is residue that needs machinery not yet built: exact coincidence or metric rungs for cone, torus and NURBS carriers, a curved inner edge's extremal points, and the in-band and rim-pair readings. Each case stays loud and could-not-look, never a wrong verdict, so it is P2 work rather than a P1 defect.

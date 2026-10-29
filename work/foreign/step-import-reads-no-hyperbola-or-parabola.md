---
id: step-import-reads-no-hyperbola-or-parabola
kind: issue
title: STEP import has no HYPERBOLA or PARABOLA reader: the window-taking conic arc needs the edge's ends, which the curve reader does not see
status: open
opened: 2026-10-25
priority: P3
cost: M
---


`crates/step-import/src/entities.rs` `Reader::curve` reads an edge's
curve record alone and refuses any keyword it has no arm for
(`UnsupportedEntity`), so a file's `HYPERBOLA` or `PARABOLA` edge stops
there. The kernel now mints a plane×cone parabola or hyperbola exactly,
as a rational-quadratic `Curve3::Nurbs` arc over a window
(`crates/geom-brep/src/intersect/conic_arc.rs`, `ConicBranch::arc`;
C1 of `crates/geom-brep/README.md`). Importing either record as the
same arc needs two things the curve reader lacks: the window (the
edge's two vertices; the reader re-derives a parameter interval from
them only after the carrier is built, `geometry::endpoint_params`), and
a door that mints the arc from the conic's own placement and semi-axes
rather than from a plane×cone pair. Found by the OFFSET conic-arc unit
(branch `offset/conic-arc-and-axial-section`), which left it as this row
because the reader would have to be re-routed through the edge layer.

---
id: geom-brep-readme-c12-misstates-the-mesh-lanes
kind: issue
title: crates/geom-brep/README.md C12 (6) says pcurve-trimmed faces are not implemented and cites UnsupportedCurvedShape for it
status: open
opened: 2026-09-20
priority: P4
cost: E
---


Found by TESS-1's lane (its first report's "looked off"), verified at
the fix pass by reading `crates/mesh/src/trimmed.rs` and
`crates/mesh/src/types.rs`. Filed on TESS because the sentence
describes `crates/mesh`; the file itself is a design page
(`crates/geom-brep/README.md`, clause C12), so the re-wording is a
describing-text change that lands with whoever takes this row, not a
design decision.

**The sentence** (C12, item 6): "general trimmed faces with
pcurve-driven trim loops are not implemented
(`UnsupportedCurvedShape`)."

**Both halves are wrong as the tree stands.**

1. Pcurve-driven trimmed faces ARE implemented, for two chart classes:
   `trimmed::tessellate_trimmed` meshes a cylinder face bounded by
   conic/B-spline trim carriers and a described NURBS / `Approx` face,
   from stored pcurves, with per-triangle certificates. What is not
   built is the trimmed lane on a cone, sphere or torus chart.
2. The refusal for that frontier is `UnsupportedCurve` (raised by
   `trimmed::trim_frontier`, its `note` naming the lane), not
   `UnsupportedCurvedShape`. `UnsupportedCurvedShape` is the
   iso-rectangle SHAPE door: an iso-bounded domain that is not a chart
   rectangle (a keyway, an L), with props' refusal as its `source`.

Present-tense replacement, for whoever takes it: curved tessellation
takes iso-rectangle chart domains from the boundary walk
(`UnsupportedCurvedShape` for an iso-bounded domain that is not a
rectangle, `MeridianFreeCurvedFace` for a loop with no meridian);
pcurve-trimmed faces mesh on cylinder and spline charts
(`mesh/src/trimmed.rs`) and refuse `UnsupportedCurve` on cone, sphere
and torus charts.

Not checked: the other eight items of C12.

---
id: split-section-boundary-curved-arm-untested-past-the-edge-gate
kind: issue
title: The section-boundary describer's Spiric/Nurbs arm has no test, because the split operand gate refuses those edge kinds first
status: open
opened: 2026-10-01
priority: P3
cost: E
---

Filed by CLEAVE's edge-midpoint lane (PR 3645).

`crates/topo/src/splitting/finish.rs` `describe_section_boundary` reads
every boundary edge through `geom_brep::IntersectionDraft::of`
(`crates/geom-brep/src/certify.rs`). A curved certified carrier
(`geom::Curve3::is_curved`: Circle, Ellipse, Spiric, Nurbs) keeps its
carrier and interval and is classified at its mid-parameter point and
`edge_extent`. Only Circle and Ellipse reach that arm today.
`splitting/classify.rs` `gate_operand` refuses any operand edge whose
carrier is `Spiric` or `Nurbs` (`SplitReduceError::CurvedEdgeUnsupported`)
before the split runs, and the plane x {plane, cylinder} sections it
admits mint only Line, Circle and Ellipse. So the Spiric and Nurbs arm
is unexercised: no test has seen it classify or certify a section edge.

**When** the gate admits either kind (a torus, cone or sphere face whose
section with the cutter is a spiric or a fitted spline, or an operand
edge of those kinds), the PR that retires that refusal adds a test. The
test splits through such an edge and asserts that the edge's stored
`Intersection` witness is its carrier's `mid_point`, and that the dihedral
verdict matches the one taken at that on-edge point, not at the chord
midpoint.


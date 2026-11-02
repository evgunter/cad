---
id: split-section-boundary-curved-arm-untested-past-the-edge-gate
kind: issue
title: The section-boundary describer's Spiric/Nurbs arm has no test, because the split operand gate refuses those edge kinds first
status: open
opened: 2026-10-01
priority: P1
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


## Moved to SPLICE (2026-10-25)

From GAUGE. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P3 → P1: it becomes live the moment the gate goes, so it ships with the deletion.

## The dihedral it reads (SPLICE, 2026-10-27)

`describe_section_boundary` classifies a boundary edge through
`geom_brep::classify_dihedral`, whose implicit forms are poison on a
spline face, so a section edge on a loft wall would refuse at the arm
(`"dihedral_arm"`, invalid) before it is described. The boolean's seam
describer reads the same question through `geom_brep::classify_seam`,
each spline face at the edge's pcurve on it (`topo::boolean::ops::seam_face`);
the split's describer is the same shape and wants the same reading when
its gate admits a NURBS edge. The split's sector rule
(`splitting::rules`, its `classify_dihedral` at a vertex) is the same
case at a vertex.

## The edge extent's new home (OFFSET, 2026-10-27)

`geom_brep::edge_extent` no longer exists (OFFSET, PR 5875). Its home is `geom_brep::extent`, beside the reach family, as three functions: `edge_extent_inf` (≤ the edge's diameter `D`: the chord, the minor-axis folds and the sampled pairs; the lever of a reading whose Zero only refuses), `edge_extent_sup` (≥ `D`; a reading whose Zero is acted on and whose definite signs only refuse) and `edge_reach_from` (the edge's farthest distance from a point of it; a reading that acts on every sign, read from that point). The curved arm this row cites reads `edge_extent_inf` through `ConicDeparture`.

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

---
id: a-quadric-pairs-section-spline-carries-a-c0-knot-the-chord-pass-refuses
kind: issue
title: a cylinder-ball boolean builds and never meshes - its section edge's spline carrier has an interior knot of full multiplicity, which the chord pass refuses
status: open
opened: 2026-11-14
priority: P0
cost: M
---


Found by TESSLANE's reach measurement (2026-11-14, `d6e1eb1f`; probe `reach_probes::a_sphere_face_bounded_by_a_spline_section_refuses_in_the_chord_pass` on branch `tesslane/measure-reach`).

## Measured

A cylinder of radius 0.5 against a ball of radius 0.3 centred at `(0.5, 0, 0)` (the first pose of `cylinder_sphere_frame`): all six boolean ops build, and `mesh::tessellate` refuses all six at δ 2e-2, 5e-3 and 1e-3:

    UnsupportedCurve { note: "B-spline carrier with a C⁰ kink (interior multiplicity = degree) …" }

The raising site is `chords::nurbs_chord_count`, in the chord pass, before any face is dispatched. The section edges carry `Curve3::Nurbs` with `Pcurve::Projected` images on both the cylinder and the sphere face. Three more poses (the ball's centre inside the cylinder, outside it, and a two-loop section; ∩ only, δ 5e-3) refuse the same way. The cylinder face has a trimmed arm and would mesh; the refusal is ahead of it.

So no boolean of a cylinder and a ball in a general pose can be drawn.

## Where the fix belongs

`nurbs_chord_count`'s secant bound needs C¹, and it refuses any interior knot of multiplicity above `p − 1`. Ev's ruling on PR 5913 (DESIGN `[topology]`) says an edge carrier is C¹ by its knot structure strictly inside its cell, and that a construction whose carrier is not puts a vertex there; no reader carries a crease arm. Read with that, the carrier this section mints is the defect, and the chord pass is right to refuse it. Not yet measured: whether the knot is a true tangent break or a full-multiplicity knot on a smooth curve (a piecewise-Bézier fit), which decides between cutting the edge at the knot and minting the curve with a C¹ knot vector.

The mint is on other ground: `geom-brep`'s `ssi.rs` (`cylinder_sphere_ssi`; SSIARITH, SSIEDGE) and `pair_section.rs` (the SECT family). This row sits here because the refusal is the mesh's and the mint's owner is not yet identified; whoever owns the mint claims it by moving it. `tier-3-refuses-a-carrier-with-an-interior-c0-knot` (MEASUREDOOR) will refuse these bodies at rest once it lands, so this row is on its path too.

## Done when

The six ops of the measured pose mesh and pass `check_mesh`, and the probe is re-pinned to that.

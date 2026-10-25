---
id: loft-of-three-or-more-sections-at-degree-one-is-unmeshable
kind: issue
title: A degree-1 loft of three or more sections refuses to mesh at its interior section crease, polygon profile included
status: open
opened: 2026-10-23
priority: P0
cost: M
---


(ISO measurement lane, 2026-10-23, on main `e4045c3132`; filed by the
ISO orchestrator — off ISO's ground.)

## What

A pure-translation `sweep::loft_body::<f64>` at v-degree 1 through
n ≥ 3 sections builds, passes `topo::validate_geometric` (tier 3),
`validate_pseudomanifold` and a re-run of `topo::mint_pcurves`, and then
`mesh::tessellate(body, 1e-2, tol)` refuses at every n = 3..9 and every
spacing measured (uniform, `0.6t + 0.4t²`, jittered):

```text
TessellateError::UnsupportedNurbsFace { face: FaceKey(3v1), note: "NURBS
face of degree 1 with a sharp crease inside it. Recourse: split the face
at the crease" }
```

It is raised by the C¹ gate, `geom-brep/src/patch_bound.rs`
`check_direction`, through `geom_core` `KnotVector::c1` →
`C1Break::Degree1Crease`. The loft's interior sections are
multiplicity-1 interior v knots of a degree-1 wall, so every such wall
carries a crease. A **square** profile refuses the same way, so this is
not about arcs.

## Why P0

The GUI's loft node admits any v degree in `1 ..= sections − 1`
(`editor-core/src/eval/wire.rs`, the `SlotId::VDegree` read), so a
user can author a body that is valid at rest and that the viewer cannot
draw. The recourse names a split the user has no verb for.

## Context

`lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`
(closed, PR 5499) admitted degree-≥2 creases on the rational arm of
`patch_bound::patch_cells_across_creases` and recorded that the
integral arm and degree-1 creases still refuse. No live row held that
residue. Whether the fix is the crease as a CDT constraint, as there,
or the loft splitting its walls at the sections, is for the taking unit.

## The integral arm refuses degree-≥ 2 creases too (ISO, 2026-10-24, PR 5635)

This gap is wider than degree 1. `patch_bound::patch_cells_across_creases`
admits a C⁰ crease only on its rational arm. The integral arm
differences the whole net and keeps `check_direction`'s
`PatchBoundError::Crease` at every degree. PR 5635 moved the arm choice
to `NurbsSurface::constant_weight`, so a creased net with one constant
weight `c ≠ 1` now refuses there as well, as the unit net already did.
The PR 5635 reviewer's witness is the half cylinder of
`a_rational_crease_reads_as_two_patches`
(`crates/geom-brep/src/patch_bound.rs`): two quarter arcs at degree 2
joined at a double knot, with weights `[2; 10]`. The rational arm used
to admit it; the integral arm now refuses it `Crease`. The same test
pins it at `c ∈ {1, 2}`. No shipped construction mints a constant
non-unit creased net, so this is disclosure. The fix this row takes
(per-span reading on the integral arm, or the crease as a constraint)
should cover both degrees.

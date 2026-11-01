---
id: loft-of-three-or-more-sections-at-degree-one-is-unmeshable
kind: issue
title: A degree-1 loft of three or more sections refuses to mesh at its interior section crease, polygon profile included
status: closed
opened: 2026-10-23
priority: P0
cost: M
closed: 2026-10-28
pr: 5923
branch: tess/degree1-loft-slabs
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

## Weighed (2026-10-28, design-fork row 209)

Two designers weighed it over three reconciliation rounds. Both measured in scratch copies.
- **The refusal is two doors deep.** With the patch-bound gate opened on both arms, a 3- or 4-section square loft at degree 1 refuses next at the chord pass. Its wall–wall strut is a degree-1 B-spline through the section corners (knots `[0,0,0.3387…,1,1]`), which `chords::nurbs_chord_count` refuses as "a C⁰ polyline". With that door opened too, the loft meshes at n = 3 and 4, δ ∈ {1e-1, 1e-2, 1e-3}, and no third door fires. A unit sized to the patch bound alone does not close this row.
- **Agreed:** at v-degree 1 the loft builds one wall per segment per slab, with an edge ring at each interior section and a straight strut per slab, which meshes today. The patch bound becomes one per-span door with no crease gate.
- **Not agreed:** whether a smooth joint the representation introduces (the skin's quarter-arc double knot, and a foreign C⁰ knot at import) is also cut. That is `[open.representation-joint]`, put to Ev on the `[ev]` PR.
- **Stale on the closed predecessor row:** QUAD does not refuse today's quadratic lofted cylinder. At ε = 1e-9 it stops `Met`; at 1e-12 it stops `Wall` with pad 5.3e-9.
- **Dead option:** a semicircle as one rational cubic span (weights 1, ⅓, ⅓, 1) is refused at the pcurve mint (`pcurve_cache` `IsoUnsupported`: the `IsoArc` class certifies only the rational-quadratic arc).

## Ruled (2026-10-28, Ev on `[ev]` PR 5913)

- **Decision 1:** the loft cuts at every section at v-degree 1, and the finishing join merges the slabs whose carriers coincide. The slab names key on the sections' input identity. This is built on PR 5923, which closes this row.
- **Decision 2 (S):** no face chart or edge carrier carries a C⁰ knot inside its cell. The rest of this row, the integral arm's crease at every degree, is carried by these rows:
  - `work/carvetail/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint`;
  - `work/foreign/import-splits-a-foreign-face-or-edge-at-each-c0-knot`;
  - `work/measuredoor/tier-3-refuses-a-carrier-with-an-interior-c0-knot`;
  - `work/tesslattice/the-crease-arms-retire-once-no-carrier-carries-a-c0-knot`.
- **Also filed:** `work/coincide/the-carrier-ladder-cannot-read-a-spline-carrier-so-coincident-spline-faces-never-merge`.

## Closed (2026-10-28, PR 5923)

At v-degree 1 through k ≥ 3 sections, the loft (and `sweep_body`) now cuts its walls at every section. That gives one single-span wall per segment per slab, a ring of edges at each interior section, and one straight strut per slab.
- **Measured:** every loft on this row's grid (n = 3..9, three spacings, square and circle) meshes at δ = 1e-2 and 2e-3 and passes `check_mesh`, and its mesh volume matches the closed form.
- **Tiers:** tiers 2, 3 and 3′ pass.
- **Unchanged:** degree ≥ 2 and two-section lofts are byte-identical to main.
- **Reviewed:** single FULL review, APPROVE-WITH-FIXES; the fix pass addressed every finding.
- **New refusal:** an interior arc row refined by its neighbours' sub-arc joints refuses typed with its recourse, as evidence on `work/isotail/a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim`.
- **Moved off this row:** the slab names' positional placeholder is `work/refs/loft-slab-and-section-names-are-positional` (P1, open). The rest of the crease work is carried by the four rows the Ruled section names.

---
id: the-plane-cone-arc-is-minted-exactly-and-the-axial-door-reads-the-moved-section
kind: unit
title: Conic build unit 2: plane_cone_section mints a parabola or hyperbola as an exact rational-quadratic arc through a window-taking door, and the axial door reads the moved pair's section for every axis-parallel-plane edge
status: closed
opened: 2026-10-25
priority: P0
cost: H
parent: offset-lane-has-no-conic-carrier
closed: 2026-10-25
pr: 5692
branch: offset/conic-arc-and-axial-section
---


Ev's ruling on PR 5652 (`offset-lane-has-no-conic-carrier`, "Ruling");
C1 of `crates/geom-brep/README.md`.

- `plane_cone_section` (`crates/geom-brep/src/intersect.rs`): on
  `pn_conic_type` positive or zero, mint the section's arc exactly as a
  rational-quadratic `Curve3::Nurbs` (weights `(1, w, 1)`; ends from the
  window, shoulder from the end tangents in the plane, `w` from one more
  section point), through a window-taking door shaped like rung 3's
  `branch_at` (which seed each caller passes: the axial door the solved
  corners, the general door the old edge's middle, the boolean the
  section-certificate rows). Split long hyperbolic arcs into spans so `w`
  stays moderate. `OutsideConic`'s refusal and its "by decision" texts
  retire (`intersect.rs`, `topo/src/boolean/mod.rs`, `boolean/join.rs`,
  `docs/DRAFT-DESIGN.md`, `docs/KERNEL-VERBS.md`,
  `docs/GERM-CONE-SECTOR-SPEC.md`). Verify `NurbsCurve3::new` takes
  `w > 1` and the chord meter is not loose on a steep arc.
- The axial door (`crates/topo/src/offset_axial.rs`): an edge between a
  surface of revolution and an axis-parallel plane takes the moved pair's
  own section, branch picked at the old edge; `Constraint::Generator`
  joins `Ball`/`Torus` in the rim arm. Until such an edge is certifiable
  (unit 3, or the cone enclosure on CHARTTAIL), the door refuses it at
  its gate by name, before the corner solve.
- STEP import of `HYPERBOLA`/`PARABOLA` as the same arc
  (`crates/step-import/src/entities.rs`), if small; else a row.

Un-parks SHELF's `partial-cone-frustum-three-quarter-turn-refuses-edge-disagreement`
once unit 3 lands. A boolean cutting a cone with a flat still needs a
NURBS crossing layer (not this unit).

## Un-parked (2026-10-25, OFFSET orchestrator)

It was parked on unit 1 by mistake. The arc is a `Curve3::Nurbs`
carrier, so it already owes limbs 2 and 3 through the existing NURBS path
(`edge_nurbs::analytic_rung3`), whatever unit 1 decides for the
closed-form kinds. Unit 1 is now parked on its own fork
(`a-lane-less-door-refusing-closed-form-carriers-refuses-every-dual-evaluation`).
Like every NURBS carrier today, the arc refuses at a `Dual64`
(`NurbsLaneUnsupported`). That is existing behaviour, not this unit's.

## Closed

PR 5692. `plane_cone_section` serves a parabola or hyperbola as
`PlaneConeSection::OpenConic`; `OpenConicSection::branch_at` picks the
branch by a seed and `ConicBranch::arc` mints the window's arc exactly
as a rational-quadratic `Curve3::Nurbs`
(`crates/geom-brep/src/intersect/conic_arc.rs`), a hyperbolic window cut
into `2^k` equal-parameter spans of `w ≤ √2`. The "by decision" refusal
is retired; the Boolean (`GermSectionOpenConic`), the split
(`SplitJoinError::OpenConicChord`) and the general offset door
(`SectionVerdict::OpenConic`) refuse typed. The axial door takes the
cone-cap section and refuses the hyperbola at its gate
(`ReplaceFaceError::TogetherConicArc`) until unit 3 or CHARTTAIL
certifies the arc. Residue filed:
`work/foreign/step-import-reads-no-hyperbola-or-parabola.md`,
`work/select/edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short.md`.

---
id: the-plane-cone-arc-is-minted-exactly-and-the-axial-door-reads-the-moved-section
kind: unit
title: Conic build unit 2: plane_cone_section mints a parabola or hyperbola as an exact rational-quadratic arc through a window-taking door, and the axial door reads the moved pair's section for every axis-parallel-plane edge
status: parked
opened: 2026-10-25
priority: P0
cost: H
parent: offset-lane-has-no-conic-carrier
blocked_on: [closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them]
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

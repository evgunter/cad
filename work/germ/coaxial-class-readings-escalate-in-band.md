---
id: coaxial-class-readings-escalate-in-band
kind: issue
title: The coaxial and axis-parallel readings of the cone pairs and the sphere x torus pose escalate in band where D3 may take the general lane
status: open
opened: 2026-11-20
priority: P2
cost: M
---


Filed by GERM's `section-class-lane-choices-escalate-in-band` lane, from
its sweep of section-class readings in `intersect.rs` and its
neighbours.

## What

Each reading below picks the coaxial (or axis-parallel) closed form on
Zero and routes its definite side to the general rung, a lever-only
"not yet"; in band it escalates with a tolerance lever
(`SectionError::Escalated`):

- `crates/geom-brep/src/intersect.rs` `cone_sphere_section` (`coaxial_cone_sphere`):
  `kos_coaxial` (the sphere centre's distance from the cone axis).
- `intersect.rs` `cone_torus_section` (`coaxial_cone_torus`): `kot_coaxial` and
  `kot_axes_parallel`.
- `intersect.rs` `cone_cylinder_section`: `coc_axes_parallel`, and
  `coc_coaxial` (`decide_across` with the swing beside it), whose
  definite side also routes to the general rung.
- `crates/geom-brep/src/pair_section/torus.rs` `SphereTorusPose::decide`:
  `st_coaxial`, whose definite side is the served general march
  (`StStance::Apart`), and whose in band refuses through
  `undecided_symmetry`.
- `pair_section/torus.rs` `axis_pose`, read by `CylinderTorusPose::decide`
  (and the torus pair): its `AxisPose::Coaxial` arm takes the coaxial
  circles, the rest the swept general march; how it reads the band is
  unmeasured.

`cs_coaxial` (cylinder x sphere) is the same shape and was ruled
"escalate" by SECT on 2026-10-27
(`work/sect/in-band-axis-offset-is-noarm-at-one-arm-and-escalates-at-another.md`),
after D3's lane-choice clause (2026-10-14); whether that ruling read it
as a structural act or predates applying the clause to it is part of
this row.

The STEP snap does not depend on these rows: since PR 7171,
`step_import::snap::relations` refuses a relation of any kind read in
the file's band on its own reading (`step_snap_relation`), so a lane
choice here moves no import's refusal.

## Settle first

Whether each is a lane choice or a structural act (D3,
`[verdicts.consumers]`): the plane x torus pair's evidence in the PR that
filed this row is the method. There the loop count changes across the
axis band only where the closed form's own count row (`pt_cap_gap`, the
spiric regime) and the general lane's own count reading (topo
`section_cert::plane_cut`) are each in band, and the two agree wherever
they decide. Measure the same for each reading here (the off-axis
sphere's and torus's sections near the coaxial pose, and the tilted
cylinder's), at the three ε rows.

## Fix

Where it is a lane choice: in band takes the definite arm's lane
through `intersect::closed_form_shown`, recording nothing, with a row
per site at an in-band pose and a mutant restoring the escalation.

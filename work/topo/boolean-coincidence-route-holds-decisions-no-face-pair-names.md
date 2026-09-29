---
id: boolean-coincidence-route-holds-decisions-no-face-pair-names
kind: issue
title: topo: the Boolean's coincidence route still carries lever-arm, operand-radius and frontier decisions a face-pair declaration cannot name
status: open
opened: 2026-09-29
---

(TOPO, the §5 second pass of PR 3493.)

## What

PR 3493 routes `BooleanError::Escalated` by predicate
(`crates/topo/src/boolean/refusal_routes.rs`, `fn lever`). Following
the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`,
everything except the sector rung, the pierce normal, face
containment, where a crossing lands on an edge, and the kernel checks
keeps the coincidence story and its declare lever. Some names in that
coincidence group are decisions a face-pair declaration cannot name:

- the lever-arm gates `enters_material_arm`,
  `tangent_sector_order2_arm` (`geom-brep/src/enters.rs`) and
  `dihedral_arm` (`geom-brep/src/dihedral.rs`): whether a length to
  measure over is positive;
- the operand guards `cs_cylinder_radius` and `cs_sphere_radius`
  (`geom-brep/src/intersect.rs`, `cylinder_sphere_section`): whether a
  face's radius is positive;
- `bool_point_in_solid_denom` and `bool_ray_cylinder_disc`, which reach
  `Escalated` through `reduce`'s line × wall roots
  (`solid_contain::line_wall_roots`), and the `bool_ray_torus_*` rows
  through `line_torus_roots`: where an edge crosses a curved wall.

Three definite `BooleanError` arms offer the same menu
(`crates/topo/src/boolean/mod.rs`):

- `CurvedPierceUnsupported` (near :1659);
- `CurvedSectorSideUnsupported` (near :1667);
- `RestZipUnsupported` (near :1951). This one offers "declare the
  coincidence" on a pair already declared, after the stage label
  "declared-REST union zip:".

## Repair shape

Decide per name whether a declaration can settle it. Move each that
cannot into a lever of its own in `refusal_routes::lever`, with words in
`boolean::decision_words`.
`refusal_routes::tests::every_escalation_lever_renders_its_subject_and_its_recourse`
takes one row per lever. The three definite arms want the same
question asked of their recourse.

---
id: boolean-coincidence-route-holds-decisions-no-face-pair-names
kind: issue
title: topo: the Boolean's coincidence route still carries lever-arm, operand-radius and frontier decisions a face-pair declaration cannot name
status: open
opened: 2026-09-29
---

(TOPO, the §5 second pass of PR 3493.)

## What

PR 3493 carries a closed decision type on `BooleanError::Escalated`
(`boolean::BooleanDecision`, `crates/topo/src/boolean/refusal_routes.rs`),
set at the site that wraps each escalation. Following the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`,
every wrap site except the sector rung, the pierce normal and the
torus convention, face containment, where a crossing lands on an edge
and in what order, the split point on its circle, the arc span and the
volume backstop sets `BooleanDecision::Coincidence`, which keeps the
coincidence story and its declare lever. Some decisions those sites
wrap are ones a face-pair declaration cannot name:

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

D4 ¶1 (i): the decision is a closed type at its site; never route by
`diag.predicate`.

- Decide per decision whether a declaration can settle it. Each that
  cannot gets a `BooleanDecision` variant with its subject, and an
  ending in `BooleanDecision::ending` from what it passes on (a
  `geom_brep::recourse::SizedDecision`, or `Unsized` for a residual).
- Where one wrap site receives several decisions, the raiser carries
  which one, as `splitting::ConicRootFault` does for the conic root
  lane: the lever-arm gates in `geom_brep::enters_material` and
  `classify_dihedral` need a typed rung on their escalation, as do the
  `cylinder_sphere_section` operand guards (`SectionError`), and
  `solid_contain::line_wall_roots` / `line_torus_roots` need one on
  theirs, so `reduce`'s wrap sites can set the variant.
- `refusal_routes::tests::every_escalation_ends_as_its_decision_and_verdict_give`
  takes one row per variant through its `DECISIONS` list and `want`.
- The three definite arms want the same question asked of their
  recourse.

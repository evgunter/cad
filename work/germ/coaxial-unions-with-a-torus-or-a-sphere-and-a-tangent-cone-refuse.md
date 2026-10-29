---
id: coaxial-unions-with-a-torus-or-a-sphere-and-a-tangent-cone-refuse
kind: issue
title: A cylinder unioned with a coaxial torus or sphere, and a cone unioned with a sphere it is tangent to, refuse at f64, so glue's CylinderTorus and CylinderSphere rows are reached by no document
status: open
opened: 2026-10-29
priority: P2
cost: M
refs: [coaxiality-of-two-revolution-carriers-is-tested-five-ways, tangent-and-coaxial-scenes-are-in-the-corpus]
---

Found by INTENT stage 6 PR A (`work/intent/tangent-and-coaxial-scenes-are-in-the-corpus.md`),
writing the scenes stage 6 proves. Three unions of full revolves refuse
at `f64` at the default ε, each pinned by
`coincidence_door::the_curved_stage6_scenes_refuse_at_their_unions`
and standing beside the corpus registry for it
(`corpus::stage6_beside_the_registry`):

- **Cylinder × coaxial torus.** `corpus::shaft_and_bearing::rounded_mouth`:
  a ring revolved about its `y`, its bore's mouth a constructed profile
  fillet (a ring torus, major 5/8, minor 1/8), and a revolved shaft of
  radius 9/16, between the bore's and the mouth's, placed on the ring's
  `axis` output. The union refuses `CurvedBooleanUnsupported { operand:
  A, face, kind: Torus }`, with no escalation.
- **Cylinder × coaxial sphere.** `corpus::shaft_and_bearing::with_a_ball`:
  the same ring and a shaft of the bore's radius, and a revolved ball of
  radius 3/4 centred on the shaft's axis below the ring, turned so its
  poles lie across the axis and its seam circle square to it (no edge of
  the ball crosses the shaft's wall). The union refuses
  `CurvedBooleanUnsupported { operand: A, face, kind: Cylinder }`. With
  the ball's poles on the axis, its meridians crossing the wall, it
  refuses the same. The ring and shaft alone union green.
- **Cone tangent to a sphere along a circle.** `corpus::cone_on_sphere`:
  a sphere of radius 5/4 and a cone, apex `(0, −25/16)` and base the
  chord `y = −1`, `ρ ≤ 3/4`, whose wall is tangent to the sphere along
  its base rim, both revolved about one `y`. The union refuses
  `CurvedPierceUnsupported { operand: A, face, edge, band }` after
  in-band escalations on `bool_conic_quadric_sub_side` and
  `bool_conic_quadric_sub_clear`.

So the section value's `CylinderTorus` and `CylinderSphere`
classifications, which `boolean::glue::section_door` records as
`Coaxial` at `CoaxialCylinderTorus` and `OnCarrier` at `CoaxialSphere`,
are reached by no document: on 2026-10-29 the per-row door census
(`coincidence_door::census_the_door_per_row_over_the_corpus`) holds no
row at either site, and INTENT stage 6 PR B (the door proving a
coaxial row from one `Axis` read twice) has none to prove. The raising
sites are not measured here (`memories/refusal-text-is-not-cause.md`):
`CurvedBooleanUnsupported` is raised at a dozen sites in `boolean/`.

**Done when** each union evaluates, or refuses at a site that says why,
and the pins above are flipped: the two `shaft_and_bearing` scenes then
join the registry, and stage 6's census counts their coaxial rows.

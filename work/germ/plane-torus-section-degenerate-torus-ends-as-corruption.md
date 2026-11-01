---
id: plane-torus-section-degenerate-torus-ends-as-corruption
kind: issue
title: germ: the plane×torus section ends a definite non-ring torus as 'corrupt' and its in-band sibling as an escalation
status: closed
opened: 2026-09-30
closed: 2026-10-14
branch: germ/pt-degenerate-torus-ending
pr: 5173
priority: P3
cost: E
---


(TOPO, the §5 second pass of PR 3506: the other doors that read the
ring-torus convention, checked for D4 ¶1 (iv).)

## What

`geom_brep::intersect`'s plane×torus section (`crates/geom-brep/src/intersect.rs`,
the ring-convention guard near :1985) decides the convention's two
halves (`pt_tube_guard`, then `geom::ring_torus`):

- in band, it returns `SectionError::Escalated`, whose `Display` renders
  the whole `Indeterminate` ("the two surfaces' configuration is
  ill-conditioned at this tolerance: {diag}"), so it ends in
  `COINCIDENCE_RECOURSE`;
- decided `Zero | Negative`, it returns `SectionError::DegenerateTorus`,
  whose `Display` ends "a validated body has a ring torus, so this one
  is corrupt": a defect ending, no lever.

PR 3506 gave the convention one story at the pierce
(`geom_brep::TorusConvention`, `crates/geom-brep/src/torus_convention.rs`,
read by `BooleanError::DegenerateTorus`):
both arms name the half's lever ("make the tube radius clearly smaller
than the ring radius"), the band-decided arms with the tolerance the
margin gives; tier 3's `ValidationError::DegenerateTorus` reads the same
`SizedDecision`. The section's two arms tell two other stories, and
neither is the pierce's.

## Repair shape

Carry which half refused and its verdict (`geom::torus_tube`, which
keeps its reporting margin, in place of `pt_tube_guard`), as a
`geom_brep::TorusConvention` in this crate, and end both arms from its
`sized()` and `refused()` at the door that reads them (a build, or at rest through `join`'s frames and
`replace_face`), or say why the section reads only validated bodies and
end both arms as the defect.

## Measured (2026-10-14)

Whether a non-ring torus reaches the section, at `CAD_TOLERANCE_EPS`
unset, `1e-6` and `1e-12` (band `(ε, 10ε)`):

- **Build door.** `sweep::revolve` of a circle profile refuses every
  non-ring pose: `R − r` in band refuses as a near-axis sliver, decided
  zero as `RevolveError::UnsupportedToroid`; a tube `r ≤ 1.2·10ε` is
  refused by the profile validator (arc sweep, chord, full-circle
  questions). `R − r = 1.5·10ε` and `r ≥ 2·10ε` build rings.
- **Validated doors.** Tier 3 decides the ring half with
  `geom::ring_torus` on the same band (`topo::validate`, check 1's
  torus arm), `AtRestBody` carries the run's one `Tol`, a rigid map
  copies the radii verbatim (`topo::transform`), `step-import`
  validates at tier 3, and `geom_brep::offset_surface` decides both
  halves of the realized torus on the same margins before it mints. So
  the Boolean's `join` frames read no ring-half refusal; the tube half
  is the posture gap of
  `the-tube-and-radius-guards-decide-on-the-band-where-check-1-reads-lo`
  (no door found that builds one).
- **A door that does not validate.** `topo::replace_face_offset` takes
  `&mut Body` and plans every edge through `geom_brep::route_pose`. A
  revolved half torus at `R − r = 1.02·10ε`, re-charted through the
  public `set_face_surfaces_describing` to `R − r = 0.98·10ε` (outer
  equator held, so every edge still certifies), reached the guard at
  all three ε. Before the fix the door rendered "whether the offset
  face and its edges land where they should is undecided … Recourse:
  move the geometry, …", while tier 3 on the same body ended in the
  ring half's lever. A decided horn or spindle could not be re-charted
  onto (the edges' residual exceeds the band), so the decided arm is
  shown through the public `geom_brep::plane_torus_section` and
  `route_pose`.

## Resolution

The section carries the refusal: `SectionError::DegenerateTorus(TorusRefusal)`
(`crates/geom-brep/src/intersect.rs`, `plane_torus_section`'s guard,
`TorusRefusal::require`), where `geom_brep::TorusRefusal`
(`crates/geom-brep/src/torus_convention.rs`) is the half and its
verdict, `Undecided(Indeterminate)` or `Refused(Refused)`, and renders
both from `TorusConvention::sized()` / `refused()` at a build. The tube
half reads `geom::torus_tube` (`torus_tube_positive`) in place of
`pt_tube_guard`. `route_pose` returns the refusal instead of reading it
as an unserved pose; `topo::replace_face` and `offset_axial` carry it
as `ReplaceFaceError::DegenerateTorus`; `join`'s plane×torus frame maps
it to the pierce's own refusals (`BooleanDecision::Torus` in band,
`BooleanError::DegenerateTorus` decided); `chord_join` renders it
through `SplitJoinError::Section`. Rows:
`intersect_table::the_ring_guard_ends_each_half_in_its_lever`,
`torus_frame_rows::a_torus_off_its_ring_is_refused_as_the_pierce_refuses_it`,
`sweep` `a_torus_off_its_ring_reads_as_at_rest`.

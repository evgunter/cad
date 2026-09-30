---
id: section-arm-guards-escalate-untyped-and-certify-reads-the-dihedral-arm-as-transversality
kind: issue
title: geom-brep: the section arms' other operand guards escalate untyped, and certify and must_carry drop the dihedral's arm rung
status: open
opened: 2026-09-30
---


(TOPO, the §5 second pass of PR 3513, on `geom-brep` ground: no single
owner, since `crates/geom-brep/src/intersect.rs` is germ's, reach's and
tang's, and `certify.rs` is unowned.)

## What

PR 3513 gave two raisers typed rungs:

- `SectionError::RadiusEscalated { radius: SectionRadius, diag }` for
  `cylinder_sphere_section`'s `cs_cylinder_radius`/`cs_sphere_radius`
  and `cone_cylinder_section`'s `coc_cylinder_radius`;
- `geom_brep::LeverEscalation { rung: LeverRung, diag }` on
  `enters_material`, `enters_material_order2` and `classify_dihedral`.

The same class remains untyped elsewhere:

- The other section arms' operand guards still escalate as
  `SectionError::Escalated`, beside the pose trileans: the cone
  aperture guards (`coc_aperture_sin`, `coc_aperture_cos`, and
  `plane_cone_section`'s guard loop) and the plane × torus ring
  convention (`pt_tube_guard`, then `geom::ring_torus`), whose decision
  is `geom_brep::TorusConvention`. `route_pose` forwards them to
  `topo::replace_face`, which reads every escalation alike.
- `certify` reads both of `classify_dihedral`'s rungs as
  `CertCheck::Transversality` (`crates/geom-brep/src/certify.rs`, the
  `wedge_decided` match), so an in-band arm (a short edge, a tight
  radius of curvature) is told "the faces meet at too shallow an
  angle". `must_carry_over_edge`
  (`crates/geom-brep/src/dihedral.rs`) folds the rung into
  `MustCarryVerdict::InBand`, and the non-Boolean callers PR 3513
  touched mechanically drop it the same way:
  `topo::validate`'s rim screen, `topo::census`, `topo::splitting`'s
  rules, neighbourhood and finish, and `sweep::extrude` and
  `sweep::revolve::upgrade`.

## Repair shape

Give each guard its decision (`SectionRadius` for a radius,
`TorusConvention` for the ring convention, a new variant for the cone's
aperture), and have each door that wraps a `LeverEscalation` end the arm
rung in its length lever, as the Boolean's `BooleanDecision::LeverArm`
does.

## Since (PR 3513's fix pass)

`dihedral_arm`'s gate is `geom_core::k_stats::decide_positive_reported`
now: a decided-zero arm escalates with its decided margin rather than
`INVALID`. `certify`'s `Transversality` escalation reads that margin
(`wedge_decided` shares the gate), so a collapsed arm there quotes a
zero-band margin where it quoted an invalid one; it is still the arm's
question read as transversality, which is this row.

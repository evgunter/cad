---
id: circle-pair-radical-chord-has-no-shared-home
kind: issue
title: The circle-pair radical foot and half-chord have no shared home, so each site that fixes its cancellation mints its own copy
status: closed
opened: 2026-10-13
priority: P1
cost: M
branch: paths/circle-pair-shared-home
closed: 2026-10-14
---

Found by both reviews of #5032 (`arc-arc-secant-arm-misses-crossings-at-unequal-radii`).
No program owns `geom-core`'s ground for this, so it is filed here.

## The finding

Two circles' radical foot `a` and half-chord `h` have one
cancellation-free spelling: `r₁ − a = (r₁ + r₂ − d)(d − r₁ + r₂)/2d` and
`h² = (r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`, each
factor a clearance that the caller's own tangency decisions already
read. #5032 put it in `crates/profile/src/seg.rs` `Circles`, private to
profile, where it shares the margins with `arc_arc` and
`joint_tangency`. Five filed rows point other programs at the same
algebra:

- `work/restfront/circle-pair-edge-candidates-cancel-at-unequal-radii.md`
  (topo `validate.rs` `segments_meet`),
- `work/hone/circle-crossing-closed-forms-cancel-at-unequal-radii.md`
  (topo boolean `carrier_cross.rs`, `rim_wedge.rs`, `section_cert.rs`),
- `work/roots/sphere-sphere-section-radius-cancels-at-unequal-radii.md`
  (geom-brep `intersect.rs` `sphere_sphere_section`),
- `work/band/sheet-center-circle-pair-cancels-at-unequal-offsets.md`
  (sweep `blend/arms.rs` `sheet_center`),
- and the textbook form still sits in `topo/src/boolean/join.rs`
  `parallel_radical_plane`, where it is measured harmless.

Each of those fixed on its own mints a copy of the algebra, and the
soundness of each copy rests on spelling `r₁ + r₂` and `r₁ − r₂` the
same way as that site's margins, so a copy that drifts from its
decisions can poison at `Interval` where the decisions were definite.

## Where to look

A `geom_core` circle-pair helper, generic over `Real`: built from two
centres and radii (or from `d` and the radii), it forms `d`, `r₁ + r₂`
and `r₁ − r₂` once, hands out the identity, external and internal
margins, and the foot and half-chord from the same fields, 2D and on a
plane in 3D. profile's `Circles` would then be its first caller, and
the rows above would take it rather than each writing one.

## Outcome

Fixed on `paths/circle-pair-shared-home`. The home is
`geom_core::Circles` and `geom_core::Circles2`, in
`crates/geom-core/src/arc/pair.rs`, beside `Arc2`, the planar arc
carrier profile and geom-brep share. `Circles` is built from `d` and
the two radii, forms `r₁ + r₂` and `r₁ − r₂` once, and hands out the
identity, external and internal margins, `radical()` (the foot's
distance from `c₁`), `half_chord_squared()` and `half_chord()`; it is
plane-agnostic, so a caller in 3D places the points in its own frame.
`Circles2` adds the two centres and places the foot and the crossings
in the plane. Generic over `Real`; it decides nothing.

Readers: `profile/src/seg.rs` (`arc_arc`, `joint_tangency`), bit for
bit (400,000 pairs at `f64` and `Interval`, every margin, foot and
crossing coordinate, against the old private `Circles`: 0 differing),
and `topo/src/validate.rs` (`circle_pair`, arm 5), which closes
`circle-pair-edge-candidates-cancel-at-unequal-radii` (RESTFRONT P0).
The accuracy row `seg::circles_accuracy_tests::circle_pair_points_sit_within_a_few_ulps_of_scale`
stays in profile beside its double-double oracle (`dd.rs`) and now
measures the geom-core type. The HONE, ROOTS and BAND rows carry a
dated pointer to the home. The sweep found one more copy, on PATHS's
own ground: `work/profilerest/arc-fillet-corner-circle-pair-cancels-at-unequal-radii.md`.
`parallel_radical_plane` stays as #5032 measured it (harmless).


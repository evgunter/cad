---
id: a-rectangle-the-walk-refuses-has-no-witness-the-chart-lane-can-read
kind: issue
title: A rectangle the curved walk refuses cannot be handed to the chart lane by any route, so whether that lane meshes it is unmeasured
status: open
opened: 2026-11-16
---



## What

`tessellate::route_of` routes a ringless analytic curved face whose
stored images are iso forms by the curved lane's two doors
(`curved::iso_rectangle_shape`): a face both pass goes to the curved
lane's walk. Three walk refusals stand behind the doors, for faces they
pass that the walk still cannot take: `UnsupportedCurvedDomain` (the
zero-width slit, `curved::require_swept_rectangle`),
`MeridianFreeCurvedFace` (`walk::require_a_meridian`) and
`SingleColumnCurvedFace` (`walk::require_two_columns`). The chart lane
(`crate::trimmed`) reads the domain from the stored images and needs no
rectangle, so it might mesh such a face; the route never offers it one,
since both doors passed.

Whether it would is unmeasured. The witnesses are assembled through the
Euler doors (`mesh/tests/common/witness_bodies.rs`'s `slit`,
`one_circle_cut`; the single-column rows in `mesh/tests/errors.rs` and
`walk`'s units), and `topo::mint_pcurves` mints their images (it does on
the slit, the keyway, the lens, the half cap and the bow tie:
`witness_bodies::minted`). The slit with its images minted still
refuses `UnsupportedCurvedDomain`, because both doors pass it and the
route sends it to the walk. What is missing is a way to hand such a face
to the chart lane: a route override for a test, as
`tessellate::Routes::with` is for the curved lane's unit rows, or a
public one.

## What would settle it

Each walk refusal's witness, minted, handed to the trimmed lane by a
test-only route override, meshed at the reach probes' δ, with
`reach_probes::geometry` read on the result. If the chart lane meshes
them, the route could send a face the walk refuses there; that is a
routing change with its own measurement, not this row's.

---
id: per-chart-door-transports-a-torus-rim-the-section-would-mint
kind: issue
title: the per-chart door transports a plane×torus rim the C5 pose gate now serves, so an off-axis torus cap refuses at the re-anchor rather than at the pose
status: open
opened: 2026-10-06
---


## What

Found by `shell/axial-closed` (PR 4151). `geom_brep::plane_torus_section`
now serves the axis-parallel pose off the axis (its `SpiricOvals` arm),
so `geom_brep::route_pose` reports that pose served and the per-chart
door's neighbour gate (`replace_face.rs`, `neighbour_section`) no
longer refuses it as `NeighborPoseUnroutable`. The door then TRANSPORTS
the old rim carrier (`transport_curve`) rather than sectioning the moved
pair, so a torus rim beside an offset cap — a spiric — lands its moved
corner off the transported circle and refuses one step later:

`ReanchorOffCarrier { gap: 8.331e-4 }` on the klein elbow's cap offset
`-0.05` by `topo::replace_face_offset`
(`crates/sweep/tests/offd2_r1_probes.rs:probe_late_err_leaves_body_untouched`,
re-baselined in that PR; the body is still untouched).

Both refusals are typed and no wrong body is built. What was lost is
the refusal's precision: the old one named the plane × torus pose, the
new one a gap.

## Where it bites

Only operands the axial gate declines reach the per-chart door, and an
axial body (every torus elbow `shell` builds) takes the axial door,
which sections the pair. So the reach is a torus wall in a body that is
not axial — a torus beside an oblique plane — or a direct
`replace_face(s)_offset` call.

## Shape of a fix

Either the per-chart door's carrier for a curved-wall × plane rim comes
from the section, as the axial door's does (`offset_axial.rs`,
`mint_carrier`'s wall-and-cap arm), or its neighbour gate refuses, by
name, a pose whose section changes the carrier's kind.

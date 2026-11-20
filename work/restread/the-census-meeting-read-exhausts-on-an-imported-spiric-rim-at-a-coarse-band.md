---
id: the-census-meeting-read-exhausts-on-an-imported-spiric-rim-at-a-coarse-band
kind: issue
title: the census's edge-meeting read exhausts on an imported spiric rim at ε = 1e-6
status: open
opened: 2026-11-18
priority: P2
cost: H
---


Found by `importgate/build` (the build that makes every imported edge's
carrier by the kernel's own constructions).

## What

`crates/step-import/tests/spiric_roundtrip.rs`
(`a_spiric_rim_exports_as_a_spline_and_reimports_at_its_volume`)
builds a vessel cavity natively whose two plane × torus rims are
`Curve3::Spiric` carriers, exports it (the rims as B-splines), and
imports it again. The build now reads each rim back as its plane ×
torus section, a `Curve3::Spiric` (`geom_brep::plane_torus_section`,
through `topo::section_near`), with the native rim's carrier fields
(centre, axis, `u_ref`, radii, offset) and the same arc. At ε = 1e-9
and 1e-12 the imported body passes the aggregate gate. At ε = 1e-6 the
tier-3′ census refuses it:
`TierInvalid { errors: [CensusUnsupported { subject: Entity(Edge(..)),
cause: Meeting(MeetUnread(Unsettled(Exhausted))) }, ..] }` on the rims
and the edges they meet. Whether the native body, the same rims as
carriers, passes the same census at 1e-6 is not yet measured; before
the build the import carried the file's B-spline rims, and the census
read their meetings.

The parameter window is not the difference: the import first stored
the rim on `(5.39, 7.17)` where the native stores `(−0.89, 0.89)`, and
reading the spiric's start within half a turn of zero
(`step_import::geometry::endpoint_params`) leaves the refusal
unchanged. What differs between the two bodies at the rims is not yet
measured (the vertices are the build's roots, within rounding of the
native's).

## Where it stands

The test pins the 1e-6 cell as that refusal; the default and fine
bands measure the volume.

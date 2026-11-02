---
id: carrier-eq-decides-cylinder-coaxiality-beside-the-pose
kind: issue
title: The carrier-equality ladder decides two cylinders one carrier on its own margins (carrier_cyl_axis_parallel, carrier_cyl_axis_offset, carrier_cyl_radius) beside the pose's cc_axes_parallel, cc_coaxial and cc_radius_equality
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by the PR 6373 review (cylinder × cylinder stage 3). The stage's
§5 pattern matched `Surface::Cylinder` pairs and could not see the
ladder's `CarrierDesc::Cylinder`.

## What

`topo::boolean::carrier_eq` (CONTACT-DESIGN C4's `Rest` table) answers
"same carrier?" for two cylinder walls by its own data rungs
(`cylinder_data`, `data_rungs`), each read at the consumed extent
(`at_consumed_extent`):
- `carrier_cyl_axis_parallel`, levered by the arm;
- `carrier_cyl_axis_offset`;
- `carrier_cyl_radius`.

The section value's pose decides the same fact on its own rows: two
cylinders are one surface where `cc_axes_parallel`, `cc_radius_equality`
and `cc_coaxial` all decide Zero (`SectionError::CoincidentSurfaces`).
The two readings can disagree inside the band (`[one-way.readings]`).

The torus arm already reads the pair's own coincidence
(`geom_brep::tori_coincide`); the cylinder arm does not.

## Why it was not fixed in stage 3

The ladder's answer carries more than the pose does:
- the decided margin it hands the glue record (`CarrierReading`);
- the orientation bit (`outward`), for its `Same±` verdicts;
- per-datum readings at a corner site.

The pose's coincidence refusal carries no margin. Re-hosting the arm on
the pose needs a margin-bearing coincidence reading on the pose, as
`tori_coincide` gives the torus.

## The shape of a fix

Give the cylinder pose a coincidence reading that returns its decided
margins: parallelism, radius equality and the coaxial distance. Then
read the ladder's cylinder arm from it, as the torus arm reads
`tori_coincide`.

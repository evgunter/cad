---
id: the-carried-lane-chords-only-spline-components
kind: issue
title: The join's carried lane chords only spline components, so a counted closed-form component (parallel rulings, the bisector arcs, coaxial circles) rides a per-pair plane lane beside it (stage 3b)
status: open
opened: 2026-11-02
priority: P1
cost: H
refs: [skew-cylinder-germ-pair-has-no-section-frame]
---


Filed by the cylinder × cylinder stage 3 lane (branch
`sectarms/cylinder-join-reads-value`, 2026-11-02), per the orchestrator's
F3 ruling: the parallel-rulings lane stays until this extension lands.

## What

`chord_join::carried_chord_spec` cuts a carried chord from its
component's carrier as a NURBS net. Two things refuse a closed form:
- "a carried component's carrier is not a spline";
- `SplineArc::of_section`, which needs the component's C2 certificate.

So only a marched, fitted component rides `GermLane::Carried`. The join
now reads every cylinder pose on the section value (`join::cylinder_read`).
Two of them still ride per-pair plane lanes, with each plane taken from
the value:
- **the parallel pose's rulings** (`ClassCarrier::Span` lines cut by
  the region) ride `GermLane::Rulings`, in the plane from
  `CylinderCylinderPose::rulings_plane`;
- **the bisector pose's four arcs** (`ClassCarrier::Span` ellipse arcs)
  ride `GermLane::Bisector`, in the planes from
  `CylinderCylinderPose::bisector_planes`, split by `LoopSplit::Bisector`.

The same holds for cylinder × sphere's coaxial circles
(`ClassCarrier::Closed`), which the frame lane reads.

## What the extension has to supply

A carried chord on a closed-form component:
- the arc of the closed form between two sites, cut exactly;
- its evidence of lying on both surfaces (an exact carrier needs no
  certificate);
- a site at a node read as the node (C5), with the germ's arc named by
  its branch at mint (`vtxfac::mint_node`'s plane branch), not by a
  positional foot.

After that, the rulings and bisector lanes, and the plane-lane rows that
pin them, retire into the carried lane.

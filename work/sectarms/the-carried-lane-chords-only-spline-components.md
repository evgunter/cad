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

## 2026-11-02: split into 3b-i (landing) and 3b-ii (waits on F1)

**3b-i**, branch `sectarms/cylinder-carried-closed-form`:
- `chord_join::carried_chord_spec` gains a closed-form arm. The chord on
  a line, circle or ellipse component is the carrier itself, over the
  two sites' parameters, run back by `Curve3::reversed` where it leaves
  back. It has no spline arc, since an exact carrier is its own
  evidence, and the ring lane reads its arc as the curve it is.
- `PairSection::seat` seats a closed circle or ellipse by its span.
- The join admits a closed form to the carried lane on any pair with no
  torus (`join::chord_spec_carries`). A torus pair's closed forms are
  filed apart: `a-torus-pairs-closed-form-circles-take-no-join-lane`.
- The parallel pose's rulings lane retires: `GermLane::Rulings`,
  `KindArm::Cylinders`, `rulings_are_straight`, `rulings_normal` and
  `CylinderCylinderPose::rulings_plane`.
- Cylinder × sphere's coaxial circles now build every op. Before, they
  refused at the join (`CurvedBooleanUnsupported`); the correction to
  the paragraph above is that the frame lane never read them.
- The probe the plan named reads: a cylinder island closed by a ruling
  winds by a path (`chord_join::wall_ring_rows`), so no new reading was
  needed.

**3b-ii**, the bisector lane's retirement, waits on the node-germ fork
F1. That fork decides how a germ at a node names its arc once the
bisector planes no longer do. The orchestrator is running it through the
design-fork procedure. Until then the bisector pose keeps
`GermLane::Bisector` exactly as before.

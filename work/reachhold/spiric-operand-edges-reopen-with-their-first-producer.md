---
id: spiric-operand-edges-reopen-with-their-first-producer
kind: unit
title: spiric operand edges re-open with their first producer, through the section reduction to the conic x torus lanes
status: deferred
opened: 2026-10-03
priority: P3
cost: M
refs: [3984]
---


Deferred by the REACH orchestrator on the NURBS/spiric operand fork,
adopting both designers' reconciled reports
(`analysis/design-fork/nurbs-spiric-operands-d1` and `-d2`,
`design.md`, round 1): build nothing for the spiric now. This is the
orchestrator's ruling on agent-written design reports, not an Ev
ratification; no ratified clause decides it either way.

## Why not now

No public door produces a spiric-bearing operand that a boolean can
consume. The shelled partial revolve, the one producer
(`offset_axial::mint_carrier`), stops at tier 3's props door
(`VolumeUncomputable`: a spiric-bounded cap's area is an elliptic
integral; teapot wall 1). Its `offset_charts_together` cavity does
reach the boolean, which refuses it at the operand gate today
(`sweep/tests/spiric_rim.rs` row 11); with the gate deleted it refuses
on its torus face first (measured on PR 3984's backed-out attempt:
`CurvedPierceUnsupported`, an elbow line edge against the cavity's
torus wall, and `Containment` against bricks far and around).

## What re-opens it

The first producer whose spiric-rimmed body reaches a lane that reads
the edge, once `delete-the-boolean-operand-edge-gate` lands: the sweep's
arms refuse it `CrossingCarrierUnsupported` today. The lane
to build then is the section reduction: a spiric edge on
`O = T ∩ Π` meets a face carrier `S` exactly where the planar section
`Π ∩ S` (a C5 line, circle or ellipse) meets the torus `T`, so
`line_torus_roots` and `circle_torus_roots` answer plane and sphere
faces with no new root lane, the minor angle of each root is the edge
parameter exactly, and cylinder, cone and torus faces refuse typed
until their sections have torus lanes.

## Evidence: a minted spiric edge whose near touch nothing reads (2026-11-04)

Added by SPLICE's `splice/spiric-on-cap-plane` lane. A spiric edge now
reaches the boolean without a spiric operand: the join mints one as a
section chord (torus wall ∩ a plane cap). The tilted rod's β 0.7 union
in `crates/sweep/tests/germ_tilted_rod.rs` mints one, and the slit
pass's rider reading asks it against the rod's cylinder wall.

`slit::spiric_on_quadric` reads such a spiric `Off` a sphere, cylinder
or cone by degree. It does so only where a point of the span's interior
reads definitely off. A span in band at every sample stays unread. A
shorter stretch of the span within the band of the wall, between points
read off, is a near touch. No reading takes that: `curve_face::route`'s
`(Spiric, Plane | Cylinder | Cone | Sphere | Torus)` is
`Door::Spiric`, unbuilt.

The section reduction above is that reading. A spiric's touch with `S`
is a touch of `Π ∩ S` with `T`. For a sphere face that is
`circle_torus_roots`, and the cylinder and cone faces wait for their
sections' torus lanes as this row says. The minted chord is the
producer this row waits on, for the riders' reading at least; the
pierce lanes are another matter.

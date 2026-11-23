---
id: carrier-frames-the-production-mints
kind: unit
title: Carrier frames (ii): every production construction site mints its carrier's frame at the lane scalar, per crate
status: parked
opened: 2026-11-17
priority: P1
cost: H
parent: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
blocked_on: [carrier-frames-the-types-and-the-lift]
---


Rides on `carrier-frames-the-types-and-the-lift`. Every production
construction site mints its carrier's frame at its lane scalar, with a
typed refusal plumbed through where the mint can refuse. The sites are
`step-import` (file data goes through a decided mint), `sweep`, the
`topo` operations and offsets, and `geom-brep`. It is to be fanned
out one lane per crate group.

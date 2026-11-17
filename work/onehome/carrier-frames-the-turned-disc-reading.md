---
id: carrier-frames-the-turned-disc-reading
kind: unit
title: Carrier frames (iv): read check 1 on the turned disc (30° ± 1°) before and after, and pin it
status: parked
opened: 2026-11-17
priority: P1
cost: M
parent: carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes
blocked_on: [carrier-frames-check-1s-frame-half-goes]
---


Closes `check-1-reads-a-turning-frames-length-off-its-bracket` with
the parent. It reads check 1 on that row's turned-disc document
(radius 5e-3, one copy spun about world y by 30° ± 1°, united with a
clear copy) before and after
`carrier-frames-check-1s-frame-half-goes`. It pins that no
`SurfaceDatumUndecided` or `CurveDatumUndecided` finding with
`measure: Length` or `Tilt` remains. It records what then refuses
instead, for the parent row's "the turning-frame row's case builds".

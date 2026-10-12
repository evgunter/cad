---
id: limb3-ladder-stops-on-a-certified-second-arc
kind: issue
title: limb 3's ladder probes every rung after a certified second arc within the narrowest rung's reach; such an arc could end the ladder
status: open
opened: 2026-10-04
priority: P3
cost: M
---


## Found (PR 4012, Ev's question on the miss cost; filed by the limb-3 lane, 2026-10-04)

`limb_three` (`crates/geom-brep/src/ssi/certify.rs` ~1136) probes every
rung of the tube ladder, widest first, until one is a graph proved one
arc. A rung whose walk certifies a second arc (`Shortfall::Count(n)`,
n ≠ 2) hands the ladder on to a narrower rung. When that second arc
lies within the narrowest rung's reach of the carrier, no narrower
rung can exclude it, and the rest of the ladder is wasted work. The
at-rest fold rows (`ssi_limb3_one_arc.rs`) spend about 20 s per call
in debug that way.

## Repair shape

End the ladder at a certified count other than two when the box that
holds it lies within the narrowest rung's radius of the carrier. That
box's second arc is in every narrower tube, so the refusal is the same
one, reached earlier.

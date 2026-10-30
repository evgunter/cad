---
id: cone-on-surface-meter-folds-its-gap-into-the-meter
kind: issue
title: conic_quadric: the cone's on-carrier reach reading folds its gap reading into Unsettled::Meter, a limit
status: open
opened: 2026-10-29
priority: P3
cost: E
---

Found by the review of PR 5993 (`splitdoor/roots-unsettled-split`).

`crates/topo/src/boolean/conic_quadric/mod.rs:430`: a conic whose
cone-residual quadratic reads `Roots::On` stands on the cone only once
its reach through the floor decides zero (`CONE_ON_SURFACE`). A definite
nonzero reach is the meter's limit (`Unsettled::Meter`), but the arm is
`Ok(Sign::Positive | Sign::Negative) | Err(_) => Err(Unsettled::Meter)`:
a reach in the band's gap, which is the geometry's ambiguity and owes an
escalation (`RootRefusal::Touch`, or `Escalated` under the door's
`ArcConeRoots`), leaves as the limit instead, and the pierce keeps the
`CurvedPierceUnsupported` frontier where it owes `CurvedPierceEscalated`.

The repair splits `Err(diag)` off to the band's ending and a row that
puts a conic's reach in the gap.

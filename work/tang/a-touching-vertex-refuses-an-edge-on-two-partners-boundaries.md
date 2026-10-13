---
id: a-touching-vertex-refuses-an-edge-on-two-partners-boundaries
kind: issue
title: A touching vertex refuses an edge on two partners' boundaries that the partner-faces reading decides
status: open
opened: 2026-10-12
priority: P3
cost: M
refs: [pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided]
---

Filed by the sweep of `pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided`.

## What

`crates/topo/src/boolean/vtxfac.rs` `touch_classes` reads its own
side's partners with `layered`, whose `None` for an edge on two
partners' boundaries refuses `BooleanError::VertexReadTwice`
(`SectorRead::Pierce`, `SectorRead::Pair`). That error's Display ends
"There is no way through this in the kernel yet". The unit test row
"on two arches' shared ray" in `vtxfac::tests` pins the refusal.

`pair_classes` now reads such an edge with `vtxfac::along_partners`
(`sectors::half_planes`): `On` where a half-plane one partner alone
leaves the edge in is decidedly apart from every other, and an
`Escalated` `BooleanDecision::PartnerFaces` otherwise. The same reading
applies to a touch's partners on one side of the pierced face: they are
the other solid's material there beside the face's half-space.

## The shape to give

Route `touch_classes`' two-`On` edge through `along_partners`, keeping
the refusal beside a partner that reads nothing or is not `pointed`.
No suite row is known to reach the refusal. Add a public witness, or
show that none exists.

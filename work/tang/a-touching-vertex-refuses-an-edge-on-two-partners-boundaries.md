---
id: a-touching-vertex-refuses-an-edge-on-two-partners-boundaries
kind: issue
title: A touching vertex refuses an edge on two partners' boundaries that the partner-faces reading decides
status: closed
opened: 2026-10-12
priority: P3
cost: M
refs: [pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided]
closed: 2026-10-15
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

## Closed

`touch_classes` takes the touching vertex's own sectors and the band,
and reads an edge on two or more of its side's partners' boundaries
with `along_partners`, as `pair_classes` does: `On` where a half-plane
one partner alone leaves the edge in is decidedly apart from every
other's, an `Escalated` `PartnerFaces` otherwise. The refusal beside a
partner on the edge's side that reads nothing or is not `pointed`
stays, ahead of the faces' reading. `vtxfac::TouchUndecided` carries
either.

**Public witness** (`a_vertex_read_by_two_sector_passes`,
`a_touching_vertex_along_two_arches_shared_ray_builds_in_every_op`,
slow set, and its per-PR pose `…_builds_at_rest`):
the plate united with two pyramids standing on `MEET` that share an
edge ray, against a pyramid standing there with an edge along that ray,
outside both arches, in the wedge between them, or with a face in each.
On main all 18 cells (3 pyramids, every op, both slots) refused
`VertexReadTwice(Pierce, Pair)` from `touch_classes`. Now 16 build,
sound against tier 3, tier 3′ as the file reads it, the naming's
classes, the material at the probes and inclusion–exclusion, at every
pose. The other 2, the pyramid less the arches where the pyramid's
faces lie in them, refuse `Revert(WedgesOverlap)`: reverting the arches
refuses, their coincident edges' wedges along the ray overlapping. They
build once revert re-pairs the sheets on that line, which re-baselines
them (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`,
which cites this witness).
The arches alone, without the plate, are classed by `pair_classes` and
answer the same 18 cells identically on main and here.

The `Escalated` arm and the kept refusal are pinned by the unit row
`vtxfac::tests::a_touching_edge_on_two_partners_boundaries_reads_their_faces`;
no public pose is known to reach either beside a touch.

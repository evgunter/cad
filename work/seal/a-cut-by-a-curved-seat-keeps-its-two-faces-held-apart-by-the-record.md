---
id: a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record
kind: issue
title: A cut whose tool's seat is curved, crossed by faces alone, keeps the seat's two faces inside one shell held apart by the record: a silent wrong body
status: open
opened: 2026-11-09
priority: P1
cost: M
refs: [a-declared-wall-is-read-only-by-a-vertex-crossing-it, a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections]
---


Filed by SEAL's lane on PR 6613's fix pass, from R2's review (by
inspection, not executed: the lane did not build a coaxial fixture).

## What

On main, `cap − seat` (the bolt on the plate, its seat recorded, cut
from a cap that covers the whole of the bolt's foot) built a silently
wrong body. It was one solid of 11 faces, volume 0.168. Its floor was
the bolt's foot reverted and lying on the plate's top reverted, inside
one shell, held apart by the carried record, so tier 3 and 3′ passed
it. PR 6613 refuses that shape at the result gate
(`ops::Held::apart`): a shell that holds both faces of a carried area
record refuses `ToolSeatCrossed` in a cut by the record's holder, and
`TouchesItself` otherwise.

The gate reads only a record of two **planar** faces
(`crate::contact::area_record`), because a curved record may be a
tangency along a line, which tier 3′ reads inside one shell (`sweep`'s
`face_face_touch`, a rod resting on a plate, bridged). So where the
tool's seat is curved, the same body is still built:

- a shaft in a bore, the seat the shaft's wall and the bore's;
- cut from a block covering the whole seat, or one that crosses the seat
  by faces alone, as the cap does;
- with no vertex of the cut part on the seat, so the vertex pass reads no
  wall;
- the result keeps the bore's wall and the shaft's wall, both reverted,
  on each other inside one shell.

## Done when

A coaxial fixture (a shaft in a bore as a `⊔`, its seat recorded) cut
from a block that crosses the seat by faces alone either builds the
op's body or refuses typed. The gate must tell a curved area record
from a tangency without re-deciding the carriers (the carry's lineage,
or the record's own backing), and a row must pin the outcome.

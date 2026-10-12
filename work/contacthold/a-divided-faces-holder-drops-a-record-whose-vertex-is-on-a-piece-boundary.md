---
id: a-divided-faces-holder-drops-a-record-whose-vertex-is-on-a-piece-boundary
kind: issue
title: The boolean's divided-face holder drops a vertex-face record whose vertex lies on a piece's boundary
status: closed
opened: 2026-10-12
priority: P2
cost: E
refs: [a-cut-through-a-recorded-touch-mints-a-pair-no-record-backs]
closed: 2026-10-12
pr: 4820
branch: contacthold/one-record-carry-boolean
---


## What

`Descendants::holder` (`crates/topo/src/boolean/ops.rs`) places a carried
`(vertex, face)` record among a divided face's pieces with `piece_holds`,
and returns `None` — the record leaves the carry — where no live piece
holds the vertex, including where the vertex lies on a piece's boundary
(the chord the boolean divided the face along). A resting vertex on that
chord then loses its record, and the result's census would refuse the
touch undeclared. A drop on absence, the class D1's "never inferred"
and the census's "never scans to bless" both forbid.

Found by a designer of the record-carry fork (`[ev]` PR 4749), read from
the code, not measured: build a fixture (a cube resting a corner on a
face the boolean then divides through that corner) before fixing. If
PR 4749 is ratified, this retires with `holder` into the one carry; the
fixture is owed either way.

## Resolved (PR 4820)

`holder` retired into the one carry: a divided face reads as its pieces
and the chords between them, so a vertex on the chord lands on the chord
(`ops::found_on_a_divided_face::a_rest_on_the_dividing_chord_lands_on_the_chord`,
red on main, where it dropped). The integration fixture the row asked
for is not reachable today: each pose of a corner resting on a face a
boolean divides through that corner refused first `VertexReadTwice`
(the cutter's edge or corner on the resting corner).

---
id: an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane
kind: issue
title: The census refuses an edge-face record off a line and a plane typed, away from a conventional vertex: no curve-touch lane reads an edge's carrier against a face
status: open
opened: 2026-10-10
---


Found by CONTACTHOLD building `at-rest-records-are-unordered-cell-pairs`
(PR 2 of its stack).

`census::confirm_edge_face` (`crates/topo/src/census.rs`) confirms an
edge–face record on a line and a plane by reconstruction, and one off
them only at a conventional vertex of its edge lying in the face (the
case the boolean's carry writes when a vertex-on-face record's vertex
is a closed edge's conventional vertex). Anywhere else, such as a curved
edge whose interior rests on a face at a point the topology does not
hold, or a circle lying in a plane, it refuses
`CensusUnsupported(ContactLane(NotCertifiable))`, so a boolean carrying
such a rest refuses at its result gate where it used to refuse
`CurvedRestUnrecorded` at its carry. C3 says the locus of an edge–face
curve touch is the edge's carrier. What that lane certifies for an edge,
which has no normal of its own to oppose the face's, is a design
question: coincidence of the carrier with the face's surface over the
schedule, region confinement, and a point touch's isolation. The census
never clears on this record (it backs only the conventional vertex's
touch), so the refusal is the sound side.

## 2026-10-11 — the producer waits on this lane too (FLUSH)

The boolean now records a LINE edge's span lying in a plane face or
along a line edge (`boolean::ops::record_spans`, FLUSH's
`an-apex-edge-…`). It mints nothing for a curved edge lying in a face
(a circle in a plane) or for a line in a curved face (FUSE's parked
`a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`).
Minting either today would turn a correct body into this row's typed
refusal at the result gate. When the lane lands, the same function is
where the op records them.

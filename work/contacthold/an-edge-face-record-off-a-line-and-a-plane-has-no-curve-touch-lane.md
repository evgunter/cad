---
id: an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane
kind: issue
title: The census refuses an edge-face curve touch typed, away from a conventional vertex: no lane reads an edge's carrier lying in a face (a circle in a plane, a line in a curved face)
status: parked
opened: 2026-10-10
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
cost: M
priority: P2
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

## 2026-10-12 — a carried edge–face record now refuses the next op (CONTACTHOLD)

`contacthold/face-face-touch-row` gives an operand's edge–face and
face–face records a carried kind (`OperandRecords::ef`, `::ff`), so a
record that `ContactRecords::carried` used to drop now goes through the
one carry, whose landing is this census's confirm
(`records::landings`, `census::RecordConfirm::meets`). Where this lane
is missing, the carry refuses `CarryRefused { Unread }` instead of
dropping the record.

Measured: a rod (r 0.5, vertices off the ruling) resting on a plate's
side face unions into two solids whose records hold the face–face pair
and two edge–face pairs, the rod's cap circles each touching the plate
face at one point of the arc's interior with no vertex there. 3′ of
that union refuses `CensusUnsupported` on both cap edges (this row's
message), and a second union bridging the rod to the plate, carrying
every record, refuses `CarryRefused { Unread }` on the first. Carrying
the face–face record alone, the bridge lands it and passes 3′
(`sweep` `face_face_touch::a_rod_on_a_plate_records_its_touch_and_a_bridge_lands_it`).

That is a point touch of a curved edge, not a curve touch: the lane this
row owes has to say what it certifies for both, and the CONTACTHOLD
brief that found it asked for the curve-touch entry at P1, since the
face–face record's own ends are such points.

## 2026-10-13 — the point touch reads; the curve touch is what is left (CONTACTHOLD)

`contacthold/point-touch-reading` builds C3's point-touch reading
(`census::point`): an edge–face record of a circle on a plane or a line
on a sphere is confirmed at the carriers' touch point (κ_rel definitely
positive, coincidence within ε, the point strictly inside both cells),
read stale apart or crossing. This row's measured case, the rod's cap
circles on the plate face, now reads: `plate ∪ rod`'s 3′ refuses
nothing on the cap edges, and the bridge carrying every record answers
with 3′ Ok. Narrowed to the curve touch: a circle lying in a plane away
from a conventional vertex still refuses typed (the message now says
"a circle lying in a plane"), and a line in a curved face, and the
other carrier pairs (a circle on a sphere or a cylinder, a line on a
cylinder) refuse naming the point lane's coverage.

## 2026-10-17 — parked on the D10 hold, moved from RESTREAD

RESTREAD read this row against the D10 hold (`docs/DESIGN.md`, State of
the build) and the open [ev] PR 4666 (the census records what it
decides): it stands on contact records, declared pairs or the shape of
at-rest findings, which INTENT stages 4 and 5 rebuild. Per RESTREAD's
plan (a row needing declared pairs or contact parks and moves to
CONTACTHOLD), it moved here by `git mv` with its id unchanged. Priced
`M` where it carried no cost. (RESTREAD orchestrator)

## 2026-10-25 — banded P2 (band rule)

`a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`
(P2) is parked on this row, and the line-in-a-curved-face case its title
names is the lane that row's producer waits for. A blocker inherits the
band of what waits on it (`work/README.md`, Priority), so this row,
unbanded until now, carries P2. Its own blocker, the D10 hold, is P0.
(band-rule triage)

**Re-read 2026-10-29 (INTENT, D10 hold):** kept on the hold, pending a decision. Its D10 ground is built (stages 4 and 5; `census::confirm_edge_face`, `crates/topo/src/census.rs:9608`, still refuses the curve touch typed, `census/point.rs:301`). What is left is the question this row names itself: what the lane certifies for an edge, which has no normal of its own. Opening it means flagging that `design` question or ruling on it, and that is the INTENT orchestrator's call, not this re-read's.

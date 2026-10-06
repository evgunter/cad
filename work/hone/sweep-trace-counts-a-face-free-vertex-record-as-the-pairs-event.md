---
id: sweep-trace-counts-a-face-free-vertex-record-as-the-pairs-event
kind: issue
title: The sweep trace counts a face-free vertex record as the (edge, face) pair's event, so the pruning superset check flags pairs whose boxes never meet
status: open
opened: 2026-10-01
---

Found by TANG's circle × cylinder crossing cell (branch
`tang/circle-cylinder-crossing`), on the one corpus pair of
`crates/sweep/tests/n3r1_prune.rs` the cell let through the sweep.

## The finding

`sweep_direction` (`crates/topo/src/boolean/reduce.rs`) pushes
`(edge, face)` onto `SweepTrace::accepted` whenever `curved_face_arm`
returns anything but `CurvedEvent::None`. One way an arm returns
`Recorded` is face-free: `vertex_on_curved_face`, finding the endpoint
definitely outside this face's trim, still looks the point up among
ALL of `y`'s vertices and records a v-v contact on a hit — by its own
comment "the SAME record the holding face's pair produces (the
accumulator dedups)". So the trace attributes that record to every
face on the carrier, including faces the edge's box never meets.

The realized sweep prunes those pairs (their boxes are disjoint, with a
gap of order the face's own size) and loses no contact, because the
record is a duplicate. But the pruning comparators read `accepted` as
"events box pruning must not lose", so they flag them.

## Measured

`three_arc_cylinder` at the origin against the same shifted 0.3 in `x`
(`n3r1_prune`'s "cylinder x cylinder shifted 0.3"). Each rim arc
pierces the partner wall at a rim point (`OnEdge`, which splits the
partner's rim and records the v-v). The pierced arc's two fragments
then each take `Recorded` against all three partner wall faces in the
idealized walk — the two non-holding thirds through the face-free
lookup. The realized walk examines only the holding faces, so 16
idealized-accepted pairs (4 in A → B, 12 in B → A) are unexamined. With
the pad raised to 10 the realized trace equals the idealized one
exactly. With the pad at 0.05 the A → B loss falls from 4 to 2, which is
the gap between the fragment's box and the wall third's box.

The row pins that 16 as a named exemption (`FACE_FREE_RECORDS`), so
this item has a red-able witness.

## What would close it

Make the trace's `accepted` mean a FACE-BOUND event: a pierce, a split,
or a record this face's own trim placed. That is either a separate
`Placement` (and `CurvedEvent`) for the face-free hit, or the face-free
lookup moved out of the per-face arm into one per-vertex pass. Then
drop the exemption. Whichever is chosen, `Placement::declared` and
`undeclared_no_interior` decide on `Recorded` today and must keep
deciding the same way.

---
id: the-parting-re-decides-an-edge-lying-in-a-face-the-sweep-decided
kind: issue
title: The parting re-decides that an edge lies in a face, which the reduction sweep decided and recorded no row of, so its ArcOnFace row is a second decision
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Filed by `heldrecords/parting-division` from the design note on
`the-partings-curved-face-gate-refuses-a-division-its-reason-does-not-cover`
(both designers: one final state reads one home). Measured by reading
on `heldrecords/split-on` at `80f11deb86`.

## What

The reduction sweep decides an arc lying on a curved face's carrier
(`boolean/reduce.rs` `lying_on`, reached from `curved_face_arm` where
the span reads `SpanVerdict::LiesOn`) and keeps of that decision only
its ends: each end's vertex-on-face record, or a split where the arc
crosses the face's boundary, and a `LyingIn::Curved(face, on)` entry
in the sweep's local worklist (`reduce.rs`, the `CurvedEvent::Lies`
arm), which no row outlives. No coincidence row says the arc lies on
the face.

After the join, the parting decides it again on the result
(`boolean/slit.rs` `riders` → `holding` → `carrier_on_surface`: the
conic's samples against the surface's variety, `Variety::read_piece_on`,
and on a torus `torus_between`), and mints its own row of it,
`DecisionSite::ArcOnFace` (`slit.rs` `arc_on_face`), which the arc's
edge–edge record cites. Two decisions of one fact, at two sites with
two margins, and nothing ties the second to the first.

## Why it is not a change inside the parting

Citing the sweep's decision needs three things the parting does not
have:

1. the sweep to mint a row of the arc on the face, with its margin
   (today `lying_on` returns `CurvedEvent::Lies` with no margin of the
   interior; `boundary_meets_circle_only_at` and `arc_chain_reaches`
   answer booleans);
2. that row to name operand cells the join's lineage maps to the
   result's (the arc is split at crossings in the sweep, and the join
   re-keys both);
3. the parting to find the row by its result cells.

(1) and (2) are the sweep's and the lineage's ground, a design of
their own: whether the sweep's row is the census's edge-face curve
touch (`work/censusread/an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane.md`,
which a curved edge-face record waits on) or a coincidence row only.

## Done when

The parting cites the sweep's row of an arc lying in a face, plane or
curved, and `arc_on_face` and the parting's own `carrier_on_surface`
reading of a recorded arc retire.

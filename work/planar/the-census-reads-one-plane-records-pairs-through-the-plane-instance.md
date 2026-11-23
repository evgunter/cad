---
id: the-census-reads-one-plane-records-pairs-through-the-plane-instance
kind: unit
title: the census reads two edges of one plane record through the plane instance, not meet
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [two-curves-in-one-plane-meet-by-the-in-plane-arms]
---


PLANAR's fourth unit; runs beside units 3 and 5 once the instance lands.

## The ruled text

C5 (`crates/geom-brep/README.md`): *"Two curves on one surface record are
not a curve pair to this table"*, and among the co-surface arm's readers
*"the census's pairs on one plane record ... and none hands the pair to
`meet`"* (PR 7290; PR 6567 for the rule).

## The defect (read, not run)

Tier 3′'s pass 5′ (`crates/topo/src/census.rs` `sweep_curved_edge_edge`)
reads each curved edge against the line edges and later curved edges
whose padded boxes meet its own, through `census::curved::interiors_meeting`
→ `interior_meeting` → `geom_brep::curve_pair::meet`, with no filter on
a shared face: two edges of one planar face go to `meet` against 6567 as
written. Pass 5 (`sweep_edge_edge` → `pair_edge_edge` → `ee_meeting`)
reads two line edges through its own reader, beside the table's
`LineLine` arm, for every pair including same-record ones.

## What changes

- In pass 5′, and in the curved confirm lanes that share
  `interior_meeting`, a pair whose two edges lie on one plane record (a
  face side of each on the same `Plane` surface key) is read through the
  instance, its spans' ends the edges' vertex positions; every other
  pair still reads `meet`.
- In pass 5, a same-record line pair is read through the instance's
  line × line arm. Pass 5's own reader for the remaining pairs is
  ONECOPY's `census-pass-5-reads-two-lines-through-its-own-reader-beside-the-tables-line-line-arm`,
  filed with this cut.

## Done when

- Tier 3′ is green on its suites at the default ε row, and every
  verdict that moves is listed with its reason (an expected class: a
  same-face pair that escalated on `meet`'s tilt or slant rows now
  decides).
- A row: a planar face with an arc edge and a line edge whose carriers
  graze in the band off both spans (one of the five fixtures of
  `a-pose-proves-two-spans-apart-from-their-covers`, as a face) at
  `Interval` on a plane tilted and placed off the origin, records no
  `UnrecordedContact` and no escalation.
- A row: two edges on two different plane records that are coplanar
  (the shape `work/export/an-imported-union-ships-coplanar-same-record-neighbours-no-boolean-takes.md`
  names) still reads `meet`, so the type's key comparison is what
  decides the route.

## First check, and what to do if it fails

**What "on one plane record" is for an edge.** An edge has two face
sides; two edges are on one record when a side of each is a face on the
same `Plane` surface key. Read `census.rs`' candidate sets (`cands.curved`,
`geo.edges` and their `f_plus`/`f_minus`) to see that the keys are in
hand at the pair; and whether an edge seated on two plane records (a
crease between two planar faces) is read once per record or once. If
the pair cannot name its record without a new reading, stop and report.

## Leaves for later

Pass 5's own reader for cross-record pairs (ONECOPY's row above).
`a-touchs-crossing-or-side-is-re-decided-from-tangents-at-its-one-point`'s
census readers (`confine`, `shared_interior_witness`), ONECOPY's.

## Rows

Changes ONECOPY's touch-kind row (its census readers now see the
instance's meetings for same-record pairs; the lane notes it there).

## Territory

`crates/topo/src/census.rs`: ATREST, CONTACTHOLD, HELDRECORDS, ONEDOOR;
`crates/topo/src/census/curved.rs`: ATREST, HELDRECORDS. Announce on
ATREST's and HELDRECORDS' logs; the perf12 at-rest census golden
(ONECOPY's `main-red-the-perf12-at-rest-census-golden-moved-with-the-backstop-abort`)
may move.

## Review

**Single FULL**, difficulty **H**: the routing is mechanical once the
record is in hand, but the census's verdicts move and each move must be
judged.

---
id: two-ops-placements-state-a-carrier-held-premise-unheld
kind: issue
title: Four placements (ops.rs place_witness and piece_reading, census curved_overlap_cells and point.rs in_face) state a carrier-held premise and read through the unheld face_placement
status: closed
opened: 2026-11-10
priority: P3
refs: [6669, 6705, carrier-held-points-are-trusted-not-decided]
closed: 2026-11-10
pr: 6741
---


Filed by `onecopy/held-and-pierce` (PR 6705). That PR routed four
carrier-held Boolean sites through `topo::held::Held` and
`placement_on_carrier`: node, rim_wedge, overlap and
`line_through_both_faces`. Its closing sweep of the remaining
`face_placement` callers found two more that state the premise (and its review two more, in the census) and still
place through the unheld door, each with its own `OffCarrier` policy:

- `crates/topo/src/boolean/ops.rs` `place_witness` (about line 1850):
  "the witness was built on the carrier, so that answer contradicts its
  construction". `OffCarrier`, and an on-plane margin in band, answer
  `None` (no verdict), when they are a defect of the construction.
- `crates/topo/src/boolean/ops.rs` `piece_reading` (about line 3640):
  "the on-plane margin re-reads a plane the census already put the point
  on", which is a decided premise. Yet `OffCarrier` reads as
  `PieceRead::NotIn`, and the on-plane margin escalates as
  `SelfCheck(CarriedLineage)`.

- `crates/topo/src/census.rs` `curved_overlap_cells` (about line 4497):
  the midpoint of a span of an edge `e` that lies on face `f`'s carrier,
  placed through `face_holds` (`pm_census_ef_curved_contain`).
- `crates/topo/src/census/point.rs` `in_face` (about line 242): a point
  the census reads on the face's plane, placed through `face_holds`
  (`pm_census_point_on_plane`).

Both census sites go through `census::face_holds`, whose doc (and
`topo::face_holds`' in `lib.rs`) used to claim the point was held. 6705
corrected both docs to say the door decides the point against the
carrier. Moving those two callers onto a held door is this row's job.
`face_holds` itself stays the unheld reader for callers that hold
nothing.

**Not this class:** join's `holding_face_normal` (`face_placement` at
about `crates/topo/src/boolean/join.rs:5675`) searches `other`'s faces
for the one holding `q`, and `q` is off most of them, so its unheld
door is right. The same goes for `census/in_plane.rs` and `cover.rs`'s
nearest foot, which read points nobody holds.

**Fix, as 6705 did:**
1. Probe each site first: residual and both doors' answers, at both ε.
   The probe branch `onecopy/held-sites-probe` is the template.
2. Mint the `Held` its premise states (`constructed` for the witness;
   `decided` with the census's margin for the carry, if it is in hand).
3. Make `OffCarrier` the site's defect arm.
4. Read the k-stat ledgers. Removing the on-plane decision moves
   `bool_section_witness_on_plane` and `bool_carry_vf_on_plane` counts,
   and their audit rows.

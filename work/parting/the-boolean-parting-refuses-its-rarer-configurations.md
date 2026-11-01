---
id: the-boolean-parting-refuses-its-rarer-configurations
kind: issue
title: The Boolean's parting refuses typed where a wall is tangent, an end lies on the face more than once, on one hole, on its boundary mid-edge, touching an edge, or under a record it would move
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by the Boolean slit-form lane (`cleave/boolean-builds-the-slit-form`,
the parent row `a-boolean-builds-a-reflex-edge-lying-inside-a-face`).

## What

`topo::boolean::slit` parts an edge lying in a face whose material both
the edge's faces leave the line into, and refuses
`BooleanError::EdgeInFace` with an `EdgeInFaceGap` construction gap,
rather than keep the rider form, where:

- `Shared`: an end lies on the face's loops more than once, or the
  edge's walls go into two faces' material (two coincident faces);
- `Pocket`: both ends lie on one hole of the face, so the parting cuts
  a piece of face off the hole, which needs a containment reading to
  tell the piece from the hole;
- `EndOnBoundary`: an end touches the face's boundary inside an edge of
  it (a vertex-on-edge record says so). An end a vertex-vertex record
  puts on a vertex of the face's loops is joined to it
  (`slit::part`'s joints, built with
  `a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted`);
- `Recorded`: a contact record names the face the parting divides;
- `Tangent`: a wall is tangent to the face along the line, so each of
  the two edges bounds a cusp (D1's doubled cusp), and the reading of a
  cusp on a line of coincident edges is not built (PR 5357's gate
  refuses `LineWedges { Unread { why: Degenerate } }` there). Reached by the holed plate
  less a cutter tangent to its hole
  (`crates/sweep/tests/wedge_end_doors.rs`,
  `a_plane_tangent_to_a_hole_answers_every_op`;
  `crates/sweep/tests/review_4831_probes.rs`, `p5`), where main built
  the ruling as one edge the cut face passes through, and by `p12` there,
  whose ruling runs on past the pocket's wall: the tangency refuses
  before the division. Built, `p12`'s `A ∖ B` returns to its body,
  `30 − π`, a target this row carries from
  `a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted`;
- `DividedEndTouched`: an end the parting divides touches an edge's
  interior (a vertex-edge record), and the census reads that touch at
  neither copy: built, 3′ refuses `CensusUndecidable`. Reached by
  `crates/topo/tests/a_vertex_read_by_two_sector_passes.rs` (the glued
  blocks, out the bottom, `y − x`) and
  `crates/topo/tests/three_solids_on_one_line.rs`
  (`a_contact_line_beside_a_face_holding_it_builds_in_every_op`, towards
  350°);
- `NoNormal`: a wall has no normal at the edge's midpoint.

`Shared`, `Pocket`, `EndOnBoundary`, `Recorded` and `NoNormal` are
reached by no test in the topo, sweep and editor-core suites at
default, 1e-6 or 1e-12. Each is a configuration the ratified sentence
decides; each is a construction gap, with no lever. An arc refuses
`Curved` (`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`).

An edge running past the face is divided where it crosses the face's
boundary (`slit::divide`, `crates/topo/src/boolean/slit.rs:1034`, built
with `a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted`).
Its refusals: `Overlap`, a boundary edge running along it
(`crates/sweep/tests/an_edge_running_past_a_face_reads_its_crossings.rs`);
`Unplaced`, another record naming it
(`crates/topo/tests/three_solids_on_one_line.rs`, `UNPARTED`, and
`an-edge-running-past-a-face-along-coincident-edges-is-not-divided`).
Reached by no test in the topo boolean and witness suites at default,
1e-6 or 1e-12, nor by the PR 5546 review's probe scenes: `Unrecorded`
(no record names a crossing), `CrossedCurve` (a crossed boundary edge
that is no line; the curved lane's), `AtACrossing` and the escalation of
`BooleanDecision::PartingOrder` (two division points at one, or in band,
along the line), `Unjoined` (a division point bounding no piece in the
face, as an edge touching a corner from outside), and the defect
`Redivided` (`slit.rs:353`).

`Unjoined` fires because `divide` takes every vertex-on-edge record of
a corner of the same face as a crossing. A corner that bounds no piece
lying in the face could stay a resting record instead, as the review's
probe `s19` already does where the touching apex lies on a separate
face. Witness: the review's `s19b`, a connected non-convex wall whose
protrusion's apex touches the hole's edge from outside at `z = 0.75`.

## Fix

Build each: `Pocket` by nesting the cut-off loop as `nest_slit` nests a
slit (PR 5350); `EndOnBoundary` by dividing the boundary edge at the end,
onto its point, and joining the end to the new vertex, as `slit::divide`
and `slit::part` do for a crossing; `Recorded` by placing the record's
cell on the piece holding it; `DividedEndTouched` by giving each copy
the touch's record; `Tangent` once the cusp-line reading is built (the
wedge reader's, with the torus's bend for a curved wall).

**PR 5479 cleanup notes (its re-review, 2026-10-21).** None of these are blocking.
- `off_wedge`'s `ClassificationInvariant` arm is unreachable.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). PARTING holds the parting's reading of edges lying in faces and the cuts on spline faces; id and body unchanged. (CLEAVE orchestrator)

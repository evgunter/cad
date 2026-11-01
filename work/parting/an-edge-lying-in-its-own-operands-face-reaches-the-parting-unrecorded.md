---
id: an-edge-lying-in-its-own-operands-face-reaches-the-parting-unrecorded
kind: issue
title: An edge lying in a face of its own operand reaches the parting with no record, so only the geometric reading parts it
status: parked
opened: 2026-11-01
priority: P1
cost: M
blocked_on: [a-finished-body-holds-its-contact-records-at-every-door]
---


Found by the PARTING lane on `parting/parting-reads-records`, measuring
before it rewired `topo::boolean::slit::riders` to ask a plane face of a
line edge only where an edge-on-face record names the pair (its row,
`the-parting-reads-no-edge-lying-in-a-face-no-record-names`). Blocks
that rewiring.

## What

`reduce::sweep_direction` reads one operand's edges against the other
operand's faces only, so a line edge lying in a plane face of its own
operand is recorded only if the operand brings the record with it
(`Glue::records_a` / `records_b`, `OperandRecords`). The public doors
over at-rest operands (`subtract(&AtRestBody, &AtRestBody, tol)` and
its siblings) bring none: `AtRestBody` holds no records. Such a pair
reaches `slit::part_riders` with no record, and today only the
geometric reading in `slit::holding` (`carrier_on_surface`, then the
midpoint's containment) finds it.

Measured instance: `crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`,
`every_edge_a_vertex_read_again_reads_is_classed_against_the_germ`, the
scenes over `tipped_b` ("a tip in the notch": a block less an octant at
`MEET`, union a tet tip whose edges lie on the notch's faces). The tip's
edge along `(1, 0, 1)` lies in the notch face `y = 1` of `tipped_b`
itself; in `tipped_b` both its walls leave on the face's empty side, so
it stays one edge in contact. In an op that flips `tipped_b` (it is the
subtrahend), the walls go into the face's material, and the result
needs the parting. In 10 result cells (over the poses) the records
`part_riders` receives are empty, and:

- the geometric reading parts the edge (the result has one more edge
  and one more face);
- a records-only reading leaves one edge, and `validate_geometric`
  passes the result, so the test (tier 3 asserted) passes too: D1-wrong
  topology shipped silently.

The same measurement saw unrecorded line-in-plane pairs the geometric
reading finds, both walls leaving on the empty side (so no parting
moves), in `a_vertex_read_again_classes_every_edge`
(`partners_along_the_face_build_sound_against_the_germ` and the row
above), `a_vertex_read_by_two_sector_passes`,
`three_solids_on_one_line::a_contact_line_beside_a_face_holding_it_builds_in_every_op`,
`three_solids_on_one_line::a_split_of_an_edge_lying_in_an_earlier_face_reads_its_vertex_against_it`
and `editor-core`'s `names::emit_topo::touch_reread_rows`; unclassified
whether each is the same class (operands built by earlier ops) or a
sweep gap.

## Fix

Either record an operand's own edge-in-face contacts where the Boolean
takes the operand, or carry a result's records on the body the next op takes, so
that every line lying in a plane face reaches the parting with its
record. Then the parting's rewiring can drop the geometric reading for
lines in planes.

## Parked (2026-11-01)

The second fix this row names is HELDRECORDS' ratified unit
`a-finished-body-holds-its-contact-records-at-every-door` (Ev, PR 5967): an `AtRestBody` holds its contact records and
every door carries them, so `tipped_b`'s own tip-in-notch record,
written by the union that built it, reaches the next op. Parked on it.
When it lands, re-take this row's measurement: a body no door built
(a primitive, an import) whose edge lies in its own face still arrives
unrecorded unless its build door records it, and the walls-outside
reads above may be that class. (PARTING orchestrator)

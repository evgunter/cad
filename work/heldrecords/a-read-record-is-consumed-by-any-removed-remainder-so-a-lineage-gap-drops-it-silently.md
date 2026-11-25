---
id: a-read-record-is-consumed-by-any-removed-remainder-so-a-lineage-gap-drops-it-silently
kind: issue
title: The carry consumes a read, unlanded record wherever any remainder was removed, so a lineage gap drops the record silently instead of refusing it stale
status: dispatched
opened: 2026-11-23
priority: P1
cost: M
refs: [a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key, 7593]
branch: heldrecords/removed-members
---

Found by the review of ORBIT's PR 7593 (`orbit/cut-in-carry`), which
demonstrated it on main; filed by that lane.

## What

`boolean::records::landings` (`crates/topo/src/boolean/records.rs`, the
tail of its per-row loop: `if read { if removed || moved { continue; } ...
Stale }`) consumes a record that landed on no pair, after some pair was
read, whenever ANY of its cells' fates removes something
(`crate::Fate::removes`): a cell removed whole, or divided with its
remainder removed. It does not ask whether the removed part is where the
touch was. So a lineage that hands a cell's images incompletely, and
also removes some unrelated piece of it, reads the record against the
wrong pieces, finds it nowhere, and drops it, where the rule would
otherwise refuse it `CarryRefusal::Stale`. Every lineage gap of that
shape therefore surfaces as a silent drop, not a refusal.

Demonstrated on main before PR 7593: the banded ball carrying a
vertex-on-face rest on the piece a sphere cut-in minted, united with
or subtracted by the cutting slab, returned a body with no record. The
fates there were `[Kept { vertex }, Divided { pieces: [Face(14v1)],
minted: [], remainder: Removed }]`: the carry read the face by its
pre-cut key, so its only image was the OTHER piece, and the removed
remainder (the cap the slab took) licensed the drop. PR 7593 closes that
gap (the re-entered op reads the cut-ins' divisions), so this pose no
longer reaches the rule, but the rule still hides the next gap.

## Why it is a design question

The rule is ratified text: `crates/topo/README.md`'s carry paragraph,
"consumed where one of its cells was removed, divided with its remainder
removed, or moved". Requiring the removed part to have held the touch
(for instance, consuming only where the record's point, read against the
removed part, lies in it) changes what that clause decides, so it goes
through designers and an `[ev]` PR.

## Way through

Weigh the fork first. Then measure what moves: every boolean and
surgery-verb test whose records are consumed through a removed
remainder, to see which drops were touches the op really removed and
which were gaps.



## 2026-11-25 — what moves (lane `heldrecords/removed-members`)

The build on `heldrecords/removed-members`: `Division { pieces,
minted, removed }`, `Remainder` gone, `Fate::removes` whole removals
only; `boolean::records::landings` reads an unlanded record against the
named removed members (in the operand clone, or in the two clones'
joint body for a reduction row across operands) and consumes only a
landing there; "moved" consumes only for a cell kept or substituted
whole; the shell and blend name no removed member (1a).

Measured on the gate's `ci` set of topo and sweep (7186 tests, 15
moved). Each was consumed on main through "any removal":

- **Ruled, rewritten.** A record touching nothing now refuses `Stale`:
  `sweep::blend_carries_records::a_record_touching_nothing_on_a_support_face_refuses_stale`
  (was `a_record_on_a_support_face_is_consumed`), the bookkeeping
  record in `a_turns_third_edge_lands_a_resting_corners_record_on_the_surviving_piece`,
  the notch in `a_carried_record_no_reading_covers::a_record_an_op_divides_is_read_and_found_stale`.
  A real touch the shell's opening removed now refuses `Stale` (1a):
  `surgery_carries_records::a_rim_that_opens_under_a_resting_corner_refuses_its_record_stale`.
- **Still consumed, now through a removed member:**
  `one_record_carry::a_pit_under_a_carried_rest_consumes_it`,
  `a_notch_that_removes_a_recorded_overlap_consumes_it`, and the cone
  apex, ruling, slit-tip and arc-tangent booleans (their reduction
  rows on removed fragments, read in the joint body).
- **Valid bodies now refusing: a reduction row on the section between
  a kept piece and a removed fragment** (`CarryRefused { Found, Stale }`).
  Twelve edge-in-face rows: the face is cut along the edge, and in the
  result the edge bounds a kept piece (that pair is structure) and
  reads apart from the others; the removed fragment holds the edge on
  its boundary, not its interior. Two vertex-in-face rows: the vertex
  lies on the section at a kept piece's boundary, a touch another row
  records (vertex–vertex). The section's edges and vertices are in the
  face's interior but the division names them neither minted nor
  removed (`Incidence::chords` takes only edges every real face of
  which is a fragment; a section edge's other side is a null face).
  Tests: `band_co_requested_boundary` (2), `band_planar_cut_off_meters`,
  `join1_mechanisms::the_incidence_check_reads_the_whole_site`,
  `join_whole_orbit_cylinder`, `wedge_through_a_full_turn_collar`,
  `boolean::holdings_rows::a_face_parted_after_a_substitution_holds_the_tile_only_where_it_lies`,
  `a_figure_eight_hole_cut_through_its_pinch`,
  `a_ring_crossed_at_its_vertices_is_divided_or_refused`,
  `union_flush_onto_edge_contact` (4), and the vertex rows
  `records_cite_their_decision::a_decision_at_copies_an_operand_already_held_is_a_row`,
  `vertex_on_edge_records::a_split_at_the_other_edges_end_records_a_vertex_vertex_pair`.
  Held for a ruling: the ratified clause consumes only on removed
  cells, and these touches are on neither a kept piece nor a removed
  member's interior.

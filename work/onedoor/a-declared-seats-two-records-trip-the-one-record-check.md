---
id: a-declared-seats-two-records-trip-the-one-record-check
kind: issue
title: A Boolean on a body whose seat is a declared face-face contact trips the one-record check at the seat's two records
status: closed
opened: 2026-11-10
closed: 2026-11-11
---


Found merging main into `onedoor/one-surface-record` (2026-11-10). Main's
seat rows run a Boolean on one body of two solids whose seat is a
recorded face–face contact (`vtxfac::Wall`). In the debug build the
branch's one-record check (`boolean/carrier_pair.rs`
`assert_one_record_per_carrier`, called before the gate in
`boolean/ops.rs`) panics: an edge of the result lies at rest between the
seat's two surface records, which its rest reads as one carrier, and no
act left them one record.

Witnesses, red on the branch head and not on main:
- `crates/topo/tests/a_vertex_through_a_declared_seat.rs`
  `a_union_bridging_a_seat_touches_itself_and_one_on_a_piece_builds` and
  `a_cut_across_its_tools_seat_refuses_typed`;
- `crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`
  `a_seat_unioned_with_a_bead_keeps_its_pieces_and_a_bridge_touches_itself`.

The seat is a contact the body declares, not a glue the Boolean decided,
so `OneCarrierPairs::unify` never sees it. Either the check exempts a
pair a body's record holds as contact, or the seat's two faces belong on
one record too. That is a question about which acts leave two records
of one carrier, and it waits on the coincidence act's restructure.

## Closed (2026-11-11)

The one-record check is gone: nothing reads values at rest
(`crates/topo/README.md` `[topo.one-record.rest]`), so a seat's two
records at rest are two surfaces, as `[coincidence.door]` says, and its
edges rest on their pair. The three witnesses are green on the branch:
`a_vertex_through_a_declared_seat`'s
`a_union_bridging_a_seat_touches_itself_and_one_on_a_piece_builds` and
`a_cut_across_its_tools_seat_refuses_typed`, and
`intent_dprime3_assemble_product_slot`'s
`a_seat_unioned_with_a_bead_keeps_its_pieces_and_a_bridge_touches_itself`.

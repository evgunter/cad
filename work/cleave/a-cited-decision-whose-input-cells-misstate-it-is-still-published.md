---
id: a-cited-decision-whose-input-cells-misstate-it-is-still-published
kind: issue
title: A decision a contact record cites is published as a row even where its input cells state another relation (an edge on a face for a point of the edge)
status: open
opened: 2026-10-19
priority: P2
cost: M
design: true
---


Filed by WIRE's `wire-citerows` lane (PR 5352's fix pass).

## What

PR 5352 spells a boolean decision's cells through one door
(`boolean/ops.rs` `spell`) and holds unspelled
(`BooleanNaming::unspelled`) every decision whose input cells would
state another relation: a vertex minted on an edge, read as that edge
and decided on a face, would say the edge lies on the face. A decision
a surviving contact record cites is the exception. The record's backing
must name a row (`Backing::Decided`), so the row is published as read,
misstated, as it was on main before the PR.

Measured on the PR's head (`cargo nextest run -p topo -p editor-core`,
counting the cited misstatements `cite_rows` let through): 15, in four
tests, every one `OnCarrier`:
- `names::emit_topo::touch_reread_rows::a_vertex_read_twice_names_its_result_in_every_op`
  (2) and its `_at_every_pose` sweep (8): `[Edge, Face]`.
- `face_face_touch_carry::a_later_op_on_a_rod_resting_on_a_plate_carries_its_cap_records`
  (2): `[Edge, Face]`.
- `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`
  (3): `[Edge, Edge]` within one operand.

## What it needs

The record's backing names the decision without a misstated row: the
meeting spelled by the cells that meet (WIRE unit 2's k-ary row), or a
backing that cites a held decision. Then no row in the output of
`cite_rows` or `respell_through_cuts` is published past `spell`'s `Err`.

Signed (WIRE implementer, wire-citerows).

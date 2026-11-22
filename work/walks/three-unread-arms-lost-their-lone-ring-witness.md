---
id: three-unread-arms-lost-their-lone-ring-witness
kind: issue
title: Three refusal arms that read an unread piece (a carry's CarryRefused::Unread, slit's EdgeInFaceGap::Unread, rim_wedge's Unplaced::Door) were witnessed only by a lone-vertex ring, which the face door now reads
status: open
opened: 2026-11-22
---

Found by the face-region unit (branch `walks/face-region`). Before it,
`contain::boundary_pre_pass` refused a whole face holding a lone-vertex
ring as `ContainError::EmptyLoop`, though the ring bounds no area. Three
rows used that refusal as their witness of a face door that cannot read
a piece:

- `boolean/ops.rs`, `found_on_a_divided_face` — the carry's
  `CarryRefused { refusal: CarryRefusal::Unread { .. } }`;
- `boolean/slit.rs`, `unread_piece` — `EdgeInFaceGap::Unread(Some(_))`
  where the edge's ends name the pair;
- `rim_wedge.rs`, `unplaced_rows` — `RimUnread::Unplaced { why:
  Unplaced::Door(_) }`, named by its side.

The face door now reads past a lone ring (`FaceView::area_loops`), so
those rows were re-aimed at that reading and the three arms have no
witness. Each needs one: a face whose door refuses on no margin by
another route (a face with no outline, `ContainError::EmptyLoop` from
`NoOutline`, is one where a fixture can carry it; an uncrossable or
unreadable loop is another).

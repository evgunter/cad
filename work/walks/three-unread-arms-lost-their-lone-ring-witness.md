---
id: three-unread-arms-lost-their-lone-ring-witness
kind: issue
title: Three face-door refusal routes (slit's EdgeInFaceGap::Unread(Some), rim_wedge's Unplaced::Door, the carry's Unread by its door) were witnessed only by a lone-vertex ring, which the face door now reads
status: open
opened: 2026-11-22
priority: P3
cost: M
---

Found by the face-region unit (branch `walks/face-region`, PR 7474).
Before it, `contain::boundary_pre_pass` refused a whole face holding a
lone-vertex ring as `ContainError::EmptyLoop`, though the ring bounds no
area. Three routes by which a face door's refusal reaches a caller had
that refusal as their only witness:

- `boolean/slit.rs`, `unread_piece` — `EdgeInFaceGap::Unread(Some(_))`
  where the edge's ends name the pair;
- `rim_wedge.rs`, `unplaced_rows` — `RimUnread::Unplaced { why:
  Unplaced::Door(_) }`, named by its side;
- `boolean/ops.rs`, `found_on_a_divided_face` — the carry's
  `CarryRefusal::Unread` reached through the face door's refusal. The
  arm itself keeps a witness by another route,
  `a_rest_in_band_of_the_plane_refuses_unread`; only the door route lost
  one.

The face door now reads past a lone ring (`FaceView::area_loops`), so
those rows were re-aimed at that reading. Each route needs a witness
of a face door that refuses on no margin by another cause: a face with
no outline (`ContainError::EmptyLoop` from `NoOutline`), where a
fixture can carry one, or an uncrossable or unreadable loop.

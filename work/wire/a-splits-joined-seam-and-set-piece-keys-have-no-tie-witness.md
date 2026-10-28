---
id: a-splits-joined-seam-and-set-piece-keys-have-no-tie-witness
kind: issue
title: A split's per-candidate keys for joined seams and set pieces have no tie witness
status: open
opened: 2026-10-27
priority: P3
cost: E
---


Filed by WIRE's `wire-seamgroups` lane, which was asked to add these
witnesses and could not build one.

## What

`crates/editor-core/src/names/emit_topo.rs` `name_split_edges_vertices`
keys `edge_groups` by `(half, name, ParentEntities)`. The `SectionEdge`
arm has a tie witness
(`lib_g14_split_walls::a_tied_faces_undivided_chords_tie_under_the_bare_name`);
the `Joined::Seam` and `Joined::SetPiece` arms have none, so a revert of
their keys to the name alone stays green.

## Why the obvious fixtures do not reach them

- **`Joined::Seam` with the tied face on the merged side.** Every face
  of a half holding part of the plane is linked into ONE parent
  (`SplitNaming::plane_pieces`), so two candidates the plane merged are
  one parent and their seams are rightly pieces of one seam.
  `lib_g14_split_walls::a_split_plane_parents_joined_seams_are_pieces_of_one_seam`
  pins that (`fixture::u_cutter_tie`, a slot parting the top, split at
  z = 3).
- **`Joined::Seam` with the tied face on the other side.** The tied
  face must cross the plane and hold an operand edge lying in it,
  collinear with its chord, beside a merged face. `u_cutter_tie`'s ties
  are its caps, horizontal; a notch giving a floor such an edge is a
  target face the two floors border differently (`Borders`), which
  breaks the tie.
- **`Joined::SetPiece`.** Needs a joined edge along several tied operand
  edges, part of one of them, with no chord; no planar fixture was found.

## What it needs

A tie whose candidates differ only in place, each holding an in-plane
rim collinear with a chord (or a run of in-plane operand edges), split
by the plane holding that rim; assert each candidate's joined edge is
the bare name, tied, and that it fails with the key reverted to
`(half, name)`.

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

## `Joined::Seam`: witnessed in `wire/tie-summed-decline-witnessed`

`split_joined_keys_tie::a_tied_faces_joined_seam_beside_the_merge_is_the_bare_name`
splits the U-cutter tie along a vertical plane, x = 3. The tool is the
U united with a pocket x 3..5, y 1.2..2.8, z 0.5..1.5. Each tied floor
crosses the plane for y 1..1.2. For the rest of its width it shares an
edge with the pocket's end wall, which lies in the plane and merges into
the x < 3 section. One joined seam per candidate, between the floor and
the merged face, is the bare name `Tied` over both. With the arm's key
reverted to `(slot, name)` the half holds two rows, and the test fails.
The fixtures above missed this because they split horizontally, where
the tied faces (the caps) lie in the plane.

## `Joined::SetPiece`: still no witness

What was tried, and why each try fails:

- **Straight roots crossing the plane.** The join vertex between two
  roots is then an operand vertex off the plane, and the split leaves
  its degree alone. That degree is at least 3 in an extrude, and in a
  boolean, whose closing join removes collinear degree-2 vertices. So
  the vertex is never joined.
- **In-plane roots, partial because their copy lives in the other
  half.** A tie needs each root to be an edge of a tied face lying in
  the plane. Run with the U-cutter tie split at z = 1: the floors'
  edges along the wall x = 4 alternate with that wall's chords. The
  Below half joins the whole line x = 4, z = 1 into one edge, which
  holds both candidates' roots and is `Joined::Seam`, not `SetPiece`.
  For roots alone to cover a line, two in-plane faces of one half must
  abut along it, and the boolean merges them (N3) first.
- **Curved roots.** An untied `SetPiece` is reached: the split in
  `coincidence_door::an_arc_crossed_twice_reunites_by_construction`,
  where a rim's arcs meet at degree-2 vertices. A tied one needs two
  candidates' curved roots crossed by the plane. This kernel's ties
  come from one tool face's pieces bordering one target face, such as
  the caps against a wall. Their edges on a curved target face end at
  the prongs' walls, at degree-3 vertices. Not built.

None of these proves `SetPiece` cannot be reached, so its key stays
unwitnessed. The per-candidate key is still right: `ParentEntities`
is the one rule every name-keyed pass uses, and here it costs one line,
so it is not dead complexity.

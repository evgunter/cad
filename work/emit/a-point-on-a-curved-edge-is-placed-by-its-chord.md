---
id: a-point-on-a-curved-edge-is-placed-by-its-chord
kind: issue
title: Two vertex readings place a point on a possibly curved edge by the edge's chord
status: closed
opened: 2026-10-08
closed: 2026-10-14
priority: P3
cost: E
branch: emit/union-readings
---


## What

Found by the sweep for straight-only lies-on tests on the branch that
closed `a-chord-across-a-curved-rim-cut-in-pieces-refuses-shared-rim`.
Two readings decide whether a point lies on an edge with
`Segment::of_edge(..).place(..)`, which tests against the segment
between the edge's ends, and neither checks that the edge is straight:

- `emit_topo::name_boolean_edges`'s `set_holding` (near
  `emit_topo.rs:1660`): the operand edges of a joined edge's set that
  hold vertex `v`. `joined_cover` builds a set for a curved joined edge
  too (`JoinedCover::Set` off `Track::cover`). A vertex inside a curved
  set member reads as off it and would cite the wrong member edge, or
  none.
- `emit_union`'s crossing ranking for a member line (the `on_line` rows
  near `emit_union.rs:1316`): each crossing is read on the row of the
  line it lies on, else the line's least row. A crossing inside a curved
  piece other than the least lies on no segment and is read on the least
  row. Every piece of one circle shares its carrier, so the parameter
  probably matches, but where a closed carrier's pieces straddle its
  period cut, `param_along` reads within the wrong piece's interval.

Neither is reproduced. The rows in `emit_curved_rim_chord` cross each
piece once per face, so no crossing there is ranked.

## What a fix looks like

A curved edge's "holds this point" test, `Track` offset within its span
(the reading `rim_holding` and `joined_cover` take for a chord), next
to `Segment::place`, and both sites pick by the carrier kind. A row for
each: a curved joined set whose end lies inside a member, and two
crossings by one face of a curved line in pieces.

## Closed (branch `emit/union-readings`, 2026-10-14)

`names/emit_topo.rs` gains the curved edge's "holds this point" test:
`Track::holds` (on the carrier at an offset within the span, the ends
included; a closed edge holds its whole carrier; a point a hair short
of a periodic edge's start reads a period on), and `edge_holds`, which
picks it or `Segment::place` by the carrier kind. Both sites now call
it:

- the pair boolean's `set_holding`, through `edges_holding`;
- the union's crossing ranking in `cite_member_edges`, whose `on_line`
  rows are now read through `edge_holds`.

Row: `a_point_on_a_curved_piece_is_held_along_its_carrier` (the split
prism's rim in three arcs): a point on the arc inside the middle piece
is held by it alone (the chord reading held it by none), the midpoint
of that piece's chord by none (the chord reading held it by the
middle), a split point by both pieces it ends, each rim end by its
piece.

Not built: the two end-to-end rows the issue proposed (a curved joined
set whose end lies inside a member; two crossings by one face of a
curved line in pieces). Neither shape was reproduced before, and the
union does not name a curved joined edge by a set at all
(`Flush::edge_cover` reads straight edges only,
`a-unions-flush-reading-is-straight-only`), so only the pair's set and
the union's ranking reach the helper.

Sweep: every `Segment::of_edge`, `.place(`, `.on_line(`, `runs_with(`
and `straight_segment(` in `names/`. Each other hit is behind an
`is_straight` guard (`chord_on_rim` via `chord_within`, `same_way`,
`least_per_line`, `rank_along_seam`) or is a unit test. Blind spot: a
lies-on test written over raw point distances rather than `Segment`;
`holds_whole` and `least_vertex` compare points to points, not to an
edge's span, so they are not this shape.

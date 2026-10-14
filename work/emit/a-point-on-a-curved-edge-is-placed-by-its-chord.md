---
id: a-point-on-a-curved-edge-is-placed-by-its-chord
kind: issue
title: Two vertex readings place a point on a possibly curved edge by the edge's chord
status: open
opened: 2026-10-08
priority: P3
cost: E
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

---
id: the-corner-presentations-each-spell-their-reach-preamble
kind: issue
title: The corner presentations each spell their reach preamble, sliver and leaves_line call: present_line_corner by hand beside the shared reach_holds_only and in_frame of the two affine ones
status: open
opened: 2026-11-17
priority: P1
cost: M
---

## Finding

`topo::injective`'s three corner presentations read one logic in
three spellings:
- `present_plane_corner` (a plane's line edge at a corner);
- `present_vertex_corner` (a vertex shared alone);
- `present_line_corner` (a chart's coordinate line at a corner).

PR 7093 shared what its diff touched:
- `reach_holds_only`: the reach rectangle's ball and `only`;
- `in_frame`: a harmonic carried into an affine frame. Before, one
  copy carried the linear term and the other zeroed it;
- `copy_misses`: the scan of a curve's copies by whole turns, also
  `leaves_line`'s.

What is still spelled more than once:
- **`present_line_corner`'s reach.** It builds its ball by hand:
  `Ball::holding(surface.point_eval(..), surface.rect_enclosure(..))`
  and then `only`. That is `reach_holds_only` on a `Map::Chart`
  rectangle, except that it uses a point evaluation where the helper
  encloses a degenerate rectangle.
- **The sliver block.** `present_line_corner` and
  `present_plane_corner` each compute a width and decide
  `SELF_INTERSECTION_SLIVER` (frozen ground: the sliver row is with
  designers, so this is the spelling only, not what it concedes).
- **The `leaves_line` call and its speeds.** Each site computes its
  speeds its own way: `speeds` (a rectangle jet), an exact plane
  speed, and the reach rectangle's jet.
- **`leaves_graph`'s segment scan of a straight edge's copies.** It
  reads a hull, not pieces, so it is not `copy_misses`. That is fine,
  but it is a fourth copy rule beside `present_vertex_corner`'s
  argued one.

By `work/README.md`, two implementations of one logic is P1.

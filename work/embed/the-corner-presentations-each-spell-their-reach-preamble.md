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

## Further evidence: "the shared cell is one vertex alone" (lane conic-vertex, PR 7105 fix pass, 2026-11-18)

The PR 7105 review found the vertex-alone test spelled four ways in
`topo::injective`. The fix pass gave it one home, `vertex_alone` (no
shared edge, one shared vertex), and used it at the two sites that ask
exactly that: `corner_of`'s `alone` corner and `graph_of` at a vertex
alone. Three spellings stay, because each accepts something else, and
unifying them would change what they read:
- `vertex_corner` (the quadrant at a vertex alone) asks one shared vertex
  and neither of the corner's walls near. It does not ask for no shared
  edge, so a cell whose shared edge is a seam (no wall) still reads.
- `planes_through`'s plane through a vertex asks no shared edge and the
  first of the shared vertices, however many there are.
- `corner_of`'s lens is read at its end among the cell's local vertices
  (`common == [v, w]`), against `vertex_corner`'s single common end.

**Which vertices a separator lists is also decided per arm.**
`planes_through`'s `through` lists every end of each held edge.
`graph_of` lists none across a shared edge, and at a vertex alone that
vertex only. Both concede the same thing: the stored vertex's certified
tolerance off the carrier (the module docs' separator concession). But
nothing ties the two lists to one rule.

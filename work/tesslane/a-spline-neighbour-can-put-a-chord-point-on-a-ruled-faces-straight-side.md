---
id: a-spline-neighbour-can-put-a-chord-point-on-a-ruled-faces-straight-side
kind: issue
title: a spline neighbour's lattice can put a chord point inside a ruled face's straight side, which the rectangle lane refuses
status: open
opened: 2026-11-17
priority: P3
cost: M
---


Found by PR 7049's review (`a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it`).

## Mechanism

`chords::compute_chords` gives a line edge one chord, then
`chords::tighten` lets each side's lattice raise the count, skipping a
side the rectangle lane takes. By reading, which chart-lane arms can
raise a LINE's count:

- the trimmed cylinder and cone arms: no. Their step along a ruling is
  `hv = ∞` (`chart_table::AnalyticRow::steps`), and the cylinder arm
  does not chord to its lattice at all (`chorded_to_lattice`);
- the sphere, torus and spindle arms: no line edge bounds such a face;
- the spline arm (a NURBS or approximating face): yes, from the image
  speed it reads off the edge.

So a straight edge shared by a ruled face the rectangle lane takes and
a spline face on the chart lane can get a chord point inside it. The
ruled face's walk then holds a point inside a straight side, which
`walk::splits_a_straight_side` cannot see (it reads the loop's vertices,
not its chords), and `curved::tessellate_curved` refuses
`UnsupportedCurve { note: curved::SIDE_CHORD_POINT }` on that edge,
where the base laid its fallback lattice.

## Survey (2026-11-17, head of `tesslane/cone-axis-winding`)

Every line edge whose final chord count is above one, with both sides'
surface and lane, instrumented in `compute_chords`:

- the `mesh` suite: 236 such edges, all between `{Plane, Nurbs}` or `{Nurbs, Nurbs}`;
- `sweep`'s 63 meshing modules: 80, between `{Plane, Nurbs}`, `{Nurbs, Nurbs}` or `{Plane, Approx}`;
- `step-import`: 3, the same kinds.

None has a rectangle-lane ruled face on a side. The tour was not
surveyed: in a debug build its `eps_regression` rows ran past 600 s.

## The cure seen

Two, neither built. (1) The rectangle lane takes a chord point on a
straight side as what it is, a boundary vertex dividing the end
triangle of that column in its plane, and refuses only a vertex the
route should have caught. (2) The route also sends a ruled face one of
whose line edges is shared with a spline face; that reads a
neighbour's surface kind, still from the loop. A body is owed first.

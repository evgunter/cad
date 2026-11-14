---
id: fillet-side-and-centre-arcs-and-off-axis-hops-record-no-direction
kind: issue
title: Fillet, side and Center arcs and off-axis hops record no direction, so they still reach the doors as a quotient with width
status: open
opened: 2026-11-14
priority: P1
cost: M
refs: [6963]
---



Found by PR 6963's review (the profile half of the recorded
directions), and disclosed in that PR's body with no file behind it.
Line numbers are at that PR's frozen head `8835b1bb`.

**What is wrong.** A validated segment records the unit directions its
construction made, and PR 6963 fills them for the circle, the split
circle, the rectangle, directed legs, bulge, via and tangent arcs,
`arc_onto`, and hops along an axis. These verbs record nothing, so
validation mints the direction as a quotient of two points, which over
a parameter box has a width the geometry may not have:

- **Fillet arcs** (`crates/profile/src/path.rs:4387` `fillet_arc`,
  `:4450` `fillet_arc_carrier`). A fillet between two lines has its
  radials in hand: they are the two lines' unit normals, which the
  construction holds exactly where the lines' directors are exact.
- **Side arcs and `Center` arcs** (`path.rs:3811` `side_arc`, `:3836`
  `center_arc`). The centre is authored, so the radial is
  `(v − c)/‖v − c‖`.
- **`line_to` hops off an axis, and legs to an authored point**
  (`path.rs:5247` `chord_director`, `:4293` `emit_straight_leg_at`).
  The direction is the chord's unit.

**Two different things are in that list.**
- Where the construction holds the direction (a line-line fillet), it
  is dropped, which is this row's defect and is fixed by recording it.
- Where the direction really is a quotient of authored points (a hop
  between two boxed points, a `Center` arc), it turns over the box.
  The quotient's enclosure is then honest, but its length does not
  read as 1, which is the turning-frame question on ONEHOME:
  `work/onehome/check-1-reads-a-turning-frames-length-off-its-bracket.md`.
  Recording cannot fix that half.

**Not measured:** which shipped documents reach a door with such a
direction over a macroscopic box. The reviewer infers the tour's
rocker does (it is fillets and side arcs throughout,
`demos/tour/src/rocker.rs`); whether the rocker is in a driven
document is not checked. The doors' unit
(`the-sweep-doors-read-the-profiles-recorded-directions`) measures it
through its closing tests; if the tour's certified stop is held by
this, this row is P0 and blocks the switch's deletion.

**The test that says it is closed.** A filleted rectangle with a boxed
side and a boxed fillet radius, extruded at `Sym<Interval>` over a
macroscopic box, stores each fillet wall's `u_ref` with no width; and
`ProfileLoop::directions` is `Some` on a line-line fillet.

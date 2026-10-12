---
id: offset-doors-are-one-door-with-a-held-distance
kind: issue
title: the three offset doors compute one thing by three rules: one door taking per-chart distances (0 holds) should absorb the planar and axial solves as shortcuts
status: open
opened: 2026-10-08
priority: P3
cost: H
refs: [shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam]
---

Filed by the unit 9 designer pair (analysis branch
`analysis/design-fork/shell-lofted-oblique-corner`), as a follow-up the
pair agreed is the final shape and called sequencing.

Once the per-chart door re-derives instead of transporting
(`shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam`,
its `## Decided`), `replace_faces_offset`
(`crates/topo/src/replace_face.rs`), `offset_planes_together`
(`crates/topo/src/offset_together.rs`) and `offset_charts_together`
(`crates/topo/src/offset_axial.rs`) compute the same body by three
rules, and `shell` still picks one per solid. A per-chart move is a
simultaneous move in which every other chart holds: the axial door
already reads distance 0 as held (`chart_moves`).

Owed: one door taking a distance per chart (0 holds), with the planar
solve, the axial solve and decided rigid transport as closed-form
shortcuts inside it; `replace_faces_offset` as sugar over it; shell's
door choice gone. Solving the moved charts together needs only
(moved, moved) section arms, where one at a time also needs every
intermediate (moved, held) arm, so this should land before or with
`a-fitted-wall-has-no-section-with-a-moved-cap`'s arm. O4's paragraph in
`crates/geom-brep/README.md` (the door ladder) is re-worded with it.

## Where it stands (unit 18)

`shell-moves-every-chart-of-a-solid-through-one-simultaneous-door` landed the general door, `topo::offset_surfaces_together`.
- It takes a distance per chart; a decided-zero distance holds.
- `replace_faces_offset` is its one-chart spelling: one body, `replace_face::offset_charts_staged`.
- `shell`'s per-chart arm is gone, and its rim lift reads the same door decision as the cavity.

What remains of this item is the closed forms and the door choice:
- `offset_planes_together` and `offset_charts_together` are still separate doors.
- `shell` still picks one of them per solid.

That unit's Decided 4 kept the closed forms as the ladder's fast paths. Each one carries an edge's conventional data through the move, which the general door's sections would re-mint. Folding them in is therefore a bit-moving change that this item would have to argue.

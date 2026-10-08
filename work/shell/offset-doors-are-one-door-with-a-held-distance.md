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

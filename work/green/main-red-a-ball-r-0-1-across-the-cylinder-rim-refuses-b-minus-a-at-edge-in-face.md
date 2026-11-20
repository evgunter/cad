---
id: main-red-a-ball-r-0-1-across-the-cylinder-rim-refuses-b-minus-a-at-edge-in-face
kind: issue
title: Main red: cylinder_sphere_frame's ball r 0.1 across the rim refuses B ∖ A at EdgeInFace (CarrierUnread)
status: open
opened: 2026-11-20
priority: P0
cost: M
---


Found by SECT's `sect/ball-cylinder-cost` lane (2026-11-20), running the
slow sphere × cylinder rows its diff touches.

## Measured

`cargo nextest run -p sweep --test all -E 'test(=cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim)'`
at opt-level 2 (debug assertions on), default ε, on main `cd469f2d`
(the lane's branch with `crates/geom-brep/src/intersect/section_bounds.rs`
put back to main's, so main's kernel): FAIL in 10.9 s, at
`builds_at_its_volume_or_refuses_at_its_door`'s
`assert!(off.is_empty())` (`crates/sweep/tests/cylinder_sphere_frame.rs`):

```
"the row's ball r 0.1 across the rim, B ∖ A: refused at None: EdgeInFace { edge: EdgeKey(16v1), face: FaceKey(4v5), gap: CarrierUnread }",
```

The row wants a body at the oracle's volume (`Ends::Builds`). The
refusal is the slit's `EdgeInFaceGap::CarrierUnread`
(`topo::boolean::slit`): an edge whose carrier is not certified off the
face's surface over its whole length. The same failure on the lane's
branch. Not bisected; the other eight `cylinder_sphere_frame` pose rows
pass on both.
`work/embed/the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause.md`
lists this row among those red in a mutation build "whether they are
red on a clean main was not checked": it is.

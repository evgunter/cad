---
id: walls-across-a-mirror-off-a-wall-do-not-prove-at-sym
kind: issue
title: A wall perpendicular to a mirror plane read off a lateral face meets its image unproven at Sym, where the same wall across a plane read off a cap proves
status: open
opened: 2026-10-23
priority: P2
cost: M
---

Found by INTENT's `a-symmetric-parts-seam-is-not-walked-as-one-construction` (2026-10-23), on the scenes of `crates/editor-core/tests/a_mirror_is_a_construction.rs`. The `Decided` section of that unit named this case: "perpendicular walls in the slid-plane scene that do not prove at `Sym`". The fixture fix in that unit makes the scene read its plane plainly. The walls still do not prove.

The scene is the L plate from that test file: `2 × 2` with a `1 × 1` notch, extruded `0.5`. It is mirrored across a plane read plainly off one of its own faces (`Plane { face }`, no slide, no flip), and unioned with its image. The union records a `SameOriented` `PlaneLadder` row for each face that is coplanar with its own image. Those are the faces perpendicular to the plane that the plane does not cut. The door's verdicts on those rows:

| mirror plane read off | perpendicular faces | verdict |
|---|---|---|
| the start cap (`z = 0`) | the six walls | all `Structural(CarrierVerdict)` |
| the `−x` wall (`x = 0`, a `Lateral`) | both caps | `Structural(CarrierVerdict)` |
| the `−x` wall (`x = 0`, a `Lateral`) | the `y = 0` and `y = 2` walls | `Unproven`, replay `Numeric`, recourse `Budget` |

The C of `a_prong_on_the_mirror_plane_by_value_alone_is_left_unproven` gives the same result. It is mirrored across its lower prong's end wall (`x = 0`). Its `y = 0`, `y = 1`, `y = 2` and `y = 3` walls are all `Unproven` `Numeric`, while its caps prove.

Each cell walks: rung 1 reads the source wall as `maps: []` and the image as `maps: [Mirror(m)]`, as decided. So the walk leaves these rows to the replay, which is right. The replay at `Sym` is what fails to show the image of a `y = c` wall, across an `x = 0` plane read off a lateral face, to be the wall itself. A cap's plane reflected the same way proves, and so does every wall reflected across a cap's plane. The difference lies in how the replay's carriers represent a lateral face's plane and its reflection, not in the walk. Measure which atom fails within the budget (`coincide::replay`'s carrier verdict and margin on the y-wall pair), and decide whether it is a budget or a representation gap.

The rows are not asserted in the test file: their verdict is what this item would change.

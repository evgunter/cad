---
id: a-crescent-between-aligned-faces-is-read-as-a-touch
kind: issue
title: A tangent crescent between two aligned plane faces is read as a touch; read as an overlap it builds the two-arc form's every op but moves the one-segment union to SingleSiteSectionLoop
status: open
opened: 2026-11-02
priority: P2
cost: M
---



Found by the lane that built `a-union-across-a-tangent-crescent-refuses-unbuilt`
(`tangrim/tangent-crescent`). Pinned by
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose` (its `aligned` rows).

## The pose

The boss `r = 0.3`, `z ∈ [0, 1]`, beside a plate `z ∈ [0, 0.5]` holed by
`r = 0.1` about `(0.2, 0)`: the two floors lie on each other with
aligned normals over a crescent narrowing to `(0.3, 0, 0)`, the same
crescent the opposed pose glues. `boolean/sectors.rs` `crescent_records`
records a crescent as a coplanar overlap only between opposed faces
(`direction_sense`), so here the floor pair stays `touching`. On main
the unions build `SOUND` in the one-segment form (the join reads its
section loops at the seam, `join.rs` `seam_germ_side`), ∩ refuses
`MixedSeam` and the differences `EdgeUndescribed`; the two-arc form
refuses `Join(UnpairedLooseEnds { count: 4 })` in every op.

## Measured with the crescent read in either orientation

(2026-11-02, the branch with `crescent_records`' `direction_sense` gate
removed, default ε, every op in both orders.)

| form | ∪ | ∩ | ∖ |
|---|---|---|---|
| two arcs | `SOUND` (was `UnpairedLooseEnds { 4 }`) | `SOUND` (was the same) | `SOUND` (was the same) |
| one segment | `SingleSiteSectionLoop { count: 1 }` (was `SOUND`) | `SingleSiteSectionLoop { count: 1 }` (was `MixedSeam`) | `SOUND` (was `EdgeUndescribed`) |

So reading it builds ten cells and moves the one-segment union from
built to refused: one one-site record is left for `join.rs`'s
`SingleSiteSectionLoop` refusal (`one_site_segments`). Which record,
and why the join takes it with the crescent unread, is unmeasured (the
two walls are also tangent along the ruling through the touch, from
`z = 0` to `0.5`). Owed: the join taking that one-site loop with the
crescent read, then dropping the `direction_sense` gate.

---
id: props-measures-a-cone-loop-pinched-at-its-apex
kind: issue
title: A cone loop pinched at its apex, two lobes meeting only there, is a valid face the props closed form refuses
status: open
opened: 2026-11-01
priority: P2
cost: M
---

Filed by SPLITDOOR (`splitdoor/cone-apex-boolean`, PR 6266), on CONEARM's slate because `crates/geom-brep/src/props/curved.rs`'s cone arm is CONEARM's.

## What

A Boolean whose planar face passes through a cone's apex can now cut the
cone along two section rulings. Where the cone face's one corner at the
apex is its whole turn round it (a cone whose apex is reached by one wrap
edge, `crates/sweep/tests/common/wrap_tip.rs`), the cut leaves one face
whose outer loop is two lobes meeting only at the apex vertex, any notch
nested as a ring. The body is well formed and its signed area is the
lobes' sum, but `require_cone_loop_vertices_distinct`
(`crates/geom-brep/src/props/curved.rs:2665`) refuses any loop that
passes one vertex twice (`props_cone_loop_vertex`): the apex has no
azimuth to tell a seam's two copies apart, and the doc reads a twice-met
apex as a pinch. The finished-body gate then refused the result
`ResultInvalid { VolumeUncomputable { ConeLoop { props_cone_loop_vertex } } }`.

Witness: the one-wrap-edge cone of height 1, half-angle 15°, 30° or 45°,
apex `(0, 1, 0)`, against the box `[-0.3, 0] × [0.95, 1.2] × [-0.3, 0.3]`,
c ∪ b, b ∪ c and c ∖ b, ε 1e-9. The Boolean now refuses it by name before
the gate, `BooleanError::ResultPinchedAtConeApex`
(`crates/topo/src/boolean/ops.rs`, `pinched_at_cone_apex`), pinned in
`crates/sweep/tests/a_cone_apex_on_a_plane_through_it.rs`
(`a_cut_along_two_rulings_through_a_wrap_tipped_apex_refuses_the_pinch_by_name`).
The same pose on a cone revolved with its seam (`sweep::revolve`) builds:
the seam splits the lobes into two faces.

## Where to look

Whether the cone loop can be measured lobe by lobe: split the walk at a
twice-met vertex decided the apex, wind each lobe on its own, and sum
them, the apex being where every ruling meets and so a legal pinch of one
face's boundary. Once props measures it, retire `pinched_at_cone_apex`
and turn the pin into a build.

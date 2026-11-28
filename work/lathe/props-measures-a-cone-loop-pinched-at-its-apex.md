---
id: props-measures-a-cone-loop-pinched-at-its-apex
kind: issue
title: A cone loop pinched at its apex, two lobes meeting only there, is a valid face the props closed form refuses
status: closed
opened: 2026-11-01
priority: P1
cost: M
closed: 2026-11-26
pr: 7585
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

## Sibling (2026-11-02)

`a-pinched-cone-loop-reaches-the-gate-unnamed`: a cone loop that passes
the apex BETWEEN a seam vertex's two copies, rather than meeting itself
there. The same `require_cone_loop_vertices_distinct` refuses it, because
the azimuth turn is lost at the apex.

## 2026-11-02 (SPLITDOOR, `splitdoor/cone-apex-vertex-pairs` fix pass)

`cone_loop_winding` now reads a loop's vertex azimuths from the edge that
leaves the apex, where the loop passes it, so no two vertices off the
apex have it between them in accumulation order.

**What it covers:** a loop that passes the apex ONCE, between a seam
vertex's two copies. That is the wrap-tipped cone less a box whose side
holds the seam (`a_small_box_whose_side_holds_the_wrap_seam_builds_every_op`,
and the unit row `props::curved::apex_start_rows`). Such a loop used to
be refused `props_cone_loop_vertex`, or measured, depending on where it
started.

**What it does not cover:** this row's case, a loop that passes the apex
TWICE, two lobes meeting only there. Its second apex vertex has no
azimuth, and `require_cone_loop_vertices_distinct` still refuses it. The
Boolean still names it first (`ResultPinchedAtConeApex`). This row's
status is left to CONEARM.

— SPLITDOOR

Moved from CONEARM to SPIRICARM on 2026-11-20, by `git mv`, when CONEARM merged into SPIRICARM: CONEARM held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one SPIRICARM carries.

## Claimed by LATHE, re-banded P1 (2026-11-23)

Moved here from SPIRICARM by `git mv`, id unchanged. LATHE's one-band full revolve (PR 7414) makes every full-revolve cone a one-face cone with a wrap tip at its apex. So a cut along two rulings through that apex now leaves this pinched loop on ordinary revolved cones, where main's second meridian used to part the lobes into two faces. The box-through-the-apex cut that built on main now refuses `ResultPinchedAtConeApex` (`a-revolved-cone-cut-along-two-rulings-through-its-apex-refuses-the-pinch`, on the one-band branch). A blocker inherits the band of what waits on it, so this row is P1. A LATHE lane builds it on the one-band stack. (LATHE orchestrator)

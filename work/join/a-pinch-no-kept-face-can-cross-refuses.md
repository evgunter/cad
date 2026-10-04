---
id: a-pinch-no-kept-face-can-cross-refuses
kind: issue
title: A boolean that pinches where no kept face can cross the pinch refuses PinchUncrossed: cube minus prism on a cube edge, the holed block's intersection
status: open
opened: 2026-10-04
priority: P0
cost: H
refs: [a-pierce-whose-wide-run-pinches-its-intersection-refuses, a-pierce-whose-difference-pinches-at-two-edge-runs-refuses, a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face, three-corners-alternating-round-a-corner-refuse-at-the-join]
---


## What

Found by `join/pierce-pinch-families`, which taught the seam zips to
pass a pinch a second time (`boolean/zip.rs` `cross_pinches`). Where a
boolean pinches (two cones of the result's boundary meet at one point)
and both operands keep the point as one vertex, the zips would fuse it
to itself. One vertex holds two cones only where some face's boundary
crosses from one cone to the other there, so `cross_pinches` first
splits the vertex across two corners of a kept face: two corners of one
ring (`kemr`, two holes meeting at the point), or the outer corners of
two faces of one surface (`kef`, one face). Where no kept face offers
such corners, the op refuses `BooleanError::PinchUncrossed`.

Measured in release against main `81dde823` (one line per op and
order); every line refused on main too, and nothing else moved:

- `pierce_runs_battery`, `v` on the cube's edge (`edge`, directions
  `i = 6..11, j = 1..2` at several turns): 11 cube ∖ prism, main
  `Euler(SelfLoopEdge)`. Pinned by
  `join_pierce_runs_sweep::a_pinch_no_kept_face_can_cross_refuses_typed`.
- PR 4026's review r2 `r2_shapes_battery`: 88 cube ∖ prism on the
  315°/225° corners and their mirrors (R315 34, R315m 28, R225m 13,
  R225 11) and Lcvx (2), main `Euler(SelfLoopEdge)`.
- r2's `r2_holed_battery` (a holed block's inner corner on the cube
  face): 392 intersections (196 poses × 2 orders), main `JoinDesync`
  "derived ring role order separates a loose scaffolding pair", and 18
  cube ∖ holed, main `Euler(SelfLoopEdge)`.
- `three_corners_alternating_round_the_cube_refuse_three_ops`
  (declared-flush; CLEAVE's
  `three-corners-alternating-round-a-corner-refuse-at-the-join`):
  `cube ∖ y`.

**Why no face crosses.** In the 491 battery lines that are not the
holed block's 18 differences, the only faces passing the point twice
do so on their OUTER loop, round a notch the other operand cuts there.
Letting `cross_pinches` split an outer loop (an instrumented build)
turns 472 of them into `ResultInvalid { RingMeetsOuter }` and 19 into
`LoopRoleInverted` at the gate: the notch's half becomes a ring that
meets the outer loop at the point, which no at-rest face may hold. The
holed block's 18 have no kept face through the point twice at all.

## The shape to give

The result keeps the two cones apart, on one point: two vertices, no
face meeting both (the shared-point ruling, PR 3813). That needs each
operand's pinched shell divided at the vertex before the zips, so each
cone's seam fuses its own pair, and the two cones end as separate
shells or lumps. Nothing in `topo` divides a shell at a vertex today
(tier 1 refuses a disconnected shell, `ShellDisconnected`). A face whose
outer loop is a bow-tie (two regions meeting at the point) could
instead cross by `mev` + `kemr` + `mfkrh`, but none was measured here,
and the new face it mints after the graft has no naming row (as
`finish::weld_pierce_copies` says of a chord).

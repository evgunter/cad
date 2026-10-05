---
id: a-pinch-no-kept-face-can-cross-refuses
kind: issue
title: A boolean whose pinch only an outer loop could cross, or none, refuses PinchUncrossed (cube minus a reflex corner on a cube edge or face, the holed block's intersection, some unions)
status: open
opened: 2026-10-04
priority: P0
cost: H
refs: [a-pierce-whose-wide-run-pinches-its-intersection-refuses, a-pierce-whose-difference-pinches-at-two-edge-runs-refuses, a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face, three-corners-alternating-round-a-corner-refuse-at-the-join]
---


## What

Found by `join/pierce-pinch-families` (PR 4038), which taught the seam
zips to pass a pinch a second time (`boolean/zip.rs` `cross_pinches`).

**The class.** A boolean pinches where two cones of the result's
boundary meet at one point. Where both operands keep the point as one
vertex, the zips would fuse it to itself. One vertex holds two cones
only where a face's boundary crosses from one cone to the other there,
so `cross_pinches` first splits the vertex across two corners of kept
faces: two corners of one ring (`kemr`, two holes meeting at the
point), or the outer corners of two faces of one surface and sense
(`kef`, one face). Where neither offers, the op refuses
`BooleanError::PinchUncrossed`. Every such line refused on main too
(`Euler(SelfLoopEdge)`, or the join's "derived ring role order" where
the old intersection rule faced the struts).

**Three sub-families**, told apart by an instrumented build that lets
an outer loop cross (`mev` + `kemr` on it). That build's gate verdicts
are the measurement; none of the sub-families' cures is built or
measured.

- **Nested**: the only face through the point twice passes it on its
  outer loop, round a notch the other operand cuts there (a hole
  touching the outer boundary). Crossing leaves a ring meeting the
  outer loop at the point: `ResultInvalid { RingMeetsOuter }`. PR 4038's
  batteries 472 lines, its review r1 44, review r2 26.
- **Bow-tie**: the face's outer loop bounds two regions meeting at the
  point. Crossing leaves a ring that is not a hole:
  `ResultInvalid { LoopRoleInverted }`. PR 4038's batteries 19, r1 7,
  r2 184.
- **None twice**: no kept face passes the point twice at all; the
  outer-loop build changes nothing. The holed block's 18 cube ∖ holed
  lines.

**Reach.** Release, base vs head, one line per op and order:

- PR 4038's own batteries, against main `81dde823`:
  - `pierce_runs_battery`, `v` on the cube's edge: 11 cube ∖ prism.
    Pinned by `join_pierce_runs_sweep::a_pinch_no_kept_face_can_cross_refuses_typed`.
  - PR 4026's review r2 `r2_shapes_battery`: 88 cube ∖ prism (R315 34,
    R315m 28, R225m 13, R225 11, Lcvx 2).
  - r2's `r2_holed_battery`: 392 intersections (196 poses × 2 orders),
    and the 18 cube ∖ holed.
- PR 4038's review r2 (`crates/sweep/examples/r2_pinch_probes.rs` on
  `join/pierce-pinch-families-review-r2`, base `8793177b`), 210 lines on
  its cube set:
  - cube ∖ prism with `v` on a cube edge, every corner (shallow200 29,
    Lbot 27, notchbot 26, notch307 26, vee224bot 25, asym 20, Ltop 18,
    vee300 10);
  - **face placements**, cube ∖ prism: vee300 `fib62 65 67 73 75 86 88
    94 96 107 109`, asym `fib62 65 70 78 83 91 99 104 112 117`, vee224bot
    `fib23 28 44 49 57`;
  - **unions**: `vee300 fib100 corner psi=0.3` (pc ∪ and cp ∪),
    `notch307 fib103 edge psi=1.9` (pc ∪).
  - Near-tangent tilts (`nt` set): 182 lines, all cube ∖ prism except
    vee300's 3 pc ∪ and 3 cp ∪.
  - A cylinder's side as the pierced face (`cyl` set): 35 cube ∖ prism.
- PR 4038's review r1 (`crates/sweep/examples/r1b_pinch_probes.rs` on
  `join/pierce-pinch-families-review-r1`): 51 lines on turned L corners,
  a 327° notch and a U's inner corner, and 43 of the staircase `u2`
  ops, which cross their first pinch and refuse at the second.
- CLEAVE's `three-corners-alternating-round-a-corner-refuse-at-the-join`:
  `cube ∖ y`, declared-flush.

**On a curved wall the crossing itself reaches a volume limit.** On
review r2's cylinder, 76 cube ∖ prism lines that main refused
`Euler(SelfLoopEdge)` now cross on one ring of the cylinder's wall and
refuse at the gate `ResultInvalid { VolumeUncomputable { RingOnCurvedFace } }`
(Ltop 19, Lbot 16, vee300 14, notch307 7, asym 6, notchbot 6,
shallow200 4, vee224bot 4; e.g. `Ltop cyl fib33 psi=0.9 off cp S`). That
is FLUX's `an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane`,
not this row; it is listed because the pre-pass made the crossing.

## The shape to give

Per sub-family; nothing below is measured.

- **Bow-tie** (*likely*, review r2 m1): the body is one vertex whose
  face's outer loop meets itself at the point, the shape the `kef`
  crossing already ships. What is missing is an Euler sequence that
  crosses the corners without leaving a ring: a splice of the face's
  two loops back into one at the shared vertex, or `mfkrh` of the
  region-ring into its own face with a naming row for a face divided
  after the graft.
- **Nested**: a ring may not meet the outer loop at rest, and the face
  passes the point from both cones. Whether the right body is two
  vertices on one point (the shared-point ruling, which then needs that
  face to meet both, review r1 S6) or a divided face is open.
- **None twice**: the cones are kept apart on one point, two vertices
  with no face meeting both, which needs each operand's pinched shell
  divided at the vertex before the zips (nothing in `topo` divides a
  shell at a vertex; tier 1 refuses a disconnected shell).

## After PR 4036 (branch `join/reflex-corner-vertex-vertex`, merged with PR 4038)

The `pierce_runs_battery` witnesses no longer reach this class. All 11
cube ∖ prism lines with `v` on the cube's edge are four-germ vertex
pairs. PR 4036 starts their pairing where A's runs lie on the side the
op keeps of A, so each kept run of A is a copy of its own and the
result needs no crossing at `v`. On the merged head, every one of the
battery's 4 536 runs is `SOUND` or rightly empty.

The pin moves with that: it is now
`join_pierce_runs_sweep::a_four_germ_pinch_the_pairing_start_avoids_builds_every_op`,
a build in every op. Under mutant M4 (start at A's first germ) it goes
red.

Not re-run on that head: the other witnesses above, which live on
review branches. They are r2's `r2_shapes_battery` (88) and
`r2_holed_battery` (392 + 18), and PR 4038's review probes. Their edge
and corner placements are four-germ vertex pairs too, and may move the
same way. `PinchUncrossed` itself is untouched and needs a committed
witness off this lane.

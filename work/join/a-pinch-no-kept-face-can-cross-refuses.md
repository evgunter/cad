---
id: a-pinch-no-kept-face-can-cross-refuses
kind: issue
title: A boolean whose pinch only a face's outer loop could cross refuses PinchUncrossed (cube minus a reflex corner on a cube face, the holed block's intersection, a staircase's second pinch)
status: open
opened: 2026-10-04
priority: P0
cost: H
design: true
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
faces (`split_across`): two corners of one ring (`kemr`, two holes
meeting at the point), or the corners of two faces of one surface and
sense, one of them ringless (`kef`, one face). Where neither offers,
the op refuses `BooleanError::PinchUncrossed`.

**What is left is one shape.** On every remaining line the only face
through the point twice passes it on its outer loop, round a notch the
other operand cuts there: a hole touching the outer boundary at the
point (the row's old "nested" sub-family). Crossing that loop (`mev` +
`kemr` on it, an instrumented build) splits it into an outer loop and
a ring that meet at the point, and the gate refuses
`ResultInvalid { RingMeetsOuter }`.

**There is no bow-tie sub-family.** The lines the old row called
bow-tie (`LoopRoleInverted` under that build) are the same nested
shape: `kemr` left the outer role on the hole-shaped half, so both
loops read inverted, and the list goes on to `RingMeetsOuter` at the
point (`outcome` cuts error text at 110 characters, which hid it).
Measured on every such line: main's 49 (staircase 43, near-tangent 4,
notch327 1, see below), and review r2's 184 on main before PR 4036
(`cced486c`, its cube set: 184/184 carry both inversions and the
meeting). A true bow-tie, one face's outer loop passing the point twice
round two separate regions, cannot reach the pre-pass: its two corners
at the point would each join one region's edges, so the face's
boundary there is two loops already, not one.

**The old "none twice" lines were an island.** The holed block's 18
cube ∖ holed lines (and 5 on review r2's cylinder): the plane's section
closes round the hole, so the cube's plane keeps an island face inside
its near face's hole, touching that hole's ring at the point, and one
seam meets the point twice. The island's outer corner and the ring's
corner, one surface and sense, now cross by `kef` (see Built).

**Reach on main `4cf3f8b9`** (release; every line `cube ∖ ·` unless
said), 590 `PinchUncrossed` lines before this lane, 567 after:

| battery | nested | read as bow-tie | island |
|---|---|---|---|
| r2 cube, `v` on a face (vee300 11, asym 10, vee224bot 5) | 26 | 0 | 0 |
| r2 near-tangent, on a face | 24 | 4 | 0 |
| r2 cylinder wall | 30 | 0 | 5 → `RingOnCurvedFace` |
| r1 cube (Uin 30, notch327 18) | 47 | 1 | 0 |
| r1 `u2` staircase, second pinch | 0 | 43 | 0 |
| holed block (∩ both orders; cube ∖ block) | 392 | 0 | 18 → `SOUND` |
| `pierce_runs_battery` | 0 | 0 | 0 |

Batteries: `crates/sweep/examples/r2_pinch_probes.rs` on
`join/pierce-pinch-families-review-r2`,
`crates/sweep/examples/r1b_pinch_probes.rs` on
`join/pierce-pinch-families-review-r1`, and `r2_holed_battery` in
`crates/sweep/tests/join_pierce_r2_probes.rs` on
`join/pierce-two-out-runs-review-r2`. Edge and corner placements left
the class with PR 4036.

**Pinned**, each asserting the refusal and every other run `SOUND`:
`join_pierce_runs_sweep::a_pinch_round_a_notch_on_a_faces_outer_loop_refuses_typed`
(`vee300 fib62 face`) and
`join_pierce_runs_sweep::a_staircases_second_pinch_round_a_notch_refuses_typed`
(`u2 S_tt b594`, the one read as bow-tie).

## The shape to give

**This is a design fork.** The cones at the point are really two:
gluing the two seams at one vertex, the second fusion splits that
vertex's orbit in two whatever the order (the cone link is two
circles), and the notched face lies in both cones. The current
invariants admit none of the bodies that would hold that:

1. **One vertex, the face crossed:** an outer loop and a ring meeting
   at the point. The gate refuses this (`RingMeetsOuter`, check 9), and
   a ring touching its outer loop states no hole.
2. **One vertex, the face's one loop through it twice** (r2's
   proposal, the shape a `kef` crossing ships). Measured: splicing the
   crossed face's two loops back into one at the shared vertex
   (pointer surgery, instrumented) leaves `Merge(InputNotClosed {
   SplitVertexOrbit })` on all 43 staircase lines. A splice that joins
   two loops of one face changes `V − E + F − R` by one, while every
   Euler operator changes it by 0 or ±2, so no composition of them
   makes it.
3. **Two vertices on the point, the face through both.** The
   shared-point ruling (PR 3813) keeps copies apart only where no face
   meets both (`a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point`).
4. **Two vertices on the point, the face divided** by a new edge so
   that each part meets one vertex. That edge runs across a planar face
   for topology alone, and where it goes has to be chosen.

Each needs a change to an at-rest invariant, or a new kind of edge, so
it goes to the designers before a lane builds it.

## Built (branch `join/pinch-uncrossed-residue`)

`zip::split_across`'s two-face crossing no longer asks both corners to
be outer. It asks that one of the two faces be ringless, and the
ringless one dies into the other (`kef` cannot kill a face holding a
ring). That covers an island face pinched to its hole's ring:
- holed block 18 cube ∖ block lines go to `SOUND`;
- review r2's 5 cylinder lines cross, then refuse at FLUX's volume
  lane (`VolumeUncomputable { RingOnCurvedFace }`), refusal to refusal.

Pinned by
`join_pierce_runs_sweep::an_island_face_pinched_to_its_holes_ring_crosses_and_builds`.

---
id: a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale
kind: issue
title: A bar through a ball refuses Escalated(bool_join_nearest) at ×1e-3 and RingOffCylinderChart at ×1 and ×1e3: the nearest-facing rank reads chord length in absolute metres
status: open
opened: 2026-10-03
---


Found in the dual review of PR 3985 (`reach/arc-from-pairing`, NOTE N2
of the second lane, by execution), filed by that PR's fix pass: the
ranking is pre-existing, newly reached because the PR lets these
pierces past the chord's arc.

## What

The second reviewer's bar-through-ball pierces
(`analysis/reach-dual/3985-r2`, `probes/review_reach_dual3985_r2_probes.rs`:
bars and L-notches through the unit ball, off its axes), scaled about
the origin:

- at ×1e-3 they refuse `Escalated(bool_join_nearest)`, margin
  −9.7e-9;
- at ×1 and ×1e3 the same poses refuse `RingOffCylinderChart { Sphere }`
  (`work/tang/a-ring-on-a-sphere-face-has-no-island-winding.md`).

(The shipped bar, `crates/sweep/tests/snowman.rs:624`, builds at ×1.)

`find_match` orders the pairs its germs have each ranked, half-turn
first and then by chord length, `decide("bool_join_nearest",
Margin::of(cand.chord − best.chord))` (`nearer`,
`crates/topo/src/boolean/join.rs:977`): a difference of two lengths in metres, unlevered. Two candidates
whose chords differ by a fixed fraction of the body's size tie within
the band once the body is small enough, so the door a pose stops at
depends on its scale.

The refusals above were measured with the chord ranking that JOIN's
PR 4008 replaced (half-turn, then the turn along the conic); the
scale dependence of the pair ORDER remains, the poses are not
re-measured.

## Done when

A pose and its scaled copies stop at the same door (or build alike),
whether by ranking only partners the walk leaves adjacent — where no
tie can arise — or by a margin whose comparand is not an absolute
length; pinned by a row at three scales.

## 2026-10-07 — the ×1 and ×1e3 door moved (TANG, PR 4211)

A ring on a sphere face now winds its island and re-homes its rings
without a chart (`chord_join::path_island_winding`,
`chord_join::path_ring_side`). `RingOffCylinderChart` is renamed
`RingIslandUnread`, and a sphere reaches it only where a ring run
reaches both sides of its section plane or is bounded by an edge that
is not a circle. A bar through the unit ball at ×1, poles turned off
every axis (`crates/sweep/tests/a_ring_on_a_sphere_face.rs`,
`a_ring_beside_an_outer_loop_on_the_run_is_read_from_an_edge_midpoint`),
builds ∩ in both orders and bar ∖ ball (two lumps) at the slice integral.
∪ in both orders and ball ∖ bar keep the ring as a hole of the ball's
face and refuse at the result gate
(`work/flux/sphere-face-with-a-hole-has-no-closed-form.md`). The 3985
reviewer's own poses are not re-measured here, and nor is ×1e-3.

## 2026-10-07 — a run on both sides of its section plane reads (TANG)

The sphere and cone ring lanes share one reading now: a path from an
outer-loop point to the closing chord's midpoint, whose crossings and
arrival side wind the island (`chord_join::path_island_winding`). It
asks nothing of the section plane, so a run reaching both sides of it no
longer refuses. On a sphere, `RingIslandUnread` is now reached only by a
run edge that is not a circle.

## 2026-10-09 — two parallel walls a dozen bands apart tie the pair order (JOIN)

Found by the steep-ellipse travel row
(`crates/sweep/tests/band_apart_partners_on_a_steep_ellipse.rs`):
a block whose top face has a notch and a finger `12ε` wide across a
`k = 10` cylinder section (24ε in the committed row). The notch's two
walls are parallel planes `12ε` apart, and each carries one section arc
of chord 0.4767. The two chords differ by `1.65ε`, so `find_match` (`nearer`,
`bool_join_nearest`) escalates choosing which of the two pairs joins
first, in every op and both orders, at ε 1e-9, 1e-6 and 1e-12 alike
(margin −1.652e-9 at 1e-9; the pose is sized in bands). Neither pair is
the other's alternative: they are on different faces, so the order
between them is not a partner choice. This is the same class as the
scale-dependent door above, reached at a fixed scale: any two parallel
features a few bands apart carry sections whose chords differ by a
small fraction of their separation.

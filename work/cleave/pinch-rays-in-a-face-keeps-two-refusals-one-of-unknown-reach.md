---
id: pinch-rays-in-a-face-keeps-two-refusals-one-of-unknown-reach
kind: issue
title: PinchRaysInAFace keeps two refusals: three germs on one ray, and a join arm no case reaches
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [two-lumps-touching-along-an-edge-refuse-pinch-rays-against-a-flush-partner]
---

Found by PR 6170's review, after PR 6170 built the two-lumps cells.

## What

`BooleanError::PinchRaysInAFace` still has two raising sites:

- `boolean/vtxfac.rs` `germ_order` (the `after` closure, ~`:2534`):
  a third germ of a different pierce on a ray that already holds a tie,
  three cones touching along one ray in the pierced face. Only the unit
  test `germs_of_two_pierces_on_one_ray_nest_and_a_third_refuses`
  reaches it, on germs built by hand; no public-door scene is known to.
- `boolean/join.rs` `pinch_ray_refusal` (~`:5160`): a ring run of no
  area that passes a tied ring's point where `join::tie_winding` read no
  order, neither end of the run facing a tied germ, or its two ends
  reading two ties that disagree. Its reachability is unknown: no test
  reaches it since PR 6170, and the PR's review saw nothing refuse there
  across about 3,400 cells of two lumps touching along an edge against
  a flush partner (mirrored, flipped, diagonal, unequal and partial-line
  lumps, two-sided partners).

## Owed

For the first, a scene through the public doors (three lumps whose
corners pierce one face at one point, touching along one ray in it) and
the order the material decides for it, or the refusal pinned there. For
the second, a scene that reaches it, or an argument that none can, and
then the arm retired.


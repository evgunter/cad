---
id: a-seam-region-whose-boundary-carries-both-poles-is-unread
kind: issue
title: A seam region of two or more sphere faces whose boundary carries both poles encodes no side, so flipping all its faces passes tier 3
status: open
opened: 2026-10-11
priority: P2
cost: M
refs: [a-pair-of-seam-siblings-flipped-together-is-silent]
---

## What is unread

Tier 3's check 6 reads a sphere face whose own loop holds both chart
poles through its seam region: the faces of its surface it reaches
across shared edges (`crates/topo/src/validate.rs`, `seam_region`;
`crates/geom-brep/src/props/curved.rs`, `sphere_region_material_sign`).
A one-loop region is read only at a pole on an edge interior to it.
A region whose boundary carries both poles is never read that way,
since its complement has the same boundary and holds no pole either.

For a region of two or more faces, flipping every face is a real
defect. Each flipped face claims its own complement, which holds the
other members, so the faces overlap. Nothing refuses it, and the
volume is wrong.

## Witnesses (public operations)

Both are pinned in `crates/sweep/tests/a_seam_region_carrying_both_poles.rs`,
which goes red when this row's owner lands. Found by the review of PR
4746.

- **A, every ε** (`two_faces_whose_region_carries_both_poles_flip_silently`).
  A y-poled unit ball unioned with the brick `[-3, 0] × [-2, 2]²` keeps
  one 180° lune. `mef` splits it along the meridian through `(1, 0, 0)`.
  Flipping both halves passes tier 3 and measures `48 − 2π = 41.7168`
  against `48 + 2π/3 = 50.0944`.
- **B, in-band, ε = 1e-12** (`a_dimple_beside_its_polar_axis_flips_by_the_run_band`).
  The brick less a ball poled along `(0, 0.6, 0.8)`, whose face plane
  `x = 1e-11` lies beside the polar axis. At ε = 1e-12 the dimple is two
  faces of one sphere, and its boundary carries both poles within the
  band. Flipping both measures `48 + 2π = 54.2832` against
  `48 − 2π/3 = 45.9056`. The review saw the same at planes 1e-11 to 1e-8
  off the axis. At a coarser ε the dimple is one lune, and its flip is
  the boss, a valid body.

## A reading that may close it

A pole-free "tiling" reading takes the side under which the members'
Gauss–Bonnet areas sum to the region's. For members with rings it is
AMBIGUOUS, not unsound: signed Gauss–Bonnet area is additive, so the
identity can hold under both readings, and it then decides nothing.
Over the PR 4573 flip harness a prototype left 168 region reads
undecided, and every one inspected had a ringed member
(`flux/seam-siblings-prototype`, 2026-10-11).

Restricted to one-loop members and a non-empty boundary, the wrong
reading misses the identity by `4π(n − 1)R²`, a wide margin. That may
cover both witnesses, which are one-loop pairs. It is unbuilt.

## Note: a lone face is not this

A region of ONE face whose loop carries both poles flips into another
valid body. A ball unioned with a brick whose face plane holds the
ball's polar axis keeps one 180° lune. Its flip is the brick with a
hemispherical dimple, with the same faces, loops and surfaces. Tier 3
accepts it and `mass_properties` measures the dimpled brick
(`crates/sweep/tests/seam_siblings_flipped_together.rs`,
`a_lone_lune_through_both_poles_flips_into_another_valid_body`). That
row exists so no one "fixes" it into a refusal.

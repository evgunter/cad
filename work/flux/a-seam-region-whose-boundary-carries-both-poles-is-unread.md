---
id: a-seam-region-whose-boundary-carries-both-poles-is-unread
kind: issue
title: A seam region of two or more sphere faces whose boundary carries both poles encodes no side, so flipping all its faces passes tier 3
status: open
opened: 2026-10-11
priority: P3
cost: M
refs: [a-pair-of-seam-siblings-flipped-together-is-silent]
---

## What is unread

Tier 3's check 6 reads a sphere face whose own loop holds both chart
poles through its seam region: the faces of its surface it reaches
across shared edges (`crates/topo/src/validate.rs`, `seam_region`;
`crates/geom-brep/src/props/curved.rs`, `sphere_region_material_sign`).
A region whose boundary carries both poles answers `Unencoded`, as its
complement has the same boundary and holds no pole either.

For a region of two or more faces, flipping every face is a real
defect. Each flipped face claims its own complement, which holds the
other members, so the faces overlap on the sphere. Nothing refuses it.
No witness has been built. A 180° lune split along an interior
meridian into two faces, meeting other faces transversally along a
great circle through the poles, would be one.

## A reading that does not work, and one that may

The pole-free "tiling" reading takes the side under which the members'
Gauss–Bonnet areas sum to the region's. It is unsound for members with
rings. Signed Gauss–Bonnet area is additive, so with ringed members the
identity can hold under BOTH readings. Over the PR 4573 flip harness
it left 168 region reads undecided. Every one inspected had a ringed
member and a gap of zero under both readings (measured on a prototype,
`flux/seam-siblings-prototype`, 2026-10-11).

Restricted to one-loop members and a non-empty boundary, the wrong
reading misses the identity by `4π(n − 1)R²`, a wide margin. That
restriction may cover this residual. It is unbuilt.

## Note: a lone face is not this

A region of ONE face whose loop carries both poles flips into another
valid body. A ball unioned with a brick whose face plane holds the
ball's polar axis keeps one 180° lune. Its flip is the brick with a
hemispherical dimple, with the same faces, loops and surfaces. Tier 3
accepts it and `mass_properties` measures the dimpled brick
(`crates/sweep/tests/seam_siblings_flipped_together.rs`,
`a_lone_lune_through_both_poles_flips_into_another_valid_body`). That
row exists so no one "fixes" it into a refusal.


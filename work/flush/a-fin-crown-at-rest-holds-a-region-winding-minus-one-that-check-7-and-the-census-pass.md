---
id: a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass
kind: issue
title: The fin crown at rest holds a region that winds −1, and check 7 and the census pass it; which of its unions refuses is face order's choice
status: open
opened: 2026-10-14
priority: P1
cost: M
---


Found by FLUSH's shell-role reader unit (`flush/shell-role-reader`, PR
5119), at ε 1e-9. Re-aimed by that PR's review R1: the region is not
made by a join.

## Measured

**The crown alone.** `test_support_meeting::posed_crown(fin())` (two
faces folded 1e-4° apart, `crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`'s
fixture) passes the at-rest gate on main and on the branch, at every
pose. An independent generalised winding number over its planar faces
(review R1's `r1_fin_winding_probe.rs`, branch
`flush/shell-role-review-r1` at `0731fe3f7`: each face's loops as a fan
of signed solid angles) reads points of winding −1 near the mouth of
the fin's slit. A closed boundary whose winding takes −1 is no solid:
somewhere it is wound inside out.

**Its unions.** Every union of the crown with the probe pyramids 1, 2
and 4 (bearings 130°, 130° and 250°), in both orders, carries the same
points. On the branch:
- `x ∪ y` of probes 2 and 4 refuses at three poses of four (at rest,
  turned about z, turned) as `ResultInvalid { NegativeVolume }`;
- `y ∪ x` of both at every pose, probe 1 in both orders, and the
  flipped pose's `x ∪ y` build and pass tier 3 holding the region.

Check 7 reads a solid's side off the first aimed ray that decides, in
face order. The ray that finds the region (probe 2 at rest, aimed at 60°
at the fin face `1v3`) meets, in order of advance:

| face | advance | outward sign |
|---|---|---|
| 15v1 | 1.367276727 | exits |
| 7v3 | 1.367277191 | enters |
| 1v3 (aimed) | 1.367284501 | enters |
| 4v3 | 1.367284967 | exits |
| 8v3 | 1.367399225 | enters |
| 12v1 | 1.509149585 | exits |

Its start lies outside a ball holding every face, so it winds 0, and the
winding along it runs 0, −1, 0, 1, 0, 1, 0. Between faces 15v1 and 7v3,
4.6e-7 apart along the ray (≈ 460ε, definite), it is −1. Where face order
picks another ray first, that ray reads `Out` and the body passes. The
volume cannot see the region at all, being a sum, and the census does
not read windings.

`crates/topo/tests/a_vertex_read_again_classes_every_edge.rs` pins both
halves: `KNOWN_INSIDE_OUT` the cells that refuse, `KNOWN_WRONG` the
scenes whose builds hold the region.

## What to find

Which step builds the crown with faces wound against each other at the
slit's mouth, and whether the fixture means to hold that region at all.

A candidate remedy at the check, for this row and not the PR that found
it: a cross-check between readings, so that two aimed rays that decide
opposite sides refuse rather than whichever comes first answering.

---
id: a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass
kind: issue
title: The fin crown at rest holds a region that winds −1, and check 7 and the census pass it; which of its unions refuses is face order's choice
status: review
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
- `x ∪ y` of probes 2 and 4 refuses at three poses of five (at rest,
  turned about z, turned) as `ResultInvalid { NegativeVolume }`;
- `y ∪ x` of both at every pose, probe 1 in both orders, and the
  two flipped poses' `x ∪ y` build and pass tier 3 holding the region.

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

## Measured on main (`5d0e2023`)

**The fixture builds a non-solid; no kernel step makes the region.**
The crown's triangles, from `fin()` and the bottom `(0, 0.2, −0.6)`,
cross each other: an exact rational segment–triangle test finds the
bottom fan's face on the fin's long edge `c0 c1` (`c1 c0 low`) crossing
the fin's far face (`c1 c2 MEET`) and the top face beyond it
(`c2 c3 MEET`), and the face on `c1 c2` crossing `c2 c3 MEET` too.
Seen from above, the fin's ring is a slit polygon (in to the 1 mm edge
and back out), and a bottom fan bounds a solid only from a point every
ring edge faces from inside: the thin wedge behind the slit's inner
end, about the slit's bearing. `(0, 0.2)` is 0.2 off that wedge, so the
fan's face on `c0 c1` leans across the slit, 3 in depth per 1 across,
and passes through the far fin face wherever that face stands above it.
A winding number over the triangles reads −1 just off the fin faces
near the mouth; the region is the crown's from the start, and every
union carried it unchanged.

## Fix

- **The fixture.** The fin crown's bottom is `FIN_BOTTOM =
  (0, 0, −0.6)` (`test_support_meeting.rs`), straight under `MEET` on
  the slit's bearing, the dented crown's bottom. The same exact test
  finds no crossing; the winding probe reads 0 or 1 at every pose, and
  in every union with probes 1, 2 and 4, both orders
  (`crates/topo/tests/the_fin_crown_winds_as_a_solid.rs`). Every fin
  scene of `a_vertex_read_again_classes_every_edge` now builds, with no
  wrong, missing or doubled row: probes 2 and 4 build 30 cells each,
  where main refused 3 and built 27. `KNOWN_INSIDE_OUT` and
  `KNOWN_WRONG` are empty and gone.
- **Check 7 reads every aim that decides**
  (`side_reader::side_at_infinity_by_every_aim`, one caster, `cast_aims`,
  with an `Aims` mode). Every ray that decides is read; two that read
  opposite sides refuse `ValidationError::OrientationOpposed`, naming
  both rays, their aims and the faces they crossed. Rays set aside, and
  rays that read in band, change nothing. The role readers keep the first
  ray that decides. On the old crown the gate now refuses at three poses
  of five; at the two flipped poses no aimed ray's closest crossing
  enters the −1 region, and it still passes (filed on RESTFRONT,
  `the-at-rest-gate-passes-a-planar-body-whose-boundary-passes-through-itself`).
- **The cost.** Every aim is cast on a valid body, so check 7 is
  quadratic in faces: 71 ms against 2.7 s at 1026 faces (filed,
  `check-7-casts-a-ray-per-face-so-the-at-rest-gate-is-quadratic-in-faces`).

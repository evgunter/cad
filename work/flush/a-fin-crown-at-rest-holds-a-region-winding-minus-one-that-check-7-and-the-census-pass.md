---
id: a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass
kind: issue
title: The fin crown at rest holds a region that winds −1, and check 7 and the census pass it; which of its unions refuses is face order's choice
status: closed
opened: 2026-10-14
priority: P1
cost: M
branch: flush/fin-crown-fixture
pr: 5534
closed: 2026-10-21
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

## Measured on the edge-meeting ray

With the edge-meeting ray's near-normal aims (FLUSH,
`a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face`), `x ∪ y`
of probes 2 and 4 refuses `ResultInvalid` at all five poses, the two
flipped ones included; `y ∪ x` and probe 1 still build holding the
region. Pinned in `KNOWN_INSIDE_OUT` and `KNOWN_WRONG`.

## Measured on main (`be82e5ad6`)

**The fixture builds a non-solid. No kernel step makes the region.**
The crown's triangles, from `fin()` and the bottom `(0, 0.2, −0.6)`,
cross each other. An exact segment–triangle test finds the bottom fan's
face on the fin's long edge `c0 c1` (`c1 c0 low`) crossing the fin's far
face (`c1 c2 MEET`) and the top face beyond it (`c2 c3 MEET`). The face
on `c1 c2` crosses `c2 c3 MEET` too.

Seen from above, the fin's ring is a slit polygon: it runs in to the
1 mm edge and back out. A bottom fan bounds a solid only from a point
that every ring edge faces from inside, which is the thin wedge behind
the slit's inner end, on the slit's bearing. `(0, 0.2)` is 0.2 off that
wedge. So the fan's face on `c0 c1` leans across the slit, 3 down for
every 1 across, and passes through the far fin face wherever that face
stands above it. A winding number over the triangles reads −1 just off
the fin faces near the mouth. The region is the crown's from the start,
and every union carried it unchanged.

## Fix

**No single-apex bottom works.** The bottom straight under `MEET`
crosses nothing, but it holds `MEET` within 8.7e-10 of the planes of
both bottom faces on the fin's long edges, because the fin face
`c0 c1 MEET` is only 3.5e-5 tall. The census reads the fin's edges in
band of them at ε 1e-9 (two `EdgeFaceOverlap`, two `EdgeEdgeCross`, at
all five poses; the PR 5534 review). Its sweep found no single-apex
bottom that clears this without crossing again.

`meeting::posed_fin_crown` (`test_support_meeting.rs`) keeps the top,
the ring fanned to `MEET`, and the slit. It closes the slit from above
with one lid face over the slit's three corners, `c0 c1 c2`, so the fin
is a thin ridge on the crown's top. The bottom fans from `FIN_BOTTOM`
to the ring without the slit's inner corner, `c0 c2 c3 c4`, which is
star-shaped about that point. The lid meets each fin face at about 90°,
so `MEET` lies 3.5e-5 off its plane.

The two faces that hold both mouth corners, the lid and the bottom face
under the mouth, are pinned by the 7e-7 edge `c0 c2`. Newell's centroid
sum cancels there to about 1e-11, which fails `newell_plane_residual` at
ε 1e-12. So each takes the plane through its corner at `c0`, where its
edges meet square.

`crates/topo/tests/the_fin_crown_winds_as_a_solid.rs` checks every
pose, at ε 1e-9 and 1e-12:
- the census (tier 3′, no contacts) passes the fin crown;
- the exact crossing test finds no crossing;
- the winding probe reads 0 or 1 on the crown and on all 30 of its
  unions with probes 1, 2 and 4.

The old crown is pinned as refused by the census, crossing itself,
winding −1, and passed by the at-rest gate. At 1e-6 the fin's fold is
in band and nothing builds.

In `a_vertex_read_again_classes_every_edge`, every fin scene builds
with no wrong, missing or doubled row. `KNOWN_INSIDE_OUT` and
`KNOWN_WRONG` are gone.

## Resolution

**The census does catch the old crown.** Despite this row's title, it
refuses the old crown with two `EdgeFacePierce` (edge 8 × face 2,
edge 15 × face 5) at every pose, at 1e-9 and 1e-12. What let it through
is the at-rest gate, which runs tier 3 alone
(`work/restfront/a-finished-body-is-gated-at-tier-3-alone-so-a-body-the-census-refuses-finishes.md`).

**The check half was not built.** Reading every aim
(`check-7-casts-a-ray-per-face-so-the-at-rest-gate-is-quadratic-in-faces`,
closed) adds nothing on an embedded boundary, where one ray reads what
every aim reads. FLUX's seam-region witnesses, which it also refused,
are refused at tier 3 since PR 5516 (`SeamRegionOvercovered`).

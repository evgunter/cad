---
id: a-blend-corner-patch-holds-a-chart-pole
kind: issue
title: A fillet corner patch's sphere chart puts a pole inside the patch, which the ratified PR 2850 rule says must be a vertex
status: closed
opened: 2026-10-10
priority: P2
cost: M
refs: [validity-refuses-an-interior-chart-singularity]
branch: band/corner-patch-chart-pole
closed: 2026-10-14
pr: 5048
---


Found by FLUX's sphere arm (`flux/sphere-arm`, 2026-10-10), filed on
the blend's slate because `crates/sweep/src/blend/arms.rs` is BAND's.

## What

`corner_ball` (`crates/sweep/src/blend/arms.rs`) charts a fillet's
spherical corner patch with its polar axis on the patch-centre
direction, `(n1 + n2 + n3)` folded by the convexity, so a chart pole
lands inside the corner triangle by construction. The rule Ev ratified
on `[ev]` PR 4549 — *a chart singularity inside a face is a vertex of
it* (`crates/geom-brep/README.md` C12 (7)) — says no face of this kernel
holds one.

Executed: `crates/sweep/tests/m5_pr12_fix_pass.rs`,
`f4_an_oblique_trihedron_builds_and_passes_tier_3`. Its corner face has
chart axis `(0, −0.958, −0.286)` and its south pole inside the triangle
(barycentric weights 0.09, 1.04, 0.31). Props' side reader
(`geom_brep::props::curved::sphere_loop_side`) finds a pole off the
loop on each side and answers `Unencoded`, so the face measures under
its bit and nothing cross-checks the bit; the row is green. It goes red
when validity enforces the rule
(`work/restfront/validity-refuses-an-interior-chart-singularity.md`).

## What a fix owes

Choose the corner ball's chart axis so each pole lies on a vertex of
the patch or outside it — for example through one of the triangle's
corners, or perpendicular to the patch-centre direction so that both
poles fall outside a patch smaller than a hemisphere (negating the axis
only swaps which pole is inside) — and
re-baseline the corner patches' pcurves and meshes that move. Done when
every corner face of f4 reads `MaterialSign::Encoded` and agrees with
its bit.

## Closed

`sweep::blend::arms::corner_chart` charts every fillet corner patch
with its pole on one of the patch's feet (a vertex) and the antipode
outside it, derived at the site: the patch is the spherical triangle of
nonnegative combinations of its three foot directions, so `−f_c` is in
it only if the feet are dependent, which predicate 6 refuses, and no
patch reaches a hemisphere. `build::octant_chart` (production) and
`corner_ball` (the arm's own chart) both call it; box corners keep
their chart up to the sign of a zero component, oblique corners move
their pole onto a foot. `crates/sweep/tests/corner_patch_chart_poles.rs`
holds every corner face of a corpus (cubes at six poses and two
scales, the vented and skewed cavities, the f4 oblique clip, a vented
void in its shape, a cube at `Interval`) to `MaterialSign::Encoded`
agreeing with its bit and to no pole strictly inside a face.

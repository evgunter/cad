---
id: the-wall-images-foot-resolution-counts-ulps-at-the-walls-point
kind: issue
title: The wall image settles a foot and reads its own miss to a count of ulps at the wall's point: a rounding charge owed a re-posing
status: open
opened: 2026-11-23
---


Filed by the `sectarms/one-image-function` lane, which wrote the site.

## What

`geom_brep::carrier_image::resolution` reads the length `f64` resolves
at a wall point as `RESOLUTION_ULPS · f64::EPSILON · max|coordinate|`
(64 ulps). Two readers:

- `settle`: where the surface projection's own acceptance is not met
  (`geom::NurbsSurface::project_from_seed`; its coincidence and
  stagnation thresholds are absolute, `PROJECT_EPS_POINT` 1e-13 m, and
  its cosine one is relative to the standoff), the last iterate stands
  as the foot if both its tangential offsets are within that length;
- `miss`: an image's miss of the foot path under that length asks no
  knot.

Both exist for walls far from the origin: at 500 m and 1 km the
projection settles no foot that stands a few nanometres off the wall
(`work/sectarms/a-sphere-against-a-spline-wall-a-kilometre-across-settles-no-foot.md`,
`work/sectarms/a-chart-limb-1-foot-refuses-a-pose-far-from-the-origin.md`),
and the image now takes a foot at every knot of the carrier where the
trace took nine. `m5_pr7_ssi::a_far_plane_wall_pair_certifies_or_refuses_by_scale`
and `plane_nurbs_readings::the_plane_reads_the_kilometre_foot_frontier_on_both_sides`
go red without it.

It is a rounding charge at a site outside `geom-core`
(DESIGN `[simple.conditioned]`), listed `re-pose` in
`scripts/gates/bare-rounding-sites.tsv`.

## Done when

The projection's acceptance is posed so a foot settles at the wall's
own scale (the two items above), and the image reads its feet and its
miss with no ulp count of its own.

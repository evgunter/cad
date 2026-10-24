---
id: the-projected-image-inverts-chart-points-by-a-second-spelling
kind: issue
title: The projected image inverts chart points by a second spelling, and reads a cone point's v off its ruling foot
status: open
opened: 2026-10-24
---


Found by the chart-inversion sweep (`work/chart/chart-inversion-is-spelled-three-times.md`).

## What

D3 ("One reading per question") makes a point's chart coordinates one reading, `geom::ChartInverse::point` (`crates/geom/src/chart_point.rs`). The projected image (C4: "the chart's own inverse applied to the carrier", `ψ(C(t))`) spells its own inverse in `ProjectedImage::raw` (`crates/geom-brep/src/pcurve_cache/projected.rs`):

- the azimuth through `branch_azimuth` about each piece's quarter-turn centre;
- the sphere's latitude as `p.z.atan2(rho)`, raw, not in the latitude window;
- the torus's tube angle through `branch_azimuth(phi, rho - major, p.z)`;
- **the cone's v as `p.z·cos α + σ·ρ·sin α`**, the coordinate of the point's foot on its ruling, where the reading (and `topo::chart::Chart` before it) reads `h / cos α`. The two agree on the cone and differ off it, so a carrier point a band off the wall has two chart v's.

The per-piece branch centres are the image's own placement (the sector partition), which a point reading does not have; the question is whether the channel values should come from the reading with the branch taken after, as `chord_join`'s ring re-homing does (`principal` then `periodic_branch`).

## Owed

Decide which cone v is the chart's (the foot's or `h / cos α`) and make the reading and ψ agree, then route ψ's channels through the reading or state why an image's derivation is exempt. Report any moved baseline.

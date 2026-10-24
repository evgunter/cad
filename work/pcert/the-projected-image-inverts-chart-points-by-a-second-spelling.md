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
- the cone's v as `p.z·cos α + σ·ρ·sin α`, the coordinate of the point's foot on its ruling.

**The cone's v agrees now.** When this row was filed the reading read a cone point's v as `h / cos α`, which is not the foot's: a point 1e-6 off along the normal read 2.76e-6 off. PR 5659's review found that, and the reading now reads the ruling foot `h·cos α + σ·ρ·sin α` with the nappe decided on `h`. So ψ was the right side, and the two spellings compute the same channels. What remains is the unification.

The per-piece branch centres are the image's own placement (the sector partition), which a point reading does not have. The question is whether the channel values should come from the reading with the branch taken afterwards, as `chord_join`'s ring re-homing does (`principal` then `periodic_branch`). ψ also reads in the chart frame at the image's scalar without a band, so routing it through `geom::ChartInverse` changes its signature and is not a one-line swap.

## Owed

Route ψ's channels through `geom::ChartInverse` (frame-local, then the piece's branch), or state why an image's derivation is exempt. Report any moved baseline.

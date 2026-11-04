---
id: the-projected-image-inverts-chart-points-by-a-second-spelling
kind: issue
title: The projected image inverts chart points by a second spelling, and reads a cone point's v off its ruling foot
status: closed
closed: 2026-11-04
branch: pcert/projected-one-inverse
pr: 6500
opened: 2026-10-24
priority: P1
cost: M
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

## Closed

Routed, not exempt. The chart's channel table is spelled once, `geom::ChartChannels::read` (`crates/geom/src/chart_point.rs`). It decides nothing: each angle leaves as the direction it is read off (`geom::Azimuth`), and the caller names it.

- `ChartInverse::point` is that table. It names each angle in its window (`principal_in`) and decides the cone's nappe on `h`, with the same decisions in the same order as before.
- ψ (`ProjectedImage::raw`) is that table on the image's stored scalars, with the image's nappe. It names each angle on the piece's branch: the azimuth about `θ_k`, the torus tube about `φ_k`, the latitude about zero. It uses `Azimuth::about` with a `geom::Quarter`, which is `branch_azimuth` moved into geom, and its rotation is the one the frame boxes now use.

How the Interval scalar and the branch centres survive: the branch is the namer's, not a decision, so ψ still evaluates at every scalar with no band, and `Pcurve::eval`'s signature is unchanged. The branch is decided once, at derivation, by the sector condition. A per-point "principal, then periodic_branch" route was set aside for three reasons: it would read a whole-period integer off a value, which C4 forbids; it would put band decisions into every evaluation; and it would move every non-zero-centre channel by ulps.

Moved baselines: one, and no certified answer. The sphere latitude at exactly `z = −0.0` reads `+0.0` where it read `−0.0` (equal values), pinned by `the_sphere_latitude_at_negative_zero_height_reads_positive_zero`. A one-shot bitwise comparison of the new ψ against the old formula over 23 M `f64` reads found 648 differences, all the sphere latitude at `z = −0.0` exactly, which now reads `+0.0` (equal values). Over 0.29 M `Interval` reads it found 0 differences. The placed value differs only where the image's `v_off` is also `−0.0`.

The row `projected/one_reading_rows.rs` compares ψ against `ChartInverse::point` plus the branch, at `f64` and `Interval`. It covers the sphere, the cylinder, a narrow and a wide cone on both nappes, and a ring and a near-horn torus, sweeping across both cuts and off the surface.

The sweep's out-of-scope sites have their own rows:

- `a-cone-points-v-is-read-off-its-height-in-props-and-replace-face`
- `derive-class-spells-a-parallels-latitude-and-a-rims-slant-on-its-own`
- `the-winding-count-reads-chart-channels-by-its-own-atan2`
- `sphere-and-torus-levels-are-read-as-sin-cos-pairs-by-their-own-spelling`

`about` and PR 6482's `Azimuth::near` answer one question; the pcert orchestrator ruled one door, `near`, with `about`'s exact rotation as its quarter arm, folded in by 6482: `work/onecopy/azimuth-near-and-azimuth-about-answer-one-question.md`.

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

Routed, not exempt. The chart's channel table is spelled once, in `geom::ChartChannels::read` (`crates/geom/src/chart_point.rs`). It decides nothing: each angle leaves as the direction it is read off (`geom::Azimuth`), and the caller names it.

- `ChartInverse::point` is that table. It names each angle in its window (`principal_in`) and decides the cone's nappe on `h`, with the same decisions in the same order as before.
- ψ (`ProjectedImage::raw`) is that table on the image's stored scalars, with the image's nappe. It names each angle on its piece's branch with `Azimuth::about(Quarter)`, which is `branch_azimuth` moved into geom: the azimuth about `θ_k`, the torus tube angle about `φ_k`, and the latitude about `Quarter(0)`.
- The branch centres are stored as `geom::Quarter`. A missing centre fails loud (`theta`, `phi`), and the box forms rotate through `Quarter` (`rotate_box`).

**How the Interval scalar and the branch centres survive.** The branch belongs to the namer and is not a decision, so ψ still evaluates at every scalar with no band, and `Pcurve::eval`'s signature is unchanged. The branch is decided once, at derivation, by the sector condition. A per-point route, the principal read and then `periodic_branch`, was set aside for three reasons:

- it would read a whole-period integer off a value, which C4 forbids;
- it would put decisions into every evaluation;
- it would move every channel on a non-zero centre by ulps.

**Moved baselines.** Two, and no certified answer moves; both are in sector.

- The sphere latitude at exactly `z = −0.0` reads `+0.0` where it read `−0.0`. The values are equal. It is pinned by `one_reading_rows::the_sphere_latitude_at_negative_zero_height_reads_positive_zero`.
- At `Interval`, where a coordinate's enclosure has an end under the backend's 2Prod floor (`2^-960`), the azimuth's enclosure can be up to one step narrower, inside the old. The old spelling multiplied by `σ = 1`, and a product under the floor pads one step outward.

`projected/old_formula_rows.rs` keeps the old spelling as the oracle and holds the routed ψ to it bit for bit in sector, allowing only those two differences, at `f64` and at `Interval`.

**Pins.**

- `projected/one_reading_rows.rs` compares ψ against `ChartInverse::point` plus the branch, at `f64` and `Interval`. It covers the sphere, the cylinder, a narrow and a wide cone on both nappes, and a ring and a near-horn torus, across both cuts and off the surface. It guards ψ's own naming (the centres, the nappe flag), but not the shared table.
- A mutation of the shared table reds geom's `chart_point::tests::each_arm_reads_a_point_as_its_foot` and `old_formula_rows`.
- geom's `a_quarter_rotates_exactly_and_about_names_the_angle_on_its_branch` pins `Quarter` and `about` themselves.

**`about` and `Azimuth::near` (PR 6482).** These are two questions, so both stay. `near` decides a copy near a `T` reference; `about` evaluates on a known quarter branch and decides nothing. The pcert orchestrator ruled this after the FULL review (Z1). `near`'s doc gains a pointer to `about`, filed for 6482's lane with the mechanical conflicts: `work/onecopy/azimuth-near-and-azimuth-about-should-name-each-other.md` (P3).

**Rows filed for the sweep's out-of-scope sites, and for the image's box forms:**

- `a-cone-points-v-is-read-off-its-height-in-props-and-replace-face`
- `derive-class-spells-a-parallels-latitude-and-a-rims-slant-on-its-own`
- `the-winding-count-reads-chart-channels-by-its-own-atan2`
- `sphere-and-torus-levels-are-read-as-sin-cos-pairs-by-their-own-spelling`
- `the-projected-box-forms-spell-the-point-reading-apart` (P2)

**Left as they are, on the review's advice:**

- the two kind enums, `ProjectedChart` beside `ChartChannels` and `ChartArm`;
- `ChartInverse::point`'s capture of the tube direction in an `Option` for its singularity test;
- ψ's `unreachable!` on `ChartAngle::SheetTube`, since `chart_at` refuses a spindle.

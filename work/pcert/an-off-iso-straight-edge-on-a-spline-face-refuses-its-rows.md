---
id: an-off-iso-straight-edge-on-a-spline-face-refuses-its-rows
kind: issue
title: the mint refuses a spline face holding an off-iso straight edge (LoopDiscontinuity, no rows stored), because its line-rim arm picks a row through the start foot
status: closed
opened: 2026-10-12
closed: 2026-10-13
priority: P2
cost: M
branch: pcert/off-iso-line-takes-its-general-row
pr: 4955
---


Found by `pcert/fitted-row-reads-carrier-and-chart` while probing the
D7 and step-import questions its orchestrator asked.

## What happens

`nurbs_iso_derive` (`crates/topo/src/pcurves.rs`) gives a `Line`
carrier described `Chart(_)` (not this face's own chart) or
`Scaffold(_)` the line-rim arm. That arm assumes the line is a row of
the spline chart. Where the closed form misses, it measures the two
end feet, builds the `u` map from them, and picks the row with
`side_pick(&row(p0x, plx), &[cv0, cv1, f0.y])`. That pick reads only
the carrier's START point, which every row through the start foot
passes. So an off-iso straight edge (a diagonal) gets an `IsoLine`
that leaves the carrier everywhere past its start. The candidate is
never stored: the loop walk refuses the face loudly
(`LoopDiscontinuity`, zero rows stored). So this is a completeness gap,
a false refusal of a face that has a certifiable `General` image, and
not a wrong row at rest. Both PR 4900 reviews executed it and found it
pre-existing on the merge base, not widened by that PR (orchestrator,
2026-10-12: P2).

Measured on main + this branch: a triangle lamina on the bilinear
identity chart `(u, v) ↦ (u, v, 0)` (`mvfs`, two `mev_line`s, one
`mef_chord`), with the two axis-aligned edges re-stated as `General`
images and the diagonal chord left as `mef_chord` minted it
(`Scaffold`, a `Line` carrier). `pcurve_of` on the diagonal answers
`IsoLine { p0: (1, 1), pl: (-0.7071, 0) }`, a row, and
`mint_pcurves_of` answers `Err(LoopDiscontinuity)`.

## Why it matters now

The fitted-grade row certifies a line carrier's `General` image
against its chart. Since `pcert/fitted-row-reads-carrier-and-chart`,
the mint's `Intersection` arm gives an off-iso line that image after
probing its exact candidates. The `Chart`/`Scaffold` arm has neither
the probe nor the fallback. Step import's conventional rung lands
exactly here: a line on a plane × B-spline pair adopts as an image in
the plane's chart (`adopt.rs`, `nurbs_rim`), and on the wall it
reaches this arm (`work/reimport/an-imported-off-iso-line-on-a-bspline-face-takes-no-general-row.md`).

## Fix

Probe each row candidate at an interior parameter, as the
`Intersection` arm's interior-row block does
(`pcurve_iso_seam_column`). Where none holds, derive the `General`
image (`derive_general_image`) instead of refusing or guessing.

## Closed

`pcert/off-iso-line-takes-its-general-row`. A `Line` carrier described
as a chart image or a scaffold now takes the boundary-iso schedule the
`Intersection` arm takes:
- the closed-form columns and rows, in both directions;
- then the row, and for a line the column, whose moving map the end
  feet measure;
- then the `General` image.

Every candidate is judged a quarter span in by `probe_pick`, the one
home of that probe (the arc rim's direction pick goes through it too).
The triangle lamina's diagonal mints `General` and certifies at f64 and
Interval, and its row and column edges keep their exact `IsoLine`s
(`topo`'s `an_off_iso_straight_edge_takes_its_general_row`). The step
import's plane × wall shape imports through a STEP file
(`step-import`'s `an_off_iso_line_on_a_spline_wall_imports_its_general_row`).

The wall-seam arm's one-foot column pick is left as it stands:
`work/pcert/the-wall-seam-arm-positions-its-column-off-the-start-foot.md`.

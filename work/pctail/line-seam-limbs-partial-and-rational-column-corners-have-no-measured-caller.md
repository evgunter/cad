---
id: line-seam-limbs-partial-and-rational-column-corners-have-no-measured-caller
kind: issue
title: the seam class's LINE-carrier limb: its partial-column and rational-column corners have no in-tree caller, because no public door builds a curved Approx chart with a straight carrier
status: open
opened: 2026-09-22
priority: P3
cost: M
---


Filed by the TESS orchestrator from the Approx-face survey
(`tess/approx-face-survey`, `d423d1b46`), 2026-09-22.

The LINE-carrier limb in `PcurveCache::certify`'s `'seam` block
(PR 1798) is measured green on exactly one shape: a single-span
bicubic fit of a planar base, whole-column image. Its own comment says
the Greville hull "runs over the FULL column — for an image trimmed to
part of the domain that is a superset bound … and no measured caller
exercises the partial case", and it refuses typed a rational column and
a LINE seam on a non-boundary column. A CURVED `Approx` chart with a
straight carrier — the shape a real offset/shell consumer brings —
would exercise the partial, refined-column corner, and is not
buildable through the public doors today (the pull-back is exact only
to `d·Δn`, so the `IsoLine` chart-image description does not certify).
Recorded so the limb's measured coverage is stated where the limb is:
the corner is a claim without a caller until a door builds that shape.

## A measured caller of the partial-column corner (SPLICE, 2026-10-26)

Filed by the SPLICE lane building the spline point-in-face arm
(`splice/point-in-face`). The partial-column corner now has a caller
that refuses: a face trimmed to part of a creased degree-1 column. On a
chart ruled along `x` through the `(y, z)` rows `(0, 0)`, `(1, 0)`,
`(2, 1)` (a C⁰ knot at `v = ½`), the face `[0.25, 0.75] × [0.1, 0.4]`
has `u = const` sides whose `Line` carriers lie exactly on the chart
(`S(u₀, v)` is affine on `v ∈ [0, ½]`), stated with
`EdgeDescriptionSpec::chart_image(.., Pcurve::IsoLine { .. })`.
`mint_pcurves` refuses them `Certify { error: ResidualExceeded { check:
Envelope, sample: 0 } }`: `seam_envelope`'s line limb takes the Greville
hull over the full column, and the column's third control point,
`(2, 1)`, sits a metre off the line extended. The same side stated as a
`Pcurve::General` degree-1 image certifies, since the general lane's
hull reads the image's own span. `spline_region_rows.rs`'s `sheet`
fixture states its creased-column sides that way for this reason.

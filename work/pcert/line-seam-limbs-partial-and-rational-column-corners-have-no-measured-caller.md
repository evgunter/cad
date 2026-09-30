---
id: line-seam-limbs-partial-and-rational-column-corners-have-no-measured-caller
kind: issue
title: the seam class's LINE-carrier limb: its partial-column and rational-column corners have no in-tree caller, because no public door builds a curved Approx chart with a straight carrier
status: open
opened: 2026-09-22
priority: P3
cost: D
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

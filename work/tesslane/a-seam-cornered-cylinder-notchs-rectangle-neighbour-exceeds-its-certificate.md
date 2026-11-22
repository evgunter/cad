---
id: a-seam-cornered-cylinder-notchs-rectangle-neighbour-exceeds-its-certificate
kind: issue
title: A cylinder notch cornered on the seam leaves a rectangle neighbour that refuses CertificateExceeded at every delta
status: open
opened: 2026-11-17
---



## What

`rod(1, -2, 4)` ∖ `brick((0, 3), (0, 3), (0.5, 3))`, and its variant with
`z ∈ (−0.5, 0.5)` (`mesh/tests/reach_probes.rs`' builders): the cut
leaves the pipe wall notched, and the wall face beside the notch that
is a rectangle — one of its sides lies on the cylinder's seam — refuses
`CertificateExceeded { bound ≈ 0.41–0.44 }` at every δ the reach probes
use, on main and on `tesslane/route-by-domain` alike (found by that
unit's review, 2026-11-17). The notched face itself routes to the
trimmed lane; this refusal is the curved lane's, on the rectangle.

A bound of about 0.4 against δ ≤ 5e-3 is no rounding. The cause is
unmeasured; the side on the seam (both `u = 0` and `u = 2π` of the
chart) is where to look first.

## What would settle it

Measure the face's walk polygon and the grid it lays (`curved`'s
`walk_results` on the body), find which column sits a period off, and
whether the walk's seam rule (`walk::wrap_strip`, the iso-side
collapse) or the chord pass places it there. Pin the body in
`reach_probes` with its answer.

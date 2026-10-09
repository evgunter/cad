---
id: torus-chart-box-check-passes-an-l-shaped-face
kind: issue
title: bool_torus_chart_box compares total variation to twice the span, which an L-shaped (orthogonally convex) face also satisfies, so the torus trim can serve an L its bounding box
status: closed
opened: 2026-09-29
priority: P1
cost: M
parent: CONTACT-11
closed: 2026-10-08
---


Found by the VERBS-CONE U3 fix pass (PR 3395), which ported this check
to the cone trim and found it could not catch the case the cone's
reviewer built. **Unmeasured on a torus face**: the argument below is
arithmetic, and no torus L has been minted to run it.

## The defect, by reading

`torus_chart_windows` (`crates/topo/src/boolean/solid_contain.rs`)
decides `bool_torus_chart_box` on `variation − 2·span` in each chart
channel, and its docs say "the L-shape has strictly more". It does not.
A rectilinear polygon has total variation `2·span` in a channel exactly
when it is monotone in that channel, and an L is monotone in both. Take
the chart L `(0,0) → (2,0) → (2,1) → (1,1) → (1,2) → (0,2)`. Its `u`
steps are `2, 0, −1, 0, −1, 0`, total 4 = 2·2, and its `v` steps are
`0, 1, 0, 1, 0, −2`, total 4 = 2·2. Both checks pass, and the window
over-covers the notch `(1,2) × (1,2)`. Only a notch that breaks
monotonicity (a U) is caught.

## The fix: a linear test in metres, shared with the cone

The area test the cone carried (`chart_box_defect`, the polygon's
shoelace area against its box's) is unsound as a Zero verdict. A notch
`s` metres on a side changes the area by about `s²`, so the margin
divided back to a length is quadratic in the notch. A small notch reads
Zero ("the face is its box"), and the door answers `In` at a point
`s/2` from the face. One decade up, it escalates where it should refuse.

The test both trims now share (`solid_contain::chart_polygon_box`):
every side of the rectilinear chart polygon lies on a side of its
bounding box. Each side's distance is crossed to metres by an upper
bound on its channel's rate (`SupSpeed`, `Margin::metered_sup`), so a
notch's inner sides read at their own metric distance, linear in the
notch. The rows are in `section_cert_rows.rs`: the small-notch rows on
four rings and two frusta, the smallest-notch row that pins the arms'
direction, the walk-gap row, and the rectangle rows.

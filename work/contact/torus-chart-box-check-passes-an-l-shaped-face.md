---
id: torus-chart-box-check-passes-an-l-shaped-face
kind: issue
title: bool_torus_chart_box compares total variation to twice the span, which an L-shaped (orthogonally convex) face also satisfies, so the torus trim can serve an L its bounding box
status: open
opened: 2026-09-29
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

## The fix, as the cone took it

Compare the polygon's shoelace area with its bounding box's
(`chord_join::chart_box_defect`, `bool_cone_chart_box`). A rectilinear
polygon equals its box exactly when the areas agree. The cone's rows are
`section_cert_rows.rs` `an_l_shaped_cone_face_refuses_rather_than_trim_by_its_hull`;
a torus L built the same way (Euler ops on a torus sheet: parallels and
meridians) is the red row this needs.

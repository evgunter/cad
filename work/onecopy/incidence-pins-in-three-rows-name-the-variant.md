---
id: incidence-pins-in-three-rows-name-the-variant
kind: issue
title: Three rows pick out an incidence read by its PcurveCertifyError or CertifyError variant instead of the classifier
status: open
opened: 2026-11-09
---


Filed by the ONECOPY `onecopy/unbuilt-recourse` lane in its sweep for the
shape that `the-cone-face-at-rest-pin-names-carrier-off-chart-by-variant`
closed. That lane rerouted `review_m3_pr4`,
`loop_reparenting_pcurve_rows` (two pins), `euler_site_row_frontiers`
and `a_curve_off_its_face_at_a_build_is_the_kernels` through the
classifier.

## What

"Is this refusal a decided incidence miss, or one read in band" has one
home: `geom_brep::IncidenceRead::of`, and for the edge certificate
`CertifyError::incidence_read`. These rows still answer it by naming the
variant:

- `crates/sweep/tests/the_football_revolves.rs`, `off_sheet`:
  `CertifyError::CarrierOffChart { chart: Spindle, .. }`.
- `crates/sweep/tests/a_wall_row_read_in_band_takes_the_lane_a_miss_takes.rs`:
  `PcurveCertifyError::Escalated { check: ChartIncidence, .. }` for the
  in-band read, and `CarrierOffChart { chart: Nurbs, carrier: Circle, .. }`
  for the miss.
- `crates/topo/tests/a_circle_not_shown_on_a_spline_row_refuses_alike_in_every_form.rs`:
  the same pair, `CarrierOffChart { chart: Nurbs, carrier: Circle, .. }`
  and `Escalated { check: ChartIncidence, .. }`.

Each also pins the chart and carrier kinds, which the classifier does
not return. The lane left these alone because rerouting them needs a
decision about where those kinds are checked. Moving them onto the face's
surface and the edge's curve, as `review_m3_pr4` keeps its cone check,
is the likely spelling.

## What closing it takes

Match each through the classifier (`Off(_)`, `Undecided(_)`). Check the
chart and carrier kinds on the body's own face and edge where the row
means them.

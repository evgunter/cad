---
id: uncovered-chart-classes-have-no-incidence-test
kind: issue
title: An uncovered chart_pcurve class has no incidence test, so an off-chart carrier among them is excused as uncovered
status: open
opened: 2026-10-01
---


The residual of D36 (`pcert/d36-unsupported-carrier-split`). The ground
is `crates/geom-brep/src/pcurve_cache.rs` (PCERT's and PCTAIL's per
`work.py territory`; filed here because that lane does not write
`work/pcert/`).

D36 split `UnsupportedCarrier` (uncovered: the carrier can lie on the
chart, no lane covers it — the only refusal `topo::pcurves::mint_faces`
excuses) from `CarrierOffChart` (a body defect, propagated). Where an
incidence test exists the split is decided by it — one-sidedly, on the
sphere, cone and torus (`pcurve_cache::chart_incidence`): the largest
exact distance from the chart at 64 carrier samples is a lower bound,
so `CarrierOffChart` is certified, and a carrier not shown off goes on
to its image's certificate or refuses as grazing the chart: the sphere's general
circles (`sphere_circle_incidence`, `pcurve_sphere_chart_incident`;
no longer excused at all — the mint images them through the fitted
route),
the cone's non-rim conics (`cone_conic_incidence`,
`pcurve_cone_chart_incident`; a circle on the cone that is no rim now
refuses as grazing it, `CarrierGrazesChart`, since a cone holds no
other circle), the torus's oblique circles (`torus_oblique_circle`,
`pcurve_torus_chart_incident`; a circle not shown off the torus is a
Villarceau circle and mints its focal-section image, or grazes it),
and the spiric's foreign tori (the
mirror-torus coefficient match). The classes below have NO such test
and are `UnsupportedCarrier` wholesale, so a carrier in them that does
NOT lie on the chart is still excused by the mint, leaves its face
uncached, and passes tier 3 clean:

- ~~**`UncoveredClass::SplineCarrier`**~~ — resolved on
  `pcert/projected-image` (2026-10-08): the class is deleted. A spline
  carrier on an analytic chart takes the projected-image route, which
  first decides incidence (`net_incidence`, against the chart's
  canonical composite). A net shown off the chart refuses
  `CarrierOffChart`
  (`envelope_lemma_fuzz::projected::an_off_chart_spline_refuses_by_incidence`),
  and one that is not certifies or refuses through its envelope's
  incidence term.
- **`UncoveredClass::NoFittedClass`** (`run_fitted_checks` check 1): a
  line, ellipse or spiric offered a `Fitted`/`General` image, on any
  chart — `stated_general_image_mint` reaches it at four scalars with a
  line on a spline chart. Nothing checks the line lies on the chart; the
  test would be the same foot schedule `general_image` runs, against the
  carrier.

Each would move its off-chart members to `CarrierOffChart` and make
the mint loud on them, as D36 did for the sphere and the cone.

A separate, smaller gap: every class decision is about the CARRIER, an
infinite line or a whole circle. A short edge on a carrier that is off
the chart can still lie within ε of it (a tangent segment of length
`≲ √(8εR)` on a sphere), and is called off-chart. That posture is the
lane's throughout (chart images are carrier images), and no producer is
known to reach it.

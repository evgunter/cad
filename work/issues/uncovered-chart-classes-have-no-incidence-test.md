---
id: uncovered-chart-classes-have-no-incidence-test
kind: issue
title: Three uncovered chart_pcurve classes have no incidence test, so an off-chart carrier among them is excused as uncovered
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
incidence test exists the split is decided by it: the sphere's general
circles (`sphere_circle_incidence`, `pcurve_sphere_chart_incident`),
the cone's non-rim conics (`cone_conic_incidence`,
`pcurve_cone_chart_incident`), and the spiric's foreign tori (the
mirror-torus coefficient match). The classes below have NO such test
and are `UnsupportedCarrier` wholesale, so a carrier in them that does
NOT lie on the chart is still excused by the mint, leaves its face
uncached, and passes tier 3 clean:

- **`UncoveredClass::TorusGeneralCircle`** (`chart_pcurve`'s torus arm):
  an oblique circle (`pcurve_torus_chart_meridian` failing) — Villarceau
  circles lie on the torus, an arbitrary tilted circle does not — and a
  circle ⊥ the axis centred off it (`pcurve_torus_chart_centered`
  failing), which near the tube's top leaves the torus only by `δ²/2r`
  and so cannot be called off-chart by the centring gate. The test:
  `(|p|² + R² − r²)² − 4R²ρ²` along `c + a cos t + b sin t` is a
  trigonometric polynomial of degree 4 in `t`, nine coefficients, each
  meterable over a `≈ 4R·r` lever.
- **`UncoveredClass::SplineCarrier`** (`chart_pcurve`'s top,
  `carrier_harmonic` answering `None`): a spline carrier on an analytic
  chart. No closed-form test; it needs the fitted lane's on-locus hull,
  which needs a mate. A real producer reaches it: a STEP re-import of an
  exported spiric rim (`spiric_roundtrip`) lands a spline on a torus.
- **`UncoveredClass::ZeroOffsetSpiric`** (`spiric_off_own_chart`): a
  spiric of `|offset| ≤ ε` is a meridian circle of its torus, and on a
  cylinder, cone, sphere, perpendicular torus or drifted torus it is let
  through whether or not that circle lies on the chart. The test is the
  chart's own circle incidence, run on the circle the spiric is
  (`center + m·R`, radius `r`, in the plane spanned by `m` and the axis).
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

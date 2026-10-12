---
id: no-fitted-class-misuse-is-excused-by-not-owed
kind: issue
title: not_owed excuses UncoveredClass::NoFittedClass, a wrong image offered to the fitted door, as if it were a class with no route
status: open
opened: 2026-10-07
priority: P2
cost: H
design: true
refs: [mint-has-no-route-to-the-spiric-and-no-fitted-classes]
---


Found by the spline-carrier designers (PR 4261, fork-log row 85).

`UncoveredClass::NoFittedClass` (`crates/geom-brep/src/pcurve_cache.rs`) is what the fitted door answers when it is offered a carrier that is a line, an ellipse or a spiric. Those carriers' images are closed-form or iso. So this is a caller's misuse, not a class of carrier the chart holds with no route yet.

`not_owed` (`crates/topo/src/pcurves.rs`) excuses it alongside the genuinely uncovered classes, so a misrouted face is left uncached silently instead of refusing loudly.

It should leave the excusal and refuse as a defect. Check first whether any producer reaches it, because if one does, that producer is wrong.

## Measured (branch `pcert/mint-routes-spiric-nofitted`, 2026-10-12)

This row now carries the class: `mint-has-no-route-to-the-spiric-and-no-fitted-classes`
closed on its mirror-torus half.

**Reach.** `NoImage::Uncovered(..).refusal` and `not_owed`'s first arm
were instrumented, and the `--profile ci` suites of topo, sweep,
step-import, mesh, editor-core and geom-brep were run at default ε,
1e-6 and 1e-12. Only two tests reach the class:

- geom-brep's `pcurve_general::an_analytic_chart_refuses_a_fitted_grade_image`,
  a direct `certify` call;
- topo's `stated_general_image_mint::*::a_line_carrier_leaves_the_face_uncached`,
  at all four scalars. It is the only excusal site, and it pins the
  excusal as correct.

The path is the public `Body::set_edge_curve` with
`EdgeDescriptionSpec::chart_image(nurbs_face, Pcurve::General(..))`
over a LINE carrier. Edge certification admits the stated image (it
meters it per sample). The mint's own-chart arm (`nurbs_iso_derive`)
hands it verbatim to `certify_general`, check 1 answers `NoFittedClass`,
and the face is left uncached. No kernel producer states a `Fitted` or
`General` image over a non-spline carrier: step-import, sweep,
editor-core and verbs were grepped, and the suites confirm it.

**Not misuse, so the fix this row first proposed is a fork.** The
lamina's stated image is the identity chart's degree-1 net, so it is
exact. What is missing is a certifier. The fitted door certifies only a
spline carrier, against its operand pair (C2's hull and tube), and the
projected image is analytic-chart only because it needs ψ in closed
form. So there are two answers:

- **(a) A new lane.** A chart-only fitted-grade certificate for a line
  or a conic on a spline chart: the hull sup of `S∘P − C`, with the
  carrier lifted to its exact rational net and no mate. A spiric on a
  spline chart has no route either way, because it is non-rational.
- **(b) Refuse at the producer.** Edge certification, or check 1,
  refuses a stated fitted-grade image over a non-spline carrier. That
  refuses an exact image.

Both answers move `not_owed`, so the row is `design: true` and goes to
designers before a lane builds it.

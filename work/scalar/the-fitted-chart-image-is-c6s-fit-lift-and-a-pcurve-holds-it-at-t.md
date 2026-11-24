---
id: the-fitted-chart-image-is-c6s-fit-lift-and-a-pcurve-holds-it-at-t
kind: issue
title: "edge_nurbs::on_carrier_domain lifts a fitted chart image into T: C6 says a fit enters once through from_computed, and Pcurve<T> and the certificate's composite hull hold it at T"
status: open
opened: 2026-11-21
priority: P1
cost: M
design: true
refs: [f64-built-data-is-built-at-t, from-computed-claims-a-zero-tangent-at-a-dual]
---


Filed by the `f64-built-data-is-built-at-t` lane, 2026-11-21. That unit lists this site as "a fitted pcurve image ... it stays `f64` data", and names as not checked "whether a `Pcurve<T>` consumer forces the fitted image into `T`". Traced: one does, twice, and the lift is the one C6 already describes. The site is left as it was.

## What the site is

`on_carrier_domain` (`crates/geom-brep/src/edge_nurbs.rs`) takes the image `NurbsCurve2::<f64>::interpolate_with_params` fitted to the schedule's measured feet, re-expresses it on the carrier's domain at `f64`, and enters its knots and poles through `Real::from_computed` and its unit weights through `from_f64`. Two callers, `chart_image` and `general_chart_image`'s `climb`, both bounded `CertifiedEnclosure`, so no dual reaches it: there is no zero tangent here. At `Sym` each knot and pole is an opaque unknown.

That is C6's own sentence (`crates/geom-brep/README.md`): "a numerical routine's output (a fit's knots, weights and poles) once, at the fit, through `Real::from_computed`". The unit file sets "C6's own lift" aside as "behind the fitted lanes and ... not this row's"; this site is that lift, not a second one beside it.

## Who holds the image at `T`

- **The body.** `topo::pcurves::mint` (`derive_general_image`, through `FittedLane::general_image` and `pcurve_cache::general_image_lane`) stores the image as `Pcurve<T>::General(Arc<NurbsCurve2<T>>)`, and the SSI trace's as `Pcurve<T>::Fitted`. `Pcurve<T>` is a body's stored description, `T` by type.
- **The certificate.** `ssi::certify::incidence_limbs` reads `IncidenceOperand::Chart(&NurbsSurface<T>, &NurbsCurve2<T>)`: limb 2 is the composite hull of `|S(P(t)) − C(t)|`, the wall composed with the image as two nets at one scalar. `edge_nurbs`'s own callers (`plane_nurbs_limbs`, the pair lane's `exact_chart_image`) hand the image to the same reader.

So an image typed `NurbsCurve2<f64>` would be lifted by each of those readers instead (the composition needs the image's poles at `T`), which moves the conversion downstream and multiplies it. Keeping it `f64` by type means `Pcurve<T>`'s two fitted arms hold `f64` nets and the composition takes a mixed pair: a redesign of the fitted lane, and a change to C6's sentence above, which is ratified text.

## The question

Which of these is meant:

- **A.** The fit's lift is C6's and stays where it is: one `from_computed` per fit, at the fit. Then `from_computed` has two callers by design (the solved-value door and the fit), and the gate of `a-solved-value-the-body-stores-is-admitted-by-its-check` counts this one as listed.
- **B.** The fitted image is `f64` data by type: `Pcurve<T>::Fitted`/`General` hold `NurbsCurve2<f64>`, and the certificate composes an `f64` image with a `T` wall. C6's sentence is re-worded, which waits for Ev.

Nothing in DL5.2's table settles it: the image is not "data built whole at `f64`" that could be built at `T` (the fit is a search over measured feet, structure by C6), and it is not a solved value the door admits by a residual (the certificate's limbs are its check, and they run after it is held).

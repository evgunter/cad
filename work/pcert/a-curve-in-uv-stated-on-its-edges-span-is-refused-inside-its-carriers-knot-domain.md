---
id: a-curve-in-uv-stated-on-its-edges-span-is-refused-inside-its-carriers-knot-domain
kind: issue
title: A degree-1 curve in UV stated on its edge's span, inside its spline carrier's knot domain, is refused NoGeneralLane (Envelope) where the same line on the carrier's knots certifies
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by the review of BOXFINISH's iso-image unit (PR 7393, 2026-11-23)
and measured again in its fix pass.

**What happens.** A straight line of a plane's chart that is neither a
row nor a column, over a degree-1 spline carrier on the knots
`[0, 0, 1, 1]`, stated as a `Pcurve::General` and certified through
`EdgeCurve::certify` (`EdgeDescriptionSpec::chart_image`):

- on the CARRIER's knots (`[0, 0, 1, 1]`, through the line's values at
  0 and 1): certifies, over the whole domain and over sub-spans, at
  `f64` and at `Interval`;
- on the EDGE's span (`[t0, t0, t1, t1]` with `0 < t0 < t1 < 1`,
  through the line's values at `t0` and `t1`): refused
  `CertifyError::NoGeneralLane { chart: Plane, carrier: Nurbs, gap: Envelope }`.
  Underneath, `PcurveCache::certify_stated_bound` answers
  `FittedCertificate { what: "the tensor composite refused the
  carrier/pcurve pair (mismatched channel counts or knot domains — the
  shared-parameter identity is the entry requirement)" }`
  (`crates/geom-brep/src/pcurve_cache/stated.rs`, the `StatedDoor::Patch`
  arm: `lane.fitted_certificate(carrier, &net, chart, span.params, band)`).

The two images are the same map on the same parameter over the edge's
span; only the knot vector they are written on differs.

**Evidence.** The review's probe
(`origin/boxfinish/review-iso-image` @ `f0e5cce7c`,
`probe/review-iso-image/diagonal-merge-base.txt`): 112 sub-span draws
refused at both scalars, the twin on the carrier's knots certified
200 of 200. Pinned at the head as what the door does today:
`crates/geom-brep/src/pcurve_cache/analytic_image_rows.rs`
`a_slanted_line_is_stated_as_a_curve_in_uv_on_its_carriers_knots`
(three spans certify on the carrier's knots; two sub-spans refuse on
their own).

**Why it surfaced now.** Before PR 7393 a slanted line could be stated
as a `Pcurve::IsoLine { p0, pl }`, which the stated door turned into a
degree-1 net on the carrier's knot domain itself
(`stated.rs` `iso_net(p0, pl, domain)`, `domain` the carrier's
`net.domain()`), so no caller wrote the knots. An iso image now holds a
row or a column of its chart, and a slanted line is a `General` whose
knots the caller writes. No shipped caller states one: on `main` at
`e1830a1c` every non-test constructor of an `IsoLine` is a row or a
column (`Pcurve::iso_row`/`iso_column`; `topo` `pcurves/mint.rs`'s
measured rows and columns, whose other slope is a literal zero;
`replace_face`'s shift, which keeps the slope; `step-import`'s
`IsoMap::image`), and `pncad`, `verbs` and `editor-core` state no chart
image. Suites did.

**Owed.** Either the composite reads an image on any knots that cover
the edge's span (re-knotting the image onto the carrier's domain, or
restricting the carrier to the span), or the door says in its refusal
that the image must be on its carrier's knots. Not checked: whether a
`General` on a curved chart (`PatchClass::Enclosed`) refuses the same
way; whether a carrier with interior knots asks the image for them too.

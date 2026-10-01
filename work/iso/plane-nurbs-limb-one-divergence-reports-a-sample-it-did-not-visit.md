---
id: plane-nurbs-limb-one-divergence-reports-a-sample-it-did-not-visit
kind: issue
title: edge_nurbs refusal() reports a limb-1 foot divergence as image-schedule sample 9, discarding the parameter
status: open
opened: 2026-10-01
---


## Finding

Found by PCERT's `pcurve-fit-refusal-drops-the-domain-doors-reason`
lane while sweeping the sample schedule's copies.

`crates/geom-brep/src/edge_nurbs.rs`, `refusal` (the `SsiError` →
`PlaneNurbsRefusal` map): the `SsiError::FootPointInconclusive { t,
last_distance }` arm discards `t` (`let _ = t;`) and reports
`PlaneNurbsRefusal::FootPointInconclusive { sample: CERT_SAMPLES, .. }`.
Its comment says the divergence is "reported at the sample the
parameter names", which it is not:

- `sample` on that variant is documented as "the schedule sample
  index", and the schedule `chart_image` walks is the 33-point
  `PXN_FIT_SAMPLES` one; `CERT_SAMPLES` = 9 is a real index there (the
  parameter `t0 + (t1 − t0)·9/32`), which limb 1 never projects at.
- Limb 1 (`ssi/certify.rs`, `nurbs_limbs`) re-projects at
  `certify::sample_param(t0, t1, k)`, k ∈ 0…8, which is the image
  schedule's sample `4k` bit for bit (both read
  `certify::schedule_param`; pinned by
  `certify::tests::the_schedule_assigns_its_ends_and_the_image_schedule_contains_the_certificates`).

So a limb-1 divergence at, say, the certificate's sample 2 renders as
"did not converge at schedule sample 9", and the mint path
(`pcurve_cache::general_image_lane`) would turn that 9 into a
`LastFootDistance { t }` the projection never ran at, were the
certificate's refusal ever routed there.

## The fork

Either the variant reports the parameter (`t`, as `SsiError` and
`FittedMagnitude::LastFootDistance` already do), or `refusal` recovers
the image-schedule index `4k` from `t` against the carrier's domain
(it does not have the domain in hand today). The first changes the
variant's payload and every reader of `sample`; the second keeps the
vocabulary. Not decided here.

## A second instance: `chart_foot`

`crates/geom-brep/src/edge_nurbs.rs`, `chart_foot` (the single-point
foot the pcurve mint's rim arms read) maps a non-converging projection
to `PlaneNurbsRefusal::FootPointInconclusive { sample: 0, .. }`. The
point it projects is an edge endpoint, on no schedule at all, so the
`sample` it reports names a schedule position it never visited. Its
mint-side caller (`pcurve_cache::chart_foot_lane`) already drops the
index for `FittedMagnitude::EndpointFootDistance`, which carries none;
the lane's own vocabulary still says "schedule sample 0". The same fork
as above decides it (a parameter, or no position, rather than an index
standing in for one).

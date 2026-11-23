---
id: the-spline-pair-limbs-take-params-a-spline-ignores
kind: issue
title: plane_nurbs_limbs, nurbs_pair_limbs and certify_branch take params and ignore them for a spline; about 32 sites wrap a spline as Curve3 with a clone per certification
status: open
opened: 2026-10-14
priority: P3
cost: M
---


Found by the review of PR 5097 (`pcert/spline-pair-any-carrier`).

## What happens

`geom_brep::plane_nurbs_limbs`, `nurbs_pair_limbs` and
`ssi::certify::certify_branch` take the edge's interval `params`
beside a `&Curve3`. For a closed form it is the certificate's span;
for a spline `carrier_net::carrier_span` reads the spline's own knot
domain and ignores `params`. That is the rule today (the spline's
pcurves are fitted on its own domain), but a signature that takes an
argument and ignores it for one kind invites a caller to pass a
sub-interval and believe it certified only that.

The spline-holding callers spell the conversion by hand, about 32
sites: `&Curve3::Nurbs(Arc::new(c.clone())), c.knot_values().domain()`
(`ssi::certify_rung3`, `ssi::certify::certify_located`,
`ssi/ends.rs`'s `certified`, `edge_nurbs::analytic_rung3` and the test
suites). Each clones the net once per certification, and there is no
constructor that says what the pair means.

## What closes it

Either a spline-carrier entry that takes no `params` (and a
constructor for the `(Curve3, span)` pair the others take), or a
certificate span for splines that honours `params`, with the
sub-interval question for a spline's pcurves settled first.

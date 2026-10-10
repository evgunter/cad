---
id: fitted-pcurve-envelope-re-decides-the-ssi-hull
kind: issue
title: pcert: the fitted pcurve lane's pcurve_envelope check re-decides the hull_sup that certify_rung3 already decided at the same band
status: open
opened: 2026-10-10
priority: P3
cost: E
---


(Found by `encl/plane-nurbs-limbs-once`'s sweep for a door re-deciding
a lane's already-decided output; not otherwise verified.)

## What

`crates/geom-brep/src/pcurve_cache.rs`, the fitted lane's check 4
(`run_fitted_checks`, after `lane.fitted_certificate`), calls
`check_residual("pcurve_envelope", PcurveCheck::Envelope, …,
Margin::of(ssi.hull_sup), band, …)`. `ssi` comes from
`pcurve_cache::fitted_lane`, which returns `ssi::certify_rung3(…, band)`:
`ssi::certify::certify_branch` has already decided each operand's hull
bound zero at the same `band` (`ssi_hull_sup` / `ssi_hull_sup_chart`),
and `ssi.hull_sup` is their `max`.

This is the shape `encl`'s `certify-decides-the-plane-nurbs-limbs-twice`
removed from `geom-brep/src/certify.rs`. There the second decision was
unreachable at `f64`, `Interval` and the recording scalar: `max` of
values each decided `Zero` decides `Zero`, since the `f64` max is one
of them and the `Interval` max lies inside the operand with the larger
`hi`. At `Sym<T>` it was reachable in principle, because the `max`
node folds a zero form only against another zero form. Each hull here
is `T::from_f64` of a constant, so `Sym`'s A0 fold of constants should
make this one unreachable at every scalar too. Confirm that before you
delete it.

## Repair shape

Confirm the reachability. If the decision is unreachable, delete the
`check_residual` and keep `ssi` as the certificate's envelope, so that
the hull limb is decided once, inside the lane. The `pcurve_envelope`
k-stream rows from this site then disappear; other lanes' rows under
that name stay.

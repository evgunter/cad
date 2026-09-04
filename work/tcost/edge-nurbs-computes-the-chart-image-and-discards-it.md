---
id: edge-nurbs-computes-the-chart-image-and-discards-it
kind: issue
title: edge_nurbs derives a full chart image at certification time and returns only scalars — the compute-and-discard sibling of TCOST-K3
status: open
opened: 2026-09-03
---


The TCOST-K3 shape — *a derivation is computed to decide something,
the decision is returned, and the object is thrown away* — has a
named sibling that another spec already found and could not use.

`crates/geom-brep/src/edge_nurbs.rs:330`-`:336` derives the chart image
of a declared carrier on a NURBS wall (33 certified foot points,
interpolated on the carrier's own parameter, `on_carrier_domain`-lifted)
at EDGE CERTIFICATION time, and returns only the `PlaneNurbsLimbs`
scalars. `docs/PCURVE-P2-SPEC.md:59` names this site verbatim: it says
the derivation "already derives exactly this image ... and then THROWS
IT AWAY", and that a P-2 consumer needing that image should prefer an
existing producer to writing a third.

So there are two consumers of one derivation and no way to hand it
over — the TCOST-K3 situation exactly, one layer down.

**Why this is a candidate and not yet a unit.** Unlike K3, the cost
has not been measured: nobody has instrumented how much of an edge
certification the image is, nor how often a caller that certifies also
wants it. K3's own stop clause is the model — measure first, and if
the second derivation is a small fraction of its caller, the
redundancy is not what the finding says.

**Places to look**, from the K3 sweep for this shape:

- `crates/topo/src/census.rs`' `census_and_certify` — called by the
  tier-3′ door immediately after its check-7 certificate, and its
  product is a verdict vector;
- `PropsQuadLane::recertify_approx` — re-derives a surface certificate
  per validation pass, by design (tier 3's never-trust posture), which
  is the case where the discard is CORRECT and the unit would be wrong
  to collapse it. It is listed so the sweep's blind spot is stated:
  not every compute-and-discard is redundant, and telling them apart
  is the work.

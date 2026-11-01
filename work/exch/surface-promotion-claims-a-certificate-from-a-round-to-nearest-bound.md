---
id: surface-promotion-claims-a-certificate-from-a-round-to-nearest-bound
kind: issue
title: recognize's whole-patch surface promotion calls its f64 residual bound a certificate, but sums it in round-to-nearest throughout; should it re-check at Interval?
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [curve-recognition-roots-its-certified-sups-to-nearest]
---

Found by PCERT while closing `nurbs-stretch-bounds-round-to-nearest`. It
is a question for the owner, not a measured defect.

## What happens

`recognize.rs`'s cylinder promotion (`enveloped_residual_sup`, called
from the cylinder arm at `recognize.rs:339` and the row at `:717`)
documents itself as "the certificate": no promotion without a
whole-patch certificate. The bound is
`sup |r| ≤ grid sup + L·(Du·δu + Dv·δv)`, compared against `eps_in`
(`recognize.rs:343`). Every part of it is computed at `f64` in
round-to-nearest:
- the grid residuals;
- `Du`, `Dv` from `NurbsSurface::stretch_sup`, documented as
  round-to-nearest at f64 and certified only at `Interval`;
- the products and sums.

D4 ¶2 calls the f64 lane a conservative estimate and the Interval lane
the enclosure.

Measured for the stretch part only, in PCERT's census over
step-import's suite: the f64 `stretch_sup` sat inside the Interval
enclosure of the same net every time (364 readings), at most 9e-15
relative below its upper end. The error is ulp-scale, so it matters
only at the threshold. It is recorded because the wording says
"certificate". The sibling `curve-recognition-roots-its-certified-sups-to-nearest`
is the same shape on the curve side.

## The question

Is the surface promotion meant to be certified (re-check the envelope
at `Interval`, or round each term outward), or is it an f64 estimate
by D4 ¶2, whose doc should then stop calling it a certificate?

## Addendum (PCERT, from PR 5128's review, 2026-10-14)

The question is wider than the stretch term. Per PR 5128's reviewers,
the promotion's bound also reads the grid residual, which is
round-to-nearest at f64 too and dominates the bound. So making only
`stretch_sup` outward would not make the f64 promotion certified. The
choice is about the whole bound: certify it at Interval, or word the
f64 lane as an estimate.

---
id: fitted-certificate-kinds-told-apart-by-an-option
kind: issue
title: PcurveCertificate tells its fitted kinds apart by statement plus ssi.is_some(), so an inconsistent pair is representable
status: closed
opened: 2026-10-01
priority: P2
closed: 2026-10-08
branch: pcert/projected-image
---


Found by the PCERT review of PR 3733 (Q7), filed by that PR's lane.

A `Pcurve::Fitted` row now carries one of two certificates:

- over a rung-3 carrier, the SSI C2 certificate: `statement` is
  `OnLocusHull` on an analytic chart or `MapResidualComposite` on a
  spline chart, and `ssi` is `Some`;
- over an exact `Curve3::Circle` on a sphere, the Hermite bound:
  `statement` is `MapResidualHermite`, and `ssi` is `None`.

`geom_brep::PcurveCertificate` holds the two as independent public fields
(`statement: EnvelopeStatement`, `ssi: Option<SsiCertificate<T>>`,
`crates/geom-brep/src/pcurve_cache.rs`). So `OnLocusHull` with `ssi: None`,
or `MapResidualHermite` with `ssi: Some`, is representable, and a reader
re-derives the pairing. D9 row 0 asks whether that state can be made
unrepresentable: an enum over the certificate's kinds, with the SSI
certificate carried by the arms that have one.

It is not local. The struct's fields are public and read by the tier-3
pass and by test rows in `geom-brep`, `topo` and `sweep` (`cert.ssi`,
`cert.statement`), and every lane's constructor writes both. PR 3733 left
the type as it is.

## Closed (branch `pcert/projected-image`, 2026-10-08)

The Hermite kind is retired, and with it every lane that wrote `ssi:
None` beside a fitted statement. `PcurveCertificate::ssi` is private.
The two constructors are `PcurveCertificate::closed` (no pair
certificate: the closed-form, focal, iso and projected lanes) and
`PcurveCertificate::composite` (the SSI certificate, written only by
`run_fitted_checks` for `MapResidualComposite` on a spline chart).
Readers use `ssi()`. A pair inconsistent with its statement can no
longer be constructed.

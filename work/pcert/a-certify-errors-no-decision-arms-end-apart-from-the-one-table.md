---
id: a-certify-errors-no-decision-arms-end-apart-from-the-one-table
kind: issue
title: CertifyError's no-decision arms end apart from the one decision table
status: open
opened: 2026-11-04
priority: P2
cost: E
---


(PCERT implementer, from `two-decision-tables-for-one-ending`.)

## What

`CertifyError::decided` (`crates/geom-brep/src/certify.rs`) names no
decision for seven arms (`UnresolvedSurface`, `Unimplemented`,
`IntersectionSameSurface`, `WrapOnNonPeriodic`,
`TangentCertificateUnsupported`, `StatedInterval`, `Band`), so
`CertifyError::ending` gives them none and `CertifyError::render`
writes the payload alone. Two other readers of the same refusals end
them by kinds the one decision table (`recourse::Decision`) already
writes:

- **`Band`.** At the edge door `CertifyError::render` writes
  `BandError`'s own `Display`, whose recourse for `InvalidValue` is
  "for a band derived from the tolerance, set a smaller tolerance or
  name a longer lever arm…" (`geom_core::BandError::recourse`). The
  pcurve door ends the same run's linear band in
  `Decision::Lever(Lever::Tolerance)`, `BAND_TOLERANCE_RECOURSE`, and a
  lever arm by the band's own recourse
  (`PcurveCertifyError::decision`). The at-rest classifier ends every
  `CertifyError::Band` in `TOLERANCE`, which is
  `BAND_TOLERANCE_RECOURSE`, the lever-arm arm included
  (`classify_certify`, `crates/topo/src/validate.rs`). One fact ends
  three ways.
- **`Unimplemented`, `TangentCertificateUnsupported`.** No ending at
  the edge door; "not yet" (`NOT_YET_ENDING`) at rest
  (`classify_certify`). The table's `Decision::NotYet` is that ending.
- **`IntersectionSameSurface`, `WrapOnNonPeriodic`.** No ending at the
  edge door; the defect at rest. The table's
  `Decision::Unsized(Unsized::Defect)` is that ending.

Left as they were by `two-decision-tables-for-one-ending`, which was
a refactor and could move no text.

## Repair shape

Give each arm its decision in `CertifyError::decided`, and read the
at-rest classifier's ending of it off `CertifyError::ending` rather
than its own constants. The `Band` arm takes the pcurve door's split.
The moved goldens are the point of the change.

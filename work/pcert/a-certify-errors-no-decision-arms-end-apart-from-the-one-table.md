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
decision for six arms (`UnresolvedSurface`, `Unimplemented`,
`IntersectionSameSurface`, `WrapOnNonPeriodic`,
`TangentCertificateUnsupported`, `Band`), nor for `Row` on
`RowRefusal::OffSide` (`RowRefusal::decision`), so
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
  The lever-arm arm of that at-rest `TOLERANCE` is unreachable through
  `CertifyError` today: every producer of `CertifyError::Band` builds
  the run's band with `Band::linear` (`topo`'s `euler`, `euler_kill`,
  `split` and `attach`), which refuses only `InvalidValue` or `Empty`,
  never `InvalidLeverArm`. So the live disagreement is the edge door's
  `BandError` sentence against `BAND_TOLERANCE_RECOURSE` on those two
  arms. That is what keeps this row at P2.
- **`Unimplemented`, `TangentCertificateUnsupported`, `Row` on
  `OffSide`.** No ending at
  the edge door; "not yet" (`NOT_YET_ENDING`) at rest
  (`classify_certify`). The table's `Decision::NotYet` is that ending.
- **`IntersectionSameSurface`, `WrapOnNonPeriodic`.** No ending at the
  edge door; the defect at rest. The table's
  `Decision::Unsized(Unsized::Defect)` is that ending.

Left as they were by `two-decision-tables-for-one-ending`, which was
a refactor and could move no text. Its census row
(`every_refusal_ends_through_the_one_decision_table`,
`crates/topo/src/test_support_samples.rs`) lists these arms by name in
`no_decision`: an arm this row gives a decision comes off that list.

## Repair shape

Give each arm its decision in `CertifyError::decided`, and read the
at-rest classifier's ending of it off `CertifyError::ending` rather
than its own constants. The `Band` arm takes the pcurve door's split.
The moved goldens are the point of the change.

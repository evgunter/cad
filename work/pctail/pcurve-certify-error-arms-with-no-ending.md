---
id: pcurve-certify-error-arms-with-no-ending
kind: issue
title: geom-brep: fourteen PcurveCertifyError arms end in no recourse (PcurveCertifyError::ending gives None)
status: open
opened: 2026-10-02
---

(SSI implementer, from the §5 sweep of PR "SSI: every SsiError arm ends
in the recourse its decision earns", which closed
`ssi-refusals-whose-decision-has-no-ending`.)

## What

`geom_brep::PcurveCertifyError::ending`
(`crates/geom-brep/src/pcurve_cache.rs`, ~1779) returns `None` for
fourteen arms: `UnsupportedChart`, `UnsupportedCarrier`,
`CarrierOffChart`, `ImageMismatch`, `FittedLaneUnsupported`,
`FittedMateMissing`, `ArcNearPole`, `IsoUnsupported`, `ChartRow`,
`FittedCertificate`, `CarrierDomain`, `ChartWindingUnsupported`,
`PlaceholderChart` and `Band`. The match ends in a grouped `return None`,
so a new arm can join the group silently.

The one reader that ends them is topo's at-rest validator
(`crates/topo/src/validate.rs`, the `PcurveCertifyError` arm of its
classifier, ~2765), which falls back to a second table of its own
(`DEFECT`, `NOT_YET`, `TOLERANCE`) where `ending` gives `None`. D4 ¶1 (i)
wants one ending per decision, written once; any other door that reports
these arms reports no recourse.

`FittedCertificate` flattens an `SsiError` (`ssi_refusal`, ~2393). Since
the SSI PR above, every `SsiError` arm has an ending
(`SsiError::ending` returns `String`), and the flattening drops it.

## Repair shape

As the SSI PR did: name each arm's decision in `ending`, end by an
existing table (`PcurveCheck::recourse`, `crate::recourse::Unsized`, the
defect and not-yet endings), make the match exhaustive with no grouped
`None`, and retire the validator's fallback table once nothing reaches
it. `FittedCertificate` can carry the SSI ending it flattened.

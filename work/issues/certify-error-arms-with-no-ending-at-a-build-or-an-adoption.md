---
id: certify-error-arms-with-no-ending-at-a-build-or-an-adoption
kind: issue
title: geom-brep: seven CertifyError arms end in no recourse at a build or an adoption; only topo's at-rest validator gives them one, from a second table
status: open
opened: 2026-10-02
---

(SSI implementer, from the §5 sweep of PR "SSI: every SsiError arm ends
in the recourse its decision earns", which closed
`ssi-refusals-whose-decision-has-no-ending`. Filed here because no open
program claims `crates/geom-brep/src/certify.rs`.)

## What

`geom_brep::CertifyError::decision` (`crates/geom-brep/src/certify.rs`,
~630) returns `None` for `UnresolvedSurface`, `Unimplemented`,
`NurbsLaneNotSupplied`, `IntersectionSameSurface`, `SeamOnNonPeriodic`,
`TangentCertificateUnsupported` and `Band`. So `CertifyError::ending`
gives `None` and `CertifyError::render` shows the payload alone. The
`PlaneNurbs` arm inherits `PlaneNurbsRefusal`'s own `None`s
(`work/iso/plane-nurbs-refusal-arms-with-no-ending.md`).

Readers:

- **At rest:** topo's validator (`crates/topo/src/validate.rs`,
  `classify_certify`, ~2437) ends each from a table of its own
  (`DEFECT`, `NOT_YET`, `TOLERANCE`, and a sentence for
  `NurbsLaneNotSupplied`). That is a second home for one decision's
  ending (D4 ¶1 (i)).
- **At a build:** `topo::EulerOpError::render`
  (`crates/topo/src/euler.rs`, ~1213) shows no recourse.
- **At an adoption:** `step_import`'s error `Display`
  (`crates/step-import/src/error.rs`, ~419 and ~434) shows no recourse.

## Repair shape

Give each arm its ending in geom-brep, by reading where one depends on
it (`defect_ending`), and make `ending` total, as `SsiError::ending` now
is. The validator's fallback table can then go.

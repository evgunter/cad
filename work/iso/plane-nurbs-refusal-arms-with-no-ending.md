---
id: plane-nurbs-refusal-arms-with-no-ending
kind: issue
title: geom-brep: PlaneNurbsRefusal's FootPointInconclusive and Unsupported end in no recourse, and its SSI mapping folds every other SsiError arm into Unsupported
status: open
opened: 2026-10-02
---

(SSI implementer, from the §5 sweep of PR "SSI: every SsiError arm ends
in the recourse its decision earns", which closed
`ssi-refusals-whose-decision-has-no-ending`.)

## What

1. `geom_brep::PlaneNurbsRefusal::decision`
   (`crates/geom-brep/src/edge_nurbs.rs`, ~328) returns `None` for
   `FootPointInconclusive`, `PcurveFit`, `CarrierDomain` and
   `Unsupported`, so `PlaneNurbsRefusal::ending` gives none. `PcurveFit`
   and `CarrierDomain` carry their ending inside `Display`; the other two
   carry none. Reached through `CertifyError::PlaneNurbs`, they render
   with no recourse at a build or an adoption. topo's at-rest validator
   ends both as "There is no way through yet"
   (`crates/topo/src/validate.rs`, `classify_certify`, ~2437).
2. That is a second answer for one fact. `SsiError::FootPointInconclusive`
   now ends by limb 1's decision (`SsiLimb::OnLocus.check()`, i.e.
   `CertCheck::PlaneNurbsOnLocus`): the last resort at a build, and the
   kernel-or-file defect at rest.
3. `edge_nurbs::refusal` (~812) maps an `SsiError` into this lane's
   vocabulary with a wildcard. Every arm it does not name
   (`TubeLadderEmpty`, `TubeProbeSilent`, `TubeDegenerate`, among others)
   becomes `Unsupported { what: "…outside this lane's vocabulary" }`, and
   the SSI arm's own ending is lost. The wildcard also lets a new
   `SsiError` arm in without anyone choosing where it goes.

## Repair shape

Give `FootPointInconclusive` limb 1's decision, as `SsiError`'s does.
Give `Unsupported` the not-yet ending. Make `refusal` exhaustive by
variant, as `pcurve_cache::ssi_refusal` is, carrying the SSI ending
where the lane has no arm of its own.

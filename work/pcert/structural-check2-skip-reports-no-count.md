---
id: structural-check2-skip-reports-no-count
kind: issue
title: The _structural doors' check 2 drops every verdict on an edge whose certificate needs the NURBS lane, and says nowhere how many it dropped
status: open
opened: 2026-10-12
priority: P3
cost: M
---


Found by PR 4791's review (both reviewers).

The tier-3 battery's check 2 (`crates/topo/src/validate.rs`, the
`recertify_via(.., nurbs_lane)` match in the per-edge pass, near line
6616) treats `Err(CertifyError::NurbsLaneNotSupplied)` as `Ok`. At the
`_structural` doors, which hand no lane, every edge of the three lane
classes (plane × NURBS, NURBS × NURBS, and a spline carrier between two
analytic surfaces; the list is on the variant's doc in
`crates/geom-brep/src/certify.rs`) is skipped whole. That includes the
endpoint, interval and chart-image verdicts, not only the lane's limbs.

The skip is deliberate, and it is documented at each `_structural`
door's signature: reporting it as a `ValidationError` would name a
defect in the body for a fact about the caller. What is missing is any
COUNT. A clean verdict from a `_structural` door on a body with N such
edges is indistinguishable from one on a body with none, and nothing in
the returned value or the certificate says that N edges went unchecked.

Open: whether the count rides on the clean verdict (the certificate
form already returns a `SignCertificate`), on a separate return, or is
not wanted because the door's doc is the statement. Today the
`_structural` doors have no production caller outside validate.rs, so
this is latent.

Since `pcert/fitted-row-reads-carrier-and-chart` (PR 4900), these doors no longer re-derive the uniqueness tube through the fitted-grade row either. The row's check 4 reads its chart alone, so at those doors nothing re-derives the tube of a skipped edge. That is what the doors document, and no lane is added for it (H5 ruling 3).

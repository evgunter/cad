---
id: the-tube-limbs-undecided-arm-names-a-limb-not-its-crossing-angle-bound
kind: issue
title: geom-brep: limb 3's undecided arm names the limb, not the crossing-angle bound its definite arm and check word name
status: open
opened: 2026-10-20
---


(Filed by the ENCL lane on `encl/check-word-one-noun`, from its class sweep of check words against their definite payloads.)

## What

Limb 3's decision is `CertCheck::TransversalityTube` (`SsiLimb::check`, `crates/geom-brep/src/ssi/certify.rs`). Its check word and every definite refusal of it now read one noun, `TUBE_CROSSING_BOUND` in `crates/geom-brep/src/certify.rs` ("the uniqueness tube's lower bound on the crossing angle"): the word itself, `SsiError::TubeStraddles` (`crates/geom-brep/src/ssi.rs`), and both `TubeStraddles` payloads in `crates/geom-brep/src/edge_nurbs.rs`.

Its undecided arm at two doors renders through `SsiLimb::name` instead, which names the limb, not the quantity:

- `SsiError::CertificateEscalated`'s `Display` (`crates/geom-brep/src/ssi.rs`), through `SsiDecision::Limb`: "ssi: whether the fitted carrier passes limb 3 (uniqueness tube) is undecided: …".
- `PlaneNurbsRefusal::Escalated`'s `Display` (`crates/geom-brep/src/edge_nurbs.rs`): "limb 3 (uniqueness tube) escalated: …". By `test_utils::refusal::subjectless_escalations`' rule its clause is "limb 3 escalated" (the closed group dropped), which names no quantity and no question, so a guarded row that rendered it would go red. Today the edge certifier lifts this variant into `CertifyError::Escalated` (`from_plane_nurbs` in `crates/geom-brep/src/certify.rs`), whose words are the check's, so the limb-named wording shows only where the lane's own `Display` renders.

Limbs 1 and 2 have the same shape: their `SsiLimb::name` is a limb label with the quantity in parentheses, and their check words (`PlaneNurbsOnLocus`, `PlaneNurbsHull`, …) say the quantity.

## Repair shape

Render the lane's escalation through the check's word (`CertifyError::Escalated { check: limb.check(), .. }`), as `PlaneNurbsRefusal::TransversalityEscalated` now does, or give `SsiDecision::Limb` the check's noun. Both of `SsiLimb::name`'s other uses ("{limb} measured …", "refused at {limb}") name the limb as a stage, which is right there.

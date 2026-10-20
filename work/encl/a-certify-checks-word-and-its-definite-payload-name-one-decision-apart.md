---
id: a-certify-checks-word-and-its-definite-payload-name-one-decision-apart
kind: issue
title: geom-brep: a CertCheck's word and its definite variant's payload name one decision in different nouns
status: closed
opened: 2026-10-20
priority: P4
cost: E
branch: encl/check-word-one-noun
pr: 5447
closed: 2026-10-20
---


(Filed by the ENCL lane on `encl/one-noun-per-decision`, from its class sweep.)

## What

A certification decision's undecided arm renders through its check's word (`CertCheck`'s `Display` in `crates/geom-brep/src/certify.rs`: "{check} at sample N escalated: …"), and its definite arms through their variant's payload (`impl Display for CertifyError`). For these decisions the two name the one quantity in different nouns:

- `CertCheck::ParamSpan` says "the stored interval's span"; `IntervalNotForward` and `StatedSpan` say "the stored parameter interval … (length … m)".
- `CertCheck::ParamWinding` says "the stored interval's headroom to one full period"; `StatedSpan::Winding` says "… one full turn of its carrier (headroom … m)".
- `CertCheck::ParamSpanMeter` says "the spline carrier's metered length"; `SpanMeterCollapsed` says "the spline carrier's knot domain, metered at its certified speed floor".
- `CertCheck::Transversality` says "the transversality margin"; `NotTransverse` says "the faces are not shown to cross". The plane × NURBS lane's own undecided arm (`PlaneNurbsRefusal::TransversalityEscalated`'s `Display`, `crates/geom-brep/src/edge_nurbs.rs`) says a third thing: "whether the plane and the NURBS wall cross at interior sample N is undecided".
- `CertCheck::TransversalityTube` says "the uniqueness tube's transversality bound"; its definite arm, `PlaneNurbsRefusal::TubeStraddles` (`crates/geom-brep/src/edge_nurbs.rs`), says "the uniqueness tube's transversality is not certified clear of the zero band".
- `CertCheck::TangentSecondOrder` says "the second-order margin"; `NotSecondOrderSeparated` says "the faces … curve apart at second order".
- `CertCheck::TangentTube` says "the second-order tube bound"; `TubeNotSeparated` says "the certified lower bound on how far the faces curve apart".

`CertCheck::TransversalityArm` was the same shape. It now reads `DIHEDRAL_ARM_NOUN`, "the width the faces' angle is measured across". That is the word of `DIHEDRAL_ARM`'s size ("width or the gap its faces open") and of its clause ("between faces wide enough, …").

## Repair shape

Per decision, one noun home read by the check's word and by its definite payloads, as the arm's is. Pins that move: `crates/step-import/tests/tier_gate.rs` (the ParamSpan escalation, and "the transversality margin at sample 1 escalated" ~387), `crates/editor-core/tests/refusal_concision_chains.rs` ("the transversality margin at sample 4 escalated" ~3162), `crates/test-utils/src/refusal.rs` (the same phrase in `subjectless_escalations`' self-test ~692 and `QUANTITY_WORDS`' doc), `crates/geom-brep/src/certify.rs` ("the transversality margin at sample 3 escalated" ~4485), `crates/geom-brep/src/certify.rs`'s check-word and payload tests; `crates/test-utils/src/refusal.rs`'s `subjectless_escalations` self-test quotes "the stored interval's span" as an example only. Mind `QUANTITY_WORDS` (`crates/test-utils/src/refusal.rs`): an escalated check's word has to keep a quantity word in its last clause.

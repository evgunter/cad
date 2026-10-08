---
id: carriers-compare-in-canonical-form
kind: issue
title: D10 stage 4 PR C: rung 2 at the door: CarrierForm per kind, LinForm over VarId with exact rational coefficients, each verb's CarrierFlow, and the lint's recourse
status: parked
opened: 2026-10-08
priority: P0
cost: H
design: true
blocked_on: [coincidences-are-recorded-at-one-door, operands-are-reads, select-defines-face-and-edge-variables]
---

INTENT stage 4, PR C. Spec: `docs/INTENT-STAGE4-SPEC.md` §4. Design open: FORK-S4-1 (provenance read from names vs `GeomSource`; N6) and FORK-S4-2 (a stated `CarrierFlow` with a witness vs derived).

`editor-core/src/canon.rs`: `LinForm` over `VarId` with exact rational coefficients (definitions expanded, nonlinear subterms opaque atoms), `PoseForm` modulo the kind's `Subgroup`, `CarrierForm` per kind. Each verb's `CarrierFlow` beside `ParamFlow`, with a corpus witness. The door gains `Rung::CanonicalForm` and the residual-driven recourse. A placement chain is one opaque pose atom until H. Needs stage 2 B (a frame is a read) and E (a face-read frame reduces). Releases 2 rows (spec §12).

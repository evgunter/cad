---
id: one-sector-side-refusal-has-three-constructions
kind: issue
title: CurvedSectorSideUnsupported is built from BoundSideFault::Charge, from ReadFault::Charge and directly by the held pierce: two Charge variants in two enums carry one verdict
status: open
opened: 2026-11-20
priority: P4
cost: E
---


Filed by GERM's orchestrator from PR 7174's delta review (2026-11-20, style, likely).

`CurvedSectorSideUnsupported` is constructed in three places:

- from `BoundSideFault::Charge(Refused)`, which PR 7174 says only rounding reaches;
- from PR 7174's new `ReadFault::Charge(Classified)`;
- directly by the held pierce (`vtxfac`).

The first two are variants named `Charge`, in two enums, carrying the charge's verdict. This is the fix minting a fresh instance of the class it closes. The `ReadFault` doc still reads "why a reading with no curvature charge read no side". `ReadFault::diag()` for `Charge` builds an `Indeterminate` from a decided Zero; it is unreached today.

Owed: one variant and one construction for the charge's refusal. Retire `BoundSideFault::Charge` if only rounding reaches it, and fix the doc.

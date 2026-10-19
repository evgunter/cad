---
id: a-properties-test-message-calls-the-probe-refusal-ratified
kind: issue
title: a properties-pane test's assertion message calls the range probe's refusal the ratified affordance
status: open
opened: 2026-09-28
priority: P4
cost: E
refs: [ratified-is-asserted-across-viewer-src-and-some-was-never-ratified]
---

`crates/viewer/src/pane/properties.rs`,
`tests::a_driven_slots_range_button_reads_the_refusal_the_probe_would_give`
(:1410 at `ace8aa976`), asserts with the message *"and it renders as
the ratified affordance, from its one home"*. G4
(`crates/viewer/GUI-DESIGN.md`, Micro-decisions) ratifies refusing a
DRAG on a driven dimension. This is a range probe, and G4 decides no
wording. The census in
`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`
found it PARTIAL. Its correction pass was comment-only and this is a
string literal, so it was left.

**Fix:** drop *"the ratified"*, e.g. *"and it renders as the
affordance, from its one home"*. No test outcome changes.

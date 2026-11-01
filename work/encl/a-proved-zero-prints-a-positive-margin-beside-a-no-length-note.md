---
id: a-proved-zero-prints-a-positive-margin-beside-a-no-length-note
kind: issue
title: geom-brep: a symbolic-tier proved zero prints its rounded f64 as a positive margin beside the ending's 'no length' note
status: open
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL orchestrator from PR 4976's delta review, MINOR 1.)

## What

A symbolic-tier proved zero on `CertifyError::IntervalNotForward` renders its payload with the rounding of the identity as a positive margin. The ending then attaches the decision's no-size note. The row `forward(proved)` in `certify::tests::each_zero_arm_payload_prints_its_reading_and_claims_no_size` (`crates/geom-brep/src/certify.rs`) renders:

> the stored parameter interval is not shown to run forward (margin 3.725290298461914e-9 m). Recourse: …; an edge of no length, or a reversed one, is one no kernel construction mints, so this is a kernel defect worth reporting

So a user reads a positive span next to "no length". Nothing in the payload says the number is the f64 image of a proved zero. The test skips its ending check for exactly this row, so no pin catches it.

## Question

The payload should not print a number the classification has overruled. It could print the proved zero as such (for example "(proved zero; f64 reads …)"), or print the margin the class stands on. The class comes from `MarginDiag::shows` (geom-core, SCALAR's), so the payload would read that one classification rather than a second one. The rule should be the same for every Zero-arm payload that can carry a proved margin.

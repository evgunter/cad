---
id: the-pole-twins-no-longer-witness-paramspan-at-import
kind: issue
title: step-import: no import row witnesses ParamSpan certification since the pole twins refuse at interpretation
status: open
opened: 2026-11-18
parent: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
priority: P3
cost: M
---


Found by the confirmation re-review of `importgate/interp-layer` (PR 7091).

## What

The pole twins (`crates/step-import/tests/tier_gate.rs:339-344`, the
poleguard and halfcap near-pole rows) used to reach edge adoption,
where their near-pole circle's span escalated at
`geom_brep::certify::CertCheck::ParamSpan`
(`crates/geom-brep/src/certify.rs:158`). Since the interpretation layer
reads that circle's 0.9e-9 m radius as a length the file's band cannot
tell from zero, they refuse at interpretation
(`InterpretationRefusal::LengthUnread`) and never reach the certificate.
No step-import row now asserts a `ParamSpan` verdict at the import door;
`tests/probe_refusals.rs:84` and `:158` mention it only in prose.

## Fix

A non-pole witness: a fixture whose edge's parameter span escalates at
the certificate while every length interpretation reads stands apart (a
short arc on a large circle, posed past the band), with a row asserting
the `ParamSpan` verdict and its ending.

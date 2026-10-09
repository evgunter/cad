---
id: last-resort-on-a-poisoned-margin-offers-to-loosen-the-tolerance
kind: issue
title: geom-brep: Unsized::LastResort on a poisoned margin advises loosening the tolerance, which no tolerance can answer
status: review
branch: encl/poisoned-last-resort
pr: 4453
opened: 2026-10-09
priority: P3
cost: E
---



(Filed by the ENCL orchestrator from the full review of PR 4443. Pre-existing.)

## What

`geom_brep::recourse::Unsized::LastResort`, on a refused arm whose margin is poisoned (`MarginDiag::INVALID`, NaN), ends with "loosen the tolerance, as a last resort". This is pinned at rest in `crates/topo/src/validate.rs` `certify_escalation_rows`, on the `Surface1Residual` row with `MarginDiag::INVALID`.

No tolerance makes a NaN margin readable. So that advice cannot be followed, which breaks D4 ¶1 (i): the ending must be a recourse the user can follow. The neighbouring `own_close` already maps poison to the defect ending for exactly this reason.

## Repair shape

On a poisoned arm, `Unsized::LastResort` ends with the kernel-defect ending, matching `own_close`. Re-pin the `Surface1Residual` INVALID row, and sweep the other `Unsized` arms for the same pattern. Texts move only on poisoned last-resort arms.

---
id: last-resort-on-a-poisoned-margin-offers-to-loosen-the-tolerance
kind: issue
title: geom-brep: Unsized::LastResort on a poisoned margin advises loosening the tolerance, which no tolerance can answer
status: closed
closed: 2026-10-09
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

## Closed

2026-10-09. PR 4453 merged at the head `a574cf1f13`; hosted CI was green. The orchestrator reviewed it: the production change is one guard line.
- `Unsized::LastResort` on an unreadable arm now ends in the defect ending, reusing `RefusedArm::unreadable`, which the margin docs allow as the one read. The reporting-margin-door counts are unchanged.
- `residual_in_file` follows the same path.
- **Pin:** `recourse::tests::no_ending_offers_a_tolerance_on_a_poisoned_margin`, red on main. It covers every table at every door on poisoned `Undecided`/`Zero` arms, plus a readable control.
- **The one moved text:** `topo` `certify_escalation_rows` `Surface1Residual`/INVALID now ends "There is no way through: this is a kernel defect or a damaged file; report it".
- **Follow-up:** `piece-sort-offers-to-loosen-the-tolerance-on-a-poisoned-role`.

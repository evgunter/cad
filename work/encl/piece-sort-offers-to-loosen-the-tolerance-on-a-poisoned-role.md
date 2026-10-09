---
id: piece-sort-offers-to-loosen-the-tolerance-on-a-poisoned-role
kind: issue
title: topo: PieceSortError::RoleUnread drops the role read's refusal and ends in "loosen the tolerance", a poisoned volume included
status: dispatched
branch: encl/piece-sort-poisoned-role
opened: 2026-10-09
priority: P3
cost: E
---


(Found by the sweep of the poisoned-last-resort fix, PR for `last-resort-on-a-poisoned-margin-offers-to-loosen-the-tolerance`. Pre-existing.)

## What

`topo::pieces::pieces_of` (`crates/topo/src/pieces.rs`) reads every shell's role with `ShellRead::of(..).and_then(Result::ok)`. That discards the shell's typed refusal (`ShellClassifyError`, whose `Escalated` arm carries the `Indeterminate`), and it also discards a selection that could not be read (`None`). Both become `PieceSortError::RoleUnread { shell }`. Its `Display` ends every one of them in `geom_core::KERNEL_LIMIT_RECOURSE`: "loosen the tolerance, as a last resort".

That includes a role read whose volume margin is poisoned (`MarginDiag::INVALID`). No tolerance makes that margin readable, so the ending cannot be followed (D4 ¶1 (i)). It is the same class `geom_brep::recourse::Unsized::LastResort` was fixed for: there a poisoned arm now ends in the defect ending. This site does not go through that table, and it no longer has the margin it would need to make the same test.

## Repair shape

Keep the role read's refusal on `RoleUnread`, or at least keep whether its margin was poisoned (`RefusedArm::unreadable`'s test, the one fact the margin may route on). Then end a poisoned read in the defect ending: the body may have been read from a file, so `KERNEL_OR_FILE_DEFECT_ENDING`. Also decide what an unreadable selection (`None`) ends in. Pin both texts.

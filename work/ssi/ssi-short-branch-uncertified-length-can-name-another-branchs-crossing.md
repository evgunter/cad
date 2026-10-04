---
id: ssi-short-branch-uncertified-length-can-name-another-branchs-crossing
kind: issue
title: ssi: ShortBranchUncertified's length can be the distance to another branch's crossing, not the branch's
status: open
opened: 2026-10-04
priority: P3
---


(Delta review of PR 3983 at `43998e7c81`, finding S5; predates that PR.)

## What

`Ends::branches` (`ssi/ends.rs`) sizes a branch by `near`, the distance from its crossing to the nearest crossing not yet used, and passes it to the Hermite candidate as `length`. Where branches converge, that crossing is another branch's, so `SsiError::ShortBranchUncertified { length }` and its rendering can name the gap to a neighbour, not the branch's own length. The march's step cap reads the same `near` (`StepCap::Crossing`), which is honest there (it is the distance the march must not jump); the refusal's text calls it the branch's length.

## Done when

The sized refusal names what it measured (the distance to the nearest unused crossing), or is sized by the branch's own length where one is known.

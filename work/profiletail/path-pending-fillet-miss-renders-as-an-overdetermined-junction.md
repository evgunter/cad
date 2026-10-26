---
id: path-pending-fillet-miss-renders-as-an-overdetermined-junction
kind: issue
title: profile: take_pending's missing fillet refuses OverdeterminedJunction, and two fillet-bookkeeping desyncs refuse UnderdeterminedLeg
status: parked
opened: 2026-10-13
priority: P3
cost: E
refs: [constructed-tangent-joint-refusals-are-typed-kernel-defects]
blocked_on: [profile-refusals-say-what-they-mean]
rides_with: profile-refusals-say-what-they-mean
---


## Finding

`PathError::OverdeterminedJunction` renders "the sketch junction at {site} is overdetermined" (`crates/profile/src/path.rs`); its one raise site, `take_pending`, refuses it when NO fillet is pending ("fillet resolution without an opened fillet") — a missing state, not overdetermination. `PathError::UnderdeterminedLeg` ("the sketch leg at {site} is underdetermined") is also raised for "a recorded fillet arc outside the chain it was recorded on" and "a recorded fillet arc whose segment is not an arc", which are bookkeeping desyncs.

Same shape as band's `BodyNotIntact` (PR 5021): one fixed cause sentence over sites with other causes. Found by that PR's §5 sweep.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEPROSE (`work/profileprose/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

## Rides with `profile-refusals-say-what-they-mean` (2026-10-26)

PROFILEPROSE folded into PROFILETAIL; its refusal-text rows travel as one unit. This row is that unit's spec for its part and closes with its own evidence when the unit lands.

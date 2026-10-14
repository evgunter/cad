---
id: path-pending-fillet-miss-renders-as-an-overdetermined-junction
kind: issue
title: profile: take_pending's missing fillet refuses OverdeterminedJunction, and two fillet-bookkeeping desyncs refuse UnderdeterminedLeg
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## Finding

`PathError::OverdeterminedJunction` renders "the sketch junction at {site} is overdetermined" (`crates/profile/src/path.rs`); its one raise site, `take_pending`, refuses it when NO fillet is pending ("fillet resolution without an opened fillet") — a missing state, not overdetermination. `PathError::UnderdeterminedLeg` ("the sketch leg at {site} is underdetermined") is also raised for "a recorded fillet arc outside the chain it was recorded on" and "a recorded fillet arc whose segment is not an arc", which are bookkeeping desyncs.

Same shape as band's `BodyNotIntact` (PR 5021): one fixed cause sentence over sites with other causes. Found by that PR's §5 sweep.

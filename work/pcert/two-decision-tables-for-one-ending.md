---
id: two-decision-tables-for-one-ending
kind: issue
title: The lane refusals and the pcurve door keep two decision-table types for one ending
status: open
opened: 2026-11-03
priority: P1
cost: M
---



(PCERT implementer, from the review of PR 6474, S7-S1.)

## What

Two private types name "the decision a refusal is a refused arm of" and
read its ending:

- `pcurve_cache::Decision` (`crates/geom-brep/src/pcurve_cache.rs`),
  read by `PcurveCertifyError::ending`;
- `edge_nurbs::LaneDecision` (`crates/geom-brep/src/edge_nurbs.rs`),
  read by `PlaneNurbsRefusal::ending` and `AnalyticRung3Refusal::ending`.

Their shared variants have already drifted once: `LaneDecision` held a
raw `&'static str` where `Decision` holds a `pcurve_cache::Lever`, and an
`SsiLimb` where `Decision` holds an `SsiDecision`. PR 6474 shares those
two (`Lever` is `pub(crate)`, `LaneDecision::SsiUndecided` holds the
`SsiDecision`), so both tables now read one lever table and one SSI
undecided sentence. The tables themselves are still two: a check of
each door (`PcurveCheck` against `CertCheck`), the fitted ending, the SSI
door's refusal, the one-arc proof and "not yet" are each a variant of
both, and a new ending kind has to be added twice.

## Repair shape

One `Decision` type in `recourse` (or `certify`) generic over the door's
check — `Decision<C: DoorCheck>`, with `DoorCheck::recourse(arm, at)`
implemented by `CertCheck` and `PcurveCheck` — and one `ending`. Each
refusal keeps its own exhaustive `decided()`.

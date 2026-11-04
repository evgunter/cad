---
id: two-decision-tables-for-one-ending
kind: issue
title: The lane refusals and the pcurve door keep two decision-table types for one ending
status: closed
opened: 2026-11-03
priority: P1
cost: M
closed: 2026-11-04
pr: 6493
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

## Closed

PR 6493. Every certification door's refusals (`CertifyError`,
`PcurveCertifyError`, `PlaneNurbsRefusal`, `AnalyticRung3Refusal`) name
their decision as a `recourse::Decision<C: DoorCheck>` in their own
exhaustive `decided()`, and end through its one `ending`; `LaneDecision`
and the pcurve door's private `Decision` are gone. No ending moved
text. `topo`'s `every_refusal_ends_through_the_one_decision_table`
holds each door to the table at every reading and every kind live.
Filed: `a-certify-errors-no-decision-arms-end-apart-from-the-one-table`,
`the-pcurve-mint-error-keeps-a-third-ending-table`.

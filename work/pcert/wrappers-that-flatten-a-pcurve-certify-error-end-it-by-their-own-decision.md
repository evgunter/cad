---
id: wrappers-that-flatten-a-pcurve-certify-error-end-it-by-their-own-decision
kind: issue
title: Three wrappers flatten a PcurveCertifyError and end it by their own decision, not the arm's
status: open
opened: 2026-11-03
---


(PCERT implementer, from the §5 sweep of `pcurve-certify-error-arms-with-no-ending`,
branch `pcert/certify-arms-end`.)

## What

`PcurveCertifyError::ending` now names one decision per arm, and every
door that nests the refusal whole (`read_at`, `render`) ends it by that
decision. Three wrappers do not nest it: they translate it into an arm of
their own enum, which then ends by the wrapper's decision, whatever the
pcurve arm's was.

- `geom_brep::certify`'s `mint_refusal` (`crates/geom-brep/src/certify.rs`,
  the grouped arm ending `=> CertifyError::ChartImageUnavailable`) folds
  twenty arms into `ChartImageUnavailable`, which ends by
  `CertCheck::ChartImage` (`CertifyError::decision`). A lane gap
  (`UnsupportedChart`, `IsoUnsupported`, `ChartWindingUnsupported`), a
  stored contradiction (`ImageMismatch`, `WindingMismatch`), a lever on
  the input (`CarrierDomain`, `FittedLaneUnsupported`) and a check's
  refused arm (`ResidualExceeded`, `IntervalNotForward`) all end alike.
- `AnalyticRung3Refusal::of_offset` (`crates/geom-brep/src/edge_nurbs.rs`)
  ends every arm but the escalation, the carrier enclosure and the
  sector in `_ => Self::NoOffsetBound`.
- `chord_join`'s `run_image_refusal` (`crates/topo/src/chord_join.rs`)
  ends every arm but a lane gap, an incidence read and an escalation in
  `_ => SplitJoinError::SectionInvariant` (the join's invariant break).

A fourth, `certify.rs`'s `row_image_refusal`, keeps only
`PcurveCertifyError::refused_check` and re-decides the row by the edge
check it restates (`RowRefusal::decision`, `arc_rim_check`); a refusal no
check owns reads as `PlaneNurbsHull`. That is a re-reading by the edge's
own checks, not a lost ending, but its `None` arm is the same grouping.

## Repair shape

Decide per wrapper whether the pcurve arm's decision is the one its door
should end by. Where it is, nest the refusal whole (or carry its
`PcurveCertifyError::ending` through `render(at)`); where the wrapper's
decision is the right one, say so at the arm and make the match
exhaustive by variant.

## Also: the boolean loop walk's routing

`boolean::ops::walk_certify_refusal` (`crates/topo/src/boolean/ops.rs`)
routes `SectorRefused` as `WalkRefusal::Unread`. Since this branch, that
arm is check 4's sector condition on a stored piece, ending in the
door's defect, which the walk reads as `Torn` for the other
stored-contradiction arms (`ImageMismatch`, `WindingMismatch`). Decide
which one it is.

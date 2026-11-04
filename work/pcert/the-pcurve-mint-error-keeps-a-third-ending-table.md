---
id: the-pcurve-mint-error-keeps-a-third-ending-table
kind: issue
title: PcurveMintError::ending keeps a third ending table beside the one decision table
status: open
opened: 2026-11-04
priority: P2
cost: E
---


(PCERT implementer, from `two-decision-tables-for-one-ending`'s sweep.)

## What

`PcurveMintError::ending` (`crates/topo/src/pcurves.rs`) maps each
mint refusal to its ending in its own match, beside the one decision
table (`geom_brep::recourse::Decision`) the certification doors read.
Its arms are kinds the table already writes:

- `Certify` → the nested `PcurveCertifyError::ending` (the table);
- `Escalated`, `NetStretchUndecided` → `not_yet` on the undecided arm
  (the table's `NotYet` is the definite arm's);
- `Band` → `validate::TOLERANCE`, which is `BAND_TOLERANCE_RECOURSE`
  (the table's `Lever(Lever::Tolerance)`; the lever-arm `BandError`
  arm, which the pcurve door ends by its own recourse, is not split
  out here);
- `PlaceholderChart` → `PLACEHOLDER_RECOURSE` (the table's
  `Lever(Lever::Placeholder)`);
- `JointWithoutRoom` → its own lever;
- seven torn arms → `None` (the door's defect).

A new ending kind added to the table does not reach this match, and
the `Band` split already differs.

## Repair shape

Outside geom-brep, so not folded by the refactor. Either read the
table through a crate-visible door (its `Lever`/`NotYet` kinds) or
state why a mint refusal ends by a table of its own.

---
id: mate-and-pose-forwarders-the-census-does-not-weld
kind: issue
title: The mate and pose refusals NodeErrorKind forwards are not welded to their arms in the feature-tree census
status: open
opened: 2026-10-19
priority: P4
cost: E
---


Found by the STYLE review of PR 5407, which welds the feature-tree
census (`every_forwarded_arm_has_a_row` in
`crates/editor-core/tests/refusal_concision_chains.rs`) to the
editor-core refusals `NodeErrorKind` forwards. `forward` cites this row
(`UNWELDED_HELD`) for three arms on mate and pose ground:

- `Mate` forwards `mate::MateFault`, twelve arms, rendered by `mate()`'s
  own list;
- `SpaceMismatch` carries two `mate::Space` values (rows
  `WorldAndLoose` and `TwoLoose`);
- `PoseRead` forwards `pose::PoseReadFault`, four arms with one row
  (`NoAxis`).

## Parked on the D10 hold (2026-10-19, WIRE)

Mate and pose are datum and placement ground, held under D10, so
`wire-p4` listed these in `forward` and did not weld them. Parked on
`transform-retires-into-a-placement`, the row whose landing releases
that ground.

## Repair shape

Give `MateFault` and `PoseReadFault` an `f6_variants!` roster each, a
row per arm labelled `{family}/{arm}`, and move their `forward` arms to
`welded(..)`.

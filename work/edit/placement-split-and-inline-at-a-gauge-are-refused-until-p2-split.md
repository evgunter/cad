---
id: placement-split-and-inline-at-a-gauge-are-refused-until-p2-split
kind: issue
title: edit: split and inline refuse the shapes A4 builds with a minted gauge — the gauge hoist, a cut holding a gauge, an inline at an offset over any other part, and a mate-placed instance
status: open
opened: 2026-10-01
priority: P1
cost: H
parent: placement-is-spelled-three-ways-node-registry-and-rule
---


Filed by the EDIT lane `place2` from the cut the orchestrator approved
(P2-core on `edit/placement-gauges`, P2-split on
`edit/placement-split-inline`): every case P2-core admits produces
exactly the document A4 produces, and every other refuses typed, naming
the recourse, so P2-split only lifts refusals.

## What P2-core refuses that A4 builds

- **Split, a cut holding a gauge**: `SplitError::CutHoldsGauge`
  (`crates/editor-core/src/refactor.rs`, the gauge rules in `split`).
  A4's gauge hoist — a cut that is one gauge with its groups, the
  gauge's placement hoisted onto the instance left behind — and the
  severed-gauge rule are not built.
- **Inline at a non-empty offset over a part that is not one group
  rooted at the empty chain on its world**: `InlineError::NeedsAGauge`.
  A4 mints a gauge for the part's content at the instance's offset.
- **Inline of a mate-placed instance**: `InlineError::MatePlaced`. A4
  admits it when the part is one group at the empty chain (spec ruling
  7); P2-core refuses every one.

## Built and admitted by P2-core

The group hoist (a cut that is exactly one placed group), the verbatim
move of a cut holding no gauge, inline's sugar (a root-placed instance
over a part that is one group rooted at the empty chain on its world,
whose root takes the offset), and the empty offset (content verbatim on
the instance's gauge). Rows: `crates/editor-core/tests/p2_gauges.rs`.

## Repair shape

Build A4's gauge hoist and gauge inline, and admit the mate-placed
inline A4 rules; each lifts a refusal and changes no admitted result.

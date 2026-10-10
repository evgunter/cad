---
id: split-sector-rules-mint-no-invalid
kind: unit
title: split's sector rules and conic graze mint INVALID after a decided sign: route each through its door and keep its ending (mints C1-C5)
status: open
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [split-escalations-end-a-poisoned-margin-in-the-plane-lever]
---

Unit 1 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`). The site ids are that section's.

## Sites

- **C1, `splitting/rules.rs:197` (`apply_rule_a`, `split_sector_extent`).**
  The face extent is a magnitude. Read it with `decide_magnitude_reported`.
  A decided Zero is a collapsed face: refuse it as `SliverSector` carrying
  the decided margin, not `INVALID`.
- **C2, `rules.rs:319` (`enters_material` reads `Tangent` after the
  parallelism gate).** One fact is decided twice. The gate's verdict
  should reach `enters_material` as typed input, so that `Tangent`
  cannot follow it (design item 7).
- **C3, `rules.rs:488` (`wall_graze`, `wall_bend_order2`, `Exits` or
  `Tangent` after rule (a) read "enters").** Same shape as C2. First
  confirm that both reads ask the same question of the same data.
- **C4, `rules.rs:500` (the sectors disagree).** Each wall's verdict is
  read over its own extent, and ledger F11 leaves this open. Decide
  whether it is a straddle (the split lever) or a disagreement of two
  `Decided` readings (a typed finding).
- **C5, `splitting/classify.rs:608` (`split_conic_graze_side` decided
  Zero, reachable at K ≤ 2).** Route it through `decide_nonzero`, whose
  rejection keeps the decided margin, tagged.

## Constraint: the endings

Today `SliverSector` renders one constant lever. CLEAVE's
`split-escalations-end-a-poisoned-margin-in-the-plane-lever` asks that
it carry how its reading stands
(`splitting::containment::Escalation::{Margin, Straddle, Decided}`) and
end through `RefusedArm`. That row waits on these mints, so build the two
together, or land this first and leave the endings to that row.

Pin each moved arm's ending before and after. An arm that stops saying
`INVALID` stops reaching any `is_invalid()` defect route.

## Sweep owed

Re-run the re-scope's two passes over `crates/topo/src/splitting/`
before landing.

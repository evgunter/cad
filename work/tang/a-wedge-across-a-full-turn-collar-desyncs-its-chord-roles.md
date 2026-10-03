---
id: a-wedge-across-a-full-turn-collar-desyncs-its-chord-roles
kind: issue
title: A partial-revolve wedge crossing a full-turn collar's cap refuses JoinDesync: every chord arc separates a loose scaffolding pair
status: review
opened: 2026-10-02
pr: 3985
branch: reach/arc-from-pairing
---


## What

The collar `ρ ∈ [0.5, 1.5]`, `y ∈ [1, 2]` revolved a full turn about
`y` (its bore one face bounded by two whole rims, as in
`crates/sweep/tests/full_turn_bore_mate.rs`) less a wedge — a rectangle
`ρ ∈ [ρ0, ρ1]`, `y ∈ [y0, y1]` revolved a partial turn about `y`
(`sweep::test_support::revolved_about_y`, `Revolution::Partial`) —
refuses under ∖, ∪ and ∩ alike. Measured on `tang/pierce-ring`
(PR 3851) and on `main` at `e50461ed`, 48 poses (`ρ` ranges
`(0.3, 0.9)`, `(0.2, 0.7)`, `(0.4, 1.2)`, `(0.3, 1.8)`; `y` ranges
`(1.2, 1.8)`, `(0.5, 1.5)`, `(1.5, 2.5)`, `(0.5, 2.5)`; angles π/2, 1,
4):

- 29 poses, identical on both: `JoinDesync "every chord arc separates
  a loose scaffolding pair"` (`crates/topo/src/boolean/join.rs:1521`,
  `choose_roles`' outer-loop arm) — every wedge that crosses a cap of
  the collar;
- 16 poses refuse a typed window case: on `main`, 8 of them
  `SectionInvariant "ring re-homing reads the divided face's plane"`;
  on the branch those 8 re-home on the wall's chart and reach
  `SectionArcWindow { BothContained }`;
- 3 build.

The desync is the kernel's: a valid wedge cut through a valid collar.
Its witness, the smallest: `ρ (0.3, 0.9)`, `y (0.5, 1.5)`, angle 1.

## Evidence (2026-10-03, PR 3985, `reach/arc-from-pairing`)

Re-measured the 48-pose matrix on `main` at `0770bfaa3`: the
`JoinDesync` no longer reproduces; 12 poses build under ∖, ∪ and ∩, and
36 refuse `Join(SectionArcWindow { BothContained })` under each op (the
bore and outer wall are full-turn faces, whose window spans a period).
On the branch, which reads no window, all 48 build under ∪, ∩,
collar ∖ wedge and wedge ∖ collar, tiers 2 and 3 clean, to the
annular-sector closed form (`crates/sweep/tests/wedge_through_a_full_turn_collar.rs`).

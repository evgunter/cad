---
id: a-cylinder-sphere-march-near-its-node-costs-seconds-and-refuses-by-budget
kind: issue
title: A cylinder × sphere march near its node costs seconds per op, and nearer the node pays the same seconds to refuse CellBudget
status: open
opened: 2026-10-27
priority: P2
cost: M
---


Found by the dual review of `the-section-of-a-surface-pair-is-one-object`
(PR 5872, 2026-10-27).

## Measured

The section table carries a cylinder × sphere pair on the first germ
that needs a carrier, once per Boolean, through
`geom_brep::SsiLane::marched` (`march_over_the_sphere` in
`crates/geom-brep/src/pair_section.rs`, which calls
`geom_brep::ssi::march_cylinder_sphere`). The march's cost climbs as the
pose nears its node, the far ruling where the two loops meet
(`R = d + r`), and at the nearest poses it spends the same order of time
to refuse.

One `PairSection::carry` per pose, cylinder radius `r = 0.5`, sphere
centre `d` off the axis, debug build, the test run alone
(2026-10-27, this box):

| pose | answer | time |
|---|---|---|
| `d = 0.5`, `R = 0.3` (centre on the wall) | one loop | 0.63 s |
| `d = 0.2`, `R = 0.69` (gap 1e-2) | one loop | 3.98 s |
| `d = 0.2`, `R = 0.699` (gap 1e-3) | one loop | 8.82 s |
| `d = 0.2`, `R = 0.6999` (gap 1e-4) | `March(CellBudget { budget: 200000 })` | 14.8 s |
| `d = 0.2`, `R = 0.701` (gap 1e-3) | two loops | 7.28 s |
| `d = 0.2`, `R = 0.7001` (gap 1e-4) | `March(CellBudget { budget: 200000 })` | 14.4 s |

The reviewer's split of the `R = 0.699` march, on their box: 6.6 s in
all, 4.9 s of it in the exhaustiveness accounting
(`crates/geom-brep/src/ssi/exhaust.rs` `account_r3`), and 10.8 s to
refuse `CellBudget` at gap 1e-4. So a Boolean whose pair sits nearer the
node than about 1e-4 pays about 11–15 s per op to refuse
`GermSectionUnbuilt { why: March(CellBudget) }`, and one between 1e-4
and 1e-2 pays 4–9 s per op to answer; every op over the same pair pays
it again, since the table is the operation's.

## What a fix has to supply

Either the near-node pose refused before the march, by a decided margin
on the reach `d + r − R` whose band is the one the cell budget can
afford (so the refusal costs a predicate, not a budget), or an
accounting that does not subdivide the whole shell near the node (the
refinement concentrates where the loops pass close). Measuring which of
`account_r3`'s seeding and accounting phases holds the time is the
first step. Cost M: one new decided row and its pin, or a change inside
the accounting with its meters.

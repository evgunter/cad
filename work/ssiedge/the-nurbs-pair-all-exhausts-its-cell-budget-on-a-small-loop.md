---
id: the-nurbs-pair-all-exhausts-its-cell-budget-on-a-small-loop
kind: issue
title: Section::all refuses CellBudget on a closed loop of radius 1e-2 m after ~33 s debug; the 4-D accounting sweep's cost grows past its budget as the loop shrinks
status: open
priority: P3
cost: M
opened: 2026-10-11
refs: [a-wall-seam-between-two-fits-has-no-section]
---

Found by SHELL's NURBS × NURBS section unit (`a-wall-seam-between-two-fits-has-no-section`,
PR 4597), at its dual review, and re-measured 2026-10-11 on a scratch
row of `crates/geom-brep/tests/nurbs_nurbs_section.rs` (debug build,
default ε): the paraboloid `z = (x − ½)² + (y − ½)²` over the unit
square (the file's `graph` helper) against the plane `z = r²` over
`[−0.2, 1.2]²`, a closed loop of radius `r` inside both rectangles,
through `Section::of(..).all()`:

| r | answer | time |
|---|---|---|
| 1e-1 | one loop | 7.2 s |
| 3e-2 | one loop | 14.3 s |
| 1e-2 | `CellBudget { budget: 200000 }` | 32.8 s |

The cost is `all()`'s 4-D sweep (`exhaust::seed_pair` and
`account_pair` over `PairCell`s, chart lane at the fastest of the
four speeds): a small loop needs fine cells to be seeded and its tube
is small, so the cells the accounting must resolve around it grow past
the budget. The refusal is typed and loud, not wrong; the offset door
never reaches it (it calls `branch_at`).

Owed before the boolean leans on `all()`: a sweep whose cost does not
grow this way as a loop shrinks — exclusion by a sharper enclosure, a
seeding rule for small loops (critical points of the distance, as
the plane lane's seeding reads), or the C10 tree in place of the
linear scan over tubes.

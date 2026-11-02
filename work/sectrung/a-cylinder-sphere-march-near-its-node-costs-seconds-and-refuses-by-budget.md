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
`geom_brep::SsiLane::marched` (`march_over_the_ball` in
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

## The chord's certificate costs as much again (SECT step 1b, 2026-10-28)

Measured on `sect/cylinder-sphere-chord` (PR 5894), debug build: each
carried chord is certified at the edge door by `analytic_rung3`, whose
limb 2 composes the chord's whole net (`pcurve_cache::limb2_offset_sup`):
about 0.75 s per operand for a 265-point arc, 0.55–1.6 s per chord over
the fixture's poses (the dual review's range). Nothing bounds it: the
fitted loop's density sets the net, and a union pays it again at the
at-rest gate's re-certification. Cutting the chord from the loop
(`chord_join::cut_arc`) rather than carrying the whole loop with an
interval took a segment from 18 s to 3.5 s; the fixture's 54 runs still
take 3–120 s each in debug (8–80 s per pose on CI).

## At ε 1e-12 the accounting scans every tube box per cell (2026-10-28)

`pair_section::a_seated_loop_is_the_loop_point_for_point` at
`CAD_TOLERANCE_EPS=1e-12` takes 472 s alone in debug on main (477 s
with `sect/near-node-at-1e-6`), against 86 s at the default ε; a lane
that killed it read it as a hang. Sampled by `gdb` two minutes in, the
test thread sits in `PairSection::carry` → `march_cylinder_sphere` →
`exhaust::account_r3` → `SweepDuty::accounts`, whose
`tubes.iter().any(|t| cell.contained_in(*t))` reads every tube box for
every cell. The march's samples are spaced as `ε^¼`, so at 1e-12 a
branch holds about 5.6 times the default ε's boxes (2460 at the
default ε at `d 0.2, R 0.699`), and each cell pays them all.

## Nearer the node, on both trees (review of PR 5961, 2026-10-28)

Measured by the review of `sect/near-node-at-1e-6` at `19b3396f83`,
the same on main and on the branch (`r = 0.5`):

- at ε = 1e-6, `d 0.2` with `R` 0.6995, 0.6999, 0.7001 and 0.7005
  refuses `TubeStraddles`;
- at the default ε, `d 0.2` with `R` 0.6995, 0.6999 and 0.7001, and
  `d 0.1, R 0.599`, refuse `CellBudget`.

At `d 0.1, R 0.599` and the default ε the tube is 2.34e-3 and no chain
box holds it (`coarse_tube_spans` names none), while the branch accounts
the same tube at 1e-6 on 477 boxes. So that budget is not a coarse box
holding the tube, the class PR 5961 fixed; its cause is unmeasured.

## The torus doors pay the same near a tangency (SECT, 2026-10-28)

Measured on `sect/torus-doors`, debug build, `crates/geom-brep/tests/torus_doors_ssi.rs`
`near_tangent_cost` (ignored; a measurement): a rod of the tube's radius
whose axis passes `δ` from a torus's core (`R = 1`, `r = 1/4`), the
straight pipe entering its bend parted by `δ`, a transverse critical
gap of `δ`. At `δ = 1e-2` `cylinder_torus_ssi` spends 9.4 s and refuses
`CellBudget` (200000 cells); at `δ = 1e-3` through `1e-6` it refuses in
0.8–1.0 s on the tube (`TubeStraddles`, a 1244-box chain whose crossing
angle bound reads zero). Far from any tangency the germ row's poses
examine 3500–150000 cells, about 2–8 s each in debug (`germ_row`, the
slow set). So the cylinder × sphere shape holds for the torus pairs too: the
cost is the accounting's, and nearer the tangency the refusal is typed
but not cheap.

The lily's stem and arch tori (`R = 5`, `r = 0.06` and `R = 1.1`,
`r = 0.052`, axes parallel) are such a pose in a real scene: where the
arch leaves the fork its core runs tangent to the stem's, its tube
8 mm inside the stem's, and `torus_torus_ssi` over the arch's ball (the
smaller) refuses `CellBudget` after about 20 s in debug
(`crates/sweep/tests/mate7a_r1_probes.rs`
`the_lilys_tori_exhaust_the_doors_cell_budget`, the pin).

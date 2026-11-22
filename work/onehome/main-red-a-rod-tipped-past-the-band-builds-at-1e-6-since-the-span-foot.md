---
id: main-red-a-rod-tipped-past-the-band-builds-at-1e-6-since-the-span-foot
kind: issue
title: Main red (ε = 1e-6): a rod tipped past the band over a long wall builds a body since PR 7112, where it took a decided door
status: open
opened: 2026-11-22
priority: P1
cost: M
---


Found by GREEN (U6, `work/green/main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17.md`).
PR 7112 (`onehome/span-foot`, fork 275 step 3b: point feet read over
the span) reddened two `sweep` rows at ε = 1e-6 only:

- `parallel_cylinder_join::a_rod_tipped_off_parallel_over_a_long_wall_takes_a_decided_door`
  stops at `drum × rod r 0.2 at 0.5, half-length 50, tipped 2e-7 | U ab: Ok("a body")`;
- `parallel_cylinder_join::a_tipped_rod_refuses_alike_in_both_operand_orders`
  stops at the same pose, `U ab: refuses`.

**Bisected on hosted CI at ε = 1e-6**, both rows run in a child at that
ε: green at 7112's first parent `e7d1a14b11` (and at 7078, 7080 and
7052 before it), red at its merge `7ad94ddffb`, and red on main
`12a43a7e68`. No merge between `e7d1a14b11` and 7112's first parent
touches a kernel crate.

**What main answers** (main `12a43a7e68`, every pose of the two rows,
each op judged by `common::differential::outcome`, ε = 1e-6). The
drum is radius 0.5 about `z`, the rod `r` at `c` on `x`, tipped about
`x` through its centre by `drift / (2·half)`:

| pose | the six ops |
|---|---|
| r 0.2, half 50, tip 2e-7 (drift 2·escalate) | bodies; tier 3′ fails (`t3p=false`); pads 2.3e-8–1.6e-6 |
| r 0.3, half 50, tip 2e-7 | bodies; tier 3′ passes; pads to 1.3e-6; I ab `SOUND` |
| both, half 50, tip 2e-6 (drift 20·escalate) | every op escalates, `Coincidence(SectorSide, Moot)` |
| r 0.2, half 200, tip 5e-8 | bodies; tier 3′ fails; pads to 2.7e-5 |
| r 0.3, half 200, tip 5e-8 | bodies; tier 3′ passes; pads to 2.3e-5 |
| both, half 200, tip 5e-7 | bodies; tier 3′ passes; pads to 2.6e-3 |

Every body certifies (`cert=true`), is a legal operand and holds the
parallel closed form inside its certified pad; the closed form's own
error from the tip is second order (about 6.5e-7 on the half-200,
5e-7 poses, which is what the volumes read past it). So no answer is
a wrong volume. What moved is the door: at the first parent these
tips past the band refused at a decided door (the rows passed), and
now most build, with pads past the suite's `VOLUME_PAD_BUDGET` (1e-7)
and the r 0.2 bodies failing tier 3′. At the default ε every pose of
both rows still escalates (`Coincidence(Section, Moot)` or
`Coincidence(SectorSide, Moot)`), as the rows want.

**Owed.** Whether the frame's lever, read by the walls' reach, still
reads these tips as skew after the foot moved to the span, or whether
the join now reads them through the rulings arm on purpose: if the
latter, the rows' claim changes with it and the tier-3′ failures on the
r 0.2 poses are the defect; if the former, the door is the defect.

Both rows are pinned at 1e-6 to that first stop
(`test_utils::pinned_red::row`, `crates/sweep/tests/parallel_cylinder_join.rs`),
so they go red when the pose stops building or stops some other way.

Reproduce: `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all -E 'test(=parallel_cylinder_join::a_rod_tipped_off_parallel_over_a_long_wall_takes_a_decided_door) | test(=parallel_cylinder_join::a_tipped_rod_refuses_alike_in_both_operand_orders)'`
passes while main stops as pinned; the stop itself is in the panic the
pin reads (`pinned_red::row` fails with it once main stops otherwise).

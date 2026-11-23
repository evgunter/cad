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
  stops at the same pose, `U ab: builds, where every op refuses` (the
  union builds where the row wants every op refused alike).

**Bisected on hosted CI at ε = 1e-6**, both rows run in a child at that
ε: green at `e7d1a14b11` (7114, and at 7078, 7080 and 7052 before
it), red at 7112's merge `7ad94ddffb`, and red on main `12a43a7e68`.
7112's first parent is `f7f9c07ada`; the three merges between it and
`e7d1a14b11` (7116, 7126, 7130, DOCTAIL) touch editor-core, pncad,
pncad-py, viewer and `profile`'s path authoring (`profile/src/path/`),
and nothing under `geom*`, `topo` or `sweep`. The rows build their rods
through `sweep::test_support::prism_at`, not through `profile::path`.

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
| both, half 200, tip 5e-7 | bodies; tier 3′ passes; pads (half-widths) to 2.6e-3 |

Every body certifies (`cert=true`), is a legal operand and holds the
parallel closed form inside its certified pad; the closed form's own
error from the tip is second order (about 6.5e-7 on the half-200,
5e-7 poses, which is what the volumes read past it). So no answer is
a wrong volume. What moved is the door: at the first parent these
tips past the band refused at a decided door (the rows passed), and
now most build, with pads (`volume_pad`, a half-width) past the
suite's `VOLUME_PAD_BUDGET` (1e-7), and the r 0.2 bodies failing tier 3′.
The widest pads are on bodies that pass tier 3′; the bodies that fail
it have pads to 2.7e-5. At the default ε every pose of
both rows still escalates (`Coincidence(Section, Moot)` or
`Coincidence(SectorSide, Moot)`), as the rows want.

**Owed.** Whether the frame's lever, read by the walls' reach, still
reads these tips as skew after the foot moved to the span, or whether
the join now reads them through the rulings arm on purpose: if the
latter, the rows' claim changes with it and the tier-3′ failures on the
r 0.2 poses are the defect; if the former, the door is the defect.
Every tier-3′ failure is the census escalating, `CensusEscalated {
Indeterminate }` on `curve_carrier_piece_clear` or `curve_carrier_gap`,
margins 9.28e-6 to 9.9999e-6 against the 1e-5 escalation (PR 7383's
review probe, run 45035517038): an honest in-band "cannot decide", not
an unrecorded or stale contact, so no body is shown wrong. What
regressed is a decided refusal turning into a body the kernel's own
tier 3′ cannot certify.

Both rows are pinned at 1e-6 to that first stop
(`test_utils::pinned_red::row`, `crates/sweep/tests/parallel_cylinder_join.rs`),
so they go red when the pose stops building or stops some other way.
The first row's tip inside the zero band (`tipped(0.3, 0.6, 5.0, zero/100)`,
every op SOUND) runs ahead of its pin, so it is read at 1e-6 too.

Reproduce: `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all -E 'test(=parallel_cylinder_join::a_rod_tipped_off_parallel_over_a_long_wall_takes_a_decided_door) | test(=parallel_cylinder_join::a_tipped_rod_refuses_alike_in_both_operand_orders)'`
passes while main stops as pinned; the stop itself is in the panic the
pin reads (`pinned_red::row` fails with it once main stops otherwise).

**Since PR 7131 (GREEN U12, 2026-11-23).** At ε 1e-12, both of this row's tests are also red, in nightly 44989234943 and not in 44870651337. That red is not PR 7112's. The tipped rod is not a finished body (`props_cylinder_circle_off_axis`, hi 1.194e-12). Cause: 7131's polar read. Mutant A alone turns both green, and F1X or F1F alone does not. The ε 1e-6 red here is unchanged. The refusal is counted in ONEHOME's `pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted` (PR 7402's branch until it merges), with probe runs 45083192761 (green at `e4d679d37e`), 45083200534 (red at `9bc9d6517b`) and 45083184695 (mutants). GREEN pins nothing until Ev rules on that count.

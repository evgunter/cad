---
id: main-red-a-rod-tipped-past-the-band-builds-at-1e-6-since-the-span-foot
kind: issue
title: Main red (ε = 1e-6): a rod tipped past the band over a long wall builds a body since PR 7112, where it took a decided door
status: review
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

**Diagnosis (ONEHOME operands lane, 2026-11-23, main `032c2f9f5`).**
The new answer is the right one, and the rows' premise at 1e-6 was
stale. The door did not regress.

- **The "decided door" at 1e-6 was the march's false desync.** At
  7112's first parent `f7f9c07ada`, every op of `r 0.2 at 0.5` and
  `r 0.3 at 0.6` (half-length 50, tip 2e-7) refused with
  `JoinDesync { "the march found one loop of the section twice" }`.
  That refusal is `distinct` (`crates/geom-brep/src/pair_section.rs`):
  the general march's carry stage asks whether a branch's witness lies
  on another branch (`other.holds(w, band)`). The section's two long
  branches stand about 0.4 apart, so that reading was false. 7112 reads
  a component's foot over the tube pieces a point lies in, where it was
  read on the whole carrier from the span's start, and with that the
  march carries both branches. The rows asked only for an `Err` other
  than `Pcurves`, so the desync satisfied them. At 7112's merge
  `7ad94ddffb` every op of both poses builds, as on main.
- **The frame still reads these tips as skew.** Each body's section
  edges are traced `Nurbs` carriers, not the `Line` rulings the parallel
  arm cuts (the `I ab` body has four faces: two drum circles, two rod
  cap circles and two `Nurbs` section curves). The pose reached the
  skew march before 7112 and reaches it now.
- **The tier-3′ failures are the fixture's.** At `x = 0.5` the r 0.2
  rod's axis lies on the drum's seam line, and tipped, the seam runs
  within the drift (to 1e-5 at the caps) of the axis of the rod's cap
  circles, which accounts for the census's in-band escalations. Measured
  at 1e-6, half-length 50, tip 2e-7, all six ops: `t3p=false` at
  `x = 0.5` for both radii; `t3p=true` at 0.45, 0.55 and the r 0.3 rod's
  0.6.
- **The pads** are the certified enclosures' widths at a 1e-6 band. With
  the r 0.2 rod off the seam, every op builds SOUND inside its own pad.
  The widest measured per pose: 1.3e-6 (half 50, tip 2e-7), 2.5e-5 (half
  200, tip 5e-8) and 2.2e-3 (half 200, tip 5e-7). `I ab` reads 2e-8 to
  8e-6.

**The rows now** (`crates/sweep/tests/parallel_cylinder_join.rs`):
- `a_rod_tipped_off_parallel_over_a_long_wall_is_read_as_skew`: at 1e-6
  every pose past the band builds SOUND within twice its measured pad
  (`built_at_coarse`), except the larger tip over half 50, which
  escalates. At the default ε every pose refuses, as before.
- `a_tipped_rod_reads_alike_in_both_operand_orders`: every op of a pose,
  in both orders, refuses alike or builds alike.
- `a_tipped_rod_on_the_drums_seam_escalates_its_census_in_band`: the
  `x = 0.5` pose at 1e-6 builds closed, certified, legal-operand bodies
  holding their closed forms, and tier 3′ escalates on
  `curve_carrier_piece_clear` / `curve_carrier_gap` alone.
- The r 0.2 rod of the first two rows stands at `x = 0.55`.

**At 1e-12** (unchanged by this work, and not fixed here): the half-50
poses refuse every op with `CurvedPierceUnsupported`, alike, and the
half-200 tipped rod is not a finished body. The latter is
`pr-7131s-frame-cost-at-1e-12-is-larger-than-it-counted`, above.

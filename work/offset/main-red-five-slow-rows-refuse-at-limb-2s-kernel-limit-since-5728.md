---
id: main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728
kind: issue
title: Main red: five slow-set rows refuse at 1e-12 as limb 2's KernelLimit since PR 5728, and the torus support plate refuses NoOffsetBound at 1e-6
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by nightly triage 2026-10-28, run
[40637814390](https://github.com/evgunter/cad/actions/runs/40637814390)
(main `a1cfd1cfdb`), job `full suite (every eps row, slow set included)`.
All five rows are in the slow set, so only the nightly runs them.

## What

At `CAD_TOLERANCE_EPS=1e-12` five rows unwrap a build that now refuses
as limb 2's kernel limit, `AnalyticRung3(KernelLimit { .. })`:

| row | operand | bound | extent |
|---|---|---|---|
| `mesh` `perf1_torus_sizing::torus_sizing_sweep_is_sound_tight_and_on_the_ideal` (`crates/mesh/tests/perf1_torus_sizing.rs:61`) | Torus | 1.75e-11 | 2.24 |
| `sweep` `band_turn_overrun_oracles::leaning_prisms_match_the_slab_oracle_over_leans_setbacks_and_scales` (`crates/sweep/src/test_support.rs:712`, "the fixture's profile extrudes") | Plane | 1.08e-12 | 2000 |
| `sweep` `reach_split_gate_azimuth::every_cut_clear_of_a_partial_turn_sphere_face_splits` (`crates/sweep/tests/reach_split_gate_azimuth.rs:120`) | Sphere | 3.35e-12 | 1734 |
| `sweep` `reach_split_gate_pose::a_cut_clear_of_the_face_splits_in_every_pose` (`crates/sweep/tests/reach_split_gate_pose.rs:122`) | Sphere | 3.35e-12 | 2236 |
| `sweep` `operand_gate_support_plates::plates_at_the_exact_support_of_a_frustum_and_a_torus_are_right_at_every_op` (`crates/sweep/src/test_support.rs:316`) | Cone | 1.76e-12 | 1131 |

The last row is also red at `1e-6`, with a different refusal:
`AnalyticRung3(NoOffsetBound { operand: Torus, why: "its metre lever is
not positive: it reaches the cone's apex or other nappe, or a spindle
torus's inner sheet" })`.

## Where it came from

Bisected on a clean checkout: all five 1e-12 rows and the 1e-6 row pass
at `b8974faabc^1` and fail at `b8974faabc`, the merge of PR 5728
(`offset/limb2-for-every-carrier`). PR 5728's body lists the 1e-12
refusals Ev accepted (fork row 203: thin tori with R/r ≥ 20, edges of
1 km or more, "a few 700–1000 m rows") and re-pins the rows it ran.
These five were not re-pinned: they are slow-set rows outside its gate.

## What closing it takes

For each row, decide whether the refusal is one of the accepted kernel
limits (then re-pin it as that refusal, fail-loud, citing the follow-up
row it waits on: `the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding`,
`the-closed-form-net-builder-rounds-at-a-km-radius-not-at-the-span`) or
a regression. Two readings need care:

- `perf1_torus_sizing` is a unit-scale torus (extent 2.24), not a far
  placement or a km-scale edge.
- The 1e-6 `NoOffsetBound` on the torus plate is not a kernel limit at
  all; "its metre lever is not positive" on a plate at a torus's exact
  support reads as a wrong refusal until shown otherwise.

Reproduce: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep -p mesh
--ignore-default-filter -E 'test(/^perf1_torus_sizing::torus_sizing_sweep/)
| test(/^band_turn_overrun_oracles::leaning_prisms/) |
test(/^reach_split_gate_(azimuth|pose)::(every_cut_clear|a_cut_clear)/) |
test(/^operand_gate_support_plates::plates_at/)'`.

## Band

P2, not P0 (2026-10-28): these are slow-set rows at interval and
tight-ε ground, which `work/README.md`'s bands put at P2; a main red is
not P0 by itself. The rows are pinned at what main produces, so main
is green on them while this row stays open for the mechanism.

## Pinned (2026-10-28)

All six refusals reproduced on main `974f160a` with the bounds and
extents above. Each row is pinned whole at the ε it refuses at
(`test_utils::kernel_limit::pinned_row` at 1e-12; the torus plate's
1e-6 `NoOffsetBound` through `panic_capture::caught`), with a comment
naming this row. Each row stops at its first refusal, so which other
cases of the row also refuse is unmeasured; closing this row narrows
each pin to its refusing cases or removes it.

## A seventh: the lily's stem tube in the K-probe sweep (2026-10-29, GREEN)

`k-lint (dev-probe)` runs `scripts/k_probe_sweep.sh`. At ε 1e-12 its
demo-scene pass (`demo-tour k-probe`) panics in the lily, on
`demos/tour/src/lily.rs` `tube_arc`'s `.expect("stem tube arc builds")`:

```
Revolve(Op { source: Certification { error: AnalyticRung3(KernelLimit {
  operand: Torus, bound: 3.201461299630955e-12, margin: MarginDiag(Value(
  3.201461299630955e-12, None, ZeroWord(0))), width: 2.5757174171303632e-14,
  extent: 0.13416407865000426 }) } })
```

It is a unit-scale torus (extent 0.134, ring against tube 0.060), the
same reading as `perf1_torus_sizing` above. It is not a far placement
or a km-scale edge.

Bisected over two points: the demo-scene pass at 1e-12 runs to the end
(exit 0) at `b8974faabc^1` and stops on this refusal at `b8974faabc`,
PR 5728's merge. It reads the same at `a1cfd1cf`, `bc30e8a3` and
`c173123e`. Nothing had reached it before: every earlier night's
dev-probe stopped at an earlier step of the sweep.

**Pinned** in `demos/tour/src/probe.rs` (`pinned_stop`). At 1e-12 the
`lily` and `lily_walls` scenes must stop on `stem tube arc builds` with
`AnalyticRung3(KernelLimit { operand: Torus`, at the bound and extent
above; they run as written at
every other ε. Both pins go red the day the stem builds. Closing this
row removes them, and un-pinning owes a k-lint re-measure: at 1e-12 the
pins drop every `lily` and `lily_walls` sample from the K-telemetry
CSV, so the large-K lint has never read those scenes at that ε.

Reproduce: `cd demos/tour && CAD_TOLERANCE_EPS=1e-12 cargo run
--features probe -- k-probe /tmp/k.csv` (before the pin, at
`b8974faabc` against its first parent).


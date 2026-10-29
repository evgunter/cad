---
id: ray-wall-twins-curve-foot-branch-margin-scales-with-the-model-squared
kind: issue
title: ray_wall_margin_twins: curve_foot_branch's decisive margins read a million to one between the millimetre and metre twins (six pairs before PR 5664, 62 since); pinned 2026-10-28
status: open
opened: 2026-10-28
---


Filed by GREEN (`green/slow-set-reds-pins`, 2026-10-28), from
`work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`.
The predicate is decided in `geom::curves::on_carrier`'s `angle_near`
(`curve_foot_branch`), which `work.py territory` gives no owner, so it
is filed here.

`crates/sweep/tests/ray_wall_margin_twins.rs`,
`the_wall_arms_margins_scale_linearly_with_the_model` (`--features
probe`, run by `k-lint (dev-probe)` once, at `PLAIN_EPS=1e-9`, and by the
nightly's `--all-features` suite at the default ε, the same row), compares every
decisive margin of a pipe's ray casts at a millimetre and at a metre,
and expects a thousand to one. `curve_foot_branch` reads 999 off that
ratio, so it reads a million to one: its margin scales as the model
squared, or reads a quantity that does.

How many pairs are off:
- 6 on every nightly that reached the row, from 2026-10-20
  (`37d8dfdecf`) to 2026-10-24. Earlier nightlies stopped before it, so
  when it first went red is unbisected.
- 62 since PR 5664, which `work/charttail/main-red-ringed-curved-doors-reds-four-sweep-rows.md`
  owns.

Main `fa2a942f45` reads `["curve_foot_branch: 62 pairs off, e.g. 9.990e2"]`
at 1e-9 (and the same at 1e-6 and 1e-12, measured locally). The row is pinned at
that string through `test_utils::pinned_red::row`, so that `k-lint
(dev-probe)` runs past it.

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo test -p sweep --features probe
--test all -- ray_wall_margin_twins::`.

**What the pin lets run, unrecorded.** `k-lint (dev-probe)` has stopped
on this row every night since 2026-10-20. With the pin it runs past it,
so the rest of the k-probe sweep, the large-K lint and the driver lint
run again with nothing having recorded their state. A red there next is
not this pin's, but nothing measured it beforehand. Un-pinning owes a
re-measure of what ran past the old stop.

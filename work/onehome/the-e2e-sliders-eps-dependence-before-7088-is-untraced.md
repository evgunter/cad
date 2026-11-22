---
id: the-e2e-sliders-eps-dependence-before-7088-is-untraced
kind: issue
title: The e2e channel slider's receipt varied with ε before PR 7088 for an untraced reason, and the ε values it varied at are gated by no run
status: open
opened: 2026-11-22
priority: P3
cost: M
---


Found by GREEN's U3 (PR 7389), refined by that PR's review (2026-11-22).

`crates/editor-core/tests/m10_5_r1_probes_interval.rs`, `e2e_channel_slider_over_an_epsilon_box`: before PR 7088 (`onehome/operand-coordinates`, merge `36395dff`), the slider's whole-body receipt took three values across ε, with no monotone order, over a box ε/64 wide. On 7088's first parent `be4e656d` it read `(10, 1282, 1272, 572)` at ε 1e-6, `(10, 1258, 1248, 526)` at 1e-7, 1e-11 and 1e-12, and `(10, 1264, 1254, 532)` at 1e-8 and 1e-10. Since 7088, main `1aa4a931` reads `(10, 1264, 1254, 532)` at every ε from 1e-6 to 1e-12, and PR 7389 holds one pin for every ε.

Two things are open:

- **Which read carried the ε-dependence, and whether 7088 fixed it or only stopped it showing.** PR 7088 moved the session box's ask from the readers to the placing doors, and the slider is a placed copy. Neither that nor any other cause has been traced. A probe that would decide it: on `be4e656d`, print the cells that split at 1e-7 and not at 1e-8, and the stored value each read.
- **The ε values it varied at are gated by no run.** The nightly runs 1e-6, 1e-9 and 1e-12 only. A recurrence at 1e-7 or 1e-11 would go unseen; the single pin catches one only at those three rows.

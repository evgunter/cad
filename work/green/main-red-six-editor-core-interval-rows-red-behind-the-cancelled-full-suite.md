---
id: main-red-six-editor-core-interval-rows-red-behind-the-cancelled-full-suite
kind: issue
title: main red: six editor-core interval rows red locally on main, hidden by the nightly full suite's cancellation
status: closed
opened: 2026-10-31
priority: P1
cost: M
closed: 2026-11-02
---


Filed by the SPLICE orchestrator from a SPLICE lane's local validation (2026-10-31).

## What

The `splice/vertex-on-spline-face` lane ran editor-core's suite locally. Six rows failed, and they failed identically on PR 6136's head (`f6058492`) without the lane's change, so they predate both:

- `m10_3_driver_interval` ×2;
- `m10_9_pins_interval` ×2. One reads `r1_annulus` 666 against the measured 654, and `r2_filleted_bracket` 1551 against 1537;
- `sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling`;
- `sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`.

`main-reds-editor-core-slow-set-and-the-k-probe-sweep` greened or pinned these same files and closed on 2026-10-30. Its Closed section says that "a pin that goes red later is a new nightly red". The nightly cannot see these: run 41172334943 (2026-10-31) cancelled `full suite (every eps row, slow set included)`. The cause of the cancellation is SECT's `main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord`, which is dispatched.

## What closing it takes

1. Reproduce the six on the current main, at the ε the lane ran (the default).
2. For each one, attribute the commit where it went red and its cause. Then re-bless it with that cause, fix it at its site, or pin it, per the closed row's convention.

It was not re-measured here; the evidence is the lane's local run only.

## Closed (2026-11-02, GREEN lane `green/main-red-editor-core`)

All six were reproduced on main `a285cdbf` at every ε row (default,
1e-6, 1e-12), opt-level 2, `--profile ci --ignore-default-filter`, and
read the same at all three. Nightly 79 (run 41276693129) shows them
too. All six were bisected to one merge, PR 6105 (`5acea3cb35`,
CLEAR's CAVITYDIAG): its first parent `2a6323786e` passes every one.
#6105 added a decision, `pcurve_cylinder_chart_axial`, on a cylinder
rim's axial terms at Interval (`geom_brep::pcurve_cache`'s
`cylinder_rim_axial`). On main, dropping only that `decide` call (the
terms still zeroed) brings all six back to their stored values.

- **Re-blessed**, the decision's own symbolic count (numeric and frozen
  unmoved):
  - `sym11_exact_channel_rows`: annulus 654 -> 666, bracket 1542 -> 1556;
  - `sym_9_retry_interval` ladder: annulus [654, 555] -> [666, 555] both ways;
  - `m10_9_pins_interval::m10_9_no_registrant_lies_on_any_measured_document`:
    annulus 654 -> 666, bracket 1537 -> 1551;
  - `m10_9_pins_interval::m10_9_what_the_registry_buys`: bracket door
    shut (1474, 1218) -> (1488, 1218).

  The issue's 1551/1537 for the bracket is the `m10_9` shipped reading;
  the `sym11` ceiling reading is 1556/1542.
- **Pinned** at every ε (`test_utils::pinned_red::row`):
  `m10_3_driver_interval`'s two-parameter drive and its banded twin,
  which refuse all four leaves (`Receipt { certified: 0, refused: 4,
  splits: 3 }`). The decision itself causes this, so it is a defect,
  not a count. Filed on PCERT, which owns `pcurve_cache.rs`:
  `work/pcert/the-cylinder-rim-axial-decision-refuses-every-leaf-of-the-two-parameter-drive.md`.

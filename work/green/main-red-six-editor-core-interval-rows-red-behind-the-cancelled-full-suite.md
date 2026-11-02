---
id: main-red-six-editor-core-interval-rows-red-behind-the-cancelled-full-suite
kind: issue
title: main red: six editor-core interval rows red locally on main, hidden by the nightly full suite's cancellation
status: open
opened: 2026-10-31
priority: P1
cost: M
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

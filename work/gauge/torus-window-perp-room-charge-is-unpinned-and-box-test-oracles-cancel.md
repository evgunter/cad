---
id: torus-window-perp-room-charge-is-unpinned-and-box-test-oracles-cancel
kind: issue
title: The torus-window perp_room charge has no row of its own, and the boxes test oracles keep the cancelling sqrt(1 - a^2) spelling
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [4123]
---



Found by the second verifier of PR 4123 (`analysis/reach-verify2/4123`).

## Measured

- **The torus-window charge has no row of its own.** PR 4123 made
  `boxes.rs` `perp_room` read `√(a_j² + a_k²)` instead of the cancelling
  `√(1 − a²)`. Its near-row row
  (`an_axis_near_a_row_keeps_its_perpendicular_room`) kills a cancelling
  mutant in the slab, cone and torus arms one at a time, but not in the
  `torus_window_extent` charge alone. With that arm alone reverted, 2,400
  torus windows per ε stayed sound by over 2e7 ulps: the `u` channel's
  term `(R + r)·p·h_u²/8` is dominated by the sample hull and the `v`
  channel's charge. So it is not unsound today, but nothing would notice a
  regression.
- **The test oracles keep the cancelling spelling.** In the `boxes.rs`
  test module, `:4142`, `:4209`, `:4623` (`(major + minor)·√(1 − a²) +
  minor·|a|`) and `:4663` (the torus window's expected charge). No
  near-row axis reaches them today, but if one did they would agree with a
  regressed spelling bit for bit.

## What closes it

A torus-window case in the near-row row, or an assertion pinning the
charge itself. Move the four oracles to the norm of the other two
components, so they are independent of the code under test.

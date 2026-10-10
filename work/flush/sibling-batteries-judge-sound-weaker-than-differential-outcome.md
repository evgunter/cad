---
id: sibling-batteries-judge-sound-weaker-than-differential-outcome
kind: issue
title: Four differential batteries judge SOUND by their own outcome, weaker than common::differential::outcome
status: open
opened: 2026-10-08
priority: P4
cost: E
---


Found by the sweep of `join1-delta-probes-keep-their-own-outcome`
(branch `join/battery-hygiene`), which routed `join1_delta_probes.rs`
through `common::differential::outcome`. The pattern was a test-side
`outcome` that prints a `SOUND` verdict of its own. Four siblings judge
`SOUND` more weakly than the shared judge, which requires tiers 2 and
3′, the at-rest certificate, a legal operand and the volume to 1e-7.

## What

- `crates/sweep/tests/join1_delta2_harness.rs` `outcome0` / `outcome`:
  a copy of the judge `join1_delta_probes` dropped. Its `SOUND` needs
  neither the certificate nor a legal operand: the `OPERAND=` column is
  printed beside the verdict, not in it. The volume is held to 1e-6
  against a 4 096-chord oracle. `d2_arc_battery` and `d2_brick_battery`
  re-run `join1_delta_probes`' two batteries with that judge.
- `crates/sweep/tests/join3_r2_r1copy.rs` `outcome`: the operand
  (`legal` / `NONOPERAND`) is printed and not in `SOUND`.
- `crates/sweep/tests/join3_r2_probes.rs` `outcome`: the same.
- `crates/sweep/tests/pocket_ring_steep_ellipse.rs`, the battery's
  inline line: no operand, and the volume held to 2e-5 against an
  approximate oracle (`want~`).

Not in the class: `join3_review_r1.rs`'s judge requires all five
checks and prints `NONOP` for a sound body that is no operand.

## The shape of a fix

As in `join1_delta_probes`: route each battery through
`common::differential::outcome`, bring its oracle to 1e-7 (closed
forms, or Richardson-extrapolated chord areas as `join1_delta_probes`
`chord_area` does), delete the local judge, and re-baseline main
against head. Where a battery is a copy of another, consider deleting
the copy instead.

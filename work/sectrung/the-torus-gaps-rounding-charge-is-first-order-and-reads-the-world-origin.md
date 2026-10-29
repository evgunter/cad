---
id: the-torus-gaps-rounding-charge-is-first-order-and-reads-the-world-origin
kind: issue
title: The torus gap's rounding charge is a first-order estimate measured from the world origin, so a pose decides by where it sits, and G1 near-miss saddles escalate
status: open
opened: 2026-10-28
priority: P2
cost: M
refs: [each-torus-pair-decides-tangency-on-one-pose]
---


Filed by SECT's orchestrator from the confirmation passes of PR 5924's
dual review (2026-10-28).

## Measured

- **The charge is not a certified bound.** `pair_section/gap.rs`'s
  `Rounding` is charged by `running::rounding_charge` (`SHORT_CHAIN_ULPS`),
  which is a first-order estimate of the readings' rounding. It held
  against every probe both reviewers ran, but nothing proves it bounds
  the rounding.
- **It reads the world origin.** The charge's `size` is measured from the
  world origin (`gap.rs` `read`), not from the pose. So a pose translated
  far from the origin escalates at gaps 10–100× the escalation width
  where the same pose near the origin decides. R1 measured this for G1
  joints at k ≥ 30, where δ from 1e-7 to 1e-6 mostly escalates.
- **G1 near-miss saddles escalate.** These are two equal-tube bends whose
  cores pass a few ε apart, with gap ≤ 0.5ε. They are genuine
  tangencies that the previous head decided Zero, and now they escalate:
  61 poses in R2's probe. That is sound, but the shipped default loses a
  decision on a realistic pose.

## What a fix has to supply

- A rounding bound that is certified, either in the certification
  arithmetic or derived.
- That bound measured at the pose's own scale: from the pair's own centre
  or pivot, never the world origin.
- Then a fresh reading of which near-miss saddles can decide.

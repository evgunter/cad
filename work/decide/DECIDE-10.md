---
id: DECIDE-10
kind: unit
title: "delete the label kind: a symbolic discharge answers zero and nothing about how"
status: review
opened: 2026-10-06
priority: P2
cost: H
branch: decide/10-delete-the-kind
refs: [the-read-at-its-node-relabels-a-cancellation-above-it, DECIDE-9]
---

## What

`the-read-at-its-node-relabels-a-cancellation-above-it`. The decision
read answers a `min`/`max`/`Select` node before its parent can cancel
it against an equal node. So `max(x + Z, 3) − max(x, 3)` counted
`sign_gated` where the read shut proves it.

The first attempt was draft PR 4095: settle the decision read-free
first. It restored the labels at 25× the cost where the class occurs.
That went to Ev. Ev ruled (2026-10-12, the item's "Question, and Ev's
answer") to delete the label kind:
- No decision type carries how a zero was proved.
- The read's `gated` flag goes, which fixes S8 and retires DECIDE-9's
  label-only zero-product rule.
- Rule C retires.
- What a dial buys is a differential against the tier with that dial
  off.

Spec: `docs/DECIDE-10-SPEC.md`. Opus implementer. Review tier: dual,
concurrent arm, difficulty H (`work/decide/log.md`, 2026-10-12).

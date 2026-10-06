---
id: DECIDE-10
kind: unit
title: "the read at its node relabels a cancellation above it: settle the parent first"
status: review
opened: 2026-10-06
priority: P2
cost: M
branch: decide/10-read-behind-the-parent
refs: [the-read-at-its-node-relabels-a-cancellation-above-it, DECIDE-9]
---

## What

`the-read-at-its-node-relabels-a-cancellation-above-it`: the decision
read answers a `min`/`max`/`Select` node before its parent can cancel
it against an equal node, so `max(x + Z, 3) − max(x, 3)` counts
`sign_gated` where the read shut proves it.

Phase 1 measures the item's three candidates (settle read-free first;
read at the decision form, as a comparison only, since shipping it
changes a ratified decision; keep the node symbolic until its parent
has folded) on the shapes and on the measured documents. Phase 2 ships
the one Phase 1 justifies, with no decision's value moved.

Spec: `docs/DECIDE-10-SPEC.md`. Opus implementer. Review tier: single
FULL review (`work/decide/log.md`, 2026-10-06).

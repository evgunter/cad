---
id: check-9-lines-meet-falls-through-to-a-definite-meet-off-both-spans
kind: issue
title: check 9's lines_meet answers a definite Meet when each segment has one end reading Zero on the other's line but outside its span
status: open
opened: 2026-10-20
priority: P1
cost: E
---


## What

`crates/topo/src/validate.rs` `lines_meet`'s fall-through `else { EdgePair::Meet }` answers a definite `Meet` when one end of each segment reads `Zero` on the other's line but lies outside the other's span. That is a contact claimed where the honest reading is `Unsure`, or `Apart` past Kε.

**Witness** (found by the FULL reviewer of PR 5492): at ε = 1e-12,
- a = (360.9480040352915, 1006.8492807060521) → (366.1870448122066, 1019.5271473318302);
- b = (366.18704481221033, 1019.5271473318414) → (367.32778902648795, 1022.9567626675529).

The exact distance is 11.86ε > Kε (K = 10). Both orders read `Meet`.

**Scope.**
- **On main**, before PR 5492, this misreading reached segments 1 m apart.
- **After PR 5492** (the one-line gap screen read first), it is confined to the screen's charge window. It is still a wrong definite answer there.

## Fix

The fall-through must not mint a definite `Meet` from in-band end readings alone. Read the ends' positions along the other's span, through `geom_core::one_line` (charged), before claiming a meeting. Where neither side is definite, the answer is `Unsure`. Add the witness as a row, red on main.

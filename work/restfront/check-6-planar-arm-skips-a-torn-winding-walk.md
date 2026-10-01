---
id: check-6-planar-arm-skips-a-torn-winding-walk
kind: issue
title: topo: tier 3's check 6 planar arm reads a torn winding walk as a loop it need not ask
status: open
opened: 2026-09-30
---

(TOPO, the D262 unit's sweep, PR 3532.)

## What

Tier 3's check 6 planar arm (`crates/topo/src/validate.rs`, the loop
over each planar face's loops near the `LoopRoleInverted` push) reads

```rust
let Ok(Some(winding)) = body.planar_loop_winding(l, outward, band) else {
    continue;
};
```

The `Err(TornLoop)` arm, a lookup on the loop's walk that did not
resolve, lands in the same `continue` as an empty loop or an unread
carrier. The comment calls it "unreachable on tier-1 input", which is
the one proof DESIGN's D2 addendum rules out for row 4 ("never by the
body's tier-1 validity"), and the answer is silent either way: a torn
walk is read as a loop the check need not ask.

Since PR 3532, `TornLoop` names the key that failed
(`TornLoop::Dangling(DanglingRef)` or `TornLoop::CycleBroken`), so an
announcement has something to name.

## Repair shape

Separate the `Err` arm from the `None` arm: push the tier-1-shaped
error the walk's key gives (or skip only where tier 1 has already
reported that key, and say so), and keep `None`'s exemption.

---
id: check-6-planar-arm-skips-a-torn-winding-walk
kind: issue
title: topo: tier 3's check 6 planar arm reads a torn winding walk as a loop it need not ask
status: closed
opened: 2026-09-30
closed: 2026-10-04
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

## Closed

`TornLoop` is deleted and `Body::planar_loop_winding` answers
`Option`: a walk that meets a dangling link, does not close, or reads
a half its edge does not claim panics naming the record
(`loop_winding::winding_walk`, `winding_of_halves`; D2 row 4), so
check 6's `let Some(winding) = … else { continue }` skips only an empty
loop or an unread carrier. Tier 3 runs on a body `validate_closed`
cleared, so no tier-3 entry reaches the panic on a body it has not
already reported. Pinned by
`merge_faces::tests::a_torn_winding_walk_panics_naming_what_it_could_not_read`
and the read sweep
`review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise`.

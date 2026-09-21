---
id: a-nan-edge-distance-wins-its-boundary-rather-than-losing
kind: issue
title: A NaN edge-pick distance is installed as best and never displaced, beating every legitimate candidate
status: open
opened: 2026-09-17
priority: P0
cost: E
---

Found by the review of #2798, correcting that PR's own sweep
disposition. #2798's table disposed of `pickindex.rs`'s two clamps as
*"a NaN distance loses every `<` comparison… the silent-discard class,
not this one."* **At the edge-pick call site the opposite happens.**

## Finding

`crates/viewer/src/pickindex.rs`, the edge-pick walk:

```
                let (distance, closest) = segment_distance_px(cursor, pixel_a, pixel_b);
                if distance > EDGE_PICK_RADIUS_PX {
                    continue;
                }
                // Strictly nearer only: a tie keeps the earlier
                // segment, which is what makes the answer total.
                if best.as_ref().is_none_or(|best| distance < best.distance) {
```

Two steps, and a `NaN` survives both:

1. The reject is `distance > EDGE_PICK_RADIUS_PX`. A `NaN` takes
   neither side of it, so it is **not rejected**.
2. The winner test is `is_none_or`, which is **true** when `best` is
   `None`. So a `NaN`-distance segment that arrives first is installed
   as `best`.

After that no later candidate can displace it, because
`distance < best.distance` is false for every `distance` when
`best.distance` is a `NaN`. The segment does not lose — **it wins its
boundary and holds it against every legitimate candidate**, which is
the substitution class and not the discard class.

The sort below carries a comment that reads as a claim about this:
*"all integers after the first, and the first is never NaN (a
projected distance is a finite pixel measure)"*. It is `partial_cmp`
with `unwrap_or(Ordering::Equal)`, so the sort itself is total either
way; what the sentence asserts is the premise this item is about.

## Reachability: not established in either direction

`segment_distance_px` answers a `NaN` when its inputs are not numbers,
which needs a projected pixel that is not a number, which needs a mesh
position or a projection that is not. Neither was traced. **Said as a
negative result, not as a proof in either direction** — this row is
about the guard, which is wrong whatever reaches it.

## Fence

`crates/viewer/src/pickindex.rs` — VIEW's, the standing double claim
with CHROME.

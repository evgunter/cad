---
id: a-nan-edge-distance-wins-its-boundary-rather-than-losing
kind: issue
title: A NaN edge-pick distance is installed as best and never displaced, beating every legitimate candidate
status: closed
opened: 2026-09-17
priority: P0
cost: E
closed: 2026-09-21
branch: vgeom/p0-fields
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

## Resolution (2026-09-21, `vgeom/p0-fields`)

**At the measurement, not at the comparison below it and not at the
sort.** The per-edge walk is now `best_segment(cursor, boundary,
&projected)`, and its admission is
`distance.is_finite() && distance <= EDGE_PICK_RADIUS_PX` — the same
ordering the walk always had, plus the domain test an ordering cannot
make.

**Why there and not at the winner test.** Vetoing a `NaN` at
`is_none_or` would stop it winning and would still leave it measured:
the candidate carries a `pixel` the occlusion probe re-picks a ray
through, and `EdgePick::distance_px` whose own doc says *at most
`EDGE_PICK_RADIUS_PX`, by construction*. The admission IS that
construction, and siting it there makes the one guard answer all three
— the reject, the install, and the sort's first key.

**And the sort's comment is no longer the evidence.** It said *the
first is never NaN (a projected distance is a finite pixel measure)*,
which is the premise this row disputes. It now names `best_segment` as
the thing that makes it true. `pickindex-tie-break-rests-on-a-comment`
(P1, open) asks for exactly this — *the candidates are filtered before
the sort* — and this unit did not claim it; its own section records the
overlap.

## Reachability: unchanged, and the public door cannot carry a row

Still not established in either direction, as the row says. What this
unit adds is a negative result about testing it: **every public door
seeds on a ray through the same cursor and refuses first.**
`hovered_for` and `edge_at_for` both call `seed`, which un-projects the
cursor and runs the face pick; a cursor that is not a number, or a
`DisplayView::moved_roots` frame that is not, makes that pick miss, so
`edge_near` is never entered and no projected pixel is ever taken. The
rows are therefore at `best_segment`, with the projection handed to it,
and the reason is written at them.

**Rows**: `pickindex::tests::a_segment_whose_projection_is_not_a_measurement_does_not_win_the_boundary`
(a poisoned segment first, a legitimate one third; the answer is the
third, by INDEX and by distance, so a door returning a different wrong
float cannot pass it),
`a_boundary_that_projects_to_nothing_measurable_offers_no_candidate`,
and `the_radius_still_ends_where_it_did`.

**Mutation**: restoring `if distance > EDGE_PICK_RADIUS_PX { continue }`
reds the first two and nothing else.

PR: `vgeom/p0-fields`.

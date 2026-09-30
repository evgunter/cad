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

## Reachability: ESTABLISHED, through the public door (corrected 2026-09-21, review fix pass)

**The negative result this section first carried was false, and the
row it said could not be written is now in the tree.** It read: *every
public door seeds on a ray through the same cursor and refuses first
… `edge_near` is never entered and no projected pixel is ever taken.*
That is true of a cursor that is not a number and of a placement that
is not. It is not true of **a projected pixel that stops being a
measurement while every input is finite**, which is the arm the
register's *hunt the producer, not the input* rule exists for.

`segment_distance_px` mints the `NaN` itself: `length2 = dx² + dy²`
overflows once a pixel separation passes about `1.34e154`, the
numerator overflows with it, and `inf / inf` puts a `NaN` in `t` and
so in the distance. A projected pixel is an NDC scaled by
`ViewportSize`, which nothing bounds above — so the walk is entered,
and poisoned, with every argument to the door an ordinary finite
number.

Measured through `PickIndex::edge_at_for` on the plate fixture, at a
viewport of `1.28e155 × 7.2e154` physical pixels with the cursor
derived as the pixel midpoint of a drawn rim segment:

| tree | 1e2, 1e100, 1e150 | 1e155 | 1e160, 1e200 |
|---|---|---|---|
| before the admission | the rim, distance 0 | `Err(the camera's cursor x is NaN)` | the same `Err` |
| after it | the rim, distance 0 | the rim | `Ok(None)` |

So the pre-fix door did not merely rank a `NaN` first: it installed
it, sorted it first, handed its `NaN` pixel to the occlusion probe's
`Camera::ray_through`, and **refused the pick by naming the caller's
cursor for a pixel the walk had computed**. This unit repaired a live
defect reachable through the public API, not a door with no producer.

**What is NOT claimed.** `ViewportSize` is a door-level input with no
upper bound; no screen is `1e155` pixels across, and no user-reachable
producer was found. Two routes that WOULD need a poisoned input are
closed, exactly as first written: `Camera::ray_through` refuses a
cursor that is not a number, and `Camera::project` answers `None` for
a mesh position that is not.

The in-module rows stay where they are, for the arithmetic; the public
door has its own row beside them.

**Rows**: `pickindex::tests::a_segment_whose_projection_is_not_a_measurement_does_not_win_the_boundary`
(a poisoned segment first, a legitimate one third; the answer is the
third, by INDEX and by distance, so a door returning a different wrong
float cannot pass it),
`a_boundary_that_projects_to_nothing_measurable_offers_no_candidate`,
and `the_radius_still_ends_where_it_did`.

**Mutation**: restoring `if distance > EDGE_PICK_RADIUS_PX { continue }`
reds the first two and nothing else.

## The guard's spelling, and the fourth row (review fix pass, 2026-09-21)

The admission landed as
`!(distance.is_finite() && distance <= EDGE_PICK_RADIUS_PX)`, whose
`is_finite` conjunct the unit's own fourth mutation showed to be
inert: a square root is never negative and never `-inf`, so `inf` and
`NaN` both fail `<=` and the term cannot change an answer. It is now

```
        if distance > EDGE_PICK_RADIUS_PX || distance.is_nan() {
```

— both terms load-bearing, no negated partial comparison, and nothing
for `clippy::neg_cmp_op_on_partial_ord` to refuse.

**Rows**, adding the public door's:
`edge_pick::a_viewport_no_pixel_distance_can_be_measured_in_picks_no_edge`
— the pair, at one picture in five pixel units: the rim is answered at
a measurable scale and nothing is refused about the cursor past it.

**Mutations**, each against the whole 720-row default-feature run:

| mutation | red |
|---|---|
| drop `\|\| distance.is_nan()` (the pre-fix guard) | the two in-module rows **and the public-door row**, and nothing else |
| drop `distance > EDGE_PICK_RADIUS_PX`, keeping `is_nan` | nine rows, `the_radius_still_ends_where_it_did` among them |

PR: `vgeom/p0-fields`.

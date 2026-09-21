---
id: a-world-axis-ruling-row-is-green-over-a-ruling-along-the-normal
kind: issue
title: datum_draw's world-axis ruling row holds 'along a world axis', not 'the other two', and seven siblings red before it
status: open
opened: 2026-09-21
priority: P4
cost: E
---



## Finding

Found by VGEOM's `vgeom/deletions` lane while closing
`viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep`, whose
third site was a dead `assert_ne!` in this row. Filed here rather than
fixed: `crates/viewer/tests/*` is VDOC's by VGEOM's `keep_out`, and a
missing assertion found there is filed and not fixed across the fence.

`crates/viewer/tests/datum_draw.rs`,
`a_world_axis_planes_ruling_stays_on_the_other_two_world_axes`. Its
doc asserts the universal its name states — *"A world-axis plane is
still ruled along the other two world axes"*. What the body holds is
weaker: every drawn direction is parallel to SOME world axis, with the
plane's own axis admitted because the normal tick runs along it.

The row as it stood said the stronger half with a dead assertion —
`if along == axis_index { continue; }` immediately followed by
`assert_ne!(along, axis_index)`, which the `continue` makes
unreachable-false by construction. `vgeom/deletions` deleted it per
`docs/prompts/implementer-discipline.md` §2 and left the live check
(the `position(...)` panic) as an `assert!`, which is what the row
actually holds. Nothing about coverage moved: the deleted line could
not fail before the deletion either.

## What would break the claim the name makes

**Measured, not argued.** With `crates/viewer/src/datums.rs`'s ruling
call mutated to `rule_patch(&mut out, origin, normal, v, bounds,
pitch)` — a plane ruled along its own normal, the exact defect the
name forbids — this row is **green**, while seven sibling rows red:

```
cargo nextest run -p viewer --features app --no-fail-fast
  -> 798 run, 790 passed, 8 failed
```

the eight being `gpu::tests::every_pass_builds_on_a_real_device` (the
standing no-Vulkan red) plus
`a_grazing_view_is_ruled_to_the_window_and_toward_the_horizon`,
`a_plane_can_lose_its_extent_while_every_point_of_it_still_has_a_scale`,
`a_frames_grid_follows_its_own_axes`,
`a_plane_draws_a_tick_along_its_normal`,
`a_window_inside_one_cell_rules_that_cell`,
`a_planes_grid_lies_in_the_plane_for_every_axis_aligned_normal` and
`the_ruling_is_anchored_on_the_origin_not_on_the_view`.

So the gap is real and the suite is not blind to it: a ruling that
wandered onto the plane's normal is admitted HERE, as if it were the
normal tick, and caught three to seven rows over.

## The two dispositions, and why this is a row rather than a commit

That is the shape `work/view/plan.md`'s register calls out on
`a-supersession-outlives-its-own-frame`: where the suite already reds
several deep in every direction, **the deliverable is a citation and
not an assertion**, and writing the row the name implies would add a
name rather than a guard. So either

- the doc says what the body holds and names the rows that hold the
  rest (the register's preferred shape, and free), or
- the body separates the normal tick from the ruling by SOURCE rather
  than by direction — the tick is one segment pair emitted after
  `rule_patch` — and then the name is true here.

Either is VDOC's call. What is not an option is leaving a title that
certifies a population the body does not produce, which is the
register's *a universal in prose owes the sweep rule that produces its
population* applied to a test name.

**Where**: `crates/viewer/tests/datum_draw.rs`,
`a_world_axis_planes_ruling_stays_on_the_other_two_world_axes`.

**Confidence**: sure — the mutation receipt is above.

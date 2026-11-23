---
id: a-balls-volume-check-at-a-holes-rim-is-the-divergence-theorem
kind: issue
title: The ball tangent to a two-arc hole's rim is checked on the inclusion-exclusion volume identities alone, which hold whatever the result's faces are labelled
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found in PR 7395's dual review, where the same oracle was the
meridian-triangle witness's first form.

`crates/sweep/tests/a_split_through_a_wrap_edges_vertex.rs`
`a_ball_tangent_to_a_two_arc_holes_rim_is_held_or_builds_sound` holds
the built results to `|A − B| + |A ∩ B| = |A|` and
`|A ∪ B| + |A ∩ B| = |A| + |B|`. Each result's volume is the flux of
its boundary. When the three results are built from the same section
edges and the same face pieces, each labelled in or out, the identities
hold by the divergence theorem *whatever the labels are*. A wrong in/out
decision moves a piece from one result to another and keeps both sums.
So the row cannot go red on the defect a boolean's volume check is
meant to catch.

## What the taker owes

An independent measure of one result, `|A ∩ B|`, beside the
identities: a closed form where the geometry has one (as the file's
other rows have, `closed_form`), or a point-membership estimate
against both analytic solids with its tolerance stated. PR 7395's
`a_cone_on_a_torus_meridian_in_every_order.rs` does the latter.

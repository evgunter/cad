---
id: a-kept-chart-is-not-re-asked-when-a-door-changes-its-extent
kind: issue
title: Doors re-ask an OnPair over a changed extent or pair but keep a derived Chart, which tier 3 can now refuse
status: dispatched
opened: 2026-10-15
priority: P3
cost: M
branch: tang/kept-chart-re-asked
---


## What

Found by the landing sweep of
`a-declared-tangent-seam-flips-invalid-when-eps-tightens` (check 4 now
keys on the description's form, so a derived non-wrap `Chart` on an
edge the must-carry rule reads determinate refuses
`TransverseNotIntrinsic` / `TangentNotIntrinsic`). No witness yet.

Four doors change an edge's extent and re-ask an `OnPair` over the new
one (`geom_brep::reask_pair`: "the verdict reads the edge's extent"),
but keep a `Chart` description verbatim:

- `boolean::edge_join::joined_spec`, its `Chart` arm
  (`crates/topo/src/boolean/edge_join.rs:1014`);
- `split::tangent_piece` (`crates/topo/src/split.rs:317`);
- `replace_face::plan_reanchors` (`crates/topo/src/replace_face.rs:3697`);
- `shell::re_anchored` (`crates/topo/src/shell.rs:3276`).

A derived `Chart` on two distinct surfaces reaches rest from
`describe_edges`' non-stale smooth keep (`crates/topo/src/boolean/ops.rs`,
the `stale` match near `:2707`), where the reading was
under-determined on a zero-side second-order station. A different
extent reads different stations and could read jet-determinate, which
tier 3 then refuses. `edge_join` is the reachable one: `join_stage`
runs after `describe_minted_edges` (`ops.rs:2435`), and `join_one`
does not re-describe a non-planar join.

The offset doors' own-chart arms share the hole, over a new pair
rather than a new extent (review of PR 5191):

- `offset_restate::restate`'s keep arm (`beside_moving` answers
  `None`: both sides move, neither does, or the image names the
  moving side) keeps the `Chart`, or asks it derived afresh, against
  a pair whose surfaces moved, without asking the rule
  (`crates/topo/src/offset_restate.rs`, the `EdgeDescription::Chart`
  arm);
- `replace_face::plan_edge`'s moved-chart shift arm shifts a non-wrap
  image on the moved chart and keeps it a `Chart` beside a neighbour
  it now meets anew (`crates/topo/src/replace_face.rs`, the arm
  commented "Every other image on the MOVED chart shifts").

`crates/sweep/tests/offset_restates_a_neighbour_chart_rim.rs`'s
`the_planar_door_derives_the_image_afresh_when_both_charts_move`
witnesses the first: `offset_planes_together` returns `Ok` with the
edge an image in the minted side's chart, and tier 3 refuses the body
`TransverseNotIntrinsic`. Its input is a hand-stated chart image on
the same corner, which tier 3 refuses too, so no valid body is known
to reach it.

## Owed

A witness from a valid body (a boolean whose joined seam is a kept
smooth `Chart` on two curved surfaces), then the fix: each of these
`Chart` arms asks the rule over the edge as it now stands where the two
sides' surfaces are distinct, as their `OnPair` arms do.

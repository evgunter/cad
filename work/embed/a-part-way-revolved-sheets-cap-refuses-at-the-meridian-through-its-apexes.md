---
id: a-part-way-revolved-sheets-cap-refuses-at-the-meridian-through-its-apexes
kind: issue
title: A football revolved part way refuses where its planar caps meet the sheet: at the meridian through both apexes (departure) or out of depth
status: open
opened: 2026-11-25
priority: P2
cost: M
refs: [a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine]
---


## Finding

Measured by lane `embedwalk-spindle` (2026-11-25) with the at-rest
gate's per-solid ask on the branch giving a spindle sheet its window.
Untraced to the cell.

A football revolved part way is one sheet face (a lune from apex to
apex) and two planar caps, each meeting the sheet on a meridian that
ends at both apexes. The cap/sheet pair refuses:

- `sweep::all the_football_revolves::a_football_revolved_part_way_is_its_lune`:
  `Uncertified { a: plane, b: spindle, shared: Edge(..), arm: Departure }`,
  two of fourteen asks, the rest `Embedded`;
- `sweep::all a_conical_ended_football_revolves::a_football_revolved_part_way_measures_exact`
  (cones at the ends): `Budget { stop: Depth { max_cell_depth: 40 } }`
  on the plane/sheet pair, two of five asks.

The sphere's analogue, a ball revolved a quarter turn
(`sweep::all self_intersection_proves_injectivity::a_quarter_balls_lens_caps_hold_read_at_each_pole`),
holds: its caps are lenses meeting at both poles, read through the
corner at each pole, and a sphere face the stored chart leaves unread
is presented in a pose (`topo::injective`'s `present_posed`). A sheet
has no pose (`poses_of` finds none on any kind but the sphere), which
is the first difference to check.

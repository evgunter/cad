---
id: a-cylinder-circle-and-a-chart-junction-escalate-in-band-at-1e-12
kind: issue
title: Main red at ε 1e-12: props_cylinder_circle_off_axis (since PR 6505) and props_chart_junction_closed (since PR 6466) read a few e-12 in band, so two far-origin rods and every boolean of the 3° slanted bores refuse VolumeUncomputable
status: open
opened: 2026-11-14
priority: P1
cost: M
---



Filed by GREEN (`green/eps-1e-12`) from nightly 43371638371 (main
`e36675c1`, 2026-11-13); reproduced on main through `2c843a2e7`
(2026-11-14). Each row passes at the default ε and at 1e-6.

## What refuses

The body's finish check escalates a `geom_brep::props` predicate whose
margin sits in the band `{ zero: 1e-12, escalate: 1e-11 }`, so the body
is `NotFinished { VolumeUncomputable }`. This is a refusal, not a wrong
volume.

- **`props_cylinder_circle_off_axis`**, check `Exact`
  (`props/curved.rs`), margin `[0, 1.0235e-12]`:
  - `editor-core`
    `union_fold_face_face_touch::a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line`,
    the back 900 order (plate, rod, block);
  - `sweep`
    `parallel_cylinder_join::a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings`.
- **`props_chart_junction_closed`**, check `Inventory`
  (`props/closure.rs`), margin a value near `3.70e-12`: every one of the six booleans of
  `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself`'s
  3° pose (∩, ∪ and ∖, in both operand orders). The 12° pose builds all
  six.

## The merges

- **The two far-origin rods:** PR 6505 (`onedoor/one-surface-record`,
  ONEDOOR's face-anchored measure, merge `9e1484043`). A first-parent
  bisect of main at 1e-12 names it, and its first parent `e996c0049`
  passes both rows. The PR re-anchors each curved face's surface at its
  loops' first point before its closed form (`topo::props`
  `face_anchored`, through `geom_brep::props::loops_anchor` and
  `geom_brep::surface_anchored_at`). How that reaches the off-axis
  circle term is not measured.
- **The 3° slanted bores:** PR 6466 (`sectarms/cylinder-node-listing`,
  merge `c053873aa`). At its first parent `6775be78c` the row builds ∩
  and stops at ∪'s tier-3′ census
  (`work/flush/main-red-slanted-bores-a-union-b-census-ray-exhausted-at-1e-12.md`).
  The row evaluates ∩ before ∪, so ∩ built there. From the merge on,
  ∩ refuses its volume here, and so does every other cell. This predates
  PR 6505.

Reproduce (each reports the pin; the row itself is `<name>_row`):

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all \
      -E 'test(a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings)'

## Who cites this row

- **The pins:** the two far-origin rows above, and the slanted bores.
- **The six-cell probe:**
  `bisector_nodes::at_1e_12_the_slanted_bores_refuse_every_cell_and_the_glued_rod_escalates_in_both_orders`
  asserts every 3° cell's refusal.

## Done when

Each body finishes at 1e-12, its margin decided rather than the band
loosened, and each pin comes out with its row green at every ε.

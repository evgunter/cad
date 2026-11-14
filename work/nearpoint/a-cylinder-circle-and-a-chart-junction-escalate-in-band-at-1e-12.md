---
id: a-cylinder-circle-and-a-chart-junction-escalate-in-band-at-1e-12
kind: issue
title: Main red at ε 1e-12: props_cylinder_circle_off_axis (since PR 6505) and props_chart_junction_closed (since PR 6466) read a few e-12 in band, so three bodies refuse VolumeUncomputable
status: open
opened: 2026-11-14
priority: P1
cost: M
---


Filed by GREEN (`green/eps-1e-12`) from nightly 43371638371 (main
`e36675c1`, 2026-11-13); each row reproduces on main `2c843a2e7`
(2026-11-14) and is pinned at 1e-12 on main's stop
(`test_utils::pinned_red::row`), so the pin goes red the day it builds.
Each passes at the default ε and at 1e-6.

## What refuses

A body's finish check escalates a `geom_brep::props` predicate whose
margin sits in the band `{ zero: 1e-12, escalate: 1e-11 }`, so the body is
`NotFinished { VolumeUncomputable }`: a refusal, not a wrong volume.

| row | face | predicate (site) | margin |
|---|---|---|---|
| `editor-core` `union_fold_face_face_touch::a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line` (back 900: plate, rod, block) | `8v1` | `props_cylinder_circle_off_axis`, check `Exact` (`props/curved.rs`) | `[0, 1.0235e-12]` |
| `sweep` `parallel_cylinder_join::a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings` | `5v1` | `props_cylinder_circle_off_axis`, check `Exact` | `[0, 1.0235e-12]` |
| `sweep` `bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself` (3° A ∩ B) | `8v11` | `props_chart_junction_closed`, check `Inventory` (`props/closure.rs`) | value `3.70e-12` |

## The merges

- **The two far-origin rods.** A first-parent bisect of main at 1e-12
  names `9e1484043`, the merge of PR 6505 (`onedoor/one-surface-record`,
  ONEDOOR's face-anchored measure, which closed this slate's
  `a-planar-face-sums-its-area-about-a-far-carrier-origin`); its first
  parent `e996c0049` passes both. The PR re-anchors each curved face's
  surface at its loops' first point before its closed form (`topo::props`
  `face_anchored`, through `geom_brep::props::loops_anchor` and
  `geom_brep::surface_anchored_at`). Both rods stand far from the origin
  by construction, so the re-anchored circle's off-axis reading is the
  first suspect; unmeasured.
- **The slanted bores' A ∩ B.** That row stopped earlier, at A ∪ B's
  tier-3′ census
  (`work/flush/main-red-slanted-bores-a-union-b-census-ray-exhausted-at-1e-12.md`),
  until PR 6466 (`sectarms/cylinder-node-listing`, merge `c053873aa`;
  first parent `6775be78c` still stops at the census) cleared it; A ∩ B
  has stopped here from that merge on, before PR 6505. Whether A ∩ B
  refused before PR 6466 is unmeasured.

The same nightly's four `props_chart_loop_whole` rows moved at the same
PR 6505; they are BOXFINISH's cylinder edge-miss mechanism and filed
there
(`work/boxfinish/a-full-turn-bead-reads-its-volume-uncomputable-at-eps-1e-12.md`).

Reproduce (each reports the pin; the row itself is `<name>_row`):

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all \
      -E 'test(a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings)'

## Done when

Each body finishes at 1e-12 with its margin decided rather than the band
loosened, and each pin comes out with its row green at every ε.

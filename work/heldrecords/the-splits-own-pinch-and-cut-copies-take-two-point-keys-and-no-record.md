---
id: the-splits-own-pinch-and-cut-copies-take-two-point-keys-and-no-record
kind: issue
title: The split's own pinch and cut copies at one point take two point keys and no contact record, so tier 3′ refuses sound sides
status: open
opened: 2026-10-28
priority: P1
cost: M
---


Found by the tier-3′ gate measurement (`splitdoor/split-gate-3prime`, 2026-10-28), cause B. It is a blocker of `the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`.

## What

Where the split pinches, or two cut copies meet at one point, the copies get two point keys. That contradicts `split`'s doc ("copies share its point key"), and the split returns no record of the touch. So tier 3′ refuses sides whose volumes are right.

- **Straight:** payload `VertexVertex` + `EdgeEdgeOverlap`.
  - `a_keel_wrapped_at_both_ends_splits_as_a_ring::a_later_cut_across_the_ring_builds_and_so_does_one_through_its_tip`: the cut at x = 2 mints 28v1 and 29v1, both at (2, 2, 0).
  - editor-core `split_pinch_keeps::a_moat_reaching_the_block_walls_at_its_corners_quarters_both_sections`.
  - editor-core `split_pinch_keeps::a_diamond_moat_reaching_the_block_wall_…`.
- **Curved:** one `EdgeEdgeOverlap` on the rim circle r = 2, plus six `CensusUndecidable MayOverlap`. Every curved instance comes from one revolved-apex split:
  - `split_pinch_keeps::{a_split_through_a_revolved_apex_keeps_its_own_rim, a_revolved_pentagons_lower_top_is_its_two_sections_merged, a_lower_half_on_the_plane_takes_a_box_hole}`;
  - `name_invariance::{a_far_feature_moves_no_other_name, a_split_planes_frame_moves_no_name}`.

The curved slit (`a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs`) is this cause's curved instance.

## Direction

Decide whether the copies share one key (as the doc says) or the split returns the touch as a record; the doc and the code disagree today.

## Moved (2026-10-28, SPLITDOOR)

- Moved to HELDRECORDS (`work/heldrecords/`) by `git mv`, id and body unchanged: SPLITDOOR's seam cut put the records chain Ev ratified on PR 5967 on a track of its own (`work/README.md` Track size).

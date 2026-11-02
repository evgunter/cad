---
id: cover-topo-faces
kind: unit
title: topo::face_cover and edge_cover replace FaceBoxRule and both box lanes; Span's arithmetic goes; the two-cover overlap reads the meet
status: open
opened: 2026-11-02
priority: P1
cost: H
---


Third unit of the cover build (`cover-geom-readings` has the order and the ratified text).

- **`topo::face_cover` and `topo::edge_cover`, both `pub`.** An edge's cover is its arc cover hulled with its vertex points. A face's cover is the carrier's cover met with every trim constraint the reading proves:
  - plane: the boundary hull;
  - cylinder: slab ∧ hull;
  - cone: frustum ∧ hull, with the apex inside the axial window where the boundary winds round it (today's `rules::cone_axial_window`). The `ConeSlab` "jurisdiction" fence comes down;
  - sphere: ball ∧ chart rectangle or zone ∧ the pole-side narrowing. Today's census-only `held_reach_in` folds in as one more constraint;
  - torus: tube ∧ the closed-form chart rectangle;
  - spline: the net.
- **An unproven constraint is absent from the meet.** That covers one left in band and one that cannot be read. The cylinder arm's `splice_superseded` branch goes: an undecided axis is an absent slab term.
- **Both box lanes become sugar or go.** `face_box`, `FaceBoxes` = `aabb(cover)`; `census::face_reach(_in)` = `box_in(frame)`. `FaceBoxRule`, `face_box_rule`, `Span`/`SpanBox`/`UnitSpanBox`, `bracket_point`/`bracket_vector`, and `conic_arc_extent_along_bracketed` are deleted. `census::depth_gap` and `in_plane` read the cover's pieces, not the rule.
- **The two-cover overlap.** `glue::overlap_ball`, read by `ops.rs::overlap_reach`, gets today's poison ∩ X = X only through `f64` NaN-dropping `max`/`min`. It moves onto the cover meet, with a row for one face uncovered.
- **Keep the lanes-agree row's coverage** across the change; it now pins one lane.

Closes into this unit: `D280`, `sphere-operand-box-is-the-whole-ball`, and most of the P3/P4 arm rows parked on it. Close each one that the new arm settles, and re-file or re-park the rest with what still stands.

- **Retire `boxes::BoxFrame`** onto `geom::cover::Frame` (PR 6383's review: `Frame` is a near-copy, about 58 references remain). The `orthonormal_basis` gate's second home for `cover.rs` then becomes the only one.

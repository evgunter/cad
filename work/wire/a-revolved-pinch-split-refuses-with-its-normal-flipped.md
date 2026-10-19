---
id: a-revolved-pinch-split-refuses-with-its-normal-flipped
kind: issue
title: A revolved pinch split names with normal +y and refuses Emission 'section chord adjacent to a section face' with normal -y
status: closed
opened: 2026-10-18
priority: P3
cost: M
closed: 2026-10-19
---


## Finding

From the review of PR 5275 (2026-10-18). Re-measured by the `wire-keeps`
lane on that PR's head. The reviewer measured it at the base too.

Reproducer: the pentagon `(1,0) (3,0) (3,2) (2,1) (1,2)` drawn on the `xy`
frame, revolved a full turn about `y` (`axis_through (0,0)`,
`axis_heading turn_over(4)`), and split by the plane through `(0,1,0)`.
- With normal `+y`, it names. It is pinned in
  `crates/editor-core/tests/split_pinch_keeps.rs`
  `a_split_through_a_revolved_apex_keeps_the_circle_it_sweeps`.
- With normal `-y`, it refuses
  `Naming(Emission { what: "section chord adjacent to a section face" })`.
  The refusal comes from the chord pass of `emit_topo.rs`
  `name_split_edges_vertices`.

Flipping the normal should only swap the halves' tables, so one orientation
naming and its mirror refusing is a bookkeeping defect, not a naming choice.
The cause is not isolated. An unverified guess: the in-plane apex circle's
copy, which bounds a section face on both sides in the half that holds the
pinch.

Signed (wire-keeps lane).

## Resolution (2026-10-19)

Closed by PR 5334: a split finishes each half with maximal faces (recording a curved skip), names in-plane edges and merged groups per N2/N3 as Ev approved on PR 5306, and reads `Keeps` by descent in one builder. Pinned in `split_pinch_keeps.rs`, `m3_pr3_split.rs` and `a_lower_half_on_the_plane_takes_a_box_hole`.

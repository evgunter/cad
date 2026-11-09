---
id: a-loop-through-one-place-at-two-point-keys-is-read-by-its-first-vertex
kind: issue
title: a curved face's loop that passes one place twice through two distinct point keys within band is grouped by neither check 9's corner arm nor arm 1, so one_region's Any read answers its crossing by the first vertex placed
status: open
opened: 2026-11-09
priority: P3
cost: M
---


## Finding

Filed by `onecopy/shared-vertex-crossing` (PR 6611's review).

Check 9's corner arm (`validate::pinch_corner_errors`) reads a loop that
passes a point at two or more corners. It groups those corners by
point-key equality (`loop_point_groups`). Contact arm 1 reads two
distinct loops. So one loop that passes one place twice through two
distinct point keys within band, for instance two surface cycles of it
split by a strut, is read by neither. If the cycles cross there,
`one_region::read_cycle`'s `Answering::Any` ladder answers by the first
vertex placed.

**Measured.** The pinch probe in
`one_region::tests::two_surface_cycles_of_one_loop_crossing_at_a_shared_point_are_check_9s`
was rerun with the polygon's corner put 1e-12 from the outer corner on a
key of its own, instead of on the outer corner's key, and with the
cycles crossing there. `validate_geometric` reported no
`PinchCornerCrossed`. `one_region` reported `CycleOffFaceSide`, but only
because the first vertex it placed happened to lie off the face's side.
`validate_pseudomanifold` added nothing at that body, which tier 3
already refuses for its scaffold edges. Whether the census's
vertex–vertex coincidence (`UnrecordedContact`, or
`BoundaryCrossesItself` under a record, which is read on a solid) refuses the shape on a body
tier 3 passes is not shown. The at-rest gate runs no census.

**What closes this row:**
- show that the census refuses it on a well-formed body, and say so at
  `one_region::read_cycle`;
- or group the corner arm's corners by point within band, as
  `test_support_meeting`'s oracle does;
- or have the agreeing check (`a-ladder-that-checks-its-answers-agree-has-no-home`)
  read every vertex there.

**At a distinct key, a touch as well as a crossing goes unread by the
corner arm.** At a shared key both are refused (see
`one-region-reads-a-cycle-crossing-at-a-shared-vertex-by-its-first-vertex`):
the strut-split cycles' corners there are not slices. At two keys within
band the arm groups neither, so `one_region`'s read answers by its first
vertex either way.

## Resolved (`onecopy/first-vertex-reads`)

The third option, in its general form: a validator now reads a loop's side at every vertex. `topo::loop_side::inside_at_every_vertex` sits on `ladder::climb_agreeing`, and `one_region::read_cycle` and check 9's `ring_nesting` both read through it. A cycle with vertices placed on both sides of the cut is `CycleOffFaceSide` at an off vertex, whichever vertex the walk starts at. The type offers no first-vertex answer.

Probe: `one_region::tests::a_loop_crossing_itself_at_two_keys_within_band_reads_one_verdict_in_every_walk_order`. It is red on main (`["Face", "Off", "Face"]`) and red under a first-vertex mutant. The corner arm still groups corners by key. `pinch_corner_errors`' doc says where the two-key crossing is caught.

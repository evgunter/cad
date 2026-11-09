---
id: two-cycles-crossing-at-a-cones-apex-are-read-by-their-first-vertex
kind: issue
title: two surface cycles of a cone face's loop that cross at the cone's apex are read by neither check 9's corner arm (silent at an apex, which has no normal) nor arm 1, so one_region's Any read answers by its first vertex placed
status: open
opened: 2026-11-09
priority: P3
cost: M
---


## Finding

Filed by `onecopy/shared-vertex-crossing` (PR 6611's review).
`one_region::read_cycle` reads a curved face's cycles against each other
through an `Answering::Any` ladder, which answers with the first vertex
placed. Where two cycles share a point and cross there, check 9's corner
arm (`validate::pinch_corner_errors`) is what refuses the crossing. It is
silent at a cone's apex, where `face_outward_normal_at` gives no normal
to read the wedges about (its doc lists this residue). `one_region` does
read cone faces. So two surface cycles of one loop of a cone face that
meet at the apex and cross there are refused by neither check, and the
read answers by walk order.

Not probed. **Probe:** a cone sheet whose loop passes the apex at two
corners, as two surface cycles split by a strut or seam, with the cycles
crossing there. Run `validate_geometric` in both walk orders. If nothing
refuses, read the corners there by their departure directions on the
cone's generators, which are defined at the apex, or have the agreeing
check (`a-ladder-that-checks-its-answers-agree-has-no-home`) read every
vertex.

## Resolved (`onecopy/first-vertex-reads`)

The third option, in its general form: a validator now reads a loop's side at every vertex. `topo::loop_side::inside_at_every_vertex` sits on `ladder::climb_agreeing`, and `one_region::read_cycle` and check 9's `ring_nesting` both read through it. A cycle with vertices placed on both sides of the cut is `CycleOffFaceSide` at an off vertex, whichever vertex the walk starts at. The type offers no first-vertex answer.

Probe: `one_region::tests::two_cycles_crossing_at_a_cones_apex_read_one_verdict_in_every_walk_order`, on a new cone sector sheet whose loop passes the apex at two corners. It is red on main (`["Face", "Off", "Face"]`) and red under a first-vertex mutant. The corner arm stays silent at the apex.

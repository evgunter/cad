---
id: the-merge-role-pass-reads-a-merged-faces-rims-on-row-checks-again
kind: issue
title: Since PR 7440, a union's on-row iso-boundary checks on the sheath-on-blade pair read 100 Zeros where they read 66
status: open
opened: 2026-11-25
priority: P3
cost: E
---


Found by the LATHE one-band lane's slow-set diff on PR 7414
(2026-11-25), bisected on its branch `lathe/full-revolve-one-band`.

## What

`crates/sweep/tests/a_sheath_grafted_on_its_blade.rs`
`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim` counts
the union's `pcurve_iso_boundary` verdicts decided `Zero`: the on-row
checks the arc rims' shared-row test saves, "so a change to the union's
other checks does not move it". It read 66; it reads 100 in both
operand orders. The refusal it pins (`NotTransverse` at the rim, an
exact zero) is unchanged. The pin is re-pinned to 100 on PR 7414 with
this row cited.

## Cause, as far as measured

A first-parent bisect over the branch (`84588ee6fa..b5a35b27e7`) puts
the move at the merge of PR 7440 into the stack (`cf8b37e82b`, whose
second parent `e744969640` is that merge alone). Of 7440's source
changes, the one that reads pcurves is `crate::merge_faces`'s curved
role pass, which lifts each of a merged face's loops in each role in
turn through `pcurves::face_lift_as` (`crates/topo/src/merge_faces.rs`
~3005, `crates/topo/src/pcurves_rings.rs` `face_lift_as`) before it
sets the record's roles. That the 34 extra verdicts are those lifts is
inferred, not instrumented.

## Question

Whether the role pass should take the lifts' on-row checks from the
certificates the edges already carry, as the shared-row test does, so
the count returns to 66; or whether 100 is the price of asking each
loop's role and the pin's comment should say so.

## Ground

`crates/topo/src/merge_faces.rs` (the curved role pass): topo.

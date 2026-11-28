---
id: the-interior-loop-guards-section-walk-reads-every-pair-past-the-first-refusal
kind: issue
title: Since PR 7440 the interior-loop guard's section walk certifies every pair past the first refusal; the sheath-on-blade union reads 100 on-row Zeros where it read 66, and runs 60 s where it ran 11
status: open
opened: 2026-11-25
priority: P3
cost: E
---


Found by the LATHE one-band lane's slow-set diff on PR 7414
(2026-11-25); the first cause it named was refuted by the PR's
re-review (R1), and the cause below is measured.

## What

`crates/sweep/tests/a_sheath_grafted_on_its_blade.rs`
`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim` counts
the union's `pcurve_iso_boundary` verdicts decided `Zero`. It read 66;
since PR 7440 it reads 100 in both operand orders, and the row runs
about 60 s where it ran about 11. The refusal it pins (`NotTransverse`
at the rim, an exact zero) is unchanged. The pin is 100 on PR 7414,
with this row cited.

## Cause, measured

`crates/topo/src/boolean/ops.rs` `crossings_section_pairs`, the
interior-loop guard's walk over the crossings path's section pairs,
took a `stop` argument on base (`true` from `interior_loop_verdict`:
return at the first refusing pair). PR 7440 walks every pair (`stop`
`false` in `section_pairs`), because `interior_cuts::cut_ins` reads
all of them to plan its cut-ins. On this union a pair refuses early, so
every pair after it now certifies its section. Measured on
`lathe/full-revolve-one-band` at 9412bbadd7: with the walk switched back
to stopping at the first refusal, the count is **66** (and the row
takes about 11 s); walking every pair, **100** (about 60 s).

Ruled out on the way: the output merge's curved role pass
(`merge_faces.rs` `face_lift_as`, which R1 showed never runs on this
row), and 7440's `cut_ins` and `reenter_recut` (R1: none of the 34).

## Question

Whether the walk can stop at the first refusing pair where no cut-in
could apply (no pair on a sphere face, which is all `cut_ins` plans
for), reading every pair only where a cut-in is possible; that returns
this row to 66 and its time to about 11 s.

## Ground

`crates/topo/src/boolean/ops.rs` (`crossings_section_pairs`) and
`ops/interior_cuts.rs`: topo.

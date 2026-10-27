---
id: main-red-reach-volume-backstop-midpoint-premise-and-planted-subtraction
kind: issue
title: main red: reach_volume_backstop's midpoint premise holds no more and its planted subtractions stop refusing, since 692f50942c
status: closed
opened: 2026-10-27
priority: P2
cost: E
pr: 5842
closed: 2026-10-27
---


## Finding

Found by PCERT E1b's suite round (PR #5797), and red on main itself.
Two rows of `crates/sweep/tests/reach_volume_backstop.rs` fail at the
default ε:

- `a_result_equal_in_truth_but_larger_at_the_midpoint_passes`
  (`reach_volume_backstop.rs:489`): the fixture's premise no longer holds:
  "the two rods' midpoints differ (2.7488935718910685, 2.7488935718910685)".
  The two midpoints are now equal, so the row no longer exercises
  what it pins.
- `planted_subtraction_results_refuse_down_to_the_stated_resolution`
  (`reach_volume_backstop.rs:392`): "a fattening by δ = ε/100 is below the
  stated resolution", `left: Err(ResultVolumeImplausible)`,
  `right: Ok(Validated)`.

Red on main `be2b7a139c` and on `2f9e6f9e39`, with output identical to
the PR's tree. Both rows were green in E1b's earlier full round on a
tree carrying main `692f50942c`, so the move landed between
`692f50942c` and `be2b7a139c`; not bisected.

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses or passes on it. `reach_volume_backstop.rs` is now `curved_trims_build_at_their_volumes.rs`: the midpoint row tested the backstop's enclosure arithmetic and is deleted with it, and the planted subtractions are oracle rows (`planted_subtraction_results_break_the_volume_relations`: 1e-3 and 100ε fattenings flagged, the right answer and ε/100 held).

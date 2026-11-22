---
id: an-axis-edge-or-pole-a-reshaping-takes-is-reported-nowhere-end-to-end
kind: issue
title: A reshaping that joins or breaks a run on a revolve's axis, or makes an on-axis corner a station, is held against resolution by no row
status: open
opened: 2026-11-22
priority: P3
cost: M
---

## The finding

The `SetProgram` strand report reads a partial revolve's `AxisEdge(run)`
as its run and `Pole(v)` as a corner (`RoleSeg::referents`,
`crates/editor-core/src/names/role.rs`; REFERENCES.md DM7). Only the
unit row `names::role::tests::each_locator_segment_denotes_its_run_its_corner_or_its_piece`
covers that reading. No row puts a reshaping through the door and checks
its report against resolution
(`edit_set_program::report_through`, `crates/editor-core/tests/edit_set_program.rs`)
with the run or corner lying on the axis.

The harness cannot reach it as written, and the reason is the harness's
own contract, not the door. Found while writing
`edit_set_program::a_reshaping_that_joins_or_breaks_a_run_reports_it_whole`
with the bottom run laid on the axis (`stationed(..)` about a half
turn):

- Breaking or joining a straight run along the axis cannot keep its
  vertices on the axis. A value-collinear corner is refused
  (`JunctionTangent`), so the break moves a vertex off the axis, or
  onto it. The latitude rim there (`BandRim`) then comes or goes by
  value, which no report says (DM7: "resolution diagnoses the
  difference").
- A band over an on-axis piece never resolves at all, though the
  generic partition holds its run, so the door reports it when a
  reshaping joins that run.

`report_matches_resolution` holds a report to exactly the held names
that resolved before and not after. It therefore fails both cases, and
it cannot tell them apart from a missed report.

## Done when

A row puts a reshaping that joins or breaks an on-axis run, and one that
makes an on-axis corner a station, through the door. It asserts
`AxisEdge` and `Pole` names are reported exactly when the partition
takes them, under a contract that admits the value-made differences
above: say what the generic partition holds and the values do not, as
`undrawn_by_value` does for a `Zero` fit.



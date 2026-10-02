---
id: the-turntable-bench-has-no-python-row
kind: issue
title: the tour's bench stands on a turntable gauge and carries a crate on a nested gauge; no Python row authors either
status: open
opened: 2026-10-02
---


Found by SHOW's `bench-on-a-gauge` unit, which re-authored the tour's
bench onto gauges.

## What

`demos/tour/src/assembly.rs` (`stand_doc`, `turntable`, `bench`,
`poses`) now stands the mated stand on a turntable `Node::Gauge` whose
placement chain is `[to the pivot, Rigid(angle = swing), from the
pivot]`, with `swing` an Angle document parameter; the two mates go in
through `regauge_then_mate`. A crate instance stands on a shelf-top
gauge nested on the turntable, and its rest on the shelf is a mate
across the two gauges, which declares. Three `SetDocParamValue` edits
render `bench`, `bench60` and `bench90`.

`crates/pncad-py/tests/bench_scene.py`'s `stand()` still stands the
stand on the world, and no Python row authors the turntable, the
crate or the three poses. `docs/guide/north-star-audit.md` rows 43,
51 and 52 carry `YES\*` for exactly that.

## Evidence that it is a job, not a missing door

Every door is bound. A probe run against this branch's module
authored the turntable stand from `bench_scene`'s own parts:
`DocEdit.set_doc_param(swing, DocParam.angle(0 * rad))`, `Node.gauge`
over `Placement.literal(...).compose(Placement.rigid(..., angle=
doc.parse_expr("swing"))).compose(...)`, `DocEdit.set_gauge`,
`DocEdit.set_offset`, `Doc.regauge_then_mate` for both mates, then
three `DocEdit.set_doc_param_value` edits evaluated with `prior=`:
each re-ran 4 nodes and reused 2 (the stand without the crate; the
tour's bench, with it, reads 5 and 4), and `groups` answered one group
of three.

## The job

- Mirror the turntable in `bench_scene.stand()` (or a sibling the
  tests opt into: `TestBenchStand`'s `reading_edges` and
  `last_maintenance` rows read the world-standing stand's exact edge
  set and records, and the gauge adds reading edges).
- Author the crate on its nested gauge and its declaring mate.
- A Python row executing the three swings, asserting what the tour
  asserts: the re-keyed node set, every placed vertex against the
  chain composed by hand, the volume at every swing, and a certified
  gate at each pose.
- Extend `test_assembly_eval.py`'s `TestTheSceneIsTheToursOwn` to the
  new constants (`CRATE_*`, `PIVOT`, `SHELF_TOP`, `SWINGS`).
- Flip rows 43, 51 and 52 and re-derive the page's tallies.

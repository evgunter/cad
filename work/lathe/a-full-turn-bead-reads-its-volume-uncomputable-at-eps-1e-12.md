---
id: a-full-turn-bead-reads-its-volume-uncomputable-at-eps-1e-12
kind: issue
title: The full-turn bead fixture's revolve returns a body whose volume reads Indeterminate on props_chart_loop_whole at eps 1e-12, so full_turn_wall's height-window row is red on main
status: open
opened: 2026-11-12
priority: P1
cost: M
---


Found by BAND while bringing PR 6520 up to date with main. CI on its head
`2fe187ca58` (job `test (kernel)`) went red here, and the test fails
identically on origin/main `9ee1c046ef` and `6c70512339`, so it is not
6520's.

`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all -E
'test(both_doors_answer_the_bead_by_its_height_window)'` panics at
`crates/sweep/tests/common/bead.rs:39` (the fixture's
`revolve(..).unwrap()`, test at `crates/sweep/tests/full_turn_wall.rs:93`):

```
NotFinished { findings: [VolumeUncomputable { solid: SolidKey(1v1),
  source: Face { face: FaceKey(3v1), source: Escalated { cause:
  Indeterminate { margin: Enclosure { lo: 0.0, hi: 6.84e-12 }, band:
  Band { zero: 1e-12, escalate: 1e-11 },
  predicate: Some("props_chart_loop_whole") }, check: Inventory } } }] }
```

The bead's face area encloses `[0, 6.8e-12]` at a 1e-12 band: the
measurement does not separate the area from zero at this tolerance. This is
either a fixture too thin for 1e-12 (scale it or skip the eps) or the area
enclosure being looser than it needs to be; deciding which is this row.

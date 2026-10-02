---
id: steep-tube-cut-escalates-trim-containment-at-eps-1e-6
kind: issue
title: review_cleave_wrongarc's steep tube cut escalates pcurve_trim_containment at eps 1e-6 (red on main)
status: open
opened: 2026-10-02
priority: P2
cost: M
---


Found by REACH's pre-push run for PR 3805, and red on `origin/main`
itself (`3ee0e4b6e`, measured in a clean worktree), so it is not that
PR's.

## Measured

`crates/sweep/tests/review_cleave_wrongarc.rs`,
`steep_cuts_of_tubes_chord_inside_each_bore_face`, at
`CAD_TOLERANCE_EPS=1e-6`: `TALLY p2 tube: ok 46, wrong 0, refused 2`.
Both refusals are the same pose in its two orientations — tube
`a 0.4 d 0 turn 1.0682 tilt 1.2 at [0, 0, 0.5] phi 0`, `flip` false and
true — and both are

```
split refused: Pcurves(Certify { …, error: Escalated { check: TrimContainment,
  sample: 0, cause: Indeterminate { margin: -3.0645639348e-6,
  band: Band { zero: 1e-6, escalate: 1e-5 },
  predicate: Some("pcurve_trim_containment") } } })
```

a trim-containment margin of −3.06e-6 m, in the band's gap at 1e-6.
No wrong answer: the row is red because it asserts no refusals. At
1e-9 the row is green.

## What a fix has to decide

Whether the steep tube cut's pcurve trim reading should be resolvable
at 1e-6 (the margin is three zero bands, a lever question for
`pcurve_trim_containment`), or whether the row should accept a typed
refusal at the coarse band for this pose.

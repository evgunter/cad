---
id: main-red-lattice-built-loop-row-refuses-arc-miss-below-scene-resolution-at-1e-12
kind: issue
title: Main red at ε 1e-12: pncad's validated_decides_no_consistency_check_on_a_lattice_built_loop refuses ArcMissBelowSceneResolution on a fixture ~4·10⁴ m out
status: open
opened: 2026-10-20
priority: P1
cost: E
---


## What

`crates/pncad/tests/all.rs:6893`
`validated_decides_no_consistency_check_on_a_lattice_built_loop` is red at
`CAD_TOLERANCE_EPS=1e-12` on main `be82e5ad`; it is green at the default ε.
NURBS build A found it while running PR 4 of
`work/cpair/two-spans-projections-on-one-line-are-read-four-ways`, and
reproduced it on a clean main worktree. The message:

```
the builder authors the arc: ArcMissBelowSceneResolution { piece: Piece { step: 1, role: Leg }, verb: ArcTo, predicate: "path_arc_landing", point: Point2 { x: 36968.901970701314, y: -16772.04683566585 }, miss: 1.4999770885397668e-11, scale: 40699.090051304694, resolution: 1.4459221394000357e-10, turn: -3.6733397718953453, authored: None }
```

The refusal is the arc-landing tripwire that `[ev]` PR 5449 added (Ev,
2026-10-20: "we definitely want the tripwire … loud"). The fixture sits
about 4·10⁴ m out, where an arc's landing miss (1.5e-11) is below the
scene resolution (1.4e-10) at ε 1e-12. So the row's premise, "the
builder authors the arc", no longer holds at that ε.

## To decide

There are two candidate causes:
- the row needs an ε-gated premise, or a fixture nearer the origin, and the tripwire is right;
- the arc construction is ill-conditioned at this scale and could land tighter, which Ev's ruling asks to fix where possible.

Find out which, then fix accordingly. Test-file territory is lib, tcost and tint. The tripwire's ground is PATHS.

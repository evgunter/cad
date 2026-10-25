---
id: main-red-lattice-built-loop-row-refuses-arc-miss-below-scene-resolution-at-1e-12
kind: issue
title: Main red at ε 1e-12: pncad's validated_decides_no_consistency_check_on_a_lattice_built_loop refuses ArcMissBelowSceneResolution on a fixture ~4·10⁴ m out
status: closed
opened: 2026-10-20
priority: P1
cost: E
closed: 2026-10-21
branch: nurbs/lattice-loop-1e12
pr: 5541
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

## Closed

The second cause: the tripwire is right, and the row's premise needed
the run's ε. No construction can land this arc within the band at
40 km and ε 1e-12, so there is no ill-conditioning to fix.

**Why no construction can.** The landing is an f64 point compared
with the authored `b`, another f64 point. At the fixture `x ≈ 40 699`
has an ulp of `2⁻³⁷ ≈ 7.3·10⁻¹²` m and `y ≈ −16 772` one of
`3.6·10⁻¹²` m. So any landing that is not bit-identical to `b` misses
by at least `3.6·10⁻¹²` m, already past the band's zero `ε = 10⁻¹²`.
The measured miss, `1.5·10⁻¹¹` m (about two x-ulps), is near that
floor. The read-back's headroom, `K·ε/(16·2⁻⁵²) − scale`
(`profile::seg::resolution_headroom`, `LANDING_ULPS = 16`), turns
negative past about `2.8` km at ε 1e-12. Every definite miss beyond it
refuses `ArcMissBelowSceneResolution`, which is the honest verdict:
the scene's own rounding passes the tolerance there. The D-shape spans
about 9 km itself, so it cannot be read at 1e-12 even moved to the
origin.

**The fix (`crates/pncad/tests/all.rs`).** The row's fixture is
ε-aware. The same D (a bulge is a ratio, so the shape is unchanged),
scaled about the origin by `min(1, (landing_reach/4)/reach)`. At the
default ε and 1e-6 the scale is exactly 1, so the fixture is the 40 km
one, bit for bit. At ε 1e-12 it stands about 700 m out. There the row
also pins that the 40 km D itself refuses at the builder as
`ArcMissBelowSceneResolution` on `path_arc_landing`. The row is green
at all three ε. The tripwire is untouched.

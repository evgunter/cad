---
id: nurbs-surface-project-accepts-a-zero-speed-stationary-point
kind: issue
title: NurbsSurface::project accepts a stationary point where the chart's speed vanishes as a foot, at any distance
status: open
opened: 2026-10-26
priority: P2
cost: E
refs: [the-side-of-a-spline-face-is-read-by-no-dihedral, a-derivative-is-read-at-a-foot-no-one-placed]
---



Filed by the SPLICE lane that built `geom_brep::dihedral_side` (PR
5764), on its re-review. `crates/geom/src/surfaces/projection.rs` has no
owning program (`work.py territory`), so it lands on the slate of its
consumer.

`NurbsSurface::project_from_seed` (`crates/geom/src/surfaces/projection.rs`,
the acceptance test at ~251-256) accepts an iterate when the residual `r`
is orthogonal to both partials up to `PROJECT_EPS_COSINE`:
`|S_u·r| ≤ ε·|S_u|·|r|` and `|S_v·r| ≤ ε·|S_v|·|r|`. Where a partial
vanishes, the second test passes for every `r`, so a stationary point of
the chart's speed is accepted as a foot at any distance.

Measured: a degree-2 chart along `x` with a simple knot at `v = ½` and
the two middle control rows coincident (`(y, z)` rows `(−1, 1)`, `(0, 0)`,
`(0, 0)`, `(1, 1)`, folded along the `x` axis). The point
`(0.3, 1e-3, 1e-3)` lies ON the upper half, and its accepted foot is the
fold `(0.3, 0, 0)` at `v = ½`, 1.4 mm away. A consumer that decides that
distance reads a point on the chart as off it. `dihedral_side` now
refuses to decide a distance at an irregular foot (its
`a_point_on_a_kinked_chart_is_not_refused_off_it` row); `project` itself
still returns the stationary point.

The fix is in the acceptance test: a vanishing partial meets no
orthogonality condition. Step off it along the other partial's
direction, or refuse typed. That changes every `project` consumer's
feet, so it wants its own measurement first.

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

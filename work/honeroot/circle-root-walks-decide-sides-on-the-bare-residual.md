---
id: circle-root-walks-decide-sides-on-the-bare-residual
kind: issue
title: The circle-root subdivision doors decide a piece's side, the ON test and the bisection on the residual's bare value, with no rounding bound
status: open
opened: 2026-10-10
priority: P3
cost: E
refs: [degree-2-subdivision-doors-carry-no-root-slack-meter, 4522]
---


Found by the sweep of PR 4522's fix pass (the ruling charts), which
charges this rounding on its own residual.

## What

`circle_roots::certified_subdivision` decides each cut's side, the ON
test at a bisected root, and the bisection's own steps on the
`residual` closure its caller hands it, read as an exact value. Its
callers hand it the bare `f64` evaluation:

- `circle_torus::circle_torus_roots` (`circle_torus.rs`, the
  `half_angle_roots` call): `geom_brep::implicit_residual(torus, …)`;
- `conic_quadric::conic_quadric_roots` (`conic_quadric/mod.rs`, both
  `half_angle_roots` calls): `geom_brep::implicit_residual(surface, …)`;
- `ellipse_torus::torus_walk` (`ellipse_torus.rs`, the
  `certified_subdivision` call): the same.

Each now carries a `RootSlack` meter, whose running bound
(`conic_quadric_residual`, `conic_torus_implicit`,
`conic_torus_residual`) charges a located root's own rounding; the
sides are not charged. A cut whose residual's rounding is past the
band's zero can read a definite side the exact value does not have,
and a monotone piece's two ends then disagree with its root count.

## The shape of a fix

The ruling charts (`section_cert/ruling.rs`, `shrunk`) decide every
sign on the value less its running bound; the same `Rounded` residual
the meters already compute could feed the walk's `residual` closure
shrunk, as there. Measure first: the band's zero is far above the
residual's rounding except at ε 1e-12 on large reaches, where the
meters already refuse most such poses.

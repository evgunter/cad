---
id: the-window-reads-the-level-uncharged-where-the-subdivision-needs-its-bound
kind: issue
title: the coplanar subdivision's touch arm refuses on an exact local jet of the level, where the window reads that jet uncharged
status: dispatched
opened: 2026-11-06
priority: P3
cost: M
refs: [6541]
branch: onecopy/curve-pair-tidy
---


Found by `onecopy/curve-pair-tidy` (PR 6541), moving `coplanar_ellipses` onto `Level`. The move was taken out again; see `coplanar-ellipses-reads-the-level-from-its-global-harmonics`.

## What

`geom-brep` `curve_pair::pose` `Level::jet_at` reads `b`'s level along `a` locally. The window (`pose/window.rs` `Window::read`) reads that jet uncharged, as `[simple.conditioned]` has it: at `f64`, values are taken as real numbers.

The certified subdivision's touch arm (`half_angle::certified_subdivision_touching`) does not tolerate that jet. Handed it as the subdivision's Taylor terms with every error zero, `curve_pair::meet_rows::coplanar_conic_rims_tangent_at_a_vertex_meet_once_there` refuses at ε = 1e-9 ("inside, from the touch"): `Touch` on `curve_pair_coplanar_sub_side`, margin 2.59e-9.

It passes with any nonzero charge on the terms, whether the harmonics' own `Nᵏ·noise`, a `Rounded` running bound, or `rounding_charge(1 + |Fₖ|)`. So the touch arm's placement turns on a rounding charge being present: it is ill-posed where its result is decided, and the charge only hides that.

## Owed

Re-pose the subdivision's touch arm under `[simple.conditioned]` so that an exact jet places the touch. Measure first which reading decides it: the convex test, the reach's depth (`crate::roots::past_noise` at zero noise), or a cut's side. If no re-pose exists, the question goes to Ev.

The window's uncharged read is the ratified one and is not what this row changes.

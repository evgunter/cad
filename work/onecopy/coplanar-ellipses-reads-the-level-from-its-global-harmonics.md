---
id: coplanar-ellipses-reads-the-level-from-its-global-harmonics
kind: issue
title: coplanar_ellipses reads b's level from its global harmonics with a terms charge, where the window reads it locally
status: closed
opened: 2026-11-04
refs: [6541]
closed: 2026-11-06
pr: 6565
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(a) of the decision on that PR.

## What

`geom-brep` `curve_pair::pose` `coplanar_ellipses` reads `b`'s level along `a` from `coplanar_level`. That is one trigonometric polynomial of degree two in `a`'s angle, posed about `b`'s centre, with `rounding_charge(terms)` as the subdivision frame's `noise` and `terms ≈ ((|w| + R_a)/s_b)²`.

PR 6525 moved the window (`pose/window.rs` `Window::read`) onto `Level`: a local jet at each piece's middle (`Level::jet_at`), uncharged at f64, with `Level::fourth` as the Taylor remainder's bound. Posed globally, the level's rounding grows as the square of the pair's aspect and distance. At a far or thin coplanar pair the charge reaches the band's zero and refuses where a local reading decides.

`coplanar_level` is private to `pose.rs` and documented as `coplanar_ellipses`'s only reading. So two ways of reading one level remain inside the module.

## Owed

Move `coplanar_ellipses` onto `Level` through `SubdivisionFrame`'s `jet` slot (`half_angle.rs`), and delete `coplanar_level`.

## Tried in PR 6541, and taken out

The move was made and taken out by PR 6541's review (MAJOR-1). `coplanar_ellipses` read `Level::jet_at` through a local jet for the subdivision, with `|F⁗|` from `Level::fourth`.

The subdivision's touch arm then refused at ε = 1e-9 on an exact jet: `meet_rows::coplanar_conic_rims_tangent_at_a_vertex_meet_once_there` ("inside, from the touch") failed with `Touch` on `curve_pair_coplanar_sub_side`, margin 2.59e-9. It passed only once the jet carried a running rounding bound, whether `geom_core::running::Rounded` or `rounding_charge(1 + |Fₖ|)`. That contradicts `[simple.conditioned]`: at `f64` nothing charges float rounding.

So this row waits on `the-window-reads-the-level-uncharged-where-the-subdivision-needs-its-bound`, which re-poses the touch arm. Once an exact jet places the touch, the move here is the walk taking `F` either from its harmonics or from a local jet with its own `|F⁗|` bound and degree. The frame's `jet` slot alone does not do it: with a jet given, the walk still reads `f`'s harmonics for `|F⁗|` and for the degree.

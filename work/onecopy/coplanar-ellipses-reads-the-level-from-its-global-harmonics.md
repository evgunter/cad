---
id: coplanar-ellipses-reads-the-level-from-its-global-harmonics
kind: issue
title: coplanar_ellipses reads b's level from its global harmonics with a terms charge, where the window reads it locally
status: dispatched
opened: 2026-11-04
branch: onecopy/curve-pair-tidy
---


Filed by `onecopy/p0-needle` (PR 6525), under item 7(a) of the decision on that PR.

## What

`geom-brep` `curve_pair::pose` `coplanar_ellipses` reads `b`'s level along `a` from `coplanar_level`. That is one trigonometric polynomial of degree two in `a`'s angle, posed about `b`'s centre, with `rounding_charge(terms)` as the subdivision frame's `noise` and `terms ≈ ((|w| + R_a)/s_b)²`.

PR 6525 moved the window (`pose/window.rs` `Window::read`) onto `Level`: a local jet at each piece's middle (`Level::jet_at`), uncharged at f64, with `Level::fourth` as the Taylor remainder's bound. Posed globally, the level's rounding grows as the square of the pair's aspect and distance. At a far or thin coplanar pair the charge reaches the band's zero and refuses where a local reading decides.

`coplanar_level` is private to `pose.rs` and documented as `coplanar_ellipses`'s only reading. So two ways of reading one level remain inside the module.

## Owed

Move `coplanar_ellipses` onto `Level` through `SubdivisionFrame`'s `jet` slot (`half_angle.rs`), and delete `coplanar_level`.

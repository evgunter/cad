---
id: the-window-reads-the-level-uncharged-where-the-subdivision-needs-its-bound
kind: issue
title: the window reads Level's jet uncharged, where the coplanar subdivision refuses a touch on an exact jet
status: open
opened: 2026-11-06
priority: P3
cost: E
---


Found by `onecopy/curve-pair-tidy`, moving `coplanar_ellipses` onto `Level`.

## What

`geom-brep` `curve_pair::pose` `Level::jet_at` now returns `F`'s jet with a first-order bound on each term's rounding (`geom_core::Rounded`), and `coplanar_ellipses` hands it to the certified subdivision as `Terms::Local`. The window (`pose/window.rs` `Window::read`) reads only its values, `level.jet_at(m).map(|r| r.value)`, with `[0; 4]` as the errors. Its docs: "uncharged at `f64`… their few terms round far below the band".

The subdivision cannot take that reading. Handed the same jet with every error zero, `curve_pair::meet_rows::coplanar_conic_rims_tangent_at_a_vertex_meet_once_there` refused at ε = 1e-9 ("inside, from the touch"): `Touch` on `curve_pair_coplanar_sub_side`, margin 2.59e-9. Any nonzero honest charge passed: the harmonics' own charges, or `rounding_charge(1 + |Fₖ|)`. So a reading that rests on an exact jet can turn on the charge being zero, and the window's argument for dropping it is a size estimate, not a bound.

The height's jet (`Window::height_jet`) is read uncharged on the same argument.

## Owed

Charge the window's Taylor reads with `Level::jet_at`'s bounds, and give the height's jet the same running bound. Then read what moves in `pose/window.rs`'s rows and `order_rows`.

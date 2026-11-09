---
id: coplanar-ellipse-circle-charges-its-subdivision-with-global-harmonic-terms
kind: issue
title: coplanar_ellipse_circle charges its subdivision with its global harmonics' terms
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6656
---

Filed by `onecopy/pose-arms` (PR 6642), from its sweep.

## What

`geom-brep` `curve_pair::pose` `coplanar_ellipse_circle` hands the certified subdivision `noise: rounding_charge(terms)`, with `terms = (|w| + a + b)² + r²`, from the ellipse's distance-squared harmonics posed about the circle's centre. That is the conditioning charge `[simple.conditioned]` re-poses rather than charges. `coplanar_ellipses` was re-posed onto a local jet (`Level`) for the same reason (`coplanar-ellipses-reads-the-level-from-its-global-harmonics`, closed by PR 6565). The circle–ellipse arm is the remaining global reader.

## Owed

- Re-pose the circle–ellipse residual locally, as `Level` does for two ellipses, and delete the charge.
- Pin a far, thin pair whose charge reaches the zero band. The touch arm's behaviour on an exact jet is the evidence: see `the-window-reads-the-level-uncharged-where-the-subdivision-needs-its-bound`.

## Closed (PR 6656)

The ellipse–circle pair is read as two ellipses, following
`coplanar-ellipses-reads-the-level-from-its-global-harmonics` (PR 6565).
A circle is the ellipse of its equal semi-axes, its level
`(|q − c|² − r²)/r²` is `Level`'s `F`, and `coplanar_ellipses` reads it
locally, uncharged. `coplanar_ellipse_circle`, its
`rounding_charge(terms)` and `Coplanar::EllipseCircle` are gone. The
bare-rounding pin for `curve_pair/pose.rs` falls by the one site.

Measured over 81 poses (a ∈ {10, 1e2, 1e3}, three aspects, offsets 0,
1e2 and 1e3, touch/cross/apart, both orders, needle band):
- on main, 6 refused at this arm, every a = 1e2 touch at offsets 0 and
  1e2;
- after the change, each reads one meeting at the vertex, in both
  orders, and nothing else moved.

Pinned: `order_rows::a_thin_ellipse_and_a_small_circle_touching_at_its_vertex_meet_once`,
red on main.

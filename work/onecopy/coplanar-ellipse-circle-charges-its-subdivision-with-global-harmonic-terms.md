---
id: coplanar-ellipse-circle-charges-its-subdivision-with-global-harmonic-terms
kind: issue
title: coplanar_ellipse_circle charges its subdivision with its global harmonics' terms
status: open
opened: 2026-11-09
---

Filed by `onecopy/pose-arms` (PR 6642), from its sweep.

## What

`geom-brep` `curve_pair::pose` `coplanar_ellipse_circle` hands the certified subdivision `noise: rounding_charge(terms)`, with `terms = (|w| + a + b)² + r²`, from the ellipse's distance-squared harmonics posed about the circle's centre. That is the conditioning charge `[simple.conditioned]` re-poses rather than charges. `coplanar_ellipses` was re-posed onto a local jet (`Level`) for the same reason (`coplanar-ellipses-reads-the-level-from-its-global-harmonics`, closed by PR 6565). The circle–ellipse arm is the remaining global reader.

## Owed

- Re-pose the circle–ellipse residual locally, as `Level` does for two ellipses, and delete the charge.
- Pin a far, thin pair whose charge reaches the zero band. The touch arm's behaviour on an exact jet is the evidence: see `the-window-reads-the-level-uncharged-where-the-subdivision-needs-its-bound`.

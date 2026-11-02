---
id: the-canonical-implicit-forms-are-spelled-three-times
kind: issue
title: The canonical implicit forms are spelled three times, and a curved partner's composites read the interim f64 knots
status: open
opened: 2026-11-02
priority: P3
cost: M
refs: [a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm, 6304]
---


Filed by the NURBS × implicit arm's stage 1 (PR 6304), from its dual
review.

## What

The cylinder, sphere and torus canonical forms are written out three
times:

- `geom_core::spline::compose::canonical_composite` composes them with
  a curve, and `ssi::section::Pieces::implicit` reads a wall's sides
  through it;
- `pair_section/spline_implicit/curved.rs` (`hull`) spells them again as
  hand-expanded `PatchSpans` algebra over a wall's homogeneous channels;
- `ssi/residual.rs` (`Curved::value`, `Curved::frame_gradient`) spells
  them a third time as interval polynomials over a box.

The three can drift apart. A torus whose numerator is miswritten in one
of them would read differently at the hull than at the pass or the sweep.

The same three sites hold the composite's knot inputs:
- `hull` takes the wall's `knots_u/v_f64_interim` for `TensorCoeffs`;
- `Pieces::implicit` takes the side curve's `knots_f64_interim` for
  `CurveCertData`.

Those are three of C6's interim readers
(`scripts/gates/knots-f64-interim.counts`, geom-brep 219 → 222).

## Closing it

Give the forms one home, a form generic over the ring it is written in
(curve composite, tensor patch, box), with each site reading it. Have
that home take the carrier's held knots (`Knots`, `structure()`) rather
than the interim vector. That brings geom-brep's interim count back down
by three.

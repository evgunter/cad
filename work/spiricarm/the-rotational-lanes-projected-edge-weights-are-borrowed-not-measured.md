---
id: the-rotational-lanes-projected-edge-weights-are-borrowed-not-measured
kind: issue
title: The rotational lane's work weights for a projected edge are borrowed from the spiric and rational-rim lanes, not measured
status: open
opened: 2026-11-04
priority: P3
cost: E
---


Filed by the SPIRICARM stage 1 fix pass (PR 6478, 2026-11-04), from its
dual review.

## What

`geom_brep::props::quad::rotational`'s `face_cost`
(`crates/geom-brep/src/props/quad/rotational.rs:150`) weighs a round's
work on a face against `FACE_WORK_WALL`. Its harmonic edge's weight
(`W_CYLINDER`) is the lane's own. Its projected edges' weights are borrowed,
not measured on the integrands they weigh:

- a projected net's piece is weighed as a rational rim's piece
  (`w_rational(degree)` per span, `props/taylor.rs`), though its integrand
  is `u′ = (B_x·B_y′ − B_y·B_x′)/(B_x² + B_y²)` and `B_z/w` against `F`
  and `G`, not the vector-area cross product the rim's weights were timed
  on;
- a projected circle's piece is weighed as a spiric's (`W_SPIRIC`), on
  the argument that both read the trig jets and one reciprocal;
- a projected line's piece is weighed as a degree-1 rim's
  (`w_rational(1)`).

A rational chart term (`ChartTermQ::rationals`) is weighed as its rim
is, which is the rim's own integrand, so it is measured.

## Why it matters

The weights decide where a face meets the work wall and stops refining.
A weight below the integrand's real cost lets a face spend more time than
the wall means to allow; one above it stops a face short of a round it
could afford. Neither is unsound: the enclosure holds either way.

## Done when

Each projected weight is timed in release on its own integrand, as
`w_rational`'s table was (a timing probe, reverted), against the spiric
piece's 13.2 µs, and the table's numbers replace the borrowed ones.

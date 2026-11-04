---
id: the-spline-polynomial-identity-certificate-has-two-homes
kind: issue
title: spline_off_carrier and slit::carrier_on_surface are two homes for one polynomial-identity certificate, each with its own sample schedule
status: open
opened: 2026-11-04
priority: P3
cost: M
---

Filed by SPLICE's `splice/rim-arc-on-cylinder` (PR 6481), from its review.

Two readings rest on one certificate: an analytic carrier's equation
along a polynomial (or trigonometric) piece is zero on an open set only
where it is zero on the whole piece, so a point decided off certifies the
piece lies on the carrier nowhere.

- `crates/topo/src/boolean/slit.rs` `carrier_on_surface` reads an EDGE's
  line or conic against a face's surface. Its sample schedule: the ends
  for a line on a plane; five or nine points equispaced over the conic's
  turn (`Samples`, `even_points`), with a Lebesgue bound between samples
  on a torus.
- `crates/topo/src/boolean/reduce.rs` `spline_off_carrier` reads a spline
  FACE's sheet against an analytic carrier, one Bézier piece at a time.
  Its schedule: five interior points per piece. Its gate is an allow-list
  of the carrier kinds whose decided residual is their whole real variety
  (plane, cylinder, sphere, ring torus, double cone); the spindle is out.

Each states the identity argument and its own carrier-kind gate. A third
reading of the same shape (a spline edge per Bézier piece, say) would
make a third copy. One home in `geom-brep` would hold the argument once:
the carrier kinds whose residual is the whole variety, and "a point of
each piece decided off", taking the piece and its sample points as
input. The two callers would keep their own schedules.

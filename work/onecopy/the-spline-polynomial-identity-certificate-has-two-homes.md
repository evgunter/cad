---
id: the-spline-polynomial-identity-certificate-has-two-homes
kind: issue
title: spline_off_carrier and slit::carrier_on_surface are two homes for one polynomial-identity certificate, each with its own sample schedule
<<<<<<< HEAD
status: dispatched
=======
status: closed
>>>>>>> origin/main
opened: 2026-11-04
priority: P3
cost: M
closed: 2026-11-06
pr: 6577
branch: onecopy/spline-certificates
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

## Evidence: a third reader of the same shape, held back (2026-11-04)

Added by SPLICE's `splice/spiric-on-cap-plane` lane (PR 6523). A spiric
carrier against a ring torus is a third reader of this certificate's
shape. The spiric is an irreducible quartic, and the torus's equation
along it is real-analytic in `v`. So one point of it decided off the
torus's variety reads it off along every piece.

That lane left the pair unread, at `slit::conic_or_line_on`'s
`(Spiric | Nurbs, _)` arm. Holding it back is a choice, not a need:
- a spiric lying on its own torus, the case the lane gave as its reason,
  is one the arm's band guard already decides (every sample on, so
  unread);
- the arm's logic would extend to a torus.

It was held back so as not to write a third copy here.

Its spiric × sphere, cylinder and cone reading
(`slit::spiric_on_quadric`) rests on a different certificate: a Bézout
degree bound that samples nothing. Its sample scheme nearly duplicates
`reduce::spline_off_carrier`'s, though: a few interior points of a piece
read through `census::points_on_surface`, one decided off reading the
piece off. That scheme is a band guard here, where it is a certificate
there. The one home this row proposes would take the torus reading and
could hold the sample scheme too.

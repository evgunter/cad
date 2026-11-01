---
id: an-angle-opposite-its-spans-middle-is-a-whole-turn-at-the-certified-scalar
kind: issue
title: An angle opposite its span's middle is read as a whole turn at the certified scalar, outside the torus door's point readings
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [torus-windows-a-hair-apart-in-tube-angle-are-undecided-at-the-certified-scalar]
---

## What

`near_span` (`crates/topo/src/boolean/circle_roots.rs`) moves an angle by
whole turns to within `π` of its span's middle (`reduce_periodic_centred`
about the middle). An angle exactly opposite the middle lies at that
branch's cut. There the two representatives tie, and `Interval` holds the
angle as the hull `[mid − π, mid + π]`. A span reading on it is then in
band, although the angle lies definitely past the span on either branch:
its distance from the nearer end is half the span's complement.

It happens wherever the geometry is symmetric. In the hair-gap unit it was
the donut face's seam vertex, at the azimuth opposite the windows' middle:
every meridian leg from it read the windows' parallel edges at the cut.

The torus door's readings of a point's angle on an edge's arc now read
such an angle past the span by its cosine from the middle against the
half-span's, which needs no branch (`torus_region::arc_place`, PR on
`restcurve/hair-gap`).

## The same shape elsewhere, not fixed

Each reduces an angle onto a span's middle and then reads the span's ends:
- `ruled_region`'s `Conic::angle`, a point of a conic edge read on its
  span (the parallel leg × conic arm);
- `sphere_region`'s arc landings, roots reduced by
  `first_harmonic_roots_in_band` onto the arc's middle;
- `torus_region`'s meridian-leg × circle arm, the same root reduction
  (`circle_roots`);
- `circle_torus.rs`, `thetas[count] = near_span(theta, (t0, t1))`, the
  boolean's circle × torus door.
- `region::Round::arc`, a crossing's place along a leg's circle: the
  `turn` of `atan2` from the leg's start, whose cut is the point opposite
  the start. A crossing there is placed over a whole turn at the certified
  scalar, so it cannot be ordered against the path's other crossings. This
  course is shared by every region door's legs. **Taken by RESTCURVE's
  pole-door unit** (`the-sphere-door-cannot-settle-a-pole-at-the-certified-scalar`),
  whose branch rewrites `Round`'s swept angle with no cut; it is not this
  row's.
  - The repro is from PR 6244's review, probe `review_antipodal` on
    `restcurve/review-marten-6244` @ `cf2cf225`. It plants two windows on
    the donut's face at opposite azimuths, `((-1.2, -0.2), (-0.4, 0.4))`
    and `((π − 1.2, π − 0.2), (-0.4, 0.4))`.
  - The body is valid and validates at f64. At `Interval`, tier 3
    refuses it `CycleSideUndecided` at all three ε, with margin
    `[0, 7.5066]`, which is `π·ρ`.

A root's representative is not a point, so the cosine reading needs the
root's raw angle (`φ ± half-chord`), which the reduction discards. Either
the root door returns it beside the reduced one, or the readers read the
span on the raw angle.

## Not searched

The reductions in `splitting::classify`, `geom_brep::curve_pair` and
`half_angle`, which centre on other anchors: each has its own cut.

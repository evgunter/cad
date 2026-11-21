---
id: an-interval-revolve-of-a-long-cone-a-hundred-units-out-escalates-its-pcurve-envelope-at-1e-12
kind: issue
title: Revolving a cone 50 long with its apex 100 units off the origin at Interval, ε = 1e-12, refuses Pcurve(Certify): its meridian's envelope term FidelityV is in band
status: open
opened: 2026-11-21
priority: P3
---


## Finding

From EMBED lane pole (PR 7115, 2026-11-21), building the `Interval`
twin of a body row.

The triangle `(0, 151.3) (0, 101.3) (20, 151.3)` revolved a full turn
about `y` (`sweep::revolve`, `Revolution::Full`):
- at `f64` it builds at the default ε, at 1e-6 and at 1e-12;
- at `Interval` it builds at the default ε and at 1e-6, and at
  `CAD_TOLERANCE_EPS=1e-12` it refuses:
  `Pcurve(Certify { half_edge: HalfEdgeKey(10v1), error: Escalated {
  check: EnvelopeTerm(FidelityV), sample: 0, cause: Indeterminate {
  margin: Enclosure { lo: 0.0, hi: 1.6055626459115952e-12 }, band:
  Band { zero: 1e-12, escalate: 1e-11 }, predicate:
  "pcurve_envelope" } } })`.

The enclosure's width, `1.6e-12`, is the rounding of coordinates near
150 carried through the envelope, just past that band's zero. It is a
typed refusal, not a wrong body, hence P3. Not traced past the
payload: which term of `FidelityV` carries the width.

`self_intersection_proves_injectivity::cones_far_from_the_origin_hold_across_their_apexes`
(`crates/sweep/tests/`) builds this twin and says so, loudly, where it
does not build.

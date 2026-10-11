---
id: the-skin-at-interval-encloses-the-nominal-skin
kind: issue
title: The skin at Interval encloses the nominal f64 skin: a section wider than a point refuses instead of enclosing its family
status: open
opened: 2026-10-11
priority: P2
refs: [the-skin-computes-its-values-in-t, a-loft-is-the-nominal-surface-in-every-generic-lane]
---


Since `the-skin-computes-its-values-in-t` (#4726), a loft or sweep skins
its sections in their scalar at every projective scalar. At an
enclosure scalar (`Interval`, and `Dual`/`Sym` over it) it does not.
`sweep/src/loft.rs`'s `LoftBuild::enclosure` skins the sections'
`f64` value channel and lifts the walls through `from_computed`.
`SpanLocate::split_projective` (`geom-core/src/spline/locate.rs`,
`ProjectiveSplit` in `spline/projective.rs`) is the seam that chooses.

That is honest for a point section: the body encloses exactly the
`f64` skin of the section it was given. A section wider than a point
has no value channel (`SectionLoop::channel` maps it to NaN), so it
refuses typed at the `f64` structure pass
(`a_loft_over_a_section_wider_than_a_point_refuses_at_interval`). It is
still a narrowing: a toleranced loft over a parameter box cannot be
certified, only refused.

Owed, to enclose the family a wide section spans:
- a structure witness for the box: a nominal point that the
  sub-arc counts, merged knots, degree raise and span location are
  decided on, as C6 decides them;
- a span locator, and a knot algebra (elevation, refinement), that
  enclose at the certification scalar. The projective
  `apply_rational` is sealed against it (`ProjectiveScalar`), and a
  certificate holds a refinement only as homogeneous enclosures
  (`apply_certified`);
- a collocation solve whose averaged knots can be held where they are
  not points, or a parameterization fixed on the witness, so that
  `Knots::new`'s point rule still holds.

The enclosure half of `split_projective` is where this lands.
`a-loft-is-the-nominal-surface-in-every-generic-lane`'s `Interval`
enclosure pin waits on it.

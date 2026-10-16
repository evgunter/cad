---
id: an-enclosure-carrier-holds-point-weights-and-knots
kind: issue
title: At an enclosure scalar a carrier holds point weights and knots, so an arc section, or an interior averaged knot wider than a point, refuses where the loft should enclose its family
status: open
opened: 2026-10-11
priority: P1
design: true
cost: H
refs: [the-skin-at-interval-encloses-the-nominal-skin]
---


## Finding

Since `the-skin-at-interval-encloses-the-nominal-skin`, a loft or sweep
at an enclosure scalar (`Interval`, and `Dual`/`Sym` over it) skins
sections wider than a point in `T`, and the walls enclose the family
(`crates/sweep/src/loft.rs`, `LoftBuild::enclosure`). That holds only
where the family's walls are carriers. A carrier at every scalar holds
point weights (`crates/geom/src/net.rs`, `weight_channel`, refusing
`SplineError::WeightNotAPoint`) and point knots
(`geom_core::spline::Knots::new`, refusing `KnotNotAPoint`), because a
certificate reads a payload's weights and knots on their `f64` value
channel (`certified_coords`'s doc in `crates/geom/src/net.rs`; C6 in
`crates/geom-brep/README.md`: "a value that names no single point
refuses typed"). Two cases hit this:

- **Every arc section.** The weight `cos(Δ/2)` of an arc's sub-arcs is
  an enclosure at `Interval`, from outward rounding alone, even at a
  point sweep. `sweep::segment_curve` refuses
  `SkinError::ArcWeightNotAPoint`. Pinned by
  `crates/sweep/tests/a_loft_computes_its_walls_in_t.rs`
  (`what_a_carrier_cannot_hold_refuses_typed_at_interval`). Every
  document arc loft at `Interval` under the guided lift reaches it
  (`crates/editor-core/tests/e6_drive_over_a_loft_at_interval.rs`: the
  arc row still bisects to its leaf budget). Sections whose values are
  all points still loft through the lifted `f64` skin.
- **More sections than `v_degree + 1`.** The interior averaged knots
  are chord-length parameters (Book Eq. 10.8, `skin_parameters`). Over
  a wide section they are wide, and `Knots::new` refuses them as
  `SkinError::Fit(FitError::Structure(KnotNotAPoint))`. Pinned in the
  same test. With `k ≤ v_degree + 1` the knots are the clamped ends,
  and wide parameters only enter the collocation matrix, which does
  enclose.

The knot algebra (`make_compatible`'s elevation and refinement) does
not run at an enclosure scalar either (`SkinError::IncompatibleAtEnclosure`).
It is reachable only once arcs are, since two line strips are always
compatible.

## Options (a design fork; not chosen)

- **A. A carrier holds enclosure weights and knots.** Its structure is
  what every member shares: the span order certified over the knots'
  brackets, with the locator hulling the candidate spans. Every
  `Interval` reader of a carrier's `f64` structure would then read
  enclosures: `certified_coords` and the hull bounds, `RingRefinement`
  plans (built from `f64` knots), span location, and the pcurve
  caches. This is sound and closes both cases. It crosses
  geom-core/geom/geom-brep and re-words C6, which is a
  ratified-clause change.
- **B. Fix the parameterization on a witness.** The averaged knots
  become the `f64` ones of a nominal member. Over a real box the walls
  then enclose "each member interpolated at the witness's parameters",
  not each member's own `f64` loft. That is a nominal answer over a
  box, unsound against the `f64` lane at a sample (Monte Carlo, E6).
  It does nothing for arc weights.
- **C. A value-independent v-parameterization in every lane** (uniform,
  say). The knots are then points everywhere. This changes every loft's
  `f64` surface and re-baselines broadly, which is a definitional choice
  (Q8, Book Eq. 10.8). It does nothing for arc weights.

The two designers' weighing (`memories/orchestration-model.md`) comes
first. A and C touch ratified text, so whichever is chosen goes to Ev
as an `[ev]` PR.

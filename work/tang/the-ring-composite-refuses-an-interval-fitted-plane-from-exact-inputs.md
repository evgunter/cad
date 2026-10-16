---
id: the-ring-composite-refuses-an-interval-fitted-plane-from-exact-inputs
kind: issue
title: ssi::certify::composite_form refuses a plane fitted at Interval even from exact inputs, so the plane x NURBS lane cannot certify a loft's cap at Interval
status: open
opened: 2026-10-16
priority: P2
cost: M
refs: [plane-nurbs-certificate-refuses-at-rest-rows-of-lofts-and-sweeps, on-locus-hull-is-named-for-charts-whose-metres-composite-refuses, onpair-has-no-incidence-lane-on-a-spline-adjacent-pair]
---


Filed by the designer lanes of the plane × spline rim fork, off their
question. `plane-nurbs-certificate-refuses-at-rest-rows-of-lofts-and-sweeps`
lists this refusal among the `Intersection` upgrade's measured refusals.
This item is its own cause.

## Finding

`ssi::certify::composite_form` (`crates/geom-brep/src/ssi/certify.rs:448`)
builds the ring composite's implicit form from `f64` structure. It
refuses any analytic operand whose structural parameters are not point
brackets (`exact`, `certify.rs:430`; message `WIDENED`, `:449`). Its
doc calls that operand "a family of surfaces, not the body's surface".

A cap plane computed at `Interval` (Newell-fitted, normalized) is
never a point bracket, even from exact inputs. So limb 2 against the
plane refuses through `analytic_limbs` (`certify.rs:514`, reached at
`:1898`). That covers the plane × NURBS `Intersection` limbs and the
`OnPair` incidence lane (`crates/geom-brep/src/edge_nurbs.rs:975`), at
the interval scalar, which holds the lane
(`AtRestPolicy for Interval`, `crates/topo/src/props.rs:4174`).

The refusal is not about a widened input. It is reached by an interval
replay of an f64 build. For a plane the residual `n·(P − p₀)` is
linear, and an enclosure of it over enclosure-valued coefficients is
sound for every member of the family. The gate's own doc names that as
its retirement ("giving the composite enclosure-valued coefficients").

## Measured witness

Under the fork's probe (loft door switched to `OnPair`), with exact
`f64` inputs lifted by `Interval::from_f64`:
- `loft_profiles_door::at_interval_a_point_polygon_lofts_on_its_value_channel`
  (`crates/sweep/tests/loft_profiles_door.rs:132`) fails with
  `Euler(Certification { error: PlaneNurbs(Unsupported { what: "the
  analytic operand's structural parameters are not exact at this
  scalar …" }) })`;
- so do `a_loft_computes_its_walls_in_t::a_turning_sweep_at_interval_encloses_its_f64_sweep`,
  `bool6r1_probes_interval` (2 rows), `r1_area_gauge_probes::r1_e2e_gauge_is_silent_at_the_interval_lane`
  and `wire_loft_end_profile_lift::interval::each_seam_strut_at_interval_encloses_the_f64_world_points_exactly`.

All six pass when the plane side is read instead through its chart
image (`chart_pcurve_over` + `PcurveCache::certify`). That only shows
the plane side is the sole refusal. It is not a proposed fix for
`Intersection`, whose limbs read the implicit form.

## Acceptance

A plane operand's limb 2 encloses its residual from enclosure-valued
coefficients:
- the point-polygon loft's rims certify at `Interval` on the lane;
- a genuinely widened plane widens the bound instead of refusing;
- a cone or torus still refuses for want of a root (C2).

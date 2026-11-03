---
id: site-rounding-charges-owed-a-re-posing
kind: issue
title: The rounding charges the run-owns-rounding lane could not re-pose: each a quotient by a vanishing quantity or a squared form, owed a pose or a distance form
status: open
opened: 2026-11-02
---


Filed by PR 6380 (`onehome/run-owns-rounding`), which builds the run's
admission rule (`geom_core::tolerance::admissible`) and deletes the
charges of the coordinates' floor (the short-chain charge in
`solid_contain`'s vertex gap, the coordinates' share of
`ring_path::resolution`, which now reads only a reading's own spread
about its first point). Under
`crates/geom-core/README.md`, "Float rounding", a charge that exists
because its formula is ill-conditioned is re-posed, not deleted. These
were not re-posed there; each stays, and each production file holding
one is a `re-pose` or `pose` row (or holds those words) of `scripts/gates/bare-rounding-sites.tsv`
(which counts bare `f64::EPSILON` / `UNIT_ROUNDOFF`, `rounding_charge(`
calls and `Rounded` paths). Every such row's file is named below; a
re-posing that empties a file lowers or drops its row in the same change.

## Quotients by a vanishing quantity (a pose decided first)

- `topo::vertex_section`, `read_sections`: the vertex's offset over the
  section's `sin ψ` (`Rounded::quotient`), attributed by
  `validate::resolved` at its `ε/K` share. Reading the headroom there
  instead offers the kernel's rounding as the user's lever at
  `ψ ≈ 0.1·K·u/ε` (sweep `a_vertex_on_a_grazing_plane`'s
  `a_reading_through_a_vanishing_sine_offers_no_lever` and
  `an_unresolved_section_reading_keeps_the_offsets_verdict`), so
  `validate::resolved` keeps its share until this is posed. Since PR 6376
  the quotient goes through `Rounded::divisor()`: a sine that does not
  clear its own bound leaves the offset's verdict standing (Ev's PR 4812
  rule). Re-pose it as a tangency pose decided before any division; do
  not remove `divisor()` without that.
- The charges PR 6376 routed through one home each:
  `Rounded::{room, toward_zero, divisor}` (`geom_core::running`), read by
  `vertex_section::read_sections`, `section_cert::ruling`'s `shrunk`,
  `pair_section::gap` and `solid_contain`'s `charged`. Each re-posing
  deletes one helper's site. `ruling`'s `before` is band-derived, not a
  rounding charge.
- `topo::census`, `pierce_span` / `span_reach` / the curve-clear
  charges: the pierce parameter `r0 / (r0 − r1)` and its reach, charged
  through the division by the lever.
- `topo::boolean::solid_contain`: the ray–plane crossing's `slack`, over
  `denom` (a near-grazing ray), the wall roots' `over_root_slope`, and the
  torus roots' slack.
- `topo::boolean::conic_quadric::mod`, `ellipse_torus`, `circle_torus`,
  `reduce` (harmonic rungs): `rounding_charge(h.terms) / h.per`, a
  harmonic residual's noise over its per-metre rate.
- `geom_brep::pair_section::gap` (`e_r1`, `e_r2` over `rho`) and
  `pair_section::ruling` (`tilt`).

## Squared forms and cancelling sums (a distance form, or the Interval lane)

- `geom_brep::implicit`: `sample_rounding`'s squares over `2r` (its
  point's far-centre share is below, under "Not this row"), and the
  harmonic machinery's `noise` (`conic_residual_harmonics`,
  `conic_torus_harmonics`, `CEILING_PAD`), which the arc enclosure's
  Bernstein bound reads; the residual is the squared form, so a distance
  form changes the enclosure's function.
- `topo::boolean::section_cert::ruling` (`Rounded` throughout, its own
  `CEILING_PAD`), `geom_brep::curve_pair::pose` and `pose::window`
  (the window polynomials' `noise`, six of `pose`'s seven sites and all
  of `window`'s), `geom_brep::half_angle`,
  `geom_brep::chart_singularity`, `topo::sector_shape`.
- `geom_brep::props::curved::require_turn_read`: a sign of an exact norm
  of two cancelling tangents, which a charge holds off zero; owed a
  decision at the band, levered.

## The gate's other production rows (each a charge or a pose read at rounding)

- `editor-core::resolve::pick` (`t_span`, `early_out_margin`,
  `crossing`, `quotient`, `triple_bound`): a ray–triangle crossing's
  error bounds, Shewchuk-style; owed the crossing posed as an orientation
  decided at the band.
- `editor-core::mate::value::chain_turns` (`pose`): gimbal lock read as
  `cos β > √u`; owed a decision at the band.
- `mesh::trimmed` (`sphere_polygon_shape`, `at_pole`) (`pose`): a pole
  read at a few ulps of `π/2`.
- `mesh::cert::rounding_allowance`: the verdict's own rounding.
- `geom_brep::edge_nurbs::kernel_limit_ending`: a message's "kernel
  limit" arm, read at a width of a few ulps of the extent.
- `geom_brep::extent::turn_lever`: the axial offset's rounding.
- `geom_brep::intersect` (`plane_cone_section`'s gap and axis charges,
  `pair_bound`'s seam).
- `geom_brep::pair_section::spline_implicit::bracket`,
  `pair_section::torus` (`decide`, `sphere_gap`),
  `pair_section::torus_arms` (`square_pose`, `square_folds`).
- `profile::sugar::offset_circles`: the least lever, at `LEVER_ULPS`.
- `topo::boolean::boxes::sphere_reach`, `topo::boolean::circle_roots`
  (`conic_off_plane`, `harmonic_crossing`),
  `topo::splitting::section_loops::conics_clear`.
- `topo::injective::few_ulps`, `topo::separation::image`'s slop,
  `topo::props::rounding_bound`, `topo::props::quad_lane::end_slack`.

## Not this row

- `geom_brep::carrier_resolution` and `profile::seg::resolution_headroom`
  (with `BelowSceneResolution`), which guard a carrier stored far from the
  arc it serves: they wait on
  `work/onehome/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex.md`.
- The gate's `far-centre` rows, which wait on the same row:
  `topo::ring_path` (`resolution`'s spread about the reading's own point,
  and `path_resolution`'s conic arcs through `carrier_resolution`),
  `topo::chord_join` (`carrier_resolution` on a section pin),
  `geom_brep::curve_pair::pose`'s `one_curve_gap` (its conics' stored
  semi-axes; `pose`'s row is `re-pose,far-centre`),
  `geom_brep::implicit::sample_rounding`'s point term `|C₀| + speed_hi`
  (a conic point summed from its stored centre and semi-axes; the row is
  `re-pose,far-centre`), and `geom::curves::conic`'s `running_point`
  (the same evaluation, `Rounded`).
- `geom_brep::ssi::compensated` (`exact-arith`) and
  `editor-core::test_support` (`fixture`): not charges.
- `geom_core::one_line`'s charge (inside geom-core): its precision arm is
  reached only by spans of several kilometres at K = 10, past the box,
  and by a run whose K is near 1.

- 2026-11-03 — retired on main by the boxes cover refactor (PR merged
  before PR 6380's last merge of main), so dropped from this list and
  from the gate's rows: `geom::curves`'s spiric charges
  (`spiric_period_box`, `spiric_f_enclosed`) and
  `topo::splitting::containment::conic_arc_ball`'s `|C₀|` pad.

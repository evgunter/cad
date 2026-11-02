---
id: site-rounding-charges-owed-a-re-posing
kind: issue
title: The rounding charges the run-owns-rounding lane could not re-pose: each a quotient by a vanishing quantity or a squared form, owed a pose or a distance form
status: open
opened: 2026-11-02
---


Filed by PR 6380 (`onehome/run-owns-rounding`), which builds the run's
admission rule (`geom_core::tolerance::admissible`) and deletes the
charges of the coordinates' floor (`ring_path::resolution` and
`path_resolution`, `implicit::sample_rounding`'s `|C₀|` term, the two
short-chain charges in `solid_contain`'s vertex gap and `curve_pair::pose`'s
one-curve gap). Under `crates/geom-core/README.md`, "Float rounding", a
charge that exists because its formula is ill-conditioned is re-posed, not
deleted. These were not re-posed there; each stays, and each is a
`re-pose` row of `scripts/gates/bare-rounding-sites.tsv` where it names
the unit roundoff bare.

## Quotients by a vanishing quantity (a pose decided first)

- `topo::vertex_section`, `read_sections`: the vertex's offset over the
  section's `sin ψ` (`Rounded::quotient`), attributed by
  `validate::resolved` at its `ε/K` share. Reading the headroom there
  instead offers the kernel's rounding as the user's lever at
  `ψ ≈ 0.1·K·u/ε` (sweep `a_vertex_on_a_grazing_plane`'s
  `a_reading_through_a_vanishing_sine_offers_no_lever` and
  `an_unresolved_section_reading_keeps_the_offsets_verdict`), so
  `validate::resolved` keeps its share until this is posed.
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

- `geom_brep::implicit`: `sample_rounding`'s squares over `2r`, and the
  harmonic machinery's `noise` (`conic_residual_harmonics`,
  `conic_torus_harmonics`, `CEILING_PAD`), which the arc enclosure's
  Bernstein bound reads; the residual is the squared form, so a distance
  form changes the enclosure's function.
- `topo::boolean::section_cert::ruling` (`Rounded` throughout, its own
  `CEILING_PAD`), `geom_brep::curve_pair::pose` and `pose::window`
  (window polynomials' `noise`), `geom_brep::half_angle`,
  `geom_brep::chart_singularity`, `topo::sector_shape`.
- `geom_brep::props::curved::require_turn_read`: a sign of an exact norm
  of two cancelling tangents, which a charge holds off zero; owed a
  decision at the band, levered.

## Not this row

- `geom_brep::carrier_resolution` and `profile::seg::resolution_headroom`
  (with `BelowSceneResolution`), which guard a carrier stored far from the
  arc it serves: they wait on
  `work/onehome/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex.md`.
- `topo::splitting::containment::conic_arc_ball`'s `|C₀|` pad: the same
  far-centre conic.
- `geom_core::one_line`'s charge (inside geom-core): its precision arm is
  reached only by spans of several kilometres at K = 10, past the box,
  and by a run whose K is near 1.

---
id: site-rounding-charges-owed-a-re-posing
kind: issue
title: The rounding charges the run-owns-rounding lane could not re-pose: each a quotient by a vanishing quantity or a squared form, owed a pose or a distance form
status: open
opened: 2026-11-02
priority: P2
cost: M
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
  through the division by the lever. `span_reach` also charges
  `rounding_charge(|p₀| + len·|t|)`, a world-origin term that grows with
  the edge's distance from the origin rather than with what it sums
  (PR 6638's review).
- `topo::solid_contain`: the ray–plane crossing's `slack`, over
  `denom` (a near-grazing ray), the wall roots' `over_root_slope`, and the
  torus roots' slack.
- `geom_brep::curve_face::conic_quadric`, `ellipse_torus`, `circle_torus`,
  `reduce` (harmonic rungs): `rounding_charge(h.terms) / h.per`, a
  harmonic residual's noise over its per-metre rate.
- `geom_brep::curve_face::pose` (the first-harmonic lane, moved there
  from `conic_quadric`): `fn conic_quadric`'s circle × sphere extremes
  charged the frame's defect (`lo`/`hi.with_charge(h.frame_error)`),
  and `lane`'s first-harmonic extremes, each within the harmonics'
  noise plus the second harmonic's amplitude (`extreme`).
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
  (`pose`'s sites are the gate's row for it; its line router's two
  world-origin mints are scheduled by
  `work/onecopy/the-line-router-charges-its-margins-with-world-origin-noise.md`,
  and `window`'s polynomials read locally since PR 6525),
  `geom_brep::half_angle`,
  `geom_brep::chart_singularity`, `topo::sector_shape`.
- `geom_brep::curve_pair::pose::Level::jet_at`: the level `X²/A² + Y²/B² − 1`
  is read exact by `pose::window` and by the coplanar subdivision. It
  charges nothing, but it is the same debt: it divides by the squared
  semi-axes, so its rounding as a length grows with the aspect, which
  the admission rule does not cover. Measured under PR 7221 at 0.77
  (height) and 1.5 (level times `L`) zero bands on the far needle; the
  thin far `b` pose is unbuilt; PR 6565's touch arm reads the same jet.
  Whether a definite verdict goes wrong at an admitted (ε, K) is not
  known without a probe. So it has its own row at P1:
  `the-windows-level-jet-is-read-exact-where-its-rounding-grows-with-the-aspect`.
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
- `mesh::chart_table` (`SingularPoint::passed_by`) (`pose`): a point
  past a singular row that ends the chart's domain (a sphere's pole),
  read a few ulps past the row's `|v|`.
- `mesh::cert::rounding_allowance`: the verdict's own rounding.
- `geom_brep::edge_nurbs::kernel_limit_ending`: a message's "kernel
  limit" arm, read at a width of a few ulps of the extent.
- `geom_brep::extent::turn_lever`: the axial offset's rounding.
- `geom_brep::intersect` (`plane_cone_section`'s gap and axis charges,
  `pair_bound`'s seam).
- `geom_brep::ssi::certify::frame_direction` (`FRAME_ROUNDING`, `pose`):
  a frame direction lifted to an enclosure is read at its midpoint where
  each component is at most 16 ulps wide, and refused as a family past
  that; owed with the midpoint read itself. The ring composite's
  reading of it is gone (2026-11-21, the search's limb 2 reads through
  the edge certificate's enclosure-valued reader); the chart tube's
  partner form (`chart_partner_form`) is its one reader now.
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
  `work/oneround/a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex.md`.
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

- 2026-11-08 — PR 6380's merge of main (2,450 commits behind) re-seeded
  the gate's rows. **Moved on main**, same charges, same schedule:
  `topo::boolean::{circle_roots, circle_torus, ellipse_torus}` are now
  `geom_brep::curve_face::{circle_roots, circle_torus, ellipse_torus}`;
  `geom_brep::edge_nurbs::kernel_limit_ending` is now
  `geom_brep::recourse::kernel_limit_ending`; `topo::sector_shape`'s
  `Rounded` difference is now `geom::frame::plane_offset_rounded`.
  **New on main since the gate was seeded**, each a `re-pose` row, owed
  the same re-posing as the squared and cancelling forms above:
  `topo::spline_trim` (`floor`, `blend`, `point_rounding`,
  `bezier_cells`: a spline enclosure's per-control rounding),
  `geom_brep::pcurve_cache::spline_incidence`, `pcurve_cache` itself and
  `pcurve_cache::projected`, and one more site each in
  `curve_pair::pose`, `implicit` and `pair_section::spline_implicit`.
  These entered main before the gate did, so nothing ratcheted them.

- 2026-11-09 — the gate derives its doors from `geom_core::running`'s
  `pub` items (`onecopy/gate-blind-spots`), so the rows above now also
  count `direction_rounding`, `exact_vec`, `unit_defect`, `running::dot`
  and `cross`, `SHORT_CHAIN_ULPS` and `DIRECTION_UNITS`. The same charges
  are now seen through doors the hand-spelled list missed:
  `section_cert::ruling`, `implicit`, `pcurve_cache`, `props::curved`,
  `geom::frame`, `geom::curves::conic`, `topo::vertex_section` (its
  `direction_rounding` over the section's sine) and `conic_quadric::mod`
  (a `direction_rounding` term). **New row**: `topo::boolean::fan`
  (`pose`): `conditioned` reads `direction_rounding(arm)/s` against the
  band to decide whether two fans' crossing is read. That is rounding
  read as the zero of a pose, owed a decision at the band on `s` itself.
  The same lane swept `rounding_charge` terms that grow with distance
  from the world origin. Each hit is already scheduled, here or under
  the far-centre row: `curve_face::circle_torus`'s centre term
  (`(C − O) + (c − O) + R`, beside its harmonic rung), `implicit`'s
  `sample_rounding`, `profile::seg`'s `reach(q) + reach(centre)`, and
  `curve_pair::pose`'s `one_curve_gap`. `conic_conic`'s `noise` is filed
  as `work/onecopy/transverse-range-charges-the-global-height-with-world-origin-noise.md`.

## Slate triage (2026-11-08, ONEHOME split)

- Moved to ONEROUND (`work/oneround/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows on f64 rounding, levers and the consumed extent that PR 6380 and PR 6391 filed went to ONEROUND.

## The apex start's `gap_err` (2026-11-23, `oneround/readings-not-levers`)

`geom_brep::intersect`'s `apex_start` reads the plane's stand-off from
the cone's apex as `|δ̂| + e_δ` (the rounded gap's magnitude) over a
sine. Dropping `e_δ` passed every geom-brep test of the `ci` profile
when the unit first measured it. Two rows hold it now:
`pn_apex_tangent_generator::a_far_stated_grazing_plane_is_read_to_its_gaps_rounding`
(a needle cone, `α = 1e-4`, and its grazing plane stated 900 m from the
apex: the gap reads exactly zero and its rounding over `sin 2α` is
`3.5e-9 m`, in a band of `1e-9`) and
`pn_apex_pair_over_a_span::a_plane_through_the_apex_reads_its_reach_on_its_gaps_rounding`.
The term itself still goes with this row's re-posing of
`plane_cone_section`'s gap charge.

## A sweep for a tolerance inside the reading (2026-11-22, `oneround/readings-not-levers`)

Swept: every production `from_f64` of `band.zero()`, `band.escalate()`,
their ratio or `tol.eps()` (`scripts/gates/from-f64-sites.tsv`, 37
occurrences in 21 files), each followed to its use, for a band threshold
multiplied into (or used as the arm of) a reading decided in that same
band. Three have the shape, all precision-headroom gates this row
already holds: `profile::seg::resolution_headroom`,
`profile::sugar::offset_circles`' least lever, and
`topo::census::span_charge` (`pm_census_span_precision`, of the span
family above, not named there before). Four are near it and are
scheduled: `census::span_reach` and `pcurve_cache::incidence_at`'s
charge fraction (here), `pair_section::torus_arms::parallel_torus_arm`'s
neck (here, with `torus_arms`), and `curve_pair::pose::route`'s arm
`length + 2Kε`, a pinned `Lever::unsourced` of the sibling units. One
more has the shape and is this unit's own:
`topo::boolean::join::off_conic_slack`, `2ε·cot ψ`, the band's zero
scaled by a pure number and summed into the far row `travel` decides in
that band. It is no rounding charge but a stand-off of the sites, kept
as `decide_across`'s swing by the design (`docs/DESIGN-FORK-LOG.md` row
303) and removed by
`work/oneround/the-joins-sites-are-read-on-the-conic-so-the-travel-carries-no-swing.md`.
The rest are windows, brackets, pads and additive stand-offs. Not seen by
this pattern: a threshold that reaches a reading through a record field
or a helper's return (`ruling`'s `before`), and `f64`-typed arithmetic
that never passes `from_f64`.

---
id: from-computed-re-entries-no-open-row-owns
kind: issue
title: from_computed re-entries no open row owns: places held at T, constants spelled as computed, data whose row closed without it
status: open
opened: 2026-11-23
priority: P2
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found by `a-solved-value-the-body-stores-is-admitted-by-its-check`, whose gate (`scripts/gates/from-computed-sites.sh`) lists every `from_computed` caller by class. These re-entries are neither a stored solved value nor the place step, and no open row names them; they are listed `owed` to this row in `scripts/gates/from-computed-sites.tsv`, one line each with what it is. Sort each by DL5.2's table (`crates/geom-core/README.md`) and give it to the unit of its kind:

- **Places held at `T`** (a foot or window evaluated at to decide, never stored): `geom` `surfaces/projection.rs` and `curves/projection.rs` `project_from_seed` (Newton's iterate and the stagnation foot: the foot `project` returns), `topo::face_normal::spline_outward_normal_at` and `geom_brep::dihedral_side`'s foot normal (a foot `chart_point_normal` reads), `geom_brep::certify` `run_checks` (a row piece's ends), `pair_section` `witness` (a tube run's window), `curve_pair::polish::pair_held` (a Gauss–Newton iterate re-centred at an enclosure scalar; bounded `Decide`, so a `Dual<Interval>` takes a zero tangent there), `topo::link` `Link::lowest_on` and `Round::lowest_on` (a closed-form root at `T` read down to its point; reached by a dual, where the envelope argument in the comment is what makes a zero tangent right for the least).
- **Bounds as `T`**: `pair_section/ruling.rs` `form_ceiling` (a half cell width), `topo::link` `crossing` (`rd.band.zero()`).
- **Constants spelled as computed** (an exact `f64` at concrete `Interval`, or a threshold): `pair_section/ruling.rs` `coverage`, `pair_section/winds.rs` `lift`, `props/curved.rs` `sphere_region_cover`'s `c`. These are `from_f64`'s.
- **Data whose row closed without it** (`f64-built-data-is-built-at-t`, PR 7292): the twin cell walks' origin (`topo::spline_trim::bezier_cells`, `spline_incidence::Net::of`), `Patch::surface`'s knots, `topo`'s `test_support_fixtures::cyl_wall_sheet_tilted`, and the tour demos' lifts (`lily.rs` 14, `klein.rs` 2, `projectbox.rs` 2), which that row's close says build at `T`.

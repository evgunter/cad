---
id: constants-spelled-through-from-computed
kind: issue
title: Exact f64 constants at a concrete Interval, and a census threshold, spelled through from_computed
status: open
opened: 2026-11-23
priority: P3
cost: E
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found by `a-solved-value-the-body-stores-is-admitted-by-its-check` (PR 7651), whose gate `scripts/gates/from-computed-sites.sh` lists every `from_computed` caller; each site below is listed `owed` to this row in `scripts/gates/from-computed-sites.tsv` with its function, and its entry goes when the site does. The rule is `crates/geom-core/README.md` DL5.2.

A constant of the model enters by where it comes from (DL5.2's fourth row), through `from_f64`; these are exact `f64` values spelled through `from_computed`:

- `geom_brep::pair_section::ruling` `coverage` and `pair_section/winds.rs` `lift`: exact `f64` rulings and values lifted to the concrete `Interval`, where `from_computed` is the point embed `from_f64` is.
- `geom_brep::props::curved` `sphere_region_cover`'s `c`: a census threshold (a count and a half).
- `geom::azimuth` `Azimuth::about_middle`'s `theta`: a reference angle chosen at `f64` at the concrete `Interval` (the angle of the components' middles), about which the enclosed angle is read; any choice reads an enclosure of the same angle, so it is a choice, not a computed value (landed on main beside this unit).

Cost: E. Each becomes `from_f64` with a `from-f64-sites.tsv` row (`constant`), or a literal.

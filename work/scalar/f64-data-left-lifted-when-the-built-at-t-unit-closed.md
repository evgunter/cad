---
id: f64-data-left-lifted-when-the-built-at-t-unit-closed
kind: issue
title: f64 data still lifted through from_computed after f64-built-data-is-built-at-t closed: the cell walks' origin and knots, a test fixture, the tour demos
status: open
opened: 2026-11-23
priority: P2
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


Found by `a-solved-value-the-body-stores-is-admitted-by-its-check` (PR 7651), whose gate `scripts/gates/from-computed-sites.sh` lists every `from_computed` caller; each site below is listed `owed` to this row in `scripts/gates/from-computed-sites.tsv` with its function, and its entry goes when the site does. The rule is `crates/geom-core/README.md` DL5.2.

Data built whole at `f64` is built at `T` (DL5.2's third row). `f64-built-data-is-built-at-t` closed on PR 7292 having built the loft and the skin at `T`; its own note named the first two groups below and its close left them, and its close says the demos build at `T`, which they do not yet. So they are reopened here rather than on the closed row.

- The twin cell walks' translation origin: `topo::spline_trim::bezier_cells` and `geom_brep::pcurve_cache::spline_incidence::Net::of` (`centre`, 3 each).
- `topo::spline_trim` `Patch::surface`'s knots `(a, b)`: a cell's own parameter rectangle (2).
- `topo` `test_support_fixtures::cyl_wall_sheet_tilted`: test support compiled into the library, `f64` data lifted (7).
- The tour demos: `lily.rs` (14), `klein.rs` (2), `projectbox.rs` (2), `f64` frames and points a demo lifts into its scalar.

Cost: M. The demos are a consumer's spelling: build them at `T` through the public doors.

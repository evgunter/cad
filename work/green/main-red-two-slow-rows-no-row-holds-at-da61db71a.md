---
id: main-red-two-slow-rows-no-row-holds-at-da61db71a
kind: issue
title: Main red: predicate_name_coverage at every eps and the cones-apex rows at 1e-6, red on da61db71a with no row holding them
status: open
opened: 2026-11-23
priority: P0
cost: M
---


Found by DOCTAIL's `main-red-the-identity-turns-decisions-moved-the-m10-slow-rows` lane. It ran the whole slow set on `origin/main` `da61db71a` at the default ε, 1e-6 and 1e-12, and re-ran every red on `fa42c60`, PR 7434's first parent. Both rows below are red there too, so neither is 7434's. No open row names either.

- **`test-utils` `predicate_name_coverage::every_deciding_crate_runs_a_roster_and_every_referenced_name_is_decided`** (`predicate_name_coverage.rs`, the assertion near :217). Red at the default ε and at 1e-6. Three literals are "a predicate literal no crate decides":
  - `ct_critical_gap_straddle` in `crates/geom-brep/src/pair_section/gap.rs` (about :712);
  - `pc_parallel_gap` in `crates/geom-brep/src/locus.rs` (about :1156);
  - `pc_parallel_gap` in `crates/sweep/src/offer_rows.rs` (about :357).

  The closed `main-red-predicate-name-coverage-step-import-decides-names-and-runs-no-roster` had another cause. geom-brep's `recourse_roster` also names `PC_PARALLEL_GAP`, as "a site names the const, not declared a carrier" (`main-red-geom-breps-recourse-roster-misses-three-ssi-enclose-cone-sites`). So one change may have moved both.
- **`sweep` `a_plane_through_a_cones_apex::a_plane_through_a_{30,45}_degree_cones_apex_cuts_its_closed_forms`** (`a_plane_through_a_cones_apex.rs`, the assertion near :218). Red at 1e-6 only. At α = 30° and 45°, θ = 0, tilt 0.001, normal 1·n, the plane refuses `Reduce(CrossingEscalated { edge: EdgeKey(6v1), fault: Method { door: PlaneCrossing, fault: Unsettled(Tangency) } })`, where the row pins `ConsecutiveOnSectors`. The default ε is green.

Neither was bisected.

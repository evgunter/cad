---
id: geom-core-runs-no-predicate-name-roster
kind: issue
title: geom-core decides predicate names and runs no roster: 54 names read, 18 indirect sites and a const site unlisted
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-10
pr: PRNUM
---


Found by ONECOPY's roster-reach unit. `crates/test-utils/tests/predicate_name_coverage.rs` lists `geom-core` in `KNOWN_MISSING` with this row.

geom-core decides predicate names in its `src` (`predicate_census` reads 54), and `ladder.rs`, the identity ladder, now lives here. No `tests/recourse_roster.rs` holds that nothing it decides is invisible. The census reports a const site (`linalg/unit_vec.rs: UNIT_DIRECTION_ARM`) and 18 indirect sites:

- `k_stats.rs: far`
- `k_stats.rs: floor`
- `k_stats.rs: name`
- `k_stats.rs: names`
- `ladder.rs: name`
- `ladder.rs: read_identity#1(data.iter().map(|&(n, m)| (n, Margin::of(m))))`
- `ladder.rs: read_identity#2(data.iter().map(|&(n, m)| (n, Margin::of(Probe(m)))))`
- `linalg/rigid_motion.rs: check`
- `linalg/rigid_motion.rs: site`
- `linalg/unit_vec.rs: good`
- `linalg/unit_vec.rs: site`
- `linalg/unit_vec.rs: v`
- `linalg/unit_vec.rs: v * scale`
- `line2.rs: det`
- `period.rs: name`
- `period.rs: rows.turn`
- `running.rs: name`
- `sym.rs: band`
- `linalg/unit_vec.rs: UNIT_DIRECTION_ARM`
- `spline/compose.rs:1223`
- `sym.rs:1037`
- `sym.rs:1041`
- `sym.rs:1045`
- `sym.rs:1050`
- `sym.rs:1054`
- `sym.rs:1060`
- `sym.rs:1065`
- `sym.rs:1069`
- `sym.rs:1078`
- `sym.rs:1084`
- `sym.rs:1088`

Two references sited here do not resolve, and the entry holds them:
- `lane_straddle` (`k_stats.rs`, a unit test of `straddle`) is decided through `k_stats::straddle`.
- `t_straddle` (`period.rs`) is decided through `TurnRows::bounded`.

Neither is a token the census reads here. A geom-core roster declares `straddle` and `bounded` as carriers, as geom-brep's does. The ladder's own fixture rows (`ladder.rs`, `b`) are marked `UNDECIDED-NAME`, pending `the-census-opens-no-nested-tuple-of-names`.

**Done** when `crates/geom-core/tests/recourse_roster.rs` runs `test_utils::source::roster_violations`, and geom-core's entry is gone from `KNOWN_MISSING`.

**Closed** (2026-11-10, ONECOPY roster-rest). `crates/geom-core/tests/recourse_roster.rs` runs `roster_violations` over geom-core's whole `src`, in the slow set.
- **Declared carriers:**
  - `UNIT_DIRECTION_ARM`;
  - the two direction doors whose name is not their first argument (`decide_unit_direction`, `decide_direction_by`);
  - `straddle` and `bounded`, which carry `lane_straddle` and `t_straddle` into the union.
- **Two test helpers renamed** off the `bounded` token: period's `turn_of` and vec's `stays_bounded`.
- **One reader change:** `compose.rs` mounts the exact-rational oracle from `tests/` with a `#[path]` from outside `src`. The reader now walks past such a mount while the mounted file spells no `decide`, and reds it once it does.
- geom-core's entry is gone from `KNOWN_MISSING`.

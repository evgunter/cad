---
id: editor-core-runs-no-predicate-name-roster
kind: issue
title: editor-core decides predicate names and runs no roster: 4 names read, 12 indirect sites and 12 const sites unlisted
status: closed
opened: 2026-11-10
priority: P3
cost: M
closed: 2026-11-10
pr: 6736
---


Found by ONECOPY's roster-reach unit. `crates/test-utils/tests/predicate_name_coverage.rs` lists `editor-core` in `KNOWN_MISSING` with this row.

editor-core decides predicate names in its `src`, and no `tests/recourse_roster.rs` holds that nothing it decides is invisible. `predicate_census` reads 4 names and reports 12 const sites, none declared a carrier. Among them are `coincide/replay.rs`'s `ONE_CYLINDER` and `ONE_CYLINDER_AXIS`. It also reports 12 indirect sites:

- `checks/at_rest.rs: name`
- `checks/at_rest.rs: site`
- `coincide/replay.rs: name`
- `eval/measure.rs: predicate`
- `eval/wire.rs: decision.name()`
- `eval/wire.rs: topo::clearance::CLEARANCE_MARGIN`
- `eval/wire.rs: value`
- `measure.rs: measured`
- `measure.rs: ten`
- `names/emit_topo.rs: predicate`
- `trace.rs: Margin::levered(`
- `trace.rs: Margin::of((origin - at).dot(z))`
- `checks/at_rest.rs: AT_REST_QUIET_BOUND`
- `checks/at_rest.rs: AT_REST_QUIET_CONTACT_BOUND`
- `eval/measure.rs: GAP_AXIS_PARALLEL_TO_PLANE`
- `eval/measure.rs: GAP_ROUND_LARGER`
- `eval/measure.rs: GAP_ROUND_SIDE`
- `mate/value.rs: super::MATE_READ_SLIDE_WITHIN`
- `mate/value.rs: super::MATE_SLIDES_INDEPENDENT`
- `measure.rs: ASSERT_BOUND`
- `names/emit_union.rs: ON_MEMBER_EDGE`
- `pose.rs: SEL_PLANE_DISTANCE`
- `trace.rs: TRACED_FRAME_SQUARE`
- `trace.rs: TRACE_REACHES`

**Done** when `crates/editor-core/tests/recourse_roster.rs` runs `test_utils::source::roster_violations`, declaring its const carriers and listing its indirect sites, and editor-core's entry is gone from `KNOWN_MISSING`.

**Closed** (2026-11-10, ONECOPY roster-rest). `crates/editor-core/tests/recourse_roster.rs` runs `roster_violations` over editor-core's whole `src`, in the slow set.
- **Eighteen const carriers declared** (`EVAL_DIRECTION_NORM` with them):
  - the twelve the census reported;
  - five it found once the forwarders were declared. These are the replay's `ONE_CYLINDER` and `ONE_CYLINDER_AXIS`, at_rest's two quiet rows, and measure's `AXES_PARALLEL`.
- **Three forwarders take `_as`:** measure's `parallel_as`, and the `zero_as` closures in at_rest and the replay. Their callers' names are now read.
- `CLEARANCE_MARGIN` is topo's const, read by topo's roster.
- editor-core's entry is gone from `KNOWN_MISSING`.

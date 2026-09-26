---
id: lifted-fixture-values-reached-and-asserted-by-no-row
kind: issue
title: Seven f64-to-lane point lifts in test fixtures run on every gate and no row reads the lifted value: swapped or shifted coordinates leave their suites green
status: open
opened: 2026-09-25
priority: P3
cost: E
---


## Finding

Filed by S-DUP's `dup/scalar-lift-home` lane (PR 3242), which folded
every componentwise `f64` -> lane lift of a point or vector onto the
leaf door `map` and proved the fold with plants in that door. At seven
of the folded files the lifted value is **reached and asserted by
nothing**, which is this program's charter rather than S-DUP's.

The instruments, all on the lane's fold head, each plant restored by
reverse patch and the tree diffed clean against `HEAD` after:

- **Reach**: `#[track_caller]` on the four leaf `map`s, logging each
  test-side caller's `file:line` — every site below is in the log.
- **Answer, plant 1**: `x` and `y` swapped in all four leaf `map`s.
- **Answer, plant 2** (the divergent control, because a swap cannot
  red a fixture symmetric in `x` and `y`): `x` shifted by one.
- Both plants were also run on the pre-fold tree (fold reverse-applied)
  so a red could be attributed to the fold rather than to another
  `map` caller in the same suite; at these sites neither plant reds a
  row in either tree.

| site (fold head) | row(s) that run it | what they assert instead |
| --- | --- | --- |
| `crates/geom-brep/tests/interior_iso_column.rs` `lift_surface`, `lift_curve` | the `a1`/`a2`/`a2b` rows at every lane | that an interior column certifies, or refuses typed: an outcome the chart's placement does not decide |
| `crates/geom/tests/curves/curve3_r1_probes.rs` `lift3` | the `same3` bit-identity rows | two evaluation routes over the SAME lifted curve agree, so a wrong lift moves both sides |
| `crates/geom/tests/curves/ders1_r2_probes.rs` `lift3` | the `ders1` rows | likewise, route against route on one lifted curve |
| `crates/profile/tests/review_fillet_stored_tangency_r1_probes.rs` `pt` | `report_the_interval_loops_with_the_door_read_suppressed` | nothing: a report row that prints what validation says and asserts none of it |
| `crates/sweep/tests/extrude_acceptance.rs` `dual_lane_value_channel_matches_f64_bitwise` | that row | curve certificates' `max_residual`, which is zero for the L-profile's straight chords wherever the vertices sit |
| `crates/sweep/tests/must_carry_rule.rs` `probe_surface`, `probe_carrier` (`probe` feature) | the probe row that meters the out-of-lane triple | how many K-stream samples the rule spends, which the triple's placement does not change |
| `crates/editor-core/tests/m10_p_lift.rs` `a_wide_interval_binding_aborts_typed_rather_than_certifying` (`plane.map(Interval::from_f64)`) | that row | that a wide binding aborts typed; the plane is a constant input. Reach proved separately: a `panic!` in `SketchPlane::map` reds the row on the fold head and not on the pre-fold tree |

Also dark, a different shape: `crates/geom/tests/curves/n1r2_dump.rs`
returns before its lift unless `N1R2_DUMP` is set, so on the gate the
lift never runs (no reach-log entry in a run that selected it). It is a
dump tool rather than a row; whether it belongs in the suite is this
program's call.

Not every one of these needs a guard: where the lifted value is a
constant input and the row's claim is about something else, "nothing
reads it" is the right answer, and the disposition is to say so at the
row. The swap and shift plants say only that no row notices; they do
not say any row should.

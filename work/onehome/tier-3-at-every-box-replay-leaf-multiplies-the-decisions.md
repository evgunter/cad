---
id: tier-3-at-every-box-replay-leaf-multiplies-the-decisions
kind: issue
title: Tier 3 at every sweep leaf in a box-scalar replay multiplies the decisions about 4.4× and makes docm9 rows take minutes
status: open
opened: 2026-11-02
priority: P0
cost: H
refs: [6371]
---

Split from `the-sweep-doors-finish-refuses-what-the-box-studies-read`
(PR 6371). This is one of the rows that retire the box-scalar switch at
`topo::AtRestPolicy::leaf` (`crates/topo/src/props.rs`, the switch).

**What it costs.** With the switch off, every sweep door finishes its
result at the box scalars, so tier 3 runs on every leaf in every replay.
Measured on PR 6371 at CI's opt-1:

- the m10_8 slab drive went from plain 2898/1612/0 to 12642/8573/36
  (symbolic/numeric/frozen), about 4.4× the decisions per replay
  (`editor-core/tests/m10_8_pins_interval.rs`);
- `docm9_range::a_certified_side_stops_at_the_seeds_edge` took 189–299 s,
  `a_parameter_field_boxes_directly` 186 s and
  `the_query_does_not_touch_the_input_document` 157 s. Each ran under the
  1.0 s budget on main;
- `m10_3_driver_interval::driving_writes_nothing` took 31–47 s;
- the sweep rows `pinned_corners_at_the_loft_door::an_elbow_builds_from_every_spelling`
  took 37 s, `revolve_washer::m10_9_…` 33 s and
  `one_segment_loop::a_one_segment_strut_…` 27 s.

**What the switch leaves in the slow set.** Two rows still pay a
finish the switch does not take away, so they join the slow set on
PR 6371 (CI, opt-1):

- `editor-core` `trim_3_windows_interval::a_revolved_quarter_band_is_windowed_and_holds`
  took 3.48 s, against under 1 s on main. The likely cost, unmeasured:
  its clearance query reads a placed copy through `Framed::read_in`,
  which at `f64` ends in the placement's finish.
- `mesh` `probe_review::z1_fixtures_still_tessellate_with_the_falsifier_gated_out`
  took 1.05 s. It lofts a prism, which the `f64` door finishes.

**The test that says it is closed.** With the switch's box arm deleted,
those rows run within the 1.0 s slow-test budget (or each is in the slow
set for its own reason), and the decision pins move by less than the
battery's own new decisions justify.

**A direction.** Tier 3's battery could run once per witness and be
re-verified per leaf only where a leaf's verdict vector can differ, or
tier 3 could be paid by the consumer only, which is what the switch
does now.

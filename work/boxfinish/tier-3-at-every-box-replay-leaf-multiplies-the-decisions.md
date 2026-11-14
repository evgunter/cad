---
id: tier-3-at-every-box-replay-leaf-multiplies-the-decisions
kind: issue
title: Tier 3 at every sweep leaf in a box-scalar replay multiplies the decisions about 4.4× and makes docm9 rows take minutes
status: parked
opened: 2026-11-02
priority: P0
cost: H
refs: [6371]
blocked_on: [the-side-reader-cannot-read-a-solids-orientation-over-a-box, the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box]
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
- 19 `sweep` loft and sweep rows, at `f64`, which the loft door
  finishes: `pinned_corners_at_the_loft_door::an_elbow_builds_from_every_spelling`
  38 s, `one_segment_loop::a_one_segment_strut_…` 27 s,
  `…::a_tight_bend_tube_sweeps_from_any_start` 10.6 s,
  `bool6_per_slab_stacking::a_planar_arc_spine_lofts_past_pi` 8.0 s, and
  15 more between 1.0 and 5.0 s (the line in `.config/nextest.toml` lists
  them). These are the `f64` finish the ruling keeps, so they are not the
  switch's to take away. They belong to this row's question of what
  tier 3 costs at a door.

**The test that says it is closed.** With the switch's box arm deleted,
those rows run within the 1.0 s slow-test budget (or each is in the slow
set for its own reason), and the decision pins move by less than the
battery's own new decisions justify.

**A direction.** Tier 3's battery could run once per witness and be
re-verified per leaf only where a leaf's verdict vector can differ, or
tier 3 could be paid by the consumer only, which is what the switch
does now.

## Slate triage (2026-11-08, ONEHOME split)

- Moved to BOXFINISH (`work/boxfinish/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows that let the box scalars finish their sweep leaves at tier 3 and so retire Ev's switch (PR 6371) went to BOXFINISH.

## Measured again (2026-11-14, main `b363f5fa`, switch off): mostly forced bisection

BOXFINISH's M1 lane (`boxfinish/probe-switch-off`, `probe/M1-REPORT.md`).
The numbers above are not today's: docm9's rows take 10 s, not 157 to
299 s, and pass their main-era assertions; main's m10_8 pins are
2835/2803/0.

| row | switch on | off, as built | off, normals and ray substituted by hand |
|---|---|---|---|
| m10_8 plain decisions | 2835/2803/0 | 10851/12675/42 | 4227/4703 (1.49× / 1.68×) |
| docm9 `a_certified_side_stops_at_the_seeds_edge` | 0.137 s | 9.96 s | 0.35 s |
| docm9 `a_parameter_field_boxes_directly` | 0.132 s | 9.92 s | 0.36 s |
| docm9 `the_query_does_not_touch_the_input_document` | 0.171 s | 10.22 s | 0.39 s |
| m10_3 `driving_writes_nothing` | 0.26 s | 35.5 s | 0.69 s |

- **The multiple is the refusals' bisection.** On m10_8 plain, 80 to 82%
  of the extra decisions are 8 extra replays that refuse at the finish;
  the finish on the 4 certifying quarters is the other 18 to 20%. One
  finish runs per finishing leaf, none repeated.
- **A finish whose rays read** costs 50 to 110 ms on a six-face slab at
  `Sym<Interval>` (2 ms at `f64`); one that falls to the volume, 254 to
  787 ms (`check-7s-volume-reading-is-loose-over-a-box`).
- **Most of a finish repeats the door's own build.** Of 836 decisions a
  finish makes, about 732 carry the names and counts the door's build
  decided on the same body in the same leaf (`dihedral_arm` 168,
  `dihedral_wedge` 168, `dihedral_sign` 84, `carrier_on_surface_1` and
  `_2` 108 each, and smaller rows). Equal margins are inferred.
- **A drive finishes one body many times.** In the tour, 3069 box-lane
  finishes cover 255 distinct vertex fingerprints: the plate is one
  body finished 1023 times, and 92% of the finishes repeat a body
  already finished in the same drive. Finishes are 38% of its leaf
  time. With a move, a leaf runs three finishes (the door, the moved
  leaf's re-finish, the union's result).

**So this row waits.** With the two substitutions every row above is
inside the 1.0 s budget, which is this row's own closing test. It is
parked on the two rows that remove the forced bisection, and is
measured again on the tree that has them. What is then left (the
repeated body, the build's decisions made twice) is filed from that
measurement, not from this one.

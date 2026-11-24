---
id: tier-3-at-every-box-replay-leaf-multiplies-the-decisions
kind: issue
title: Tier 3 at every sweep leaf in a box-scalar replay multiplies the decisions about 4.4× and makes docm9 rows take minutes
status: parked
opened: 2026-11-02
priority: P1
cost: H
refs: [6371]
blocked_on: [a-drive-keeps-no-node-result-from-one-leaf-to-the-next, a-holes-finish-climbs-the-ladder-on-eight-pose-disc-decisions-that-end-indeterminate]
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

## Un-parked (2026-11-15): the frames landed (PR 6972); being measured again

Both repairs M1 substituted by hand are built: the aimed ray (PR 6958)
and the frames (PR 6972). The measurement lane M4 re-takes this row's
table on three trees (the frames' head with the switch on; the probe
with main; the probe with the frames) and counts what still refuses at
a finish, with the directions the doors still derive from a segment's
ends counted apart (`the-sweep-doors-read-the-profiles-recorded-directions`,
in flight). This row is re-stated from that measurement.

## Measured again (2026-11-15, M4): the bisection is nearly gone; what is left is the finish itself

M4 (`boxfinish/probe-switch-off`, `probe/M4-REPORT.md`), three trees,
hand substitutions unset: the frames' head with the switch on
(`9ee3c1ea`), the probe with main, the probe with the frames.

| row | switch on | off, no frames | off, frames |
|---|---|---|---|
| m10_8 plain decisions | 460/504/0 | 10974/12684/100 | 2424/3151/192 |
| docm9 `a_certified_side_stops_at_the_seeds_edge` | 0.048 s | 3.55 s | 0.48 s |
| docm9 `a_parameter_field_boxes_directly` | 0.054 s | 3.39 s | 0.54 s |
| docm9 `the_query_does_not_touch_the_input_document` | 0.046 s | 3.63 s | 0.72 s |
| m10_3 `driving_writes_nothing` | 0.196 s | 5.41 s | 0.44 s |
| m10_8 `a0_set_is_a0_alone` | 0.87 s | 6.76 s | 7.46 s |

- **The frames remove all but one bisection level** on the ± 0.25
  slab: 15 replays, every one refusing at the finish, become 3. The
  level that remains is the rays on a slab boxed wider than ± 0.215 of
  its depth, with the loose volume behind them
  (`an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice`,
  `check-7s-volume-reading-is-loose-over-a-box`), which is why m10_8's
  pin is 5.3× / 6.3× and not the 1.49× / 1.68× the hand substitution
  read.
- **What a finish costs when nothing refuses** is this row's own
  content now:
  - a planar slab leaf: 55 to 100 ms and 1.77× / 1.93× the decisions;
    732 of the finish's 823 decisions (89%) carry a name and count the
    door's build decided in the same leaf, and it freezes 96 forms
    (the largest form 17 terms against a pinned 4);
  - a cylinder-walled finish at the a0 set: 0.85 to 1.07 s each, four
    finishes per leaf (the a0 row 0.87 → 7.46 s), the same without the
    frames;
  - a revolved circle's finish: 2.9 s;
  - the tour: one plate finished 1023 times (92% of finishes repeat a
    body); finishes are 35% of leaf time;
  - seven fast rows cross 1.0 s with the switch off (the revolved
    circle's row 3.40 s; six `editor-core` rows between 1.01 and
    1.37 s).
- **Corrections:** `revolve_washer::m10_9_…` is not a row on any tree;
  the elbow and strut rows are `f64` and cost the same either way;
  `frozen != 0` on the derived-frame row is there as built (192), not
  an artefact of M1's hand constants; a plate with holes runs four
  finishes per leaf, one per sweep door.

**Re-banded P1** (was P0 from PR 6371's minutes-long rows, which are
gone): on main, with the switch on, nothing here is slow; this row is
what the switch's deletion (`delete-the-box-scalar-sweep-leaf-switch`,
P1) waits on.

**The test that says it is closed, re-stated.** With the box arm off:
the seven rows above and the a0 row run within the 1.0 s budget or are
in the slow set each for a stated reason; a finish does not repeat a
decision the door's build took on the same body in the same leaf, or
says why it must; a body a drive does not change is not finished again
per leaf.

**Next:** M5 decomposes a curved finish's time by check, says whether
the repeated decisions are the same decisions, and whether the plate's
1023 finishes are of one value. The fix is weighed from that.

## Measured (2026-11-15, M5): where a finish spends its time when nothing refuses

M5 (`boxfinish/probe-switch-off` at `2f37154e`, `BOXFINISH_TRACE=3`
and `4`).

- **A curved finish is slow in one place.** On a cylinder-walled body
  82% of 842 ms is eight side-reader decisions
  (`side_at_infinity_winding`, `side_at_infinity_denom`), each decided
  numerically, each then walking the symbolic tier's whole ladder for
  the check that the two channels do not contradict
  (`sym::contradicts`, a debug assertion that ships on). The same on
  the revolved circle, across the torus readings. That is the tier's,
  and a design question:
  `work/sym/the-channel-contradiction-check-walks-the-tiers-ladder-on-every-definite-decision.md`.
- **The decisions a finish repeats from the door's build are the same
  decisions**: 732 of a slab finish's 824 carry the same symbolic node,
  enclosure and band as one the build took in that leaf (the door asks
  when it mints each edge through the scalar's edge door; the finish
  asks again in check 2, which "never trusts the stored certificate",
  and check 4's wedge fold). They cost 8.9 ms of 57: 89% of the
  decisions, 15% of a slab finish's time, 1% of a curved one's.
- **The 96 forms a slab finish freezes** are all in
  `side_at_infinity_winding`, refused by the ring's guards, not the
  budget.
- **The tour's plate is one value finished 1023 times**: the same
  boxes and the same symbolic nodes in every replay, while the leaf's
  box varies only in variables it does not read. Each hole value is
  finished about eight times.

**So this row is now three things, in the order they cost:**
1. the contradiction check on a curved body's ray decisions (SYM's
   row above; this row is parked on it);
2. a body a drive's leaf does not change is finished again per leaf
   (the tour: 1023 finishes of one plate, 35% of leaf time): a design
   question on what a leaf's finish may be keyed by, not yet weighed;
3. the repeated edge-door decisions (15% of a slab finish): small, and
   by design (check 2 re-derives).

The ray-readings unit (`an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice`)
re-poses the two readings that are slowest here; what its change does
to their cost is measured in its PR.

## Measured (2026-11-22, M7): with the check gone, what is left of a finish

M7 (`boxfinish/probe-switch-off` at `ca8ec03c8c`, `probe/M7-REPORT.md`;
main `ae076e0507` merged, the switch's box arm off) skipped by hand what
SYM's row now removes (the two walks after a definite numeric sign) and
re-took this row's tables on one binary, the skip on and off.

- **No verdict changed** over 13,636 rows; only the skipped walk's own
  counters and `frozen` moved.
- **A finish that refuses nothing:** a six-face slab 52 → 33 ms, a
  cylinder-walled body 796 → 71 ms, a revolved circle 2.66 → 1.0 s.
  What is left on the first two is building margins, not deciding.
- **M4's table:** every row is inside 1.0 s but two.

  | row | check on | check off |
  |---|---|---|
  | docm9 (three rows) | 0.42 to 0.52 s | 0.22 to 0.30 s |
  | m10_3 `driving_writes_nothing` | 0.46 s | 0.26 s |
  | m10_5 clearance (two rows), m10_5_r2 combs | 1.02 to 1.20 s | 0.34 to 0.40 s |
  | m10_8 `the_a0_set_is_a0_alone` | 5.34 s | **1.61 s** |
  | sweep `a_revolved_circles_latitude_over_a_centre_box_is_exact` | 3.40 s | **1.33 s** |

- **The revolved circle's remainder is the decision path's own.** Its
  slow decisions (`curve_pair_span_end_chord` 454 ms over 100 decisions,
  `bool_torus_region_component` 310 ms, `bool_torus_region_corner`
  116 ms) are numerically indeterminate, the tier is asked, and it does
  not prove; the time goes in building forms of one to two thousand
  terms that are kept, not in products then refused. That is SYM's
  (`work/sym/an-indeterminate-torus-margin-builds-forms-of-a-thousand-terms-and-proves-nothing.md`).
- **The tour:** 95 → 56 s wall. The finishes' share of leaf time does
  not move (34.7% → 33.6%): the plate no leaf changes is still finished
  1023 times, 22 s of a 215 s drive, and the holes' 2046 refusing
  finishes are another 50 s.
- **The enclosure check SYM's unit adds would not have fired:** 9.97
  million symbolic Zeros at `Sym<Interval>`, none with an enclosure
  excluding 0.

**So, of this row's three things:** the first is in flight (SYM's row,
`boxfinish/decide-path-check`) and removes most of the cost; the third
(the repeated edge-door decisions) stays small and by design; the
second is now the largest thing left and has its own row,
`a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it`,
a design question being weighed. The a0 row's 1.6 s is four finishes
per leaf on a plate whose blank no leaf changes; the revolved circle's
1.3 s is SYM's row above. This row stays parked until SYM's unit lands
and its tables are re-taken on main.

**Un-parked (2026-11-24).** PR 7394 is on main: a definite decision no
longer walks the tier's ladder. What this row waited for can now be
measured: the plate's and the tour's leaf cost on main with the
switch's box arm off, against M4's and M9's figures.

## Measured (2026-11-24, M10): on main after PR 7394, the holes build in every leaf and the tour is seven times dearer

`probe/M10-REPORT.md` on `boxfinish/probe-switch-off` at `7901e1b078`
(probe merged with main `82fd6a46df`; a switch-on control of the same
commit beside it). What contradicts this row's earlier sections:

- **The tour no longer refuses at the holes.** With PR 7384 the `u_ref`
  refusal is gone from every study: the box-arm-off tour certifies 211
  leaves, 94.9069%, as the switch-on tour did at M4. But each hole
  extrude now finishes in every leaf (520 ms average, where it refused
  in 40 ms), so the drive's summed leaf time is 1,304.9 s against M9's
  177.6 s, and the first drive's answer takes 328 s of wall time. The
  finish's time is on eight `curve_face_pose_disc` decisions that each
  end indeterminate after a ladder climb:
  `a-holes-finish-climbs-the-ladder-on-eight-pose-disc-decisions-that-end-indeterminate`.
- **PR 7458 removed the split waste:** 0 of 511 splits are on an axis
  the deciding node does not read (M8: 292).
- **What a store would take:** 967 s of the 1,304.9 s (74%) is on node
  keys already seen in the drive (finishes 858.7 s, build 108.4 s). The
  plate alone, one key in 1,023 replays, is 24.3 s (1.9%).
- **The repeated edge-door decisions** are small: 5.0 ms of a slab
  finish's 25 ms; 3.7 ms of a tour hole finish's 1.1 s.
- **The rows of this row's tables, box arm off against switch on:** the
  slab passes (1 leaf, 33.8 ms); the a0 plate is 1.72 s against
  1.12 s; the two-parameter and band-parameter plates FAIL at 27 s
  (switch on: 3.1 s and 4.9 s, pass), refusing in all 31 replays at
  check 7's volume reading on `props_meridian_on_surface`, which is
  radial-readers' row (PR 7370, not on main at this measurement);
  `the_certification_width_is_no_longer_bounded_by_epsilon` still
  settles at 0.2152 (the rays: PR 7480, not on main at this
  measurement); the revolved circle is 1.0 to 2.3 s against 0.16 s.
- **`docm9`'s rows this row names are gone or renamed;** the search rows
  that replaced them take 1.9 to 7.3 s with the box arm off and 0.2 to
  2.2 s with the switch on, with no refusing finish: the multiple is
  finish cost per replay.
- **New with the box arm off:** one row runs out of memory
  (`a-boxed-rectangles-certification-runs-out-of-memory-with-the-box-arm-off`),
  and `RingContactEscalated` on `ring_outer_locus_gap` appears in two
  finishes of the box studies (not filed on its own: two finishes, no
  row red on it).

**So this row is parked again,** on the two rows that hold its cost:
the per-drive store (three quarters of the tour's time) and the hole
finish's ladder climbs (what one finish costs). Its closing test
stands: the box-lane rows inside their time bars with the switch off,
taken again when both have landed, with PR 7370 and PR 7480 on main.

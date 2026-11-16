---
id: pins-a-green-lane-leaves-are-hand-kept-lists-keyed-by-name-or-float-tuple
kind: issue
title: The pins a greening lane leaves are hand-kept lists keyed by a name string or a float tuple, and a stale entry in one is silent
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Filed by GREEN's default-eps lane (`green/default-eps`, PR 6931,
2026-11-14), from its STYLE review (S3). It is about how every pin lane
works, with that PR's three pins as the instances.

## What

GREEN's rule is to pin a red at what main produces, so that the suite is
green and the pin turns red the day the behaviour changes. Each pin is a
small list beside the test, kept by hand:

| pin | file | keyed by | a stale entry |
|---|---|---|---|
| `QUOTED_AS_A_LENGTH` | `crates/topo/src/boolean/offer_rows.rs` | a case's name, as a string | **silent**: a name that matches no case (the case renamed or dropped) excepts nothing and fails nothing |
| `MOVED_THIN_READS` | `crates/topo/tests/shell_role_reader.rs` | an op's label string, under a (pose, width, tilt, ε) condition | **silent**: a label no run carries, or a condition no row meets, is never looked up |
| `floor_stops` | `crates/sweep/tests/an_aimed_ray_from_far_reads_a_split_side_right.rs` | a tuple of an azimuth index and three floats | loud: the census compares the whole list with `assert_eq!` |

The two silent ones share a shape: the list is consulted by `contains`
or `find` from inside the check, so the check never learns whether every
entry was used. A pin that excepts nothing is indistinguishable from one
that excepts what it should, and the row it names stays open against a
test that no longer needs it.

## Wanted

A pin states its entries and the check fails on an entry it did not
consume. Either each lane does that by hand (the census's `assert_eq!`
over the whole list is the pattern), or `test_utils` grows one small
helper a pin is declared through: the entries, the row each waits on,
and a drop-time or end-of-test assertion that every entry was matched at
least once. The second also gives `scripts/` one spelling to find pins
by, for the check that a pin's row is still open
(`work/green/a-pin-or-disabled-red-can-cite-a-closed-or-missing-row.md`).

## Reach

Not swept beyond the three above: the other lanes' pins from nightly
43371638371 and the earlier `nightly-*-reds-greened` units were not
read for the same shape.

## More instances (GREEN `green/eps-1e-12`, PR 6967, 2026-11-14)

**Pins that carry digits or slot keys red the nightly on moves far below
ε.** Two of the 1e-12 slate's reds were pins whose 16-digit `achieved`
moved by 1e-5 relative, from a tightened floor, with the stop unchanged:
`encl_curved_loft_shell::a_moved_curved_fitted_cap_builds_where_its_fit_certifies`
and `shell_general_door::a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold`.
Both now hold the stop by text and grid. That PR's own pins hold theirs
by refusal kind and predicate, with no `FaceKey` and no digits.

A sweep of `pinned_red::row` calls whose pieces carry a slot key
(`…Key(NvM)`) or a float of six or more decimals turned up six more, left
as they are:

- `crates/geom-brep/src/pair_section.rs` `close_rings_carry_counted`
  (a float in a `Classified` verdict);
- `crates/editor-core/tests/coincidence_door.rs`
  `the_axis_edit_mates_the_copy_no_mate_reaches_in_either_cell_order`
  (`LoopKey(1v3)`);
- `crates/sweep/tests/review_4781_r1_probes.rs`
  `review_4781_touch_pairs_every_op` (`extent: 670.8203932499385`);
- `crates/sweep/tests/review_4781_r2_probes.rs`
  `r2_every_same_side_touch_pose_answers_as_a_touch` (the same extent);
- `crates/sweep/tests/cone_operand_rows.rs`
  `an_oblique_rod_and_a_tilted_cone_inside_or_clear_build_their_closed_forms`
  (a 17-digit volume);
- `crates/sweep/tests/r1_lane0_e2e.rs`
  `the_f64_seam_answers_every_public_door` (`EdgeKey(10v1…`).

The sweep cannot see `kernel_limit::pinned*` pins or pins whose text is
built by `format!`.

**Row → pin is unchecked.** `scripts/gates/pin-cites-an-open-row.sh`
checks that every pin cites an open row; nothing checks that a row's own
list of the pins citing it is complete. PR 6967's NEARPOINT and BOXFINISH
rows each keep such a list by hand, and its first draft had one wrong (a
pin cited for a stop it was not on).

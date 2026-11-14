---
id: a-needle-channels-f64-shell-volume-rounds-by-up-to-8-6-percent-at-eps-1e-9-since-pr-6505
kind: issue
title: A needle channel's f64 shell volume rounds by up to 8.6% of w² at ε 1e-9 since PR 6505 (2.4% before), and four runs are pinned past the 5% their row holds
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Filed by GREEN's default-eps lane (`green/default-eps`, PR 6931,
2026-11-14). This is the fix row for `MOVED_THIN_READS` in
`crates/topo/tests/shell_role_reader.rs`.

## What

`shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void`
(slow set) builds a channel `w × w` across, a metre long, by five
booleans in three poses, and reads each built shell's volume
(`classify_shells`' enclosure) against `w²`, to 5% at ε 1e-9. On main
four runs of the `Pose::Moved` channel 10.5ε across read 8.0% to 8.6%
short, and the row was red on nightly 43371638371.

Nothing built is wrong: every run's corners are the channel's to
`w/1000`, and (the PR's review) the public `mass_properties` returns
the same value with `volume_pad: 0.0` and `Stop::Met`. What moved is
the rounding of an `f64` sum that states no pad. `w²` is 1.1e-16 m³ at
10.5ε; the largest error is 9.5e-18 m³.

## The distribution (ε 1e-9, percent of `w²`, measured by a print in `channel_rows`)

Every unpinned row at 1% or more, before PR 6505 (`e996c0049`), at its
merge (`9e1484043`) and on the lane's head (identical to the merge in
every one of the 80 rows):

| pose, width | run | before | since |
|---|---|---|---|
| Moved 10.5ε | `brick ∖ slab ∪ fill` | +1.50 | **−8.01** (pinned) |
| Moved 10.5ε | `slab ∖ fill`, `slab ∩ (brick ∖ fill)` | +1.50 | **−8.57** (pinned) |
| Moved 10.5ε | `(brick ∖ fill) ∩ slab` | +0.46 | **−8.56** (pinned) |
| Moved 10.5ε | `fill ∪ brick ∖ slab` | −1.28 | +2.71 |
| Turned 10.5ε | `slab ∖ fill`, `slab ∩ (brick ∖ fill)` | −1.84 | −4.82 |
| Turned 10.5ε | `brick ∖ slab ∪ fill` / `fill ∪ brick ∖ slab` | −1.04 / −0.47 | −1.59 / +1.18 |
| Turned 15ε | `brick ∖ slab ∪ fill`, `slab ∖ fill`, `slab ∩ (brick ∖ fill)` | +0.85, +0.93 | −3.62, −3.63 |
| Turned 15ε | `fill ∪ brick ∖ slab` | +0.75 | −1.41 |
| Moved 12ε | `slab ∖ fill`, `slab ∩ (brick ∖ fill)` | −2.36 | −3.07 |
| Moved 12ε | `brick ∖ slab ∪ fill` / `(brick ∖ fill) ∩ slab` | −2.14 / −0.95 | −2.53 / −1.50 |
| Moved 15ε | `fill ∪ brick ∖ slab` / `(brick ∖ fill) ∩ slab` | −0.44 / −0.73 | +1.63 / +1.36 |
| Turned 21ε | `brick ∖ slab ∪ fill` | +0.60 | +1.13 |

Every other row (all of `Pose::Aligned`, the 21ε and 30ε widths but for
the line above, and the leaning channel of
`a_needle_channel_whose_fill_face_leans_reads_its_side_off_its_seam`,
30ε, both poses) is under 1% on both sides of the merge. The turned 12ε
runs refuse before a volume is read (`Want::Corner`), so they have no
entry.

So the largest error went from 2.4% to 8.6% at that merge, the four
pinned runs are the tail of one distribution and not a behaviour of
their own, and two unpinned rows (Turned 10.5ε, −4.82%) stand 0.18
points inside the 5% bar: the next change to the order of that sum can
red a row that this table, not a new bisect, should explain.

## The merge

First-parent bisect by running the test, from `a46ac5820` (PR 6812,
which last edited the row): `9e1484043`, the merge of PR 6505
(`onedoor/one-surface-record`), is the first bad, its parent
`e996c0049` green; the table above is the same two commits read row by
row. That PR is 68 files (one-surface records, the glue, the census,
`props.rs`, `props/quad_lane.rs`). **Which of its changes moved the
sum was not measured**: not which face's term carries the error, nor
whether it is the plane term read about the face's anchor
(`work/onedoor/the-planar-face-reading-is-two-formulas-at-two-scalars.md`
is about that formula and may be the place, or may not).

## The pin

`MOVED_THIN_READS` holds the four runs past 5% to the values read,
within 1% either way; every other row keeps its 5%. The literals are
sixteen digits because that is what was printed, not because the last
digits mean anything.

## Wanted

The channel's shell volume within a stated bound of `w²` at every pose
and width of the test, with the bound derived (the sum's rounding
against `w²`) and not fitted, or the test reading the volume through a
certified enclosure where the `f64` sum cannot carry it. Then the pin
goes. `work/restfront/the-lane-free-sign-guard-has-no-witness-of-a-misrounded-sum.md`
wants a body whose `f64` sum rounds past its volume; this family rounds by a
growing share of it as the bar thins, and may be a lead for it.

Reproduce (red before the pin):

    cargo nextest run -p topo --test all -E 'test(=shell_role_reader::a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void)'

`crates/topo/src/props.rs` is BOXFINISH's, MEASUREDOOR's and TALLY's by
`scripts/work.py territory`, and `crates/geom-brep/src/props/mod.rs`
NEARPOINT's; the row is on ONEDOOR because the merge is its PR and the
code that moved is not known more finely than that.

---
id: a-seam-split-slivers-measure-stops-at-its-floor-2600-volumes-wide-since-the-rotational-cylinder-lane
kind: issue
title: A seam split's sliver side stops its measure at the floor with a volume bracket 2600 volumes wide, since the rotational lane reads the cylinder chart
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Filed by GREEN's default-eps lane (`green/default-eps`, 2026-11-14) from
nightly 43371638371 (main `e36675c1`), default ε row. It replaces
`work/green/main-red-the-aimed-ray-seam-split-census-misses-its-settle-floor.md`.

## What

`sweep::all an_aimed_ray_from_far_reads_a_split_side_right::the_seam_split_census_reads_no_side_inside_out`
(slow set) splits the unit cylinder by a plane through a seam vertex a
hair off the wall's tangent plane, and reads each side's
`mass_properties` at `topo::test_support::pinned_resolution` (1024·ε of
mean boundary displacement). At ε 1e-9 on main `b363f5fa`, four of its
384 poses build both sides, pass tier 3, and then one side's measure
does not meet the resolution:

    the pinned resolution is met: Unmet { stop: Floor }

The poses are the second azimuth (`φ = −2.8304347826086955`), `ψ = 5e-7`,
at both offsets `L ∈ {0, 0.1}` and either normal. The side that stops is
the sliver (the `above` side at `sign 1`, the `below` at `sign −1`):

| pose | side | volume enclosure | area enclosure | stop |
|---|---|---|---|---|
| `L 0`, `sign 1` | above | 2.668624995326391e-10 ± 7.039484242369427e-7 | 2.666667267690318e-3 ± 1.868570063978149e-9 | `Floor` |
| `L 0`, `sign −1` | below | 2.6685260170431096e-10 ± 7.039484242366536e-7 | 2.6666672676606243e-3 ± 1.868570063978149e-9 | `Floor` |

(`L 0.1` reads the same two lines.) The volume's half-width is about
2600 times the volume, and about 260 times what 1024·ε of displacement
over that area allows (2.7e-9 m³). The two sides' brackets together
still hold the cylinder's π, so nothing read is wrong; the measurement
stops paying before it is useful. At ε 1e-6 and 1e-12 no pose of the
census stops short.

## The merge

Bisected first-parent by running the test, from `839528164` (the merge
of PR 6174, which added the row) to `898df16c9`: every one of nine
steps is green up to `aef1e5892`, and `898df16c9`, the merge of PR 6478
(`spiricarm/rotational-lane-cylinder`), is red with the message above.
That PR's kernel change is `crates/topo/src/props/quad_lane.rs` (and
two lines of `props.rs`, `boolean/ops.rs`). Which face of the sliver
carries the width, and which rung of the lane stops, were not read.

## The pin

`read` in that test file returns `Read::Unsettled` for a pose whose
measure stops at `Stop::Floor` (any other unmet stop panics), having
checked that the sides' brackets hold π; the census asserts the
unsettled poses are exactly the four above at ε 1e-9 and none at the
other rows. It turns red when any of them meets the resolution, or when
another pose stops short.

## Wanted

The four poses read `Built`: each side's measure meets the pinned
resolution, as it did before PR 6478. Then the pinned list in the
census goes empty at every row (and `Read::Unsettled` with it, if
nothing else needs it).

Reproduce (red before the pin):

    cargo nextest run -p sweep --test all -E 'test(=an_aimed_ray_from_far_reads_a_split_side_right::the_seam_split_census_reads_no_side_inside_out)'

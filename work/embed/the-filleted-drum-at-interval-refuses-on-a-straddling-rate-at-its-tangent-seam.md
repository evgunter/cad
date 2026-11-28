---
id: the-filleted-drum-at-interval-refuses-on-a-straddling-rate-at-its-tangent-seam
kind: issue
title: With the sliver refused, the filleted drum at Interval refuses across the circle its fillet band shares with its top plane: its pairs split on a rate enclosure that straddles zero
status: open
opened: 2026-11-17
priority: P0
cost: M
---


## Finding

From the sliver measurement
(`the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause`,
its `## Measured` section, 7), main at `362cd72fd`.

`a_filleted_drum_certifies_across_its_fillet` at `Interval` holds only
through `present_chart`'s sliver: 16 `Zero` decisions, each on a cell
that lies past the fillet band's end (minor angle π/2, the circle the
band shares with the top plane), where the band's chart line is placed
over an enclosure 2.1e-14 rad wide and the sweep's cut falls inside it.
Each of those pairs certifies with neither face read.

With the sliver refused and the departure margin not scaled by the
rectangle's span, the drum refuses `Uncertified` (faces 1 and 7, edge
12, depth 40; 15,200 pairs): 11,512 pair visits split on a rate
enclosure that straddles zero, 11,275 of them on both faces, and 321
certify by the graph separator. The band meets the top plane
tangentially along that circle. At `f64` the line is one value, no such
cell arises, and the drum holds.

## Traced: three changes, each of which closes it

The fork's three designers read the numbers three ways. The
measurement lane ran each as a variant on its own, on top of the
refusing configuration (the sliver row's `## Measured`, section 9).
**All three hold the drum at `Interval`:**

| variant | verdict | pairs |
|---|---|---:|
| as measured (sliver refused, margin un-scaled) | refuses | 15,200 |
| **the line placed as a point**: the band's chart line on the shared circle given its `f64` value `[x, x]` in `placed` | holds | 559 |
| **the separator's direction from the chart's derivative across the zero side**, not from the rectangle's middle (`heading`, side and past-side zeros) | holds | 683 |
| **a separator across the seam**: the cylinder through the shared circle about its axis, tried before the graph | holds | 665 |

For scale: the drum at `Interval` on main, with the sliver, takes 567
pairs, and at `f64` 150.

- **With the line a point the far-side cell does not arise**: no
  sliver decision is taken at all, and the touch reads the cell
  outside, as at `f64`. The line's enclosure, 2.1e-14 rad where the
  construction fixes π/2, is where the refusal starts. The same
  mechanism refused the drum's wall rim until
  `a-fillet-band-rim-phase-outruns-the-touch-concession-at-interval`'s
  fix placed that rim from the sketch frame.
- **With the heading or the seam cylinder the thin rectangle is still
  presented, and is read**: 124 and 110 sliver-width rectangles arise,
  and the pairs certify (by the graph separator, or by the seam
  cylinder) or split and certify below.
- **What separates the 321 visits the graph certified from the 11,512
  it did not** is the graph separator's rate alone: decided on both
  faces where it certified (enclosures centred about 2.4 half-widths
  from zero), and under a tenth of a half-width from zero where it
  split. The cells, their widths and their place against the seam are
  the same in both groups. The trace did not record the direction
  itself; the heading variant holding is the evidence that the
  direction is what differs.

So this is not one missing reading but two places to act: where the
line is placed (it removes the cell), and how a rectangle without
width is read (it reads the cell). The first acts where the refusal
starts. The second is what any family whose line is honestly an
enclosure will need.

## What closing it takes

The drum at `Interval` holds, and its row's `Interval` half goes back
to asserting so. The sliver is retired without waiting for this row
(Ev, PR 7095: the drum is a test and no demo): since `embed/remeter`
(PR 7329) `a_filleted_drum_certifies_across_its_fillet` pins the
`Interval` half at its refusal, `Uncertified`, the top plane against
the band across their circle, arm `Departure`, and is red the day it
holds.
`teapotlid`, `hollowring` and `hollowtorus` refuse the same way once
the touch concession's `few_ulps` is also 0 (the measurement's section
8): the touch is not this row's, but the reading that closes this row
is the first thing to try there.

## A second witness: a rounded plate filleted all round, at `Interval` (EMBED, lane `embed/remeter`, 2026-11-22)

`sweep::all band_closed_mixed_chain::a_rounded_plate_filleted_all_round_removes_pappuss_volume_at_interval`
(26 faces, at the blend's door, the at-rest gate asking on
`embed/remeter-measure` at `343977aa0`, traced with `RM_TRACE`, one
thread). With the sliver retired it runs out of its 65,536 pairs
(65,408 leaves, 50 s) where the drum refuses `Uncertified`.

- **Where the pairs go.** Eight pairs of a wall plane against a fillet
  cylinder across their shared ruling take 49,836 of 69,498 visits:
  `5v1`/`21v1`, `5v1`/`29v1`, `9v1`/`25v1` and `9v1`/`33v1` at 8,125
  each; `1v1`/`29v1` at 4,254; `1v1`/`33v1`, `2v1`/`21v1` and
  `2v1`/`25v1` at 3,694 each.
- **Both faces are read**: presented with the ruling as a zero side in
  over 99% of those visits.
- **The seam is tangent.** The carrier arm's sine straddles zero in
  every visit of a pair (at the deepest, `[-3.1e-14, 7.3e-14]`, at an
  arm of 0.5 m). The plane through the ruling and the graph separator
  decide in about 200 of a pair's 8,125 visits and straddle in 160.
- **The cylinder's rectangle is the line's enclosure**: at the deep
  visits `u ∈ (2.3561944901923053, 2.3561944901923453)`, 4e-14 rad
  across, about 3π/4. The drum's is 2.1e-14 rad.

**Inferred, not measured**: that main certified these strips through
`present_chart`'s sliver, as it did the drum's 16. Sliver decisions
were not counted for this test before the change, and before it the
ask refused earlier, at another pair (`1v1`/`27v1`, edge `57v1`), so
these eight pairs' readings on main are in no log. That count is what
tells this from the next most likely cause, the pairs having certified
shallower by another separator.

Any of the three changes above is what it needs.

## A third sighting: a support plane against its corner patch across their arc (EMBED, lane `embed/tangent-corner`, PR 7320, 2026-11-23)

`sweep::all self_intersection_proves_injectivity::a_support_face_c1_through_a_vertex_at_interval`:
the rounded plate (`2 × 1.2 × 0.5`, corner radius `0.3`, every edge
filleted at `0.1`) built at `Interval`, a cap plane and a corner
patch (torus) asked alone through `topo::clearance::embedded`. On the
landing tree (`embed/wave3-land` with PR 7329 in) the pair reads
`Uncertified`, arm `Departure`, plane × torus across an edge, at the
default ε and at `1e-6`. On the lane's head before that merge
(`560e75596`) it held.

- **Where it refuses** (a print in `sign_on`'s departure, taken on the
  merged tree and removed): at mid-arc cells, not at the joint. The
  plane is presented with the arc as a `Zero::Curve` with no run, the
  torus with it as a `Zero::Side`, both under the graph separator, and
  the sine of departure straddles zero on both down to the depth
  floor: `(-1.8e-6, 2.2e-6)`, `(-7.1e-8, 2.8e-8)` on the plane,
  `(-6.8e-8, 8.6e-9)` on the torus; where one is definite it is
  `8e-8 .. 2e-7`. 1,256 of the plane's 1,313 such decisions and 1,697
  of the torus's 2,406 answer nothing.
- **The joint is read**: of the 72 decisions on a run through the
  arc's end vertex, 66 decide; the 6 that do not are coarse cells.
- **The sliver, counted on the head before the merge**: the same row
  on `560e75596` with a print at `present_chart`'s sliver `Zero`:
  170 sliver decisions in the row's run, 20 on each of four faces and
  11 on each of four more of the plate's band, and the row passes.
  With the sliver retired the row refuses. That is the count the
  second witness above did not have; it was not broken down by pair.
- **At `f64` the same bodies hold**
  (`a_support_face_c1_through_a_vertex_is_embedded_at_its_tangent_joints`,
  six `Embedded`).
- **Why no one saw it**: without the run through a vertex the pair
  refused earlier, at the joint, so its mid-arc cells were never
  reached at `Interval`.

Census tests that now get past `f64` and stop at this exit with the
at-rest gate asking (measured on `75224e843`):
`a_lattice_drawn_rounded_plate_fillets_whole`,
`an_l_plates_rounded_corner_run_fillets_whole_at_interval`,
`two_arcs_in_one_run_build`.

## A second body: the three-arc disc's rim band (EMBED collinear-cover lane, 2026-11-23)

`crates/sweep/tests/self_intersection_proves_injectivity.rs`
`a_three_arc_discs_filleted_rim_holds_across_its_joints`: the disc of
radius `0.5` whose circle is three arcs, its top rim filleted at `0.1`.
At `f64` it holds in four poses. At `Interval`, on the landing tree
(`5764e4b84` merged), every pose refuses at the top plane `1v1` against
the band `9v1` across their circle `18v1`:
`Uncertified { .., Departure }` as built (discharged 26,705, splits
38,542), turned one radian about a generic axis (48,312 / 64,888) and
a hundred units off (29,452 / 38,929);
`Budget { Pairs { 65536 }, a: 1v1, b: 9v1 }` mirrored across a generic
plane. The row pins the body as built at that refusal, by the owner's
word on PR 7320
(https://github.com/evgunter/cad/pull/7320#issuecomment-7093255162).

**The probe that puts it on this row and not on
`a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth`.**
On `c76b74b2c` alone (the sliver still read) the same body holds at
`Interval`, and a print at each sliver `Zero` and at `graph`'s `None`
shows how: the band `9v1` is closed by the sliver 458 times, 237 at the
top circle and 221 at the foot parallel on the wall, while the plane
`1v1` ALREADY fails to read its graph there (`graph_read` is `None`
7,039 times on `1v1`). So the plane's cell was never presented on
either tree; the pair was certified by the band's side reading
`Vacuous` through the sliver, and with the sliver gone nothing closes
it. On the landing tree the depth stop is at plane cell
`u ∈ (−0.396875, −0.395313)`, `v ∈ (−0.052344, −0.051563)` (the
circle at about 187°, no joint near: the joints are at 0°, 120°, 240°)
against band cell `u ∈ (1.17503, 1.17810)`, `v ∈ (π/2, 1.57693)`;
`graph` exits 2,837 times in the run, and none of the cover's readers
(`vacuous`, the clip merge, `sides_are_the_faces`) refuses.

The foot parallel, the band against the wall, is tangent too and was
closed by the same sliver (221 closures); the top circle refuses first,
so what the wall's seam reads without it is not measured.

## 2026-11-23: re-banded P0 (EMBED orchestrator)

With PR 7329 in the second landing branch, three valid bodies in three units refuse here at `Interval` and hold at f64: this row's drum (its `Interval` half pinned refusing by PR 7329), PR 7320's rounded plate (its plane against a torus patch across their arc; `a_support_face_c1_through_a_vertex_at_interval`) and PR 7322's three-arc filleted disc (top plane against the band across the top circle; its lane's probe decides whether it belongs here or on the cell-side row). Each lane appends its trace below when its unit lands. None of the three changes this row lists is built.

## 2026-11-25: the three sightings hold with the band's window narrowed (EMBEDWALK lane `embedwalk-twobands`)

Measured on the EMBEDWALK branch `embedwalk/two-bands`, which gives a torus
face the chart rectangle its boundary's pcurves span as its walk window
(`clearance::window_of`, through `cover::bounded_torus_window`, on the loop's
bounded side only) instead of the whole tube. Nothing in `injective.rs`
changed. At `Interval`: the drum holds; every plane against a torus patch of the rounded plate across
their arc is `Embedded`; the three-arc disc's top plane against its band
holds. So the straddling rate was read on cells past the band's end, which
the narrowed window no longer holds, and this row's symptom closes with
that branch. The rate reading itself is unchanged: a window that still
reaches past a tangent seam (a torus face with no chart rectangle, a
sphere, a cone's whole turn) would meet it again. The three-arc disc at
`Interval` still refuses, now on two of its wall's cylinders across their
shared edge (`Uncertified`, departure arm), which main reads with the
same receipt to the bit when that pair is asked alone (untraced).

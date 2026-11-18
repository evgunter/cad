---
id: a-small-cone-tips-apex-end-may-round-past-the-nappe-slack
kind: issue
title: face_extent compares a cone face's extent with the apex at chart_slack of the extent, while a meridian's end at the apex is rounded at the edge's own parameter, so a small tip of a large cone may refuse Structural in every cell
status: open
opened: 2026-11-18
priority: P3
cost: M
---


## Finding

From EMBED lane half-sphere's second review (PR 7086).

`face_extent` (`crates/topo/src/injective.rs`) checks a cone face's
nappe (`face_nappe`) against the extent's `v` with exact zero, the
apex, at `chart_slack(&[lo, hi])`. That slack scales with the extent:
about `1.1e-14` for a small face, the floor `few_ulps(TURN)`.

A meridian's end at the apex is `c + l·t`, evaluated at the edge's own
parameter, so its rounding scales with the edge, not with the extent.
On a small tip of a large cone the extent's low end can round past
`-slack` while the face is on the positive nappe. The face is then
read as contradicting its nappe and refuses `Structural` in every
cell.

- **Unit probe (the review's):** an extent low end of `-1e-12` on a
  unit face refuses `Structural` in every cell.
- **Unreached on built bodies:** 60 random cone tips (radius 20 to
  200, height 50 to 450, tip 0.05 to 1.55) all hold. The session box
  stopped larger ones.

It is a refusal, not a wrong reading, hence P3.

## What is needed

Read the apex's `v` at the scale its rows round at. Two ways:
- the enclosure of each meridian's end at the apex, read off its
  pcurve;
- a slack from the rows' parameter magnitudes, not from the extent.

Then build the tip of a large cone that reaches the current one, and
pin it holding.

## The same end rounded the other way left the apex unplaced; fixed in PR 7115 (EMBED, lane pole, 2026-11-18)

Found sweeping for the shape of
`a-sphere-face-whose-extent-stops-ulps-short-of-its-pole-never-places-the-pole`:
a singular point's collapsed side skipped because the rectangle,
clipped to the stored extent, stops a rounding short of it.

`present_chart`'s apex reading skipped a face on the positive nappe
where `y.0 > 0.0`, and `y` is the cell clipped to the extent. So an
extent whose low end rounded ABOVE zero never placed the apex, and the
cell at the apex vertex read `Local` at every depth.

- **Reached by plain built bodies** (the review of PR 7115): the
  triangle `(0, 2.7) (0, 1.7) (1, 2.7)` revolved a full turn about `y`
  stores `v = (2.2e-16, 1.414…)` on both half-cones and refused
  `Budget(Pairs)` at 41,321 / 24,215 / 65,530.
- **How often** (this lane's sweep, f64): six cone shapes at 61 apex
  heights `y0 = -1.0 … 5.0`, apex down and apex up, each body and its
  axial-door cavity at `t = 0.05`. Of the first 949 cases, 113 refused
  `Budget(Pairs)` before the fix, most of them the shape `r = 0.3`,
  `h = 1`.

**What landed.** `face_extent` reads a cone face's apex as it reads a
sphere face's poles (`reaching_its_singular_points`): a face holding a
vertex at its apex has an extent that reaches `v = 0`, on the end its
nappe says. All 1,464 cases of the sweep hold, and every case that
held before holds at the same receipt. Pinned by
`a_unit_cone_with_its_apex_off_the_origin_holds` and
`a_cone_face_whose_extent_starts_a_rounding_off_its_apex_places_the_apex`.

**What is still this row's.** The extent moves to the apex only across
`chart_slack`, the same scale the nappe check above uses. An apex end
rounded farther than that, on either side of zero, still refuses
`Structural`: below zero by the nappe check (this row's finding), above
it by the new one. The scale question is one decision with
`a-vertex-in-band-of-a-pole-whose-rows-end-at-its-own-latitude-refuses-structural`.

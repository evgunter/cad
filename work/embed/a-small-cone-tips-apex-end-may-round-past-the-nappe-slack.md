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

## The same end rounded the other way leaves the apex unplaced (EMBED, lane pole, 2026-11-18)

Found sweeping for the shape of
`a-sphere-face-whose-extent-stops-ulps-short-of-its-pole-never-places-the-pole`:
a singular point's collapsed side skipped because the rectangle,
clipped to the stored extent, stops a rounding short of it.

`present_chart`'s apex reading (`crates/topo/src/injective.rs`, "A
cone's apex is a collapsed side of its chart") skips a face on the
positive nappe where `y.0 > 0.0`, and `y` is the cell clipped to the
extent. So an extent whose low end rounds ABOVE zero never places the
apex, and the cell at the apex vertex reads `Local` at every depth.

- **Unit probe (uncommitted):** the sector of
  `a_cone_face_at_its_apex_is_read_on_its_own_nappe`, its meridians
  ending at the apex vertex, in the cell `u ∈ [1.2, 1.8]`,
  `v ∈ [-0.1, 0.3]`. With the extent's low end `0` or `-1e-17` the cell
  is presented with the apex's side its zero. With `1e-17` or `1e-300`
  it reads `Err(Local)`.
- **Unreached on built bodies as far as measured:** no cavity row and
  no tour stop moved with the sphere's fix or shows this refusal, and
  PR 7086's cone tips all hold. A meridian line's `v` at the apex is
  `c + l·t`, exactly zero where the line starts at the apex.

The sphere's fix reads the pole off the face's vertex in `face_extent`
(`reaching_its_poles`). The apex wants the same reading, a cone face
holding a vertex at its apex has an extent that holds `v = 0`, and it
is one decision with this row's: both are the apex end's rounding
against the extent, on either side of zero.

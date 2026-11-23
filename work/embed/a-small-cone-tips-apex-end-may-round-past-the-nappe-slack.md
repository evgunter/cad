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

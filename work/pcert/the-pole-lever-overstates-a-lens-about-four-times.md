---
id: the-pole-lever-overstates-a-lens-about-four-times
kind: issue
title: the sphere-pole meter overstates a lens between a meridian and a curved edge about four times
status: open
opened: 2026-10-14
priority: P3
cost: M
design: true
refs: [the-period-jump-has-no-ruling]
---


Found by PR 5080's review (both reviewers, executed probes).

`topo::chord_join::pole_branch` meters the angle `θ` between two edges
at a sphere pole by `θ·lever`, with `pole_lever` the shorter edge's
furthest distance from the axis, capped at its curvature radius. The
ruling (`the-period-jump-has-no-ruling`, "## The answer", group 1)
names the cap as the edge's "relative curvature radius 1/κ_rel". The
code reads that as each edge's own geodesic curvature radius `1/κ_g`;
it says so in `pole_lever`'s doc.

**The overstatement.** Take a meridian and a small circle through the
pole that bends back toward it (a lens), the two meeting again at P. On
that lens `θ·lever` overstates the pair's largest separation about four
times:

- R2's θ table (β = 0.3; the meridian at azimuth −π/2 + θ; both edges
  running from the pole to P):

  | θ | θ·lever | true max separation | ratio |
  |---|---|---|---|
  | 1e-3 | 6.19e-7 | 1.55e-7 | 4.000 |
  | 1e-2 | 6.19e-5 | 1.55e-5 | 4.000 |
  | 1e-1 | 6.17e-3 | 1.55e-3 | 3.994 |

- R1's P2 (ε = 1e-9, K = 3): θ is scaled so that the lens's true
  largest separation is 0.9·ε. Then `θ·lever` = 3.6e-9 ≥ K·ε, and the
  joint decides the branch (`Ok(0)`). A lens within ε everywhere is
  read as two definite directions.

**Where it is unsound.** Only under a band with K < 4. With K ≥ 4 an
over-levered pair lands in the band or decides correctly.

**Why nothing rides on it today.** A non-meridian circle's chart image
on a sphere is projected (`geom_brep::chart_pcurve`'s general lane,
`Underived::General` since PCERT E1), and the walk refuses a projected
image as "not yet" (`run_azimuth_images`, after `chart_azimuth_range`,
`SplitJoinError::RunImageNotYet`). So no answer
depends on the meter's curved-edge reading.

**The design question, if curved pole edges ever become reachable.**
Which lever reads a lens honestly? Candidates:
- the pair's own separation bound (θ with each edge's sagitta);
- the ruling's `1/κ_rel` read as the two edges' relative curvature;
- a factor folded into the cap.

The answer goes to designers before any code.

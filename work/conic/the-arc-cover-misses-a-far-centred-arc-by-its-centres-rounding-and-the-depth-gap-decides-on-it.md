---
id: the-arc-cover-misses-a-far-centred-arc-by-its-centres-rounding-and-the-depth-gap-decides-on-it
kind: issue
title: Cover::arc under- and over-reads a far-centred arc by about u*far, and depth_gap clears on it; a far conic's own axis as the depth datum does the same with no point read
status: open
opened: 2026-11-19
priority: P3
cost: H
refs: [a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Found by the phase-1 probe of `oneround/conic-vertex` (2026-11-19), on
merge base `b80352cd9`. Probes and run commands: branch
`analysis/oneround/conic-vertex-probes` (never merged),
`analysis/oneround/conic-vertex/` (`probe-b.patch`,
`census::depth_gap::vertex_probe`).

`a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`
listed the extent and cover readings as "bounds only (an extent or a
lever, which rounding loosens)". Rounding does not only loosen them.

## The sites

- `geom::cover::Cover::arc` and its private `Arc` piece
  (`crates/geom/src/cover.rs`: `Arc::support`, `Arc::circle_about`,
  `Arc::near`, `Arc::far`), reached through
  `topo::cover::{face_cover, edge_cover}` and `Cover::range_about`.
  They are the successors of `conic_ring` and `edge_reach_of` (retired
  by commit `f67931961`) and `axial_carrier` (`f097a63b8`). Each reads
  `frame.origin` and the semi-axes directly.
- `crates/topo/src/census/depth_gap.rs`: `face_read` proposes each
  boundary conic's `conic.frame().axis_line()`, a line through the
  stored centre; `greatest_on` reads `cover.range_about`; `clears`
  decides `lo − hi − 2·sweep_pad` under `census_backstop_depth_gap`
  (`sweep_pad` is band-only, `escalate + 2·zero`).

## Probe A: a far-centred arc's cover about an in-box axis

Pose: a circle, or an ellipse with `b = 0.8a`, in the xy-plane,
`a = far`, vertex near (40..114, 7..−15, 3), stored centre
`vertex − u·a`, window `±1/far`; read `range_about(z-axis)`. Truth:
2001 samples of the vertex form from the stored centre's exact vertex
(`two_product`, `two_sum`). The other operand is a segment parallel to
z placed `pad + m·eps` past the true extreme, `m` in
{−5, −1, 0, 0.5, 2, 5, 9}, outside and inside; 2,800 readings per row
through `clears`. "Wrong" is cleared with exact margin at most 0;
"in gap" is a definite verdict with the exact value inside the band's
gap.

| ε | far | circle wrong / in gap | ellipse wrong / in gap | the circle's greatest undershoots the true arc by |
|---|---|---|---|---|
| 1e-9 | 1e4–1e6 | 0 / 0 | 0 / 0 | 2.1e-12 … 1.9e-10 |
| 1e-9 | 1e7 | 0 / 24 | 0 / 0 | 1.8e-9 |
| 1e-12 | 1e4 | 0 / 23 | 0 / 0 | 2.1e-12 |
| 1e-12 | 1e5 | 20 / 359 | 3 / 105 | 2.0e-11 |
| 1e-12 | 1e6 | 346 / 996 | 58 / 189 | 1.9e-10 |
| 1e-12 | 1e7 | 420 / 1058 | 61 / 157 | 1.8e-9 |

At far 1e3 every count is 0. The circle's least bound overshoots by
about the same (2.3e-9 at 1e7). The ellipse's least bound is loose by
3.9e-4 and never overshoots.

## Probe B: the depth a far-centred conic proposes

Pose: the axis line through the stored centre, read on two in-box
segments parallel to it, `pad + m·eps` apart radially. Truth: the
factored gap `(p2 − p1)·(p1 + p2 − 2c)/(d1 + d2)`. 2,800 readings per
row.

| ε | far | wrong | in gap | worst abs(read gap − true) |
|---|---|---|---|---|
| 1e-9 | 1e3–1e6 | 0 | 0 | 2.3e-13 … 2.7e-10 |
| 1e-9 | 1e7 | 0 | 24 | 3.8e-9 |
| 1e-12 | 1e4 | 0 | 77 | 3.5e-12 |
| 1e-12 | 1e5 | 37 | 235 | 2.9e-11 |
| 1e-12 | 1e6 | 170 | 375 | 2.1e-10 |
| 1e-12 | 1e7 | 1 | 6 | 1.8e-9 |

No conic point is read here: the datum itself is far, so every in-box
depth about it is about `far` and rounds at `u·far`.

## A row that is green only because `eval` shares the rounding

`splitting::containment::arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size`
(`a = 1e7`) asserts each `carrier.eval(t)` inside the cover's ball.
With the point evaluated from the vertex it reds: "the arc's point at
-0.00045 lies 0.5062501916684449 out, past 0.5062501912455183". The
cover misses the arc by 4.2e-10; on main the point is rounded the
same way and the row passes.

## What a vertex-relative point does

Nothing for either probe (identical rows with the measurement switch
on): the cover reads the frame, never a point.

## The re-posing to try

- The `Arc` piece anchored at the vertex `c + u·a` with offsets
  `u·a·(cos t − 1) + v·b·sin t`, and `circle_about` / `near` / `far`
  re-derived without `off_axis ± ρ` at the far scale. A rewrite of the
  arc piece; not built, not measured.
- For the proposed depth: do not propose a far axis, or pose the depth
  difference as a difference (as the probe's truth does).

## Unprobed

`depth_gap::face_nearest` (which `least_on` uses for a line or point
depth) and `apart_in_depth` end to end: each needs a `Body` with a
far-centre rim, which the probe did not build.

## The kernel's own far cone sections, anchored and not (2026-11-21)

`Curve3::cover_over` states an ellipse's arc cover from `frame()`, which
on an anchored section is the derived centre frame
(`crates/geom/src/cover/readings.rs`). Measured on
`oneround/anchored-conic` at `8d5978300`: `plane_cone_section`'s ellipse
at semi-axes from 22 m to 2.2e7 m, a short arc about its near vertex
(half-window `3/a`), 65 points of the arc against the cover's ball. The
worst of `|P − mid| − radius` at `f64`, positive where a point of the arc
lies outside its own cover:

| semi-axis | ball radius | anchored (step A) | centre-stored, `main`'s mint | centre-stored at `vertex − u·a` |
|---|---|---|---|---|
| 2.2e1 (window 1e-3) | 2.2e-3 | +6.2e-16 | 0 | 0 |
| 2.2e2 | 9.5e-2 | +1.2e-15 | +9.5e-15 | +9.5e-15 |
| 2.2e3 | 3.0e-2 | +3.1e-14 | +2.5e-14 | +2.5e-14 |
| 2.2e4 | 9.4e-3 | −2.1e-13 | +8.2e-13 | +8.2e-13 |
| 2.2e5 | 3.0e-3 | +5.9e-12 | +3.2e-12 | +3.2e-12 |
| 2.2e6 | 9.4e-4 | +9.3e-12 | +1.1e-10 | +1.1e-10 |
| 2.2e7 | 3.0e-4 | −1.5e-10 | 0 | 0 |

So the cover of a far section's short arc does not hold the arc's own
points on `main` already, by up to 1.1e-10 m at 2.2e6 m, and storing the
section anchored neither removes nor adds to it: the miss stays at a
rounding of the semi-axis, smaller at three sizes and larger at two.
Over a wide window (0.3 rad) the ball holds every point with 0.1 to
0.2 m to spare in all three.

At `Interval` (every input a point enclosure), the same arc: the ball's
radius is itself 1e-9 wide at 2.2e2 m, 5e-6 at 2.2e4, 6e-2 at 2.2e6 and
5.9 m at 2.2e7 in both forms, and the upper end of a point's distance
passes the radius's upper end by 3.0e-10, 2.7e-6, 5.8e-2 and 5.8 m
anchored, against 6.1e-10, 5.3e-6, 1.2e-1 and 12 m centre-stored.

The anchored section changes nothing this row asks for: the cover reads
a centre. Re-posing the `Arc` piece about the stored anchor, as above,
is this row's.

## Moved to CONIC (2026-11-23)

Moved from ONEROUND by `git mv` when that track was split on Ev's answer to PR 7523 (`work/conic/log.md`); the id and the body above are unchanged.

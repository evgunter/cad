---
id: a-far-centred-conics-point-is-decided-against-an-in-box-point-at-the-centres-rounding
kind: issue
title: The cone's apex rung and the pose's clearance decide a far-centred conic's point against an in-box point at u*far: wrong at eps 1e-12 from far 1e5, and only an exact or stored vertex reads right
status: open
opened: 2026-11-19
priority: P3
cost: M
refs: [a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Found by the phase-1 probe of `oneround/conic-vertex` (2026-11-19), on
merge base `b80352cd9`. Probes and run commands: branch
`analysis/oneround/conic-vertex-probes` (never merged),
`analysis/oneround/conic-vertex/` (`measure-full.patch`
`probe_apex_rung_far_centre`; `probe-c.patch` `probe_clear_tilted`,
`probe_clear`).

Both sites compare a far-centred conic's point with something else in
the box, so the conic's vertex has to be right absolutely, not only
relative to another of its own points. Two more of the same shape sit
on other slates:
`work/select/an-endpoint-pin-passes-on-a-far-centred-conics-own-rounded-point.md`
and
`work/charttail/row-interval-accepts-a-far-centred-conics-end-inside-the-bands-gap.md`.

## `off_the_apex` (`geom_brep::curve_face::conic_quadric`, row `conic_quadric_cone_apex`)

Decides `Margin::norm3(conic.point_at(θ) − apex)` for each certified
root; not definitely positive refuses `Unsettled::AtApex`.
`through_the_apex` reads the same quantity under the same row.

Pose: an ellipse of semi-axes `(far, far/100)`, generic `u`, vertex in
the box (coordinates under 10), centre `vertex − u·far`; the apex an
exactly known `d` off the stored curve's exact vertex (`two_product`,
`two_sum`), `d` up to 40ε; the root handed in at `θ = 0`;
`Band::new(eps, 10·eps)`; 4,000 poses per cell.

| ε | far | a root within ε of the apex passed | a root past 2Kε refused |
|---|---|---|---|
| 1e-9 | 1e3 … 1e7 | 0 | 0 |
| 1e-12 | 1e3 … 1e5 | 0 | 0 |
| 1e-12 | 1e6 | 112 | 11 |
| 1e-12 | 1e7 | 109 | 0 |

The wrong kind is the first column: definite (`Positive`) with the
exact distance inside ε, so the root stands at a point where the cone
has no tangent plane.

- Point from the vertex, vertex summed in f64: identical to main in
  every cell (the plain `c + u·a` rounds as the centre form does).
- Vertex summed exactly: 0 and 0 in every cell.

## `curve_pair_pose_clear` (`Polish::closest`, `geom-brep/src/curve_pair/pose.rs`)

The successor of the `Window::read` clearance the parent row names
(no `Window::read` or `fn ring` is in `curve_pair`'s history).
`decide(CLEAR, clear − reach)` Positive answers "no meeting here",
`clear = Curve3::clearance(b, q)`
(`crates/geom/src/curves/on_carrier.rs`). The ellipse arm is
`ellipse_level_clearance(frame.in_plane(q), (a, b))` with
`F = (x/a)² + (y/b)² − 1`, which resolves to about `u·far`; the circle
arm is `ρ − r` with `ρ` read from the far centre. It is used as a
lower bound, uncharged.

Pose: `q` an exactly known distance outside the near major vertex,
swept over −2e…4e, `reach = e = band.escalate()`; 4,000 poses per
cell.

- Tilted ellipse: skipped ("no meeting") with the true distance at
  most `e`: 462 at ε 1e-12, far 1e5 (over-read up to 2.7e-11); 0 in
  the other 7 cells, though the over-read reaches 2.0e-10 at ε 1e-12,
  far 1e6 and 2.0e-9 at ε 1e-9, far 1e7. At ε 1e-12, far 1e7 the read
  collapses to 0 and never skips.
- Axis-aligned ellipse: 132 wrong at ε 1e-12, far 1e5, else 0.
- Circle arm: 0 wrong, over-read up to 9e-10. UNPROBED at far ≥ 1e6:
  with the narrow targets tried the read collapsed to 0.

The measurement switch does not reach this site (it reads
`frame.in_plane`). A test-local rewrite, `x′ = (q − V)·u`,
`F = x′(2a + x′)/a² + y²/b²`, against the tilted ellipse:
- `V = c + u·a` in f64: 0 wrong at ε 1e-9; at ε 1e-12, 0 / 0 / 858 /
  1386 wrong at far 1e4…1e7 (over-read `2.2e-17·far`, the product's
  rounding). Past 1e6 that is WORSE than main.
- `V` exact, its tail carried: 0 wrong in all 8 cells, over-read 0.

## So

Evaluating the point from the vertex is not enough at either site:
the vertex constant has to be exact, which at f64 is an error-free
sum from the stored centre and under a stored form anchored in the
box is simply the stored point
(`work/oneround/the-section-table-admits-a-curve-on-its-resolution.md`).
PR 7131's reading of a stored frame bears on the first: if the true
frame is the polar factor of the stored entries, the exact sum of the
stored entries is no longer the true vertex at the far scale.

## Moved to CONIC (2026-11-23)

Moved from ONEROUND by `git mv` when that track was split on Ev's answer to PR 7523 (`work/conic/log.md`); the id and the body above are unchanged.

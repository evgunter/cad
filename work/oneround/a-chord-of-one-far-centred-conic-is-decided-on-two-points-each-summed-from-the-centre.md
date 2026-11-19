---
id: a-chord-of-one-far-centred-conic-is-decided-on-two-points-each-summed-from-the-centre
kind: issue
title: pm_census_arc_piece, bool_factored_crossings_apart and the pose's ROOT_GAP decide a chord of one conic from two centre-summed points: definite and false at eps 1e-12 from far 1e5
status: open
opened: 2026-11-19
priority: P3
cost: E
refs: [a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Found by the phase-1 probe of `oneround/conic-vertex` (2026-11-19), on
merge base `b80352cd9`. Probes and run commands: branch
`analysis/oneround/conic-vertex-probes` (never merged),
`analysis/oneround/conic-vertex/` (`probe-b.patch`
`census::vertex_probe`, `probe-c.patch` `probe_root_gap`).

## The sites

Each decides the distance between two points of ONE conic, each point
summed from the far centre, so the chord rounds at about `2·u·far`:

- `pm_census_arc_piece` (`crates/topo/src/census.rs`, the `apart`
  closure): `norm3(q − curve.eval(t))`, `q` the piece's midpoint point.
- `bool_factored_crossings_apart`
  (`crates/topo/src/boolean/factored_crossing.rs`):
  `norm3(carrier.eval(t) − carrier.eval(s))`.
- `curve_pair_pose_root_gap` (`harmonic_roots` in
  `crates/geom-brep/src/curve_pair/pose.rs`):
  `|k.point_at(θ_hi + ψ) − k.point_at(θ_hi − ψ)|`.

## What main reads

Census and factored crossing (the same expression under each row
name, identical numbers): tilted frame (n ∝ (0.3, 0.4, 0.5)), vertex
(37.3, −12.1, 5.7), a circle and an ellipse of ratio 0.8, `t` within
`±1/far`, `s = t + d/b`, `d` in {0.1 … 30}·ε; truth by the
sum-to-product form from the semi-axis vectors; 4,000 readings per
row. "Wrong" is Positive with the true chord at most `zero`, or Zero
with it at least `escalate`; "in gap" is a definite verdict with the
truth inside the gap.

| ε | far | circle wrong / in gap | ellipse wrong / in gap | worst abs(read − true), circle |
|---|---|---|---|---|
| 1e-9 | 1e3–1e5 | 0 / 0 | 0 / 0 | 1.9e-13 … 1.6e-11 |
| 1e-9 | 1e6 | 0 / 8 | 0 / 0 | 1.5e-10 |
| 1e-9 | 1e7 | 0 / 127 | 0 / 78 | 2.2e-9 |
| 1e-12 | 1e3 | 0 / 2 | 0 / 0 | 1.5e-13 |
| 1e-12 | 1e4 | 0 / 144 | 0 / 86 | 2.1e-12 |
| 1e-12 | 1e5 | 41 / 827 | 26 / 597 | 1.6e-11 |
| 1e-12 | 1e6 | 6 / 558 | 8 / 580 | 1.2e-10 |
| 1e-12 | 1e7 | 0 / 503 | 1 / 503 | 1.8e-9 |

`ROOT_GAP`: tilted frame `u_ref = (0.6, 0.8, 0)`, semi-axes
`(far, far/100)`, `θ_hi` within `3/far` of the seam, true chord
`2 sin ψ·|P′|` swept over 0.05ε…30ε; 2,000 poses per cell.

| ε | far | Cross with true chord < ε | Touch with true chord > 10ε | max read error |
|---|---|---|---|---|
| 1e-9 | 1e4…1e7 | 0 | 0 | 1.7e-12 … 1.1e-9 |
| 1e-12 | 1e4 | 0 | 0 | 1.7e-12 |
| 1e-12 | 1e5 | 2 | 0 | 1.4e-11 |
| 1e-12 | 1e6 | 0 | 1222 | 1.0e-10 |
| 1e-12 | 1e7 | 0 | 1300 | 9.1e-10 |

## What a vertex-relative point does

It fixes all three, with the vertex summed in plain f64: the two
points share the vertex constant and it cancels in the difference.
Wrong 0 and in gap 0 in every cell at both ε; worst error 3.7–4.1e-15
(census, factored crossing) and 5.6e-17 (`ROOT_GAP`).

## The re-posing to try

Either read both points from one vertex-relative chain, or read the
chord with no point at all: `2 sin(Δ/2)` times the semi-axis
combination at the mid parameter (the form the probes' truth uses).

## Unprobed

The enclosing functions were not run on a `Body`: the root parameters
`plane_crossing_lane` hands the factored site, the cut parameters the
census pieces are built from, and the points handed on from those
sites (`landing(…, carrier.eval(t), …)`, `meets_at(…, q, …)`) are
computed upstream from the same far terms.

---
id: a-line-or-a-conic-nested-against-a-far-centred-conic-is-cleared-on-terms-that-cancel-at-the-far-scale
kind: issue
title: line_clears_conic and conics_clear read a gap as the difference of two far terms, uncharged: definite and false at eps 1e-12 from far 1e5
status: open
opened: 2026-11-19
priority: P3
cost: M
refs: [a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Found by the phase-1 probe of `oneround/conic-vertex` (2026-11-19), on
merge base `b80352cd9`. Probe, truth script and run commands: branch
`analysis/oneround/conic-vertex-probes` (never merged),
`analysis/oneround/conic-vertex/` (`probe-a.patch`, module
`vertex_probe_a`; `a_truth12.py`).

Pose family for both: an in-plane ellipse, `u = (cos φ, sin φ, 0)`,
an in-box vertex `V0` (coordinates under 10), `a = far` in 1e4…1e7,
`c = fl(V0 − u·a)`; `Band::new(eps, 10·eps)`; 1500 poses per cell;
truth in exact rationals from the dumped bits. Probed at expression
level, not through a `Body`.

## `line_clears_conic` (`topo::splitting::section_loops`, row `split_nest_line_conic`)

Reads `|m·(c − p)| − √((a·m·u)² + (b·m·v)²)`: two terms about `far`
that cancel to the gap, rounding at a few `u·far`, uncharged.

Pose: a line through `V0 + u·g`, near-tangent, length 2, `g` swept
about the boundary; ratios `b/a` 0.01, 1, 2.

| ε | far | definite and false or inside ε / definite inside 10ε, per ratio 0.01, 1, 2 |
|---|---|---|
| 1e-12 | 1e4 | 0/16, 0/15, 0/15 |
| 1e-12 | 1e5 | 55/157, 57/168, 58/183 |
| 1e-12 | 1e6 | 76/94, 80/103, 80/101 |
| 1e-12 | 1e7 | 43/44, 46/46, 45/46 |
| 1e-9 | 1e4 … 1e7 | 0 wrong; in the gap 2–4 at 1e6, 13–17 at 1e7 |

The first count mixes the two kinds of wrong (a clear whose exact
margin is at most ε, or of the other sign); the dump separates them
(`a_truth12.py`).

Re-posing tried: `s·m·(V − p) − B²/(|A| + hypot(A, B))`, `V = c ∓ u·a`
the vertex on the line's side, `(A, B) = along(m)`,
`s = sign(m·(c − p))`.
- `V` summed in f64: 0 wrong at 1e4 and 1e5; 60–71 at 1e6 and 97–113
  at 1e7 (ε 1e-12); 5–11 in the gap at 1e7 (ε 1e-9).
- `V − p` summed exactly (`two_product`, `two_sum`): 0 wrong and 0 in
  the gap in all 24 cells.

## `conics_clear` (same file, row `split_nest_conic_conic`)

Reads `(|c′| − σ − 1)·lever` and `(1 − |c′| − σ)·lever`,
`c′ = unit_in_plane(h.origin)` about 1 and `lever = e.speed_lo()`
about `far`, so `|c′| − 1` rounds at `u·far` metres. Its
`(1 + 8·EPSILON)` pad multiplies `σ` only (about `r/far`): it is worth
about `8u·r` metres and is not a bound on the centre's rounding.

Pose: a circle of radius 0.05–0.45 beside the far ellipse's seam
vertex, inside or outside by `g`.

| ε | far | wrong / in the gap, ratio 1 then ratio 2 |
|---|---|---|
| 1e-12 | 1e4 | 0/21, 0/20 |
| 1e-12 | 1e5 | 8/22, 17/52 |
| 1e-12 | 1e6 | 17/17, 33/36 |
| 1e-12 | 1e7 | 15/15, 54/54 |
| 1e-9 | 1e4 … 1e7 | 0 wrong; in the gap 1–2 at 1e6, 14–15 at 1e7 |

At ratio 0.01 the site never clears in this pose (0 of 1500 in every
cell): `σ = r/b` exceeds `(r + g)/a`. Conservative, not wrong.

Re-posing tried:
`|c′|² − 1 = (u·u − 1)(u·u + 1) + 2σ(u·u)x + x² + y²`,
`(x, y) = unit_of(h.origin − V)`. With `V` in f64: 0, 0 at 1e5; 11, 19
at 1e6; 10, 36 at 1e7 (ε 1e-12). With `V` exact: 0 wrong and 0 in the
gap everywhere. The frame's unit defect `u·u − 1` enters at `u·far`
and has to be carried.

## What a vertex-relative point does

Nothing: neither site evaluates a conic point. Each needs its own
re-posing about the vertex, and the vertex has to be exact (or
stored); a plain `c + u·a` still rounds at `u·far`.

## Moved to CONIC (2026-11-23)

Moved from ONEROUND by `git mv` when that track was split on Ev's answer to PR 7523 (`work/conic/log.md`); the id and the body above are unchanged.

## 2026-11-23 — both sites removed (HELDRECORDS, PR 7436)

PR 7436 (`heldrecords/nesting`) deletes `line_clears_conic` and
`conics_clear`. `outlines_disjoint` now reads every pair with an
ellipse edge through `geom_brep::curve_pair::meet` over the two edges'
spans (`section_loops::edges_clear`), and nothing reads the whole
carriers. What this row owes at those two sites is gone with them. The
pose family here now reaches the nesting through `meet`'s line–conic
and conic–conic closed forms (`curve_pair::pose`). Whether those read
a far-centred conic soundly is the question
`a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex` lists
them under, so it moves there. Not probed at far scale here.

## 2026-11-23 — probed at far scale through `edges_clear` (GERM, PR 7553)

PR 7553 re-poses the extreme nearer zero of `line_conic`'s first harmonic and of `conic_conic`'s parallel screen (`Conic::belly_along`, exact on the stored data). After PR 7436's merge, PR 7553's row `splitting::section_loops::far_clear_rows::a_segment_clears_a_far_centred_conic_only_where_it_is_clear` reads `edges_clear` on a ten-metre segment beside a centre-held far ellipse.
- **The judge:** an exact dyadic oracle on the stored data.
- **The exception:** a touch dropped by the carrier walk is counted apart, and is filed as `a-centre-held-far-conics-touch-is-dropped-by-a-carrier-walk-on-points-evaluated-about-its-centre`.
- **What it reads:** that row's PR body has the counts. Conic × conic nesting at far scale is not probed there.

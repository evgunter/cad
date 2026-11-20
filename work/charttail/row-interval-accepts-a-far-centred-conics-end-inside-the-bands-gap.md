---
id: row-interval-accepts-a-far-centred-conics-end-inside-the-bands-gap
kind: issue
title: pcurve_row_interval is not fail-closed on a far-centred conic: it accepts an end whose true distance is in the band's gap, from far 1e4 at eps 1e-12
status: open
opened: 2026-11-19
priority: P3
cost: M
---


Found by the phase-1 probe of ONEROUND's `oneround/conic-vertex`
(2026-11-19), on merge base `b80352cd9`; filed here because
`crates/topo/src/pcurves/mint.rs` is this program's ground. Probe,
truth script and run commands: branch
`analysis/oneround/conic-vertex-probes` (never merged),
`analysis/oneround/conic-vertex/` (`probe-a.patch`, module
`vertex_probe_a4`; `a_truth34.py`). The class is ONEROUND's
`a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex`,
which listed this site as "fail-closed only (a misread refuses)".

## The site

`row_interval` in `crates/topo/src/pcurves/mint.rs` (row
`pcurve_row_interval`): `carrier.eval(r).distance(p)`; Zero accepts,
Positive gives `RowInterval`, an undecided margin gives `Escalated`.

## What main reads

Probed at expression level, not through a `Body`. Pose: an in-plane
ellipse seamed at an in-box vertex, `a = far` in 1e4…1e7, ratios 0.01
and 1, centre `far` out; `p` the well-conditioned point plus an offset
of 0–12ε or 0–3·max(10ε, 8·EPSILON·far); `Band::new(eps, 10·eps)`;
1500 poses per cell; truth in exact rationals from the dumped bits.

- Accepted with the true distance at least 10ε: 0 in all 16 cells.
- Accepted with the true distance in (ε, 10ε): 14 and 12 at far 1e4
  and 7 and 4 at far 1e5 (ε 1e-12); 7 and 1 at 1e6 and 12 and 18 at
  1e7 (ε 1e-9).
- Refused though truly within ε: 2 each at ε 1e-12 in the cells
  1e5/0.01, 1e5/1 and 1e6/1.
- At far ≥ 1e6 and ε 1e-12 nothing is accepted at all: an on-curve end
  reads about 1e-10 to 1e-9 off (1483–1500 Positive, the rest
  undecided).

So the predominant misread refuses, but the site also accepts: the
wrong kind is definite (Zero) with the exact distance inside the
band's gap, never past it, from far 1e4 at ε 1e-12 and far 1e6 at
ε 1e-9.

## What a vertex-relative point does

No better, with the vertex summed in plain f64 (`c + u·a` still rounds
at `u·far`): one accept with the true distance past 10ε (far 1e6,
ratio 1, ε 1e-12); in-gap accepts 10–18 per cell at far 1e4–1e5
(ε 1e-12), 1 each at 1e6 and 15–17 at 1e7 (ε 1e-9). An exact-vertex
`eval` was not built for this site.

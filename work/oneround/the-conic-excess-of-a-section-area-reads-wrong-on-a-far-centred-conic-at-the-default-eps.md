---
id: the-conic-excess-of-a-section-area-reads-wrong-on-a-far-centred-conic-at-the-default-eps
kind: issue
title: split_section_area's conic excess is a sector about the far centre less a triangle: on a sliver loop of a far-centred conic it reads definitely positive where the true margin is inside the zero band, from far 1e4 at the default eps
status: open
opened: 2026-11-19
priority: P2
cost: M
refs: [a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex]
---


Found by the phase-1 probe of `oneround/conic-vertex` (2026-11-19), on
merge base `b80352cd9`. Probe source, truth script and run commands:
branch `analysis/oneround/conic-vertex-probes` (never merged),
`analysis/oneround/conic-vertex/` (`probe-a.patch`, module
`vertex_probe_a3`; `a_truth34.py`; `README.md`).

## The site

`Sweep::certify_section_area` in `crates/topo/src/splitting/join.rs`
(row `split_section_area`). For each circle or ellipse edge of the
loop it adds

`excess = [(c − O)×(B − A)]·n̂ + s_a·s_b·(axis·n̂)·Δt − [(A − O)×(B − O)]·n̂`

to the vertex shoelace, `c` the conic's stored centre
(`conic.frame().origin`), `A`, `B` the edge's vertex points, `O` the
loop's first point, and decides
`Margin::over_lever(|twice_area|, perimeter)` under
`split_section_area`.

The first and third terms are a sector about the centre and a
triangle, each about `far·chord`, and they cancel to the segment
between chord and arc. The f64 arithmetic is not what fails: against
the same formula evaluated exactly on the same inputs, f64 read wrong
0 times in every cell. The form is ill-conditioned in its endpoints:
`A` and `B` are stored points that sit off the stored curve by
`δ ≈ u·far` (they were evaluated from the far centre), and an endpoint
off by `δ` moves `2A` by `far·δ`, so the margin moves by
`far·δ / perimeter`.

## The pose, the call and the oracle

Probed at expression level (the site's arithmetic replicated for a
loop of one chord and one arc, `O = A`), NOT through a `Sweep` or a
`Body`:

```rust
let n = Vec3::new(0.0, 0.0, 1.0);
let u = Vec3::new(phi.cos(), phi.sin(), 0.0);          // phi random in ±3.2
let v0 = Point3::new(x, y, 0.0);                        // |x|, |y| < 10: the vertex, in the box
let (sa, sb) = (far, far * ratio);                      // far 1e4..1e7, ratio 0.01 or 1
let c = v0 - u * sa;                                    // the stored centre, `far` out
let e = geom::Ellipse::new(geom::Frame::new(c, n, u), sa, sb);
let m = scale * rnd;                                    // the target margin, up to 12·eps or 3·max(10·eps, 8·EPSILON·far)
let delta = (12.0 * m / sa).sqrt();                     // so the true margin a·Δ²/12 is about m
let (t0, t1) = (-delta * rnd, t0 + delta);
let (a_pt, b_pt) = (e.point_at(t0), e.point_at(t1));    // the vertices as a mint evaluates them
let chord = (b_pt - a_pt).norm();
let perimeter = chord + chord + (e.speed_hi() * dt.abs() - chord);
let excess = (c - a_pt).cross(b_pt - a_pt).dot(n) + sa * sb * e.frame().axis.dot(n) * dt;
decide("split_section_area", Margin::over_lever(excess.abs(), perimeter), band)
```

`Band::new(eps, 10·eps)`; 1500 poses per cell of (eps, far, ratio).
The oracle is the segment's own area, read with no centre and no
endpoint: `|s_a·s_b·(Δt − sin Δt)| / perimeter`, in 70-digit decimal
from the dumped bits (`a_truth34.py`, `tB`).

## What main reads

"Wrong" here is one kind: the verdict is `Positive` (a section of
definitely positive area) while the exact margin is inside the zero
band, at most ε. The true area is positive and tiny; the sign is not
false. The loop should have refused as a degenerate section.

| ε | far | worst abs(read margin − true) | Positive with true margin ≤ ε, of 1500 (ratio 0.01 / ratio 1) |
|---|---|---|---|
| 1e-9 | 1e4 … 1e7 | 1e-4 m, 2e-3, 5e-2, 3.3 | 1336–1500 / 67–108 across the four |
| 1e-12 | 1e4 … 1e7 | not recorded per ε (the four figures above are the worst per far) | 751–1457 / 48–88 across the four |

- It reads `Positive` on 1265–1500 of 1500 poses in every cell.
- The first `far` that reads wrong is 1e4, at both ε.
- It also refuses up to 109 of 1500 loops whose true margin is at
  least 10ε.

## What a vertex-relative point does

The site reads no conic point, so the measurement switch reaches it
only through `A` and `B`. With the endpoints evaluated from the seam
vertex (plain f64 vertex sum) and the site unchanged: 0 wrong at far
1e4 and 1e5 at ε 1e-9, then 7 and 6 at 1e6 and 104 and 19 at 1e7; at
ε 1e-12 unchanged from main (48–1454). The worst error falls to 7e-9
(far 1e4) rising to 9e-8 (1e7).

## The re-posing to try

Per conic edge, add the segment directly:
`s_a·s_b·(axis·n̂)·(Δt − sin Δt)`, the difference from a short series
where `Δt` is small. It reads no centre, no vertex and no endpoint.
In the probe it reads 0 wrong in every cell in every mode, and
refuses none of the loops whose true margin is at least 10ε.

## Not established

- Not run through a `Body`: whether a join reaches a sliver loop on a
  far-centred section, and what `loop_points_of` hands it there, is
  unprobed.
- The pose is a sliver (true mean width near the band) on a conic
  whose centre is past the box. A plane cutting a cylinder of radius
  `r` at angle `θ` off its axis mints `a = r/sin θ`, with the centre
  where the plane meets the axis: `far` 1e4 is `r = 1`, `θ = 1e-4`,
  or `r = 100`, `θ = 0.01`. The sliver also needs the cut to graze.

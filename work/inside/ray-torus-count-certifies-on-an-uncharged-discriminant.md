---
id: ray-torus-count-certifies-on-an-uncharged-discriminant
kind: issue
title: The quartic ladder's rung 1 certifies a root count on f64 discriminant noise far past the band; at R = 1e6 a ray gets a silent wrong Miss
status: open
opened: 2026-10-11
priority: P1
cost: M
refs: [the-half-angle-ladder-certifies-in-band-configurations, line-torus-roots-may-certify-noise-when-the-line-origin-is-far, classification-invariant-family-types-bug-only-states-against-d9-row-4]
---


Found by the review of PR 4607 (CLEAVE's
`point-in-solid-mints-outside-its-period-and-nappe-rows`, mint C17),
which checked the unit's `CountDisagrees` witnesses exactly. The ladder
lives in `crates/topo/src/boolean/solid_contain.rs`
(`depressed_quartic_roots`, behind `line_torus_roots`).

## What

Rung 1 reads the root count off the sign of the quartic's discriminant
`Δ`, metered as `Δ / L¹¹` (`L` the lever, `R + r` on the ray lane). `Δ`
is a sum of terms each far larger than it near a tangency, and its `f64`
evaluation's rounding is charged to no decision. Where that rounding
exceeds the band, the "definite" sign is the noise's. The factorization
that builds the roots afterwards is right; rung 5 (`CountDisagrees`)
sees the defect only where the two counts happen to differ.

**The four quartics pinned by
`torus_predicate_rows::the_ladder_disagrees_with_itself_on_a_lanes_coefficients`**
(the half-angle lane's, from `circle_wall_rows` and `ellipse_rows`
`a_graze_is_read_by_its_depth`, band `(1e-9, 1e-8)`), counted exactly on
the `f64` coefficients (sympy `count_roots`, exact `Δ`):

| `p`, `q̂`, `s`, `L` | exact real roots | certified | built | exact `Δ/L¹¹` | `f64` rounding of `Δ/L¹¹` (terms × u) |
|---|---|---|---|---|---|
| −1.366e7, −413.8, 4.663e13, 1000 | 2 | 4 | 2 | −8.71e-8 | 1.2e-5 |
| −4.916e4, −0.1216, 6.043e8, 60 | 4 | 2 | 4 | 9.73e-9 (in band) | 6.9e-7 |
| −4.916e6, −24.31, 6.043e12, 600 | 4 | 2 | 4 | 3.89e-9 (in band) | 6.9e-6 |
| −4.916e4, −0.2431, 6.043e8, 60 | 4 | 2 | 4 | 3.89e-8 | 6.9e-7 |

The review measured the evaluation's actual error at 3e-7 to 6e-6:
30–500× the 1e-8 escalation threshold. In every row the construction is
right and the certified count is wrong; two rows are in band exactly and
were certified anyway.

**The ray lane reaches it.** The review cast ~400k near-tangent rays
(`point_in_solid`'s quartic, `line_torus_roots`) over ten tori from
`R = 1` to `R = 1e6`, at the default band; of the ~300k whose count the
kernel decided, it counted each exactly
on the kernel's own `f64` coefficients and on the represented geometry
(rational arithmetic; the geometry's crossings by mpmath). At `R ≤ 1e3`
there were 0 mismatches. The breakdown where there were any:

| torus `(R, r)` | kernel | exact, its coefficients | exact, geometry | rays | depth into the tube |
|---|---|---|---|---|---|
| `(1e6, 1e4)` | `CountDisagrees` 2 / built 4 | 4 | 4 | 1 | — |
| `(1e6, 1e4)` | certified `Miss` | 2 | 2 | **7** | 1.5e-9 to **3.9e-8 m** |
| `(1e6, 1e4)` | certified `Miss` | 2 | 0 | 3 | — |
| `(1e6, 1e4)` | certified `Miss` | 0 | 2 | 90 | 8.6e-10 to 9.4e-9 m |
| `(3e4, 300)` | certified `Miss` | 0 | 2 | 27 | 3.1e-11 to 3.3e-10 m |

- **Rung 1's defect, rows 1–3.** Ten rays are certified `Miss` where
  the kernel's own coefficients have two real roots (exact `Δ/L¹¹` down
  to −1.7e-7). Seven of those also cross the true tube, one by 3.9e-8 m,
  four bands past the escalation threshold. These are silent wrong
  answers: a `Miss` is folded as no crossing, so the ray's parity, and
  the point's verdict, can be wrong. That is what makes this P1.
  `CountDisagrees` (one ray) is the only symptom anything sees.
- **A second cause, rows 4–5.** The coefficients themselves lose a
  shallow crossing pair: `m`, `s` and `p` are formed with cancellation at
  this scale. These crossings are within the band (in the zero band at
  `R = 3e4`, in its gap at `R = 1e6`), so a `Miss` there is an in-band
  configuration certified, the trilean violation
  `the-half-angle-ladder-certifies-in-band-configurations` describes.
  It is not a wrong answer outside the band.

The probes are the review's (session scratchpad, not committed):
`quart.py` (the four quartics), `chk.py` (the ray dump against exact
counts), `one.py` (one ray's geometry), and the 300k-line `ray.txt`
dump. The depth breakdown above is the unit lane's re-read of the same
dump. Exact count of `y⁴ + p y² + q̂ y + s` on rational `p, q̂, s`: the
sign of `Δ = 256s³ − 128p²s² + 144pq̂²s − 27q̂⁴ + 16p⁴s − 4p³q̂²`, then
for `Δ > 0` four roots iff `p < 0` and `64s − 16p² < 0`.

## What would close it

- Charge `Δ`'s evaluation rounding to its decision: a bound on the
  terms' magnitudes times the rounding unit, metered over `L¹¹`. A rung-1
  reading within that bound is `Uncertain`, the same shape as the noise
  meter `half_angle_roots` runs before the ladder. This is the same
  family as GERM's `the-half-angle-ladder-certifies-in-band-configurations`.
  `shape` and `depth` (`p`, `D`) want the same audit.
- The coefficient-formation loss (rows 4–5) wants its own meter on
  `m`, `n`, `p`, `q̂`, `s`, or a recentred formation. It is in band, so
  it is P2 inside this row.
- Pin: the seven rows-2 rays answer `Uncertain`, not `Miss`, at
  `(1e6, 1e4)`. `torus_predicate_rows::a_far_ray_reaches_the_count_disagreement`
  holds the one ray that reaches rung 5 today.

`reduce.rs` `line_roots` answers `CountDisagrees` with
`ClassificationInvariant` and the kernel-defect ending, which is true:
the cause is this defect.

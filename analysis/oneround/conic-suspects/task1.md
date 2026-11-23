# Task 1 — the suspects, run

Clean `origin/main` at `dc263f855` (worktree, own target directory), public
API only, `f64`, `CAD_TOLERANCE_EPS` 1e-9 and 1e-12, K = 10. Six wrong
answers were sent as their own messages with reproductions
(`src/suspects_repro.rs`: on main 6 of its 7 tests fail at 1e-9, 7 at 1e-12).

## What decides

- **Every firm suspect that a geometry door reaches answers wrongly**, at
  the same sizes as the three already measured: from a semi-axis of **1e8
  at ε 1e-9 and 1e5 at ε 1e-12**, and never below (no wrong answer and no definite
  answer in the gap in 1 908 runs at a ≤ 1e7, ε 1e-9, nor in 954 at
  a ≤ 1e4, ε 1e-12). The fat shape (b = 0.3a) and the
  thin shape a body's far section has (b = 1.04√a) go wrong at the same
  sizes.
- **`curve_pair::meet` is wrong in one direction only**: `Ok`, no meeting,
  for curves that meet. 663 such answers; **0 meetings reported for
  curves that are 30 ε or more apart** (800 such runs per ε), and no reported
  meeting stands past K ε from either curve.
- **`OnSurface::under` is wrong in both directions** (row 13): `Roots([])`
  for a line in the edge's plane, and `InPlane` for a line 30 ε above it.
- **The P0 row's third cause is not `height_jet` alone.** The
  circle-across-planes miss is certified in layers: the window is first
  put in the wrong place (the plane stated through the derived centre,
  rows 2 and 3), then `height_jet` and `off_by`'s clearance each read a
  point of the arc as off (rows 7 and 8), then the level reads it receding
  (rows 4 and 5). Re-posing any one, or the first three together, leaves
  the miss.
- **The suspects that need a body were not reached** (rows 9, 10, 12, and
  `solid_contain::passes`): no body is built on a section large enough
  (task 3), and at the sizes that build the centre's rounding (1e-10 m at
  a = 1e6) is a hundredth of K ε. Row 11 refuses; it does not answer
  wrongly.

## The doors, the poses, the truth

`src/suspects_t1.rs` (an example of `geom-brep`). The ellipse is
`Ellipse::anchored` with the stored numbers `plane_cone_section` mints
(anchor `(1.3, −0.7, 0.4)`, generic axis and seam, phase `(1, 0)`: task 3's
built bodies carry exactly this form), `a` = 1e2 … 1e9, fat and thin.
Target points 0, 0.3 and −3 m of arc from the anchor; the ellipse's span
±10 m of arc about the target.

**The truth** (`oracle_t1.py`, 60-digit `mpmath`, no kernel code): each
curve as its stored numbers state it (a line `o + d·t`; a conic
`S + a·u·(cos t − k_c) + b·(axis × u)·(sin t − k_s)`), read back exactly
from the run's printed `f64`s. For `meet`, the least distance between the
two SPANS (Newton on the stationarity of the squared distance from grid
seeds and the pose's own hint, plus every end against the other span).
`g ≤ ε` MEETS, `g > Kε` APART, else GAP. Wrong: `Ok` with no meeting on
MEETS, a meeting on APART, or a reported meeting past K ε from a curve. A
definite answer on GAP is counted apart ("g"). A refusal is never wrong.
Existence is judged, not the count of meetings.

**Which reader decided** is read from the kernel's own log: a public
`geom_core::k_stats::Bracket` around each `meet`, the last decided row.

## By reader

Cells: right / refused / **wrong**, per semi-axis 1e2 … 1e9 (all the
family's poses of that truth, both shapes). Full tables:
`out/t1_families.txt`, per pose `out/t1_table.txt`.

| row | reader | door and pose | ε 1e-9 | ε 1e-12 | verdict |
|---|---|---|---|---|---|
| 1 (i) | `line_conic`'s GAP row | `meet`, a line on the major axis out through the anchor vertex, starting 300, 30, 2, 0 ε inside (8) | right or refused to 1e8; 1e9 2/0/**6** | to 1e4; 1e5 4/2/**2**; 1e6 … 1e9 2/0/**6** | **ANSWERS WRONGLY.** Last decision `curve_pair_pose_gap` in 4 (ε 1e-12); the router's `curve_pair_pose_parallel` (row 1 (ii), measured before) takes the other 28 first |
| 1 (iii) | `line_conic`'s crossing arm: the seed, `Polish.reach`, `Polish.clear` | `meet`, a line piercing the plane through the arc at 90°, 30°, 5°, 0.5° (24) | to 1e7 right or refused; 1e8 10/11/**3**; 1e9 2/5/**17** | to 1e4; 1e5 9/9/**6**; 1e6 0/1/**23**; 1e7 … 1e9 **17** each | **ANSWERS WRONGLY.** `curve_pair_pose_clear` Positive in 94 of 100 (`Polish::closest`, `clear − reach`); `curve_carrier_piece_clear_by` in 6 |
| 3 | `conic_conic`'s `constant_as(PARALLEL)` | `meet`, a unit circle in the plane crossing the arc, both orders, and one reaching 30 ε inside (18) | 18/0/0 to 1e8; 1e9 0/0/**18** | to 1e5; 1e6 0/0/**18**; 1e7 6/0/**12**; 1e8 3/0/**15**; 1e9 0/0/**18** | **ANSWERS WRONGLY.** All 81 end on `curve_pair_pose_parallel` |
| 4, 5 | `level_on`, `Level::jet_at` through `Window::read` | `meet`, a unit circle across the plane through the arc at 90°, 30°, 5°, both orders (36) | to 1e7 right or refused; 1e8 0/6/**30**; 1e9 0/0/**36** | to 1e4 right; 1e5 … 1e9 0/0/**36** | **The door answers wrongly; these two are one of four layers that certify it** (below). Not wrong alone on main: the layers above decide first |
| 6 | `coplanar_ellipses` | `meet`, a 2 × 1 centre-held ellipse in the plane crossing the arc, and one reaching 30 ε inside, both orders (24) | to 1e7 right or refused; 1e8 6/17/**1**; 1e9 0/4/**20** | to 1e4; 1e5 6/15/**3**; 1e6 0/0/**24**; 1e7 0/1/**23**; 1e8 4/4/**16**; 1e9 0/3/**21** | **ANSWERS WRONGLY.** `curve_pair_coplanar_sub_clear` Positive in 4; row 3's constant answers first in 98; `curve_carrier_clear` / `_gap` / `_piece` (row 8) in 6 |
| 7 | `Window::height_jet` in `holds_only` | the same circle-across poses | as rows 4, 5 | as rows 4, 5 | **The verdict is confirmed, the cause is not `height_jet` alone** (below) |
| 8 | `Curve3::clearance`'s deciders | `Window::off_by`: the circle-across poses. `Polish.clear`: row 1 (iii). `Foot::on_carrier`: a point 5 ε off the arc, along the normal and in the plane (12) | `on_carrier`: refused to 1e7; 1e8 0/2/**10 g**; 1e9 **12 g** | refused to 1e4; 1e5 … 1e9 **12 g** | **ANSWERS WRONGLY** through `on_carrier` (re-measured, both directions of offset, both shapes: a definite `Off` in the gap), through `Polish.clear` (row 1 (iii)), and `off_by` reads 4.5e-7 m of clearance for a point of the arc at a = 1e9. `span.rs::past_by`: never the last decision of a wrong answer; not isolated. `solid_contain::passes`: needs a body, not reached |
| 9, 10 | `section_loops::line_clears_conic`, `conics_clear` | `split` of a body whose section holds two loops | not reached | not reached | **COULD NOT REACH** at a size where it can go wrong: no body past 1e6 / 1e3 (task 3), and neither is asked in a plain cone-against-plane boolean |
| 11 | `props::curved::require_cone_ellipse` | `topo::finish` inside split and the booleans (task 3) | right to 6e5 … 1e6, then **refused** | right to 3e2 … 1e3, then **refused** | **REFUSES**: `VolumeUncomputable` on `props_cone_ellipse_on_cone` escalating (an enclosure `[0, 1.19e-9]` at a ≈ 8.6e5). Yes, it is the refusal `split` gives near 1e6. Past it `KernelLimit` refuses first, so a definite `OffSurface` or a wrong side was never reached |
| 12 | `replace_face::cone_v_range` | the offset door on a cone face bounded by a far section | not reached | not reached | **COULD NOT REACH**: needs such a body |
| 13 | `on_surface.rs::plane` | `OnSurface::edge(..).under(u_line, ..)`: a line in the edge's plane; 5 ε and 30 ε above it; piercing it at 30° (6 each) | in plane: right to 1e7, 1e8 1/5/0, 1e9 2/0/**4**. 30 ε above: right at every size | in plane: to 1e4, 1e5 1/5/0, 1e6 0/0/**6**, then **5, 5, 4**. 30 ε above: right to 1e6, then 3/0/**3**, 5/0/**1**, 4/0/**2** | **ANSWERS WRONGLY, both directions**: `Roots([])` for a line in the plane; `InPlane` for a line 30 ε above it (ε 1e-12, a ≥ 1e7). The piercing line's root stands 1.2e-7 m off the true crossing at 1e9 (past K ε from 1e8, resp. 1e5) |

The three measured before, on this sha and on the thin shape too:
`meet` with the in-plane line right or refused to 1e8 and wrong at 1e9
(ε 1e-9), wrong from 1e6 (ε 1e-12); the plane door `InPlane` to 1e7,
refused at 1e8, `Miss` at 1e9 (ε 1e-9), `Miss` from 1e7 (ε 1e-12), and a
definite `Miss` for a plane 5 ε off from 1e9, resp. 1e6; `on_carrier` as
in row 8. A plane 30 ε off its own is `Miss` at every size, rightly.

Also wrong, outside the table: two arc-held ellipses through one point
(message 5): in one plane on row 3's constant; across planes on
`curve_pair_window_off` (22) and on **`curve_pair_pose_root_gap`** (14,
plus 2 in one plane), a decision in neither the 22 suspects nor the 8
untraced, which I have not read.

## Row 7: what certifies the circle-across miss

One pose in full (fat, a = 1e9, ε 1e-9, the circle square to the arc
0.3 m from the anchor; true gap 1.9e-17 m; the circle is the conic read
across, its crossing at its parameter 0), with prints on a scratch copy of
main (`src/inst.patch`):

| what | main | the truth |
|---|---|---|
| the window handed to `holds_only` (`Range::band_arc`, from `height`) | `[8.7e-8, 1.14e-7]` rad of the unit circle | holds 0 |
| `height_jet` at the window's middle | 2.98e-8 m | −7.7e-8 m there (correctly off: the window is not where the crossing is) |
| `off_by`'s `clear` there | 4.45e-7 m | |

So on main `holds_only` certifies a window that truly is empty; the
window is displaced 1.0e-7 m, the centre's rounding along the normal. With
the plane stated through a point of the arc instead (intervention **c**:
`conic_conic`'s two `on` arguments), the window is `[−1.3e-8, 1.3e-8]`,
and then `height_jet` reads **5.96e-8 m at a point 1.7e-16 m from the
plane** and `clear` 4.48e-7 m: both certify it off, wrongly. With those
two re-posed as well (**a**: `height_jet`'s plane through a point of b's
arc; **b**: the clearance term dropped), `H` reads 1.7e-16 and the miss is
still certified, on `curve_pair_window_recedes` then
`curve_pair_window_off` (the level, rows 4 and 5), or on
`curve_pair_pose_clear`.

Circle-across poses that truly meet, both orders, 288 per ε: right /
refused / wrong under each intervention (`out/t1_fix.txt`):

| intervention | ε 1e-9 | ε 1e-12 |
|---|---|---|
| none (main) | 168 / 54 / 66 | 108 / 0 / 180 |
| a | 168 / 54 / 66 | 108 / 0 / 180 |
| b | 176 / 56 / 56 | 110 / 14 / 164 |
| a + b | 176 / 56 / 56 | 110 / 12 / 166 |
| c | 168 / 54 / 66 | 108 / 6 / 174 |
| a + b + c | 180 / 60 / 48 | 108 / 10 / 170 |

No single reader is the cause. (The level's `F = X²/A² + Y²/B² − 1`
loses u·a metres by its own form, whatever its centre; I did not re-pose
it.) The truly-apart poses stay right under every intervention (288 of
288).

## Thin and weak suspects where a pose was cheap

- **Rows 19 and 42** (`pcurve_cache::anchored`, `carrier_harmonic`):
  `chart_pcurve` then `PcurveCache::certify` of the ellipse on planes
  parallel to its own. On its own plane it certifies to 1e6 (thin) or 1e7
  (fat) at ε 1e-9 and to 1e3 at 1e-12, then **refuses**: `Escalated`, then
  `ResidualExceeded` from 1e9, resp. 1e6. On a plane 5 ε or 30 ε above it
  never certifies, at any size. **REFUSES; no wrong certificate.** (The
  image alone, `chart_pcurve`, is handed out for a plane 30 ε off from
  1e7, resp. 1e4; its docs say its incidence test is one-sided and the
  certificate measures the image, and the certificate does refuse.)
  `ResidualExceeded` for an ellipse that lies in the plane to 1e-16 is a
  false refusal, as the reading predicted.
- Rows 14 to 18 (blend surgery), 20 (`conic_torus_jet`), 21
  (`carrier_net::harmonic`; it is the net `KernelLimit` reads, task 3),
  22 (`one_stated_conic`): not run. 14 to 18 and 22 need a body with such
  an edge; 20 has no deciding door of its own.

## Limits

- `f64` only. One anchor and orientation (the earlier lane's), two shapes.
- Near the anchor only (within 3 m of arc). The earlier lane's far poses
  (θ = 0.5, 2) were not repeated.
- The window print and the interventions are on main plus a scratch patch;
  every count in "By reader" is clean main.

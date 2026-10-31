---
id: an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side
kind: issue
title: An aimed ray starting far past a wall loses its roots and reads a split's side inside out, on main
status: review
opened: 2026-10-26
priority: P1
cost: M
refs: [aimed-rays-start-at-a-conic-carriers-reach-where-the-ray-quadrics-lose-their-roots]
branch: flush/aimed-ray-reach
pr: PRNUM
---

**Live on main: a correct split is refused at its at-rest gate.**
An aimed ray's count is meant to read the side wherever its aim lands
(`crates/topo/src/side_reader.rs` `aim_at`: "an aim off the face only
steers it"), so every aim must read the right side or nothing. Some do
not, and a correctly oriented side of a split is then refused with
`NegativeVolume` (check 7 reads it inside out).

**The pose family.** A unit cylinder turned `φ` about its axis, split by
a plane through its seam vertex `v`, `ψ` off the wall's tangent plane
there, its origin `L` along the rim's tangent, either normal (the
construction of `sweep` `split_seam_vanishing_tilt`). Measured over 24
azimuths `φ = −3.1 + 6.2·i/23`, ψ ∈ {1e-7, 2e-7, 5e-7, 1e-6},
L ∈ {0, 0.1} and ±:

| ε | main `e9680c52` | PR 5781 @ `27ae1eea` | PR 5781 fix pass |
|---|---|---|---|
| 1e-9 | 65 of 384 | 73 | 58 (50 shared with main) |
| 1e-12 | 14 | 18 | 13 (11 shared) |
| 1e-6 | — | 0 | 0 |

The main column is FLUSH's review measure (R1) at 1e-9, re-taken on
`flush/aims-ties` with
`an_aimed_ray_from_far_reads_a_split_side_inside_out::the_seam_split_census_counts_its_inside_out_sides`.
Examples on main: φ −3.1 at ψ 2e-7, 0.135 at 5e-7, 2.022 at 2e-7,
±0.943 at 1e-7.

PR 5781's fix pass moves which poses meet this defect (R1's delta): 6
poses that build on main now refuse with `NegativeVolume`, 15 that
refuse on main now build, and the total goes from 65 to 58 of 384. That
is this defect's aim luck, refused rather than shipped. The census
above reads 50 of the 58 shared with main (8 newcomers, not 6);
R1's 6 and 15 against 65 would give 56, so the counts disagree by two,
unreconciled.

Further, a φ 5.5, ψ 1e-5 pose refuses at L 0, 0.02, 0.1 and 0.5, both
signs, on main and on `27ae1eea` (R2: check 7 walked a pass and read
`In`), and on PR 5781's merged head at 1e-9 (not at 1e-6 or 1e-12),
where the pin carries it. With the ruled aim moved to its edges' golden sections, φ 5.5
reads `Out` and builds while φ 2.2 at ψ 1e-7 reads `In`. Which poses
fail is aim luck, both ways; the aim does not cause the defect.

**The mechanism** (φ 2.2, ψ 1e-7, with the golden ruled aim):
- The wall's aim lands on `v`'s ruling.
- Its ray starts about `4·10⁷` m out. `beyond` and `selection_reach`
  (`solid_contain.rs`) take each curved edge's ball from
  `splitting/containment.rs` `carrier_ball`, which for the section is
  the whole ellipse's (semi-axis `≈ 10⁷`).
- At that start the wall's roots (`line_wall_roots`, a quadratic whose
  constant `c₂ = |w₀ₚ|² − r²` is read at the ray's start,
  `solid_contain.rs` ~2364–2397) land about `0.1` m off, and are read
  as boundary (count 0, sideless).
- The cut plane's crossing then counts `+1` first, and the ray reads
  `In`.

**Wider than ellipses** (R2): the amplification is the reach against a
wall's radius, for any selection whose reach is large. `c₂` loses
`ulp(reach²)` against `r²`, so a root moves by about
`ulp(reach²)/(2r·√disc)`. R2 computes about `5·10⁻⁷` m for a 1 km ×
1 mm rod, past the band, with no ellipse in sight.

**What a fix has to settle.**
- A reach that bounds the edges rather than their carriers (an arc's
  own extent, not its ellipse's semi-axes).
- A root solve whose rounding stays inside the band at the model's
  extent (`[verdicts]`: "a formula whose error is amplified by a
  vanishing quantity … is a defect"). For example, solve about the
  ray's nearest point to the axis rather than about its start, since
  the start lies `2·reach` out by construction.
- Both, probably. The pins below must then build or refuse with the
  ellipse's own rounding alone (`split_seam_vanishing_tilt`).

**Pinned as an open defect**: `sweep`
`an_aimed_ray_from_far_reads_a_split_side_inside_out`.
- `a_seam_split_a_hair_off_tangency_reads_a_side_inside_out` asserts
  that a family of the census's poses, wrong on main and on the PR's
  aims alike, still reads a side inside out: at 1e-9 the 13th azimuth
  at ψ 2e-7 and 5e-7, both offsets, either normal; at 1e-12 five poses.
  At 1e-6 nothing reproduces, and the row prints that it pins nothing.
- The census row is ignored and prints the counts.
- When this row lands, the family stops reproducing, the pin goes red,
  and the file is deleted with the fix.

The P1 stands: it is a live wrong orientation verdict at the at-rest
gate on ordinary input (a split a hair off tangency). It refuses rather
than shipping, so it stays P1, not P0.

## The same defect, filed earlier (CHART, 2026-10-24)

`aimed-rays-start-at-a-conic-carriers-reach-where-the-ray-quadrics-lose-their-roots`
(closed as this row's duplicate on 2026-10-31) found the defect first, while fixing PR 5664's
review. Its executed witness is the pose `ψ 1e-5, φ 0, L 0`, turned
about `[1, 0, 0]` by 0.7, from `sweep a_rigid_turn_keeps_a_slivers_volume`.
- **The aim:** the wall's ray, aimed at its boundary middles' mean
  height on the meridian of its farthest middle.
- **What the ray reads:**
  - the section plane at advance `399999.0000117438`, counted +1;
  - a "wall" root at `399999.0000214193` that does not lie on the
    cylinder (`ρ − 1 = −1.45e-5`).
- **The result:** the true entry is lost, the side reads `In`, and
  tier 3 refuses the honest body with `NegativeVolume`.

It names `line_sphere_roots` and `line_cone_quadratic` as built the same
way as `line_wall_roots`. A fix covers all three, and is pinned on this
pose too.

## Measured on main (`bbc9b8e3`, 2026-10-31)

The census (24 azimuths × ψ {1e-7, 2e-7, 5e-7, 1e-6} × L {0, 0.1} × ±),
each split read one of five ways: built (both sides, summing to `π`),
refused at rest on the ellipse's own rounding
(`split_seam_vanishing_tilt`'s `carrier_resolution`), refused with the
side unread (`VolumeSignUnresolved`), refused before it had sides, or
refused inside out (`NegativeVolume`):

| ε | built | carrier | unread | split refused | inside out |
|---|---|---|---|---|---|
| 1e-9 | 122 | 46 | 0 | 158 | **58** |
| 1e-12 | 0 | 125 | 0 | 246 | **13** |
| 1e-6 | 192 | 0 | 0 | 192 | 0 |

**The reconciled count.** Main reads 58 at 1e-9 and 13 at 1e-12 (PR
5781 has merged; the row's "main" column, 65 and 14, was `e9680c52`,
before it). Each pose below is `(i, ψ: L and sign)`, with
`φ = −3.1 + 6.2i/23`; "all four" is both offsets and both normals.

At 1e-9, the 58: (4, 5e-7: all four); (5, 2e-7: all four); (7, 2e-7:
all four); (8, 1e-7: 0+ 0.1+); (8, 2e-7: 0+ 0.1+); (9, 2e-7: 0+ 0−
0.1−); (9, 5e-7: 0+ 0.1+); (10, 2e-7: 0+ 0.1+); (10, 5e-7: 0+ 0−
0.1+); (11, 2e-7: all four); (12, 5e-7: all four); (13, 2e-7: all
four); (13, 5e-7: all four); (14, 2e-7: all four); (14, 5e-7: 0−
0.1+); (15, 1e-7: 0+ 0.1+); (15, 2e-7: 0+ 0.1+); (22, 2e-7: all
four); (22, 5e-7: 0− 0.1−).

At 1e-12, the 13: (4, 5e-7: 0+); (5, 2e-7: 0+); (8, 1e-7: 0+); (8,
2e-7: 0+); (9, 2e-7: 0+); (9, 5e-7: 0+); (11, 2e-7: 0+); (12, 5e-7: 0+
0.1+); (14, 2e-7: 0+); (15, 1e-7: 0+); (15, 2e-7: 0+); (22, 2e-7: 0+).

R1's "6 new, 15 fixed" against `e9680c52` is not re-measured here: it
needs a census on `e9680c52` itself, and PR 5781 has merged.

**The mechanism, confirmed.** With the three changes below each
switched off by a scratch toggle, the census reproduces main exactly
(58 inside out, pose for pose). With any one of them on, all 58 read
right.

## Fix

Three changes, each closing the defect alone, together by construction:

1. **The reach bounds the arc, not its carrier**
   (`splitting/containment.rs` `conic_arc_ball`, `ArcBalls`). Each
   conic edge carries two balls, its carrier's and its arc's own (about
   the arc's middle, to half its width times
   `carrier_speed::SemiAxes::speed_over`, the speed bound over the
   span). The loop's extent takes the nearer (`extent_from`), and
   `ray_passes_beside` the clearer. On the census's sides the aimed
   rays now start about 5 m out, not `8·10⁶` m.
2. **Each ray × quadric quadratic is read about the line's closest
   approach** (`boolean/solid_contain.rs` `Centred`): the wall
   (`line_wall_roots`, the shift held to the caller's run), the sphere
   (`line_sphere_roots`) and the cone (`line_cone_centred`, both the
   trimmed and the partial cone arm). `c` is read at the body's scale.
   The discriminant and the lead are the same quantities, so every
   margin is unchanged in meaning.
3. **The roots' rounding reaches the order** (`Centred::roots`,
   `PlaneCross::slack`, `RayHit::slack`). Each root carries a
   first-order bound, linear in the start's distance. Each plane
   crossing carries `rounding_charge(|o − q| + |t|)/|d·n̂|`. Each hit's
   span is widened by its slack, and `RayHit::pair` decides two hits'
   gap less both slacks. A gap the rounding can close is in band: the
   ray is set aside, never counted.

## Measured on the branch

| ε | built | carrier | unread | split refused | inside out |
|---|---|---|---|---|---|
| 1e-9 | 168 | 54 | 4 | 158 | 0 |
| 1e-12 | 0 | 138 | 0 | 246 | 0 |
| 1e-6 | 192 | 0 | 0 | 192 | 0 |

Pose for pose against main:
- at 1e-9, 50 inside-out poses build and 8 refuse on the ellipse's
  rounding;
- at 1e-12, all 13 refuse on the ellipse's rounding;
- 1e-6 is identical.

**Four poses move from built to unread**: `i` 1 (φ −2.830), ψ 5e-7,
both L and signs, at 1e-9. On main these four built only through the
defect. Face `1v1`'s aimed ray started `8·10⁶` m out and placed the
wall's root `3·10⁻⁴` m past the cap's crossing, so the cap's `−1`
counted first and read `Out`. From the start 5 m out, that root lands
`2.7·10⁻⁶` m before the cap's crossing, and the wall's region cannot
place it (sideless). The ray is set aside. Every aim on that sliver
side grazes the same way, and its volume (`−3.6e-10 ± 8.9e-10`) cannot
sign it, so the side refuses `VolumeSignUnresolved`. That is the bar's
"set aside, never counted". The region's sideless reading there is
`a-wall-root-bounded-by-an-oblique-section-still-walks-and-ties-at-the-nominal`'s
ground.

**The duplicate's witness** (ψ 1e-5, φ 0, L 0, turned about
`[1, 0, 0]` by 0.7) reads right on main with main's aim. The defect
there was the wall root `1.45e-5` off the cylinder from a start
`4·10⁵` m out. The unit row `a_ray_from_far_keeps_a_walls_roots` holds
that start against a unit wall: off by `9.2e-6` read about the start,
exact about the closest approach.

**R2's rod** (1 km × 1 mm) builds on main and on the branch: the root
error is real (`1.1e-7` m read about the start, the same row's second
case), but no aim of the rod's reaches it. The pin keeps the rod and
its near-square cut at every ε.

## Pins

`crates/sweep/tests/an_aimed_ray_from_far_reads_a_split_side_right.rs`
replaces the open-defect pin. Its family, the whole census and the rod
read no side inside out at any ε. Every built side holds `π`, and every
at-rest refusal is the ellipse's own rounding or an unread side. The
far-start root rows in `solid_contain.rs` `wall_root_rows` hold the
wall, sphere and cone roots exact from `2·10³` and `4·10⁵` m. The arc
ball's row is in `splitting/containment.rs` `arc_ball_rows`.

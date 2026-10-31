---
id: an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side
kind: issue
title: An aimed ray starting far past a wall loses its roots and reads a split's side inside out, on main
status: dispatched
opened: 2026-10-26
priority: P1
cost: M
refs: [aimed-rays-start-at-a-conic-carriers-reach-where-the-ray-quadrics-lose-their-roots]
branch: flush/aimed-ray-reach
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

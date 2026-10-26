---
id: a-cylinder-split-just-off-its-rim-vertex-exhausts-the-volume-quadrature-at-rest
kind: issue
title: "A cylinder split by a plane tilted 1e-2 just off its seam rim vertex leaves a sliver whose wall's volume quadrature stops at its floor over the target at 1e-12, so the split refuses at rest"
status: open
opened: 2026-10-14
priority: P2
cost: M
---


Found by CLEAVE's PR 4810 (`sweep` `section_end_poses`, the square
poses' lever check), after main `05e88d1294` began gating each split
side at rest (`ResultNotAtRest`).

## What

The unit cylinder (`z ∈ [0, 1]`, extruded) split by the plane through
`(1, 0, d)` with normal `(−1e-2, 0, 1)`: the plane passes `d` above the
seam's rim vertex and leaves a sliver wedge below it. The split refuses
`Finish(ResultNotAtRest { side: Below, errors: [VolumeUncomputable {
Face { QuadratureBudget { rounds: 13 } } }] })`, about 30 s per run:

| ε | d | width_len | target_len |
|---|---|---|---|
| 1e-9 | 5e-8 | 1.0e-4 | 1.024e-6 |
| 1e-12 | 5e-11 | 1.05e-4 | 1.024e-9 |
| 1e-12 | 1e-9 | 5.2e-6 | 1.024e-9 |

The same plane at `+1e-2`, and square, builds. Before the at-rest gate
the split returned these sides; whether they validated at rest then is
not measured here. PR 4810's earlier census met the same budget on a
slab turned `1e-2` along a cylinder's ruling at the Boolean, which no
longer reproduces there.

## Owed

Say whether this sliver face's quadrature should converge (the wedge
between the rim disc and a plane `1e-2` off it, meeting at a chord near
the seam vertex), and if it should not, give the refusal its one
ending. PR 4810's pin accepts exactly this refusal on that move and
names this row.

## Where it stands (flux/cylinder-sliver-exact-bracket)

**The ends' trig brackets.** The cylinder lane read each trim edge's end
`(cos t, sin t)` off the end's vertex, a bracket `ε` wide, and that width
rode every piece of the edge: on the wedge's wall, a few `d` high, it alone
was the floor. Each end is now read at the parameter the lane integrates
from (`geom_brep::props::quad::trig_at_limit`), rotated from the exact
`(1, 0)`: 1.7e-13 wide at `π`, 3.4e-13 at `2π`. The vertex is not read. At 1e-9 and 1e-6 the split builds at `d = 50ε, 500ε, 5000ε`, and the
halves hold `π` within their pads
(`crates/sweep/tests/a_cylinder_split_just_off_its_rim_vertex.rs`).

**At 1e-12 the wall is at its floor, and refuses as one.** It stops after
one round, in 0.02 s, with `QuadratureFloor` (the lane stopped at its
floor, not at the work wall):

| `d` | wall area | flux width | `width_len` | before |
|---|---|---|---|---|
| 5e-11 | 6.67e-15 m² | 9.35e-20 | 4.68e-6 | 1.05e-4, 13 rounds |
| 1e-9 | 5.96e-13 m² | 4.22e-19 | 2.36e-7 | 5.2e-6, 13 rounds |

against the `1.024e-9` this ε targets.

## The open shape

What is left is the meter, not the integrand. The flux width is the part the
rule's remainder does not account for: each edge's rule rounding and its pcurve map-residual pad,
each at an ulp of the wall's own terms (`O(r) = O(1)` levers, the pcurve
envelope `2.2e-16`), about `1e-19` in all. The mean boundary displacement
divides it by `3·area`, and this wall's area is `≈ (4/3)·d·√(d/5e-3)`, so
`width_len` grows as `d^{-3/2}` as the wedge thins, whatever ε asks. Reading
the wall's terms about a point of the face, so they round at its own size
rather than the radius, or metering a face's width against its body rather
than its own area, would move it; both are the meter's ratified ground,
left as they are here. The recourse the refusal names, loosening ε to about
`4.6e-9` at `d = 5e-11`, puts the vertex in band of the plane there, and
the split refuses the input instead (`SliverVertex`); the wedge itself
builds from about `d = 50ε` at 1e-9.

---
id: aimed-rays-start-at-a-conic-carriers-reach-where-the-ray-quadrics-lose-their-roots
kind: issue
title: The side reader's aimed rays start at a conic carrier's reach, up to 4e5 m out on a sliver, where the ray × quadric roots carry 1e-5 m of unmetered error and a ray reads the side at infinity as In
status: closed
priority: P1
cost: M
opened: 2026-10-24
closed: 2026-10-31
refs: [an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side]
---


Found by CHART while fixing PR 5664's review (`chart/ringed-curved-doors`).
A wrong answer, not a refusal.

## What

`side_reader::cast_aims` starts each aimed ray `2 · beyond(…)` from its
aim. `beyond` takes `selection_reach`, which folds each curved edge's
carrier ball (`splitting::containment::step_ball`). That ball is the
CARRIER's, not the arc's: an ellipse of semi-major `a` counts as a ball
of radius `a` about its centre. On `a_rigid_turn_keeps_a_slivers_volume`'s
sliver (the unit cylinder cut `ψ = 1e-5` off its tangent plane), the
section ellipse's semi-major is `1/sin ψ ≈ 1e5` m. Every aimed ray there
starts about `4e5` m out.

At that start, the ray × quadric quadratics lose their roots.
`line_wall_roots` (`boolean/solid_contain.rs`) forms
`c2 = |w0⊥|² − r²` with `|w0⊥| ≈ 4e5`, so `c2 ≈ 1.6e11` m² carries
about `1e-5` m² of rounding. The near root moves by about `1e-5` m, and
`disc = b2² − a2·c2` cancels two terms near `2.6e22`. Neither the roots
nor `bool_ray_cylinder_disc`'s margin meter that error. `line_sphere_roots`
and `line_cone_quadratic` are built the same way.

## Reproduction (executed)

The pose `ψ 1e-5, φ 0, L 0, about [1, 0, 0] by 0.7` from
`sweep` `a_rigid_turn_keeps_a_slivers_volume`. Aim the wall's ray at its
boundary middles' mean height on the meridian of its farthest middle.
That is one aim PR 5664's review fix tried. The ray reads two hits:

- the section plane (face `9v3`) at advance `399999.0000117438`, counted
  `+1` (leaving material);
- the wall's near root at `399999.0000214193`.

Unturned, the section-plane hit lies on the plane and inside the cylinder
(`ρ − 1 = −5.0e-6`). The "wall" root does NOT lie on the cylinder
(`ρ − 1 = −1.45e-5`). The true entry through the wall, before the plane,
is lost. The ray reads `Side(In)`, the shell reads inverted, and tier 3
refuses the honest body with `NegativeVolume`.

With the aim PR 5664 keeps (the boundary middles' mean, carried
radially, as on main), this pose's wall ray happens to decide first and
right. The defect is on main all the same: any aim whose ray crosses a
face near another, from a start this far out, can misorder them.

## Owed

- Start aimed rays at a reach that bounds the faces' boundaries, not
  their carriers. An arc's ball is its chord's midpoint plus half its
  span, as `chart_bound` and `region` already bound arcs.
- And/or solve each ray × quadric quadratic from the ray's closest
  approach to the surface's centre or axis, not from its start. That
  keeps `c2` at the body's own scale and moves the advance back by an
  exact `t0`.
- Then meter what is left: the roots' error at the start's distance,
  and the order decision between two crossings at that advance.

## Resolution

Closed as a duplicate (FLUSH orchestrator, 2026-10-31), and claimed
from INSIDE to close it. One defect, filed twice:
- CHART filed it here on 2026-10-24;
- FLUSH's lane filed `an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side`
  on 2026-10-26, without seeing this row.

Both rows describe the same failure: an aimed ray started at a conic
carrier's reach, about `1/sin ψ` out on a seam split near tangency,
where `line_wall_roots`' `c2 = |w0⊥|² − r²` loses the wall's roots.
Both propose the same two remedies: a reach bound by the arcs rather
than their carriers, and a root solve that keeps `c2` at the body's
own scale.

That row stays open, since it is the fuller record: both dual
reviewers' censuses, a pose-family pin
(`sweep an_aimed_ray_from_far_reads_a_split_side_inside_out`), and the
wider-reach argument. This row's executed pose (ψ 1e-5, φ 0, L 0,
turned about `[1, 0, 0]` by 0.7, with the aim at the boundary middles'
mean height) is carried there as a witness.

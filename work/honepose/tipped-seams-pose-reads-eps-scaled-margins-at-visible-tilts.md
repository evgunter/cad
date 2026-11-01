---
id: tipped-seams-pose-reads-eps-scaled-margins-at-visible-tilts
kind: issue
title: A rod tipped visibly off parallel escalates on margins that scale with the tolerance
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [an-in-band-axis-parallelism-offers-a-proximity-lever]
---

Found by PR 5061's review (NOTE-5) and measured on its branch
(`cleave/join-face-surface-one-home`).

## What

`parallel_cylinder_join::seams_on_the_walls(x̂, θ)` (unit rods, B about
`(1, 1)` over `z ∈ [−0.5, 0.5]` tipped `θ` about the `x` line through
`(1, 1, 0)`, against A about the origin over `z ∈ [−1, 1]`) escalates
at tilts the user can see, on margins of a few ε:

| θ | ε | refusal | predicate | margin |
|---|---|---|---|---|
| 1e-4 | 1e-9 | `ArcCylinderRoots` | `bool_conic_quadric_odd` | −8.28e-9 |
| 1e-3 | 1e-9, 1e-6, 1e-12 | `PierceCurvature` | | |
| 1e-2 | 1e-9 | `ArcCylinderRoots` | `bool_conic_quadric_split_lead` | 2.487e-9 |
| 1e-2 | 1e-6 | `ArcCylinderRoots` | `bool_conic_quadric_split_lead` | 4.093e-6 |
| 1e-2 | 1e-12 | `GermFrameUnsupported` | | |

The margins hold to 1e-17 under three rigid motions of the whole pose
(identity; 0.7 rad about `(0.3, 0.5, 0.8)` through `(0.3, −0.2, 0.1)`;
2.1 rad about `(−0.6, 0.2, 0.4)` through `(−1.1, 0.4, 2)`), so they are not
floating round-off of the placement. But the 1e-2 rad margin scales with
ε (2.5ε at 1e-9, 4.1ε at 1e-6), which a property of the input would not
do: it reads like a residual of a construction the kernel placed to
within the tolerance (a split point, a root), read back as the user's
margin. If so, PR 4335's rule applies: an amplified kernel error is
never offered as the user's lever. Not settled.

A real coincidence in the pose is a candidate too: B's seam rulings at
`(0, 1, z)` and `(2, 1, z)` meet the tilt line, so B's seam crosses A's
wall at `(0, 1, 0)` at a grazing angle `θ` for every θ, and A's seam at
`(1, 0, z)` crosses B's wall near `z = −θ/2`, also at angle `θ`.

## Direction

Read the raising sites (`conic_quadric`'s split lead and odd rows, the
pierce-curvature gate) for what their margin measures on this pose, and
whether it is the input's or a construction's residual. Settle it with
the same pose re-run at a tilt line that misses the seams.


## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: ε-scaled margins escalate at visible tilts: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

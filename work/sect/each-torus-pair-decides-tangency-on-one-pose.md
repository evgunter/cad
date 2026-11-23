---
id: each-torus-pair-decides-tangency-on-one-pose
kind: unit
title: Each torus pair decides tangency on one pose value: the critical gap, read by C5, the SSI doors and the section certificate
status: closed
opened: 2026-10-28
priority: P0
cost: H
branch: sect/torus-pose
refs: [torus-germ-pairs-have-no-section-frame, the-section-of-a-surface-pair-is-one-object]
closed: 2026-10-28
---


## The design (two designers, converged, 2026-10-28)

Blinding record `analysis/design-fork/torus-pair-tangency`. Both
designers reached one final state, an elaboration of fork row 100's
section door, so no `[ev]` fork.

- **One pose value per torus pair** in `geom_brep::pair_section`:
  `SphereTorusPose`, `CylinderTorusPose`, `TorusTorusPose`, on the
  `CylinderSpherePose` pattern. Each holds:
  - the operand guards (radii positive, the ring-torus convention);
  - a **symmetry reading** (coaxial, parallel, square, general), decided
    on the section certificate's existing lever and pivot margins. It
    chooses the closed-form components and the coincidence records; a
    coincident torus pair is refused as a coincidence before any gap;
  - the **critical gap**, the only place tangency is spelled.
- **The critical gap** comes from offset duality. Sphere, cylinder and
  torus are each the set of points at distance ρ from a spine (a point,
  a line, a core circle). Two of them are tangent exactly where a
  critical value D of the spine-to-spine distance equals ρ₁ + ρ₂ or
  |ρ₁ − ρ₂|. The margin is `min |D − (ρ₁ ± ρ₂)|`, in metres.
  - For cylinder × torus, D² is a degree-2 trigonometric polynomial of
    the core angle, so there are at most four critical values.
  - For torus × torus, D at each point of the first core circle is the
    sphere × torus formula in that point's meridian plane, swept over
    the first core angle.
  - `cs_near_reach`/`cs_far_reach` and `ssi_st_tangency` are instances
    of the same gap.
- **It is decided by a certified 1-D sweep**, never by root formulas.
  The core angle is cut into cells. A cell clears if its enclosure of
  `|D − level|` lies past the band, or if its enclosure of `D′`
  excludes zero.
  - Every cell clears → Positive (transverse).
  - A cell within the zero band where `D′` certifiably changes sign →
    Zero (tangent; a critical point exists, without isolating it).
  - Anything else at the floor → Escalated.
- **Readers.** `route_pose`'s torus arms, the SSI doors (`sphere_torus_ssi`
  now; `cylinder_torus_ssi`/`torus_torus_ssi` in the follow-on rows),
  which march only past a Positive gap, and `section_cert` through
  `of_pair_section`, whose tangency refusal becomes the gap's Zero or
  in-band verdict. The closed-form torus arms of `section_cert`
  (`torus_pair`, `torus_parallel_cylinder`, `torus_square_cylinder`,
  `torus_parallel_torus`, the coaxial rows) move into the value as the
  symmetric poses' components.
- **The tube-circle tangency must decide Zero.** Two tori with equal
  minor radii whose core circles touch are tangent along a whole tube
  circle; this is two pipe bends joined G1 in different planes. A
  cylinder with `r_c = r` whose axis touches the core (a straight pipe
  entering a bend) is the same case. The 1-D spine sweep reads it as
  a critical value at level 0 = |ρ₁ − ρ₂|. Its rows are owed (one
  designer found that both P0 rows miss it).
- **Not decided here:** moving the torus pairs' rung 3 onto the torus
  chart (2-D accounting instead of the ℝ³ shell). Both designers named
  it as possibly the better rung 3, and as a separate question.

## Scope of this unit

- The three poses with their guards, symmetry readings and critical
  gaps.
- `pair_section` building the torus pairs: closed-form components for
  the symmetric poses, and the general carrier (marched by `carry`
  through the existing sphere × torus door) for a general sphere ×
  torus pose.
- `sphere_torus_ssi` reading the decided pose, which closes sphere ×
  torus's tangency being decided twice (`ssi_st_tangency` against
  `section_cert::torus_sphere`).
- `section_cert`'s torus arms reading the value.
- `route_pose`'s torus arms reading the poses.

Left to the two follow-on rows (parked on this one):
`cylinder-torus-rung-three-arm-is-unretired` and
`torus-torus-rung-three-arm-is-unretired`, which build the two SSI doors
over a Positive gap and flip `route`.

Review: **dual (concurrent)**. It is a new decision every torus row,
the cone pairs and NURBS × analytic build on.

## Closed

2026-10-28, by `sect/torus-pose`. `geom_brep::pair_section` builds the
three torus pairs on `SphereTorusPose`, `CylinderTorusPose` and
`TorusTorusPose`, each decided once: the guards, the symmetry reading
(`axis_pose` and the square reading, moved from `section_cert` and read
over the torus's own ball), a coincident torus pair refused, and the
critical gap: for a sphere its centre's two meridian distances from the
core (its critical values in closed form, the formula `ssi_st_tangency`
read, now one reading), for a cylinder or a torus a certified sweep of
the core angle on `D` and `D²` (`pair_section::gap`), two tori's on
either core so the verdict does not depend on their order; never a root
formula. Each of the sweep's margins is a length at its own scale less
its rounding, a slope read as the offset's component along the core. The closed-form components moved
from `section_cert`'s torus arms (`pair_section::torus_arms`); a
general sphere × torus pose carries through `march_sphere_torus`; a
general cylinder × torus or torus × torus pose answers
`RoutesToGeneralRung`, and their symmetric poses' components carry
`ClassCarrier::Unbuilt` naming the door rows. `route_pose`'s torus
arms, `sphere_torus_ssi` (its `ssi_st_tangency` retired) and
`section_cert` read the poses; `glue::section_door` keeps each
coincidence at its own site. Off the band every measured certificate
verdict stands; tangencies are named by the gap.


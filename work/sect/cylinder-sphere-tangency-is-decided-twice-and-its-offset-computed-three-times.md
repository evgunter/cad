---
id: cylinder-sphere-tangency-is-decided-twice-and-its-offset-computed-three-times
kind: issue
title: The cylinder x sphere walls' tangency is decided at two sites under two names and two radius conventions, and the axis-to-centre offset is computed three times
status: open
opened: 2026-10-04
priority: P1
cost: M
refs: [4025]
---

Found by the single FULL review of PR 4025 (its style finding Q1, with
the Q1/Q7 radius-sign note), 2026-10-04.

## Measured

- **One tangency, two decisions.** `boolean::join::cs_transverse_frame`
  decides `bool_germ_frame_cs_reach` on `|R| − |r| − d` (the walls
  touching at the far side, the figure-eight's node), and
  `geom_brep::ssi::cylinder_sphere_ssi` decides `ssi_cs_tangency` on
  `min(||d − r| − R|, |d + r − R|)`, whose second term is the same
  condition on signed radii. Neither cites the other.
- **One offset, three spellings.** The axis-to-centre distance
  `‖q − a·(q·a)‖`, `q = c − o`, is computed in `cs_transverse_frame`, in
  `cylinder_sphere_ssi` and in `geom_brep::intersect::cylinder_sphere_section`
  (`cs_declared_coaxial`).
- **Two radius conventions on one path.** The transverse frame reads
  `|r|` and `|R|` (a negative stored radius denotes the same point set);
  the cylinder-pair arm of `pair_section_frame` levers
  `bool_germ_frame_axes_parallel` on the signed `r1.max(r2)`;
  `cylinder_sphere_section` refuses a non-positive radius as
  `DegenerateOperand`. A negative radius therefore reads three ways in
  one dispatch.
- The same class elsewhere: `bool_germ_frame_axes_coplanar` restates
  `geom_brep::cylinder_cylinder_section`'s `cc_axes_coplanar`.

## What a fix has to supply

One home for the cylinder × sphere pose quantities (the offset, the
reach past each side) that the frame, the SSI door and the section table
read, so their verdicts agree by construction; and one ruling on the
radius sign at the frame dispatch (magnitude, or refuse as the section
table does), applied to every arm.

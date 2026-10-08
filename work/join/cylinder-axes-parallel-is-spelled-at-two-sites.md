---
id: cylinder-axes-parallel-is-spelled-at-two-sites
kind: issue
title: Whether two cylinder axes are parallel is decided at two sites with two names (bool_germ_frame_axes_parallel, cc_axes_parallel)
status: closed
opened: 2026-10-04
priority: P4
cost: E
refs: [parallel-cylinder-germ-pair-has-no-join-arm]
closed: 2026-10-06
---


Found by PR 4031's review (S2), on the parallel cylinder arm.

## What

One fact, two deciding sites:

- `boolean::join::pair_section_frame`, cylinder pair:
  `bool_germ_frame_axes_parallel`, `‖a₁ × a₂‖` levered by the larger of
  the radii and the germ walls' reach (PR 4031 added the reach; before
  it the lever was the radius alone, and a rod tipped 2e-10 over 100 of
  wall reached the rulings arm and failed at the pcurve pass);
- `geom_brep::intersect::cylinder_axes_parallel` (`cc_axes_parallel`),
  levered by the extent its caller hands it: `cylinder_cylinder_section`
  (handed the frame's lever by `intersecting_cylinder_axes`, so the two
  agree there by construction) and the tangent-locus lane (handed the
  reach's lever from the partner's axis foot).

The levers now agree in meaning (the length the axes run together), but
the two names and two margins are a second spelling that can drift
again.

## Also looked at

`pc_rim_alignment` (`geom_brep::plane_cylinder_section`) levers a plane's
tilt against a cylinder axis by the rim radius: the reach of the rim
circle it decides, which is the right length for that question. The
cylinder × sphere frame (`cs_germ_frame`, `cs_transverse_frame`) decides
no axis parallelism. Neither is a duplicate of this one.

## The shape of a fix

Export `cylinder_axes_parallel` from geom-brep (it is `pub(crate)`) and
have the frame call it with its lever, under one predicate name; the
audit row and the census follow the name.

## Review tier

SINGLE, FULL: rides PR 4118's fix pass, whose full review asked for it
(Q2); the second Opus review of that unit covers it.

## Closed

Closed by TANG's `classifiers-read-at-the-reach` fix pass (PR 4118).
`geom_brep::cylinder_axes_parallel` is exported and takes the reach and
the two axes; it levers the sine by `ExtentBall::lever_between` under
the one name `cc_axes_parallel`. The section table's step 3, the
tangent-locus lane and `pair_section_frame`'s cylinder pair all ask it,
so `bool_germ_frame_axes_parallel` is retired: its audit row is
re-pointed, and the sweep interval guard that refused that name now
refuses `cc_axes_parallel`.

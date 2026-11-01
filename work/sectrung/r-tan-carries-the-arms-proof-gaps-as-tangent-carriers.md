---
id: r-tan-carries-the-arms-proof-gaps-as-tangent-carriers
kind: issue
title: R-tan's sentence calls every Section::Tangent a tangency of the carriers, though several section arms raise it for a gap in their own reading
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by SECT's `sect-bisect` lane, fixing the two-tori subtract main red
(`sect/two-tori-subtract-red`), 2026-10-28.

## What

`boolean/section_cert.rs` `Refusal::what` gives every `Refusal::Tangent`
one sentence: "a curved face and a face of the other solid have tangent
or near-tangent carriers". `certify` turns every `Section::Tangent(name)`
and every unplaced `Section::Touch` into that refusal, whatever row
raised it. The predicate name is dropped there too: `FallbackExtentUnsupported`
carries only the `what` sentence, so the raising row has to be found by
instrumenting the code.

The parallel-tori arm's level-end rows (`torus_parallel_torus`,
`section_torus_parallel_tori_top`, `_bottom`, `_slabs`) were one case
of a refusal raised for the arm's own reading. The donut against its
copy offset `4.9` cut one oval at 23° to 52° between the carriers, and
was refused with the tangency sentence. That arm now reads past a node
that clears its triangle, and its remaining refusals are tangencies.
The other arms were not measured. These rows look like gaps in an arm's
own reading rather than distances between the carriers:

| site | row | why it may not be a tangency |
|---|---|---|
| `square_folds` | `section_torus_square_wall_fold_drift` | a numerical drift bound on the fold's station |
| `across_strip` | `SQUARE_WALK` (`_ => Err(Section::Tangent(SQUARE_WALK))`) | an unexpected pattern of crossings in the walk |
| `cylinder_cylinder` | `section_cylinder_axes_tilt` | the axes' tilt undecided, a pose not a contact |
| `cone_cylinder` | `section_cone_cylinder_aperture` | a generator parallel to the cylinder axis, a pose |
| `cone_plane` | `section_cone_plane_aperture`, `_facing`, `_meridian` | pose readings of the plane against the cone |
| `cone_sphere` | `section_cone_sphere_axis` | a pose reading |
| `axis_pose` callers | `Pose::Undecided(name) => Section::Tangent(name)` | the offset reads neither coaxial nor parallel |

## What the taker owes

Measure each row: does a pose that pinches it have carriers within the
band of each other? Where it does not, give the refusal a typed reason
that names the arm's gap (for example an R-tan sibling whose sentence
says the arm could not read the pose), and carry the predicate name in
the payload so the raising row is visible without instrumenting.

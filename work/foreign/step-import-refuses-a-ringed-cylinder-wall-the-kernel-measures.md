---
id: step-import-refuses-a-ringed-cylinder-wall-the-kernel-measures
kind: issue
title: STEP import refuses a ringed cylinder wall that the kernel builds and measures
status: open
opened: 2026-10-10
priority: P3
cost: M
---


## What

`step-import`'s face gate refuses every multi-bound curved face that is
not a seamless periodic band (`crates/step-import/src/entities.rs`,
`face`, the "A ring on a curved face (M7-4)" block and the gate under
"A multi-bound curved face refuses HERE"). Its stated reason is that
`topo::mass_properties` has no construction for a curved face with
rings. That has not held for cylinder walls since the chart Green form
(`geom_brep::props::curved_face_loops`, rings bounded by rims and
rulings), and since `flux/ellipse-ring` the cylinder quadrature reads
rings trimmed by ellipse arcs too (`geom_brep::props::quad::cylinder_cut_face`).
The same comment's claim is repeated in `crates/step-import/src/lib.rs`
(the seamless-band variant's docs) and `crates/step-import/src/normalize.rs`
(module docs, "The seamless band").

So a body the kernel builds, validates at tier 3 and measures — the unit
pipe ∪ a brick through its wall (`verbs_germarms::a_wall_ringed_by_ellipse_arcs_measures`,
`verbs_germarms::a_bar_whose_section_closes_inside_one_wall_face_builds`)
— cannot be read back from a STEP file stating its wall with the hole as
a bound, which is how another system writes it. Not measured here: no
round trip of such a body was run.

## The shape of a fix

Admit a multi-bound face on a cylinder (the kinds whose `RingOnCurvedFace`
gate in `topo::props::face_flux` passes), keeping the refusal for the
rest, with the outerness inference the planar path already runs; the
prose in the three files then states what the gate admits.

---
id: step-import-refuses-a-ring-the-kernel-measures
kind: issue
title: step import refuses every ring on a curved face, though the kernel measures one on a cone wall and on a rim-and-ruling cylinder or torus wall
status: open
opened: 2026-10-10
priority: P3
cost: M
---

Filed by GERM's PR 4484 fix pass, which corrected the prose that gave
the old reason.

`step-import`'s face gate (`entities.rs`, the multi-bound curved-face
arm of the face reader, after `is_periodic_band`) refuses every curved
`ADVANCED_FACE` with more than one bound that is not a seamless
periodic band, as `StepImportError::Topology`. Its stated reason was
that the kernel has no volume construction for a curved face with
rings. That is no longer so: `topo::props::face_flux` reads a ring on
a cone wall, and on a cylinder or torus wall whose edges are lines and
circles (`geom_brep::props::curved_face_loops`), and refuses only the
rest (`MassPropsError::RingOnCurvedFace`).

So a file carrying, say, a cone wall with a box's footprint as a hole
(a ring of ellipse arcs) refuses at import, although the kernel holds
and measures the same body built through its own union
(`sweep`'s `reach_cone_root_lane` rows). What the
importer needs before adopting such a face is its own business, and
not settled here: the outer bound's inference on a curved chart
(`chart::infer_outer`, already reached for a band's tie), and tier 3
at rest on the adopted body, which refuses what props cannot read.


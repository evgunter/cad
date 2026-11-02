---
id: germ-cylinder-pair-span-misses-a-curved-edges-bulge
kind: issue
title: the germ frame's cylinder-pair span is its walls' vertex ball, which a curved rim's bulge reaches past
status: closed
opened: 2026-10-07
priority: P3
cost: E
closed: 2026-11-02
---


Found by the sweep of the measured-levers lane
(`tang/measured-levers-reach-the-region`), which fixed the
plane×cylinder pair's lever beside it.

## What

`boolean::join::frame_reading` (`crates/topo/src/boolean/join.rs`)
measures the cylinder pair's `span` as the diameter of the ball
enclosing both walls' boundary VERTICES (`face_witnesses`), and
`pair_section_frame`'s cylinder arm levers `cc_axes_parallel` at the
longer of that span and the larger radius. A curved boundary edge can
reach past every vertex: an obliquely trimmed wall whose ellipse rim
carries one seam vertex reaches `2·r·tan φ` along the axis from it
(`test_support_fixtures::oblique_rim_wall`). Two such walls whose seam
vertices stand close read a span shorter than their rims' reach, down
to the radius, so a tilt the walls' real length reads in the band can
read Zero and name the rulings' straight
frame (`Ok(None)`) for walls whose axes drift apart across them. An
under-stated lever is the wrong-answer direction.

The definite side of `cc_axes_parallel` refuses there (skew keeps
`NoArm`, meeting axes take the pinch door), so a lever past the walls'
extent only escalates more; the under-statement is the defect.

## The shape of a fix

Measure each wall's farthest distance along its own axis from `at`
over its boundary spans, `splitting::rules::face_axial_range`, which
reads a conic arc over the span it holds
(`geom_brep::Reach::range_along`), and lever the pair at the
longer of the two (the axes drift apart by the sine times that), never
at a whole turn of an arc the face does not hold. A spiric or spline
edge is read at its torus's support or whole control net there, which
`spiric-and-spline-axial-levers-read-past-the-span` covers.

## Closed (2026-11-02, SECTARMS stage 3, branch `sectarms/cylinder-join-reads-value`)

The subject retires. The join no longer reads a cylinder pair through a
frame: it reads the pair's pose on the section value (`join::cylinder_read`).
That pose levers `cc_axes_parallel` across the operation's region, the
ball about the overlap of the two operands' boxes (C5). The region holds
every section point the operation can use, a curved rim's bulge
included, so no face's vertex ball is read.

Also removed, since they now have no producer:
- `frame_reading`'s cylinder-pair span;
- `FrameExtent::Span`;
- `pair_section_frame`'s cylinder arm.

The plane × cylinder pair's own lever (`face_axial_range`) is unchanged.

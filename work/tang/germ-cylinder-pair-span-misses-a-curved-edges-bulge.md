---
id: germ-cylinder-pair-span-misses-a-curved-edges-bulge
kind: issue
title: the germ frame's cylinder-pair span is its walls' vertex ball, which a curved rim's bulge reaches past
status: open
opened: 2026-10-07
priority: P3
cost: E
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

Reach the curved edges as the plane×cylinder pair now does: each
wall's farthest axial distance from `at`
(`splitting::rules::face_axial_extent`, one `geom_brep::Reach::Span`
per boundary edge), along each axis, or the curved edges' Euclidean
reach (`splitting::rules::edge_reach`) folded into the vertex ball.

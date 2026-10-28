---
id: a-ring-about-a-tilted-axis-slides-each-member-along-it
kind: issue
title: pncad's circular_pattern pins a ring's free slide along a target axis, so about an axis no target axis runs along the members drift along it
status: open
opened: 2026-10-28
---


Found building G (`patterns-are-index-variables`), by the tilted-axis
probes `mate1_r1_probes::r1_oblique_circular_axis_with_a_non_identity_group_frame`
and `mate1r2_probes::r2_oblique_circular_seat_at_a_pinned_world_frame`.

`circular_pattern` (`crates/pncad/src/pattern.rs:111`) writes a ring as
one `Axis` mate, a spin `scalar(k)·step` and a slide of zero along the
target axis `along` (the viewer picks it with `ring_slide_axis`,
`crates/viewer/src/combine.rs:850`; pncad-py's `Node.circular_pattern`
takes it as an argument). The mate's chart measures a slide as the
copy's origin along that target axis as the least turn carries it
(`crates/editor-core/src/mate/value.rs`, module docs). Where the ring's
axis runs along that target axis, the component is unchanged by a turn
about the axis, so each member is the last turned by the step. About
any other axis it is not: each member's origin, turned about the axis,
has a different component along the target axis, and the slide moves
the member along the ring's axis to hold it at zero. The members then
stand on a helix, not a ring, and nothing refuses. The function's docs
say the axis must run along `along`, and nothing checks it.

The two probes now build their rings with a second, `Point` mate at the
axis frame's origin (`fixture::circular_pivot`): the bundle leaves the
spin alone, the solve's `Revolute` subgroup, and each member is the last
turned about the axis's line.

## Repair shape

The façade writes a ring's slide so it holds whatever the axis: a
`Point` mate on the axis, as the fixture does, or a slide measured
along the mated direction. Either changes what `circular_pattern`,
`bolt_circle`, the viewer's ring and pncad-py's `Node.circular_pattern`
take (the point, or no `along`). Until then, an axis no target axis
runs along is refused at the façade, not built as a helix.

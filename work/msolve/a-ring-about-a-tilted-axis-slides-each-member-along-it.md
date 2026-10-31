---
id: a-ring-about-a-tilted-axis-slides-each-member-along-it
kind: issue
title: pncad's circular_pattern pins a ring's free slide along a target axis, so about an axis no target axis runs along the members drift along it
status: open
opened: 2026-10-28
priority: P1
cost: M
---


Found building G (`patterns-are-index-variables`), by the tilted-axis
probes `mate1_r1_probes::r1_off_origin_circular_axis_with_a_non_identity_group_frame`
and `mate1r2_probes::r2_off_origin_circular_seat_at_a_pinned_world_frame`.

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

A second, `Point` mate at the axis frame's origin would leave the spin
alone (the `Revolute` subgroup), but the solve refuses an `Axis` and a
`Point` mate together as `Overconstrained` (`free: 0`): the bundle rule
counts the freedom they share twice. So no bundle in today's vocabulary
holds a ring about such an axis, short of a slide written per member.
The two probes, which met it with an axis tilted off every target axis,
now ring an axis along z that misses the origin, which keeps their
composition-order attack.

## Repair shape

The vocabulary says a ring about any axis: a slide measured along the
mated direction (the chart's own), or an `Axis` and a `Point` mate
admitted together where the point lies on the axis. Either changes what `circular_pattern`,
`bolt_circle`, the viewer's ring and pncad-py's `Node.circular_pattern`
take (the point, or no `along`). Until then, an axis no target axis
runs along is refused at the façade, not built as a helix.

The same writer fixes the slide at zero, so a ring that stands off along
its axis cannot be written by `circular_pattern` or `bolt_circle` at all:
the tour's impeller (`demos/tour/src/impeller.rs`) writes its blade ring
by hand, slid `BLADE_Z0` up the axis, and `recognise` then does not read
it back as `Circular`. Whatever the repair says of the slide's
direction, the writer takes its length.

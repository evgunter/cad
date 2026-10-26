---
id: sphere-trim-pole-interior-reads-a-collapsing-cosine-window
kind: issue
title: sphere_chart_trim asks whether a pole lies inside a meridian edge on the cosine-window margin, which reads Zero for a pole definitely inside a narrow edge
status: open
opened: 2026-10-25
priority: P3
cost: M
---



Found by PRED's D46 dimension verdicts (the audit's F26,
`docs/predicate-dimension-audit.md`).

## What

`crates/topo/src/boolean/solid_contain.rs` `sphere_chart_trim`, on each
boundary meridian edge of a sphere face whose turn is `Short`, asks
whether a pole lies strictly inside the edge under
`bool_sphere_trim_pole_interior`:
`Margin::levered(pole·m̂ − cos(w/2), R)`, `m̂` the edge's mid-direction
and `w` its span. That is the cosine-window construction the audit's F8
retired elsewhere. For a pole an angle `δ` inside an end the margin is
about `R·sin(w/2)·δ`, where the point deviation is the arc `R·δ`. On a
narrow edge (`w/2 ≲ √(Kε/R)`, about 1e-4 at `R = 1` and the default
band) every interior pole reads Zero, and so does one near the ends of
an edge just short of a whole turn.

`Zero` is taken as "at an end, or outside". A pole the edge holds would
then be missed: the latitude fold reads only the boundary levels, and
the face's window would stop short of the pole by up to `R·w/2`, so a
point of the face nearer the pole than that reads outside the window.

## Reachability

Unverified by a run. By reading, `chord_join::face_azimuth_window`
refuses a meridian edge through a pole (it has no closed-form chart
image across the pole), which sends the face out of the class first, so
the misread may be masked today. The margin is wrong whether or not a
later step masks it.

## What would close it

Decide the membership on the signed chord to the nearer end, as
`geom_brep::props::curved`'s `span_membership` does for the same
question on the flux lane (`props_meridian_pole`): it has the sign of
the cosine test and the size of the point deviation. Pin it with a
narrow meridian edge through a pole, probed at a point of the face
nearer the pole than its ends.

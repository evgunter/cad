---
id: solid-exhaustion-in-band-reading-offers-a-declaration
kind: issue
title: point_in_solid's refusal on a ray's in-band reading, when no ray decides, renders as the pre-pass's Escalated and offers declaring the coincidence
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by `cleave/ray-walk` (the ray-walk driver unit), whose driver
reports the first in-band reading a ray was set aside on when no ray
of the schedule decides (`ray_walk::walk`, rule 2).

`point_in_solid`'s readers carry that reading as the reader's own
in-band variant, so the solid door reports it as
`PointInSolidError::Escalated { face, diag }` — the variant its boundary
pre-pass raises for `q` itself in band of a face. Its `Display` says
"one of its faces is too close to call at this tolerance … Recourse:
declare the coincidence, or move the geometry"
(`crates/topo/src/boolean/solid_contain.rs`, `impl Display`).

For the pre-pass that is one story. For a ray's reading it is not: the
decision refused is where one test ray meets the boundary, which nobody
built, so no declaration reaches it, and "one of its faces is too close
to call" names the wrong thing. D4 ¶1 (i) also wants the in-band arm of
that decision and its `Zero` arm (`RayExhausted`, every ray grazed) to
tell one story, with the margin riding as data; today they are two
variants with two texts.

The likely shape is an exhaustion variant carrying the first in-band
reading as evidence (`RayExhausted { first_in_band }`), rendered
through `ray_walk::RaysGrazed` with the reading's valued tolerance arm
where it gives one. That moves the refusal downstream readers see from
`Escalated` to `RayExhausted` (`census::Undecided::of_point_in_solid`,
`contain`'s `ContainError` routing, `editor_core`'s attribution), which
is why it was left out of the driver unit. Under the D10 hold the fix
only drops a recourse that cannot apply; it builds nothing on declared
contact.

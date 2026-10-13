---
id: an-in-band-gate-masks-a-definite-one-in-the-spiric-and-villarceau-ladders
kind: issue
title: Two pcurve gate ladders escalate on an in-band gate before a later gate decides the class
status: open
opened: 2026-10-13
priority: P3
---



## What

Two gate ladders in `crates/geom-brep/src/pcurve_cache.rs` decide "every
gate Zero" one gate at a time and return on the first in-band one through
`?`. So a gate read later that would decide the class definitely never
speaks, and the ladder escalates where it has an answer:

- the spiric chart's other-torus test (the loop over `gates` chained with
  `pcurve_spiric_chart_other`, `.map_err(winding_escalated)?`, which breaks
  on the first non-Zero gate);
- the Villarceau incidence gates (`pcurve_torus_villarceau_*`,
  `.map_err(incidence_escalated)?`, where any definite gate is `grazes`).

The identity ladder's rule is: any definite datum answers; an in-band one
is kept, and refuses only where no later datum decides. That rule now has
one home, `topo::boolean::carrier_eq::identity_data` (PR 4918, after
`rim_wedge::same_circle` refused whole ops on circles 1.4 m apart).
geom-brep cannot call into topo, so the rule needs a geom-brep home, or
these ladders spell it out.

Found by PR 4918's second-review sweep (2026-10-13). Not measured.

---
id: contact
kind: program
title: CONTACT — touches, overlaps and declared contacts: the ordinary solids the boolean lane will not combine
status: ready
opened: 2026-09-20
area: kernel
prefix: contact/
tag: (CONTACT orchestrator)
ab_band: 7100-7199
paths: [crates/topo/src/boolean/contact_verify.rs, crates/topo/src/boolean/contain.rs, crates/topo/src/boolean/solid_contain.rs, crates/topo/src/census.rs]
keep_out: [opened by REACH's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - REACH measured 151 budget points, about five sittings, and was cut into six tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on at once), the six are REACH GERM CONTACT ZIP BOXES PIN and they SHARE crates/topo/src/boolean/* by design - shared ground is legitimate per the README's 2026-09-20 rule and what is owed is awareness while a lane is live, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, TANG shares chord_join.rs and boolean/rest.rs for the declared-tangency and germ/pierce lanes, CHART owns the pcurve and SSI side, CURVED keeps offset_axial.rs]
priority: P0
---
Where two ordinary solids MEET, rather than what shape they are.
`VertexVertex`, `VertexOnEdge`, `EdgeEdgeOverlap` and `ConformalPatch`
touches between two solids block the material test outright because
none of them carries a local side analysis. Two half-overlapping cubes
whose boundaries meet only in touches clear the census gate and fail
later. A box lap whose plane CONTAINS the cylinder axis trips the
all-planar join lane's conic guard.

None of these is an exotic pose: they are what happens when a person
puts two boxes together. That is the whole argument for the band.

Charter and order: `work/contact/plan.md`; narrative in `work/contact/log.md`.

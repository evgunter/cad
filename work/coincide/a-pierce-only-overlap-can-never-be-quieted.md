---
id: a-pierce-only-overlap-can-never-be-quieted
kind: issue
title: a pierce-only overlap can never be quieted: the containment arm's solid-pair crossing names no faces
status: open
opened: 2026-10-10
priority: P1
cost: M
---


Found by stage 5 B (`interference-at-rest-is-a-finding`), by the
orchestrator's ruling there. The at-rest gate's quieting rule
(`crates/editor-core/src/checks/at_rest.rs`, `localize`'s `covered`)
quiets a finding only when every `CensusUndecidable` the census left
on the copy pair is a face pair whose two faces bound the quieted
component. Those are the verdicts its containment check covered.

An overlap the census decides only by pierces (no vertex of either
copy inside the other: a bar through a plate,
`intent_s5_b_interference::a_bar_through_a_plate_with_no_vertex_inside_is_one_finding`)
also carries the containment arm's
`CensusUndecidable { a: Solid, b: Solid }`, "another finding reports
their boundaries crossing" (`topo/src/census.rs`, the instance
containment arm). That verdict names solids, not cells, so nothing
covers it, and such a finding stays loud under any assertion.

**First step.** Have the containment arm say which crossing left it
undecided: the pierce or crossing events that stand on the pair, by
their faces, as a typed field on the verdict. A solid-pair verdict
whose standing crossings all lie on faces bounding the quieted
component is then covered like a face pair. The alternative, counting
the verdict as covered when the pair has exactly one component, needs
a ruling: it lets a crossing outside the overlap ride a quiet finding.

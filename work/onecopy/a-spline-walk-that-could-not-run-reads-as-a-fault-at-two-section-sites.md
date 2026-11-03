---
id: a-spline-walk-that-could-not-run-reads-as-a-fault-at-two-section-sites
kind: issue
title: pair_section::reads and chord_join's carried_chord_spec read a spline walk that could not run (geom::Unwalked) as a kernel fault
status: open
opened: 2026-11-03
priority: P2
cost: E
refs: [carrier-refusal-to-error-mapping-is-spelled-at-seven-sites]
---

Found while putting the escalation-vs-walk sort on `geom::CarrierRefusal::escalation`.

Two consumers read a `geom::Unwalked` (a spline's piece walk past its budget, or a spline with no speed bound) as a kernel fault:
- `geom_brep::pair_section` `CarriedSection::reads`: `SiteRefusal::Desync("a point's distance from a fitted component could not be read")`;
- `topo::chord_join` `carried_chord_spec`'s `foot`: `SplitJoinError::SectionInvariant("a carried chord end's distance from its component could not be read")`.

Both read a carried component that may be a spline (`chord_join` branches on `spline.is_some()`), and `Unwalked::Exhausted` is a budget limit, which the meeting reader types as `MeetFault::Unsettled` and the census as unbuilt, not a fault. Neither `SiteRefusal` nor `SplitJoinError` has an arm for a reading that could not run, so the fix is an arm (or a reason the walk cannot exhaust on a carried component, stated at the call).


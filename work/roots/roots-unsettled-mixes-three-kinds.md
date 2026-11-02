---
id: roots-unsettled-mixes-three-kinds
kind: issue
title: roots::Unsettled holds pose-caused, resolution-caused and self-contradiction refusals in one enum, which consumers fold alike
status: open
opened: 2026-10-15
priority: P3
cost: M
refs: [5227]
---

Carried out of stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227), which listed it "if small". It is not: the split touches every root-finder row.

`geom_brep::roots::Unsettled` holds three kinds in one enum:
- **pose-caused:** `Tangency`, plus `AtApex` and `Degenerate`, which are the cone door's vocabulary in a crate with no cone door;
- **resolution-caused:** `Meter`, `SignInBand`, `Exhausted`;
- **self-contradictions,** a kernel-bug class: `ImpossibleCount`, `Resolvent`, `CountDisagrees`.

`reduce::wall_crossing` reads `AtApex` by name. The line × torus door and `solid_contain::torus_crossings` read `CountDisagrees`. Every other consumer folds them alike, so a self-contradiction can be read as a graze.

The curve-pair reader keeps its own contradiction out of `Unsettled` (`curve_pair::MeetFault::Contradiction`, stage (c)).

A route: move `AtApex` and `Degenerate` into topo's cone door, and give the contradictions an arm a consumer cannot fold into a graze (`RootRefusal::Contradiction`, say), so each consumer matches the three kinds by what it acts on.

## What PR 5993 settled (2026-10-29)

PR 5993 (`splitdoor/roots-unsettled-split`, the SPLITDOOR item
`roots-unsettled-folds-band-precision-and-fault-into-one-frontier`)
split the type by what a consumer acts on:
`RootRefusal::Touch` (a reading in the band's gap),
`RootRefusal::Unsettled` (the limits), and `RootRefusal::Fault`
(`RootFault`, which consumers cannot fold into a graze unseen).

Its ruling moves two of the self-contradictions listed above. Legal
input reaches `Resolvent` and `CountDisagrees`
(`work/inside/torus-split-lead-escalates-a-legitimately-small-resolvent-root.md`),
so they are limits and stay in `Unsettled`. `SignInBand` is gone: the
gap reading is a `Touch`. `RootFault` is `ImpossibleCount` alone, the
subdivision's odd count or one above `2N`.

**Left:** the route's first half. `AtApex` and `Degenerate`, the cone
door's vocabulary, still live in `geom_brep::roots::Unsettled`. Moving
them into topo's cone door is this row's remaining work. The faults'
graze at the ray lanes is `work/splitdoor/ray-lanes-graze-on-a-root-finders-fault.md`.

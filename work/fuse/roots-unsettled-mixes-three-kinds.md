---
id: roots-unsettled-mixes-three-kinds
kind: issue
title: roots::Unsettled holds pose-caused, resolution-caused and self-contradiction refusals in one enum, which consumers fold alike
status: open
opened: 2026-10-15
priority: P3
cost: M
refs: [the-curve-pair-reader]
---

Carried out of stage (c) of `work/fuse/the-curve-pair-reader.md`, which listed it "if small". It is not: the split touches every root-finder row.

`geom_brep::roots::Unsettled` holds three kinds in one enum:
- **pose-caused:** `Tangency`, plus `AtApex` and `Degenerate`, which are the cone door's vocabulary in a crate with no cone door;
- **resolution-caused:** `Meter`, `SignInBand`, `Exhausted`;
- **self-contradictions,** a kernel-bug class: `ImpossibleCount`, `Resolvent`, `CountDisagrees`.

`reduce::wall_crossing` reads `AtApex` by name. The line × torus door and `solid_contain::torus_crossings` read `CountDisagrees`. Every other consumer folds them alike, so a self-contradiction can be read as a graze.

The curve-pair reader keeps its own contradiction out of `Unsettled` (`curve_pair::MeetFault::Contradiction`, stage (c)).

A route: move `AtApex` and `Degenerate` into topo's cone door, and give the contradictions an arm a consumer cannot fold into a graze (`RootRefusal::Contradiction`, say), so each consumer matches the three kinds by what it acts on.

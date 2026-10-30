---
id: census-meet-faults-read-as-census-unsupported
kind: issue
title: census: a meeting reading's fault (a root finder's contradiction, a carrier misread) is reported as CensusUnsupported, the census's frontier
status: open
opened: 2026-10-29
priority: P2
cost: E
---


Found by the §5 sweep of `splitdoor/roots-unsettled-split` (the
`roots-unsettled-folds-band-precision-and-fault-into-one-frontier` unit).

`crates/topo/src/census.rs`, the boundary-crossing read in the census's
cross pass (the `meet(..).map_err(..)` over `MeetFault`), maps
`MeetFault::Unsettled(_)`, `Unsupported`, `Desync` and `Contradiction`
alike to `CrossRefusal::Unread`, which the census reports as
`ValidationError::CensusUnsupported`. `crates/topo/src/census/curved.rs`
(the curved-edge pair read, `refuse(..)`) does the same for its pair.

After the split a root finder's fault (`geom_brep::roots::RootFault`,
today an impossible count, which no legal input reaches)
reaches `meet`'s callers as `MeetFault::Contradiction`, and a carrier
misread is `MeetFault::Desync`. Both are the kernel's defect, and the
census tells them as its own frontier ("not certifiable"), the story of
a limit (`Unsettled`) or a pose with no closed form. A touch in band
(`MeetFault::Touch`) already leaves with `MeetFault::Escalated` (the census's escalation), so
only the fault fold is left.

The repair is a kernel-defect ending for `Desync` and `Contradiction` at
both sites, and a row that drives each through the census.

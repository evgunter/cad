---
id: census-interiors-fold-a-meetings-fault-into-unbuilt
kind: issue
title: census: arc_interiors_meet folds a meeting's kernel fault (Desync, Contradiction) into InteriorsUnread::Unbuilt, the census's frontier
status: open
opened: 2026-10-29
priority: P3
cost: E
---

Found by the review of PR 5993 (`splitdoor/roots-unsettled-split`).

`crates/topo/src/census.rs:10944` (`arc_interiors_meet`, the refused
`meet` of two spans) reads `MeetFault::Escalated` and `MeetFault::Touch`
as `InteriorsUnread::Undecided` and every other fault as
`InteriorsUnread::Unbuilt`, the census's frontier ("the reading is not
built"). `Unsettled` and `Unsupported` are limits and belong there.
`Desync` (a carrier the meeting table misread) and `Contradiction` (two
readings of the pose that contradict, now including a root finder's
`RootFault`) are the kernel's defect, told as a reading not yet built.

The repair is a kernel-defect ending for those two (`InteriorsUnread` has
none today) and a row that drives a `Contradiction` through it; the
sibling `census-meet-faults-read-as-census-unsupported` is the same fold
at the census's cross pass.

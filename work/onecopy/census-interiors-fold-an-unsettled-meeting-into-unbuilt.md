---
id: census-interiors-fold-an-unsettled-meeting-into-unbuilt
kind: issue
title: census's InteriorsUnread::of files MeetFault::Unsettled as a reading not built, a consumer-side rank of a reading the band moves
status: open
opened: 2026-11-05
priority: P2
cost: C
---


Found by `onecopy/ladder-climb-3`, which ported census's four
interior-meeting searches (`shared_run`, `planar_interiors_meet`,
`shared_interior_witness`, `arc_interiors_meet`) onto
`geom_brep::ladder::climb`, ranked by `InteriorsUnread::refusal`
(Undecided in band, Unbuilt a limit).

`InteriorsUnread::of` (`crates/topo/src/census.rs:11465`) maps a refused
meeting's `MeetFault` onto those two kinds, and files
`MeetFault::Unsettled` (a root finder, a meter or the walk's budget out
before it certifies, `crates/geom-brep/src/curve_pair.rs:152`) with
`Unbuilt`, so the ladder now ranks it as a limit, ahead of any reading in
band. That is a rank chosen at the consumer, the misfiling PR 6495 found
for `chart_singularity`'s `Unread::Carrier`: whether a tolerance moves a
reading is a property of where it is minted. If a coarser band can
settle the walk, it is `Rung::Unsettled` (ranked after in band), not a
limit. The type's own docs call it "its limit", so the question is open.

The repair is to give `MeetFault` (or `Unsettled`) its standing beside
its variants, a `refusal()` the ladder reads, and to have `InteriorsUnread`
carry an unsettled kind, or read the fault's rank directly. Do not
re-rank at the census. The sibling
`work/splitdoor/census-interiors-fold-a-meetings-fault-into-unbuilt.md` is
the same fold's `Desync` and `Contradiction` arms (kernel defects told as
"not built"); it calls `Unsettled` a limit, which this row questions.

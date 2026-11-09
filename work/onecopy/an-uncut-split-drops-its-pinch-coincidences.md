---
id: an-uncut-split-drops-its-pinch-coincidences
kind: issue
title: an uncut split (no section) returns no coincidences, where the cut path carries the reduce's pinch coincidences
status: open
opened: 2026-11-09
priority: P3
cost: E
---


## Finding

Filed by `onecopy/side-at-infinity` (PR 6595's review).
`splitting::finish::whole_body_side`, the split with no section, returns
`coincidences: Vec::new()`. The cut path returns `red.coincidences`: the
reduce's ON vertices whose neighbourhood leaves two or more runs on a side
(`splitting::reduce`, `Relation::OnCarrier`, `DecisionSite::SplitOn`).
Such a vertex can occur where the plane touches the body without cutting
it (two lumps on one side meeting at a vertex on the plane), so an uncut
split may drop a pinch it decided. Whether it should carry them turns on
what the naming and records consumers of `SplitResult::coincidences`
expect from an uncut split. Carrying it is small (pass `red.coincidences`
through), but it changes the published output of every uncut split that
touches at a pinch, so it is not done blind.

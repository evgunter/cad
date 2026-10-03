---
id: select-pair-in-band-tells-poison-as-an-ambiguity-band-margin
kind: issue
title: wire: SelectRefusal::PairInBand tells a pair whose datum is not finite as a margin left inside the ambiguity band
status: open
opened: 2026-10-03
priority: P3
cost: E
---


(TOPO, found by the PR 3974 fix pass, review NOTE-2.)

## What

`topo::flush::pair_finding` now refuses a pair one of whose compared
datums is not finite as `PairUndecided::Unreadable`, and the body seat
tells it as `FlushRefusal::PairUnreadable`: the datum is not finite, ending
in `KERNEL_OR_FILE_DEFECT_ENDING`, with no lever offered.

The document seat, `editor-core/src/names/flush.rs` (~:293), takes
`undecided.diag()` from either arm and builds
`SelectRefusal::PairInBand`, whose `Display`
(`names/geompred.rs`, ~:402) says the predicate "left its margin inside the
ambiguity band". For poison that is false: nothing was in band, and no move
of the geometry reads the datum.

## Fix

Match `PairUndecided::Unreadable` in `names/flush.rs` and give it its own
`SelectRefusal` arm (or a field on `PairInBand`), told as a datum that is
not finite, as the body seat's `PairUnreadable` is. Pin the text with a
row. The pncad-py tag inventory moves with a new arm.

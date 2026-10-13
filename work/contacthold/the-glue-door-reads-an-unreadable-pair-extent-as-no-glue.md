---
id: the-glue-door-reads-an-unreadable-pair-extent-as-no-glue
kind: issue
title: The glue door reads a pair whose extent it cannot read as no glue, where the tangency check it falls through to refuses the same read typed
status: open
opened: 2026-10-12
priority: P3
cost: E
refs: [the-glue-doors-tangency-swallows-its-escalations]
---


## What

`boolean::glue::decided_declarations` (crates/topo/src/boolean/glue.rs)
matches `carrier_pair_relation`'s `Err(PairUnread::Extent(_))` together
with `Ok(Err(_))` and leaves the pair unglued (`=> None`). The module docs
license only the second: "a pair whose one-carrier coincidence does not
decide (in band, or poisoned) is left to the stage that meets it". An
unreadable face extent is neither; it is a face the door cannot read.
`boolean::verify_tangency`'s conformal screen refuses the same read typed
(`Err(PairUnread::Extent(face)) => Refused(unreadable_extent(face))`), so
the two halves of one door disagree on it.

Found by the sweep of PR 4918 on
`the-glue-doors-tangency-swallows-its-escalations`, read from the code,
not measured: no fixture is known to reach it.

## Owed

Measure whether any fixture reaches the arm; then refuse it as
`unreadable_extent` does, or say in the module docs why an unreadable
extent may be left to a later stage.

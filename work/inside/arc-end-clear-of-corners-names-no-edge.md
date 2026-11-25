---
id: arc-end-clear-of-corners-names-no-edge
kind: issue
title: ContainError::ArcEndClearOfCorners carries no key for the arc its raise had in hand, so its text names neither loop nor edge
status: open
opened: 2026-11-25
priority: P3
cost: E
---


Found by the sweep of `contain-error-drops-the-loop-its-carrier-named`
(lane `inside-contain-small`), which made `ContainError::RayExhausted`
carry and render its loop and `Uncrossable` render the loop it carries.

## The finding

`crates/topo/src/contain.rs`, `boundary_pre_pass` (the `OnEdge::End`
arm): the raise has the loop (`lk`) and the arc's half-edge
(`lp.keys[i]`) in hand, and builds
`ContainError::ArcEndClearOfCorners(Box::new(ArcEndGaps { gaps, band }))`,
which carries neither. `ContainError::write`'s arm then renders
"the point lies at an arc's end clear of its corners: {gaps}" with no
key, while every sibling arm that is about one loop or edge
(`EmptyLoop`, `LoopUnreadable`, `RayExhausted`, `Uncrossable`) names it.

The census routes the defect half as
`ValidationError::ArcEndOffCorner { face, gaps }` (`census.rs`,
`read_containment`), so the face reaches the reader there; which ARC of
the face does not, here or in the boolean's `PointInFaceRefused`.

## The repair

Carry the half-edge (or edge) in `ArcEndGaps` or beside it, and render it
in `ContainError::write` and `ArcEndOffCorner`'s text. Its consumers are
the `ArcEndClearOfCorners(_)` arms the compiler lists, and
`test_support_samples.rs`'s `arc_end_clear_of_corners` sample.

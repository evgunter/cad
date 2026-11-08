---
id: a-spin-about-a-read-axis-the-copy-is-round-about-is-not-reported-unobservable
kind: issue
title: a spin about a read axis that the copy is symmetric about is not reported unobservable, as the spin about a round mate's axis is
status: open
opened: 2026-11-02
priority: P4
cost: E
---


Found by `placing/values-direction` (the `[placement.values]` build, PR
for `a-linear-patterns-direction-components-land-in-its-slides`), by its
§5 sweep of the readers of `Coord::Spin`. Traced, not run.

`mate::unobservable` (`crates/editor-core/src/mate/unobservable.rs`,
the placement loop of `unobservable`) reports a value whose number moves
nothing only for `Coord::SPIN`, the spin about a mate's direction, and
only where the mate's own geometry is round about it
(`spin_moves_nothing`). The build added `Coord::SpinAbout`, a chain link
about an `Axis` read in the target space: a copy round about that line
(a cylinder spun about its own axis read off itself in the target) is
turned by a number that moves nothing, and nothing reports it. A pose's
`PoseDef::Spin` about such a line is reported (`pose_move`), so the two
spins about a read line answer differently.

**Owed.** The same verdict for `Coord::SpinAbout`: the copy's geometry
round about the read line, decided as `pose_move` decides a pose's spin.

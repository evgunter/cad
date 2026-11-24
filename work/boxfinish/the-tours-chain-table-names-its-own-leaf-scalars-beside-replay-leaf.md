---
id: the-tours-chain-table-names-its-own-leaf-scalars-beside-replay-leaf
kind: issue
title: The tour's chain table names its own leaf scalars beside eval::replay_leaf, which the facade does not offer
status: open
opened: 2026-11-24
priority: P3
cost: E
---


Found by the gather-carries lane (2026-11-24), on a designer's note
(`probe/briefs/fork-plain-interval/A.md` on `boxfinish/briefs`).

`demos/tour/src/chaintol.rs`'s `interval_leaf` and `sym_leaf` evaluate
the chain over the whole analyzed box by naming the scalar themselves
(`evaluate::<Interval>`, and `evaluate::<Sym<Interval>>` inside
`sym::with_session_rules` with the dials read off `DriveConfig::default()`).
`editor_core::eval::replay_leaf`'s doc calls itself "the ONE door every
certified-leaf consumer goes through", so that no consumer picks its own
scalar, and `pncad` re-exports no `replay_leaf` (grep of
`crates/pncad/src`), so the demo has no public door to go through. The
table's `Interval` column is also the drive's tier-off lane
(`SymbolicDials::off`), which no shipped caller drives.

Either the facade offers the leaf door (and the dial,
`work/pydoor/the-drivers-symbolic-dials-have-no-name-on-the-facade.md`)
and the cell goes through it, or the cell's two columns are the drive's
own lanes stated as such. Not fixed here: the lane's fence is the gather.
(gather-carries lane)

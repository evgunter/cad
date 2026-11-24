---
id: a-node-that-fails-definitely-over-a-leaf-bisects-to-the-budget
kind: issue
title: A node that fails definitely over a leaf is bisected until the budget: the drive has no class for a leaf that is definitely invalid
status: open
opened: 2026-11-24
priority: P2
cost: M
design: true
---


Found by the designers weighing
`a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`
(2026-11-24), by reading.

`classify_replay`'s first step sends every typed node failure to
bisect (`Unsettled::Failed`), including a check that refuses
definitely over the whole leaf. No narrower box changes a definite
refusal, so such a leaf costs depth-many replays and ends as `Budget`
mass that means "not proven" where the drive had a proof. E6 has no
class for "definitely invalid over the leaf" (`Infeasible` is the
solver's). Under the parted-path weighing's final state it is the
last place a decided leaf is treated as undecided: a node that failed
where the witness built is another build, proven. What the class is,
and whether it is `FlipCrossing` or its own, is a design question
weighed with that unit. (BOXFINISH orchestrator)

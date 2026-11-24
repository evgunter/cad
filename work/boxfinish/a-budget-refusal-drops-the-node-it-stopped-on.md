---
id: a-budget-refusal-drops-the-node-it-stopped-on
kind: issue
title: A Budget refusal for depth or grid drops the node it stopped on and what that node was unsettled on
status: open
opened: 2026-11-24
priority: P2
cost: E
---


Found by the designers weighing
`a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing`
(2026-11-24), by reading.

A leaf that is bisected carries the node that left it unsettled and
the kind (`LeafVerdict::Bisect { deciding, unsettled }`). When the
depth or grid budget ends the bisection, the `Budget` refusal keeps
neither, so a consumer told "budget" cannot say which node and which
undecided decision the drive ran out on. The refusal should name the
node and the unsettled kind it stopped on. (BOXFINISH orchestrator)

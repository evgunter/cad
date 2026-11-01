---
id: TCOST-9
kind: unit
title: gate the proptest population and the two content units' gate candidates with gated_to!
status: closed
opened: 2026-09-03
refs: [TCOST-1, TCOST-2, TCOST-4]
branch: tcost/9-proptest-gating
pr: 1681
closed: 2026-09-03
---


Cut at TCOST-1's merge (`log.md`): the second gating batch. Every
`proptest!` suite in the tree (22 files at TCOST-1's census) gets a
`test_utils::gated_to![…]` marker naming the module each claim rests
on (the batch-2 bar), plus TCOST-2's heavily-knotted rows and
TCOST-4's torus counterexample-search row (both named as gate
candidates at their merges). Test-infrastructure track: Opus
implementer, batched style review, no A/B row.

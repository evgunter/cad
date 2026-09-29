---
id: split-param-not-interior-offers-the-declare-menu
kind: issue
title: topo: split_edge's definite not-interior refusal still offers the declare menu its escalated sibling dropped
status: open
opened: 2026-09-29
---

(TOPO, the §5 second pass of PR 3493, which routed the escalated
sibling and was fenced to that arm in `euler.rs`.)

## What

`EulerOpError::SplitParamNotInterior` (`crates/topo/src/euler.rs`, its
`Display` arm near :1059) renders
"split_edge: the parameter is definitely not interior to edge
{edge:?}'s certified interval … if it was meant to land exactly on an
endpoint, declare the coincidence, move the geometry, or lower the
tolerance".

- It offers "declare the coincidence" to the same three doors PR 3493
  took it away from on `SplitParamEscalated`: the split
  (`SplitReduceError::CrossingInsertion`), the blend (`BlendError::Op`)
  and the Boolean (`BooleanError::CrossingInsertion`). None of them
  takes a declaration that names where on an edge a crossing lands.
- The two arms are one decision (`split_edge_param_interior`), so
  D4 ¶1 (i) wants one recourse for both. After PR 3493 the escalated
  arm ends in `boolean::CROSSING_RECOURSE` and this one does not.
- It opens with the stage label `split_edge:` and names an arena key.

## Repair shape

End the arm in the escalated arm's subject words
(`boolean::CROSSING_INTERIOR`) and its recourse
(`boolean::CROSSING_RECOURSE`), drop the label and the key, and
re-baseline `euler::tests::split_param_pair_carries_the_shared_recourse`,
which pins the menu on this arm today.

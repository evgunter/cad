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
  D4 ¶1 (i) wants one ending computed from the decision and the
  verdict. After PR 3493 the escalated arm ends through
  `boolean::refusal_routes::SPLIT_PARAM_INTERIOR` (a
  `geom_brep::recourse::SizedDecision` passing on a positive margin)
  and this one does not.
- The arm carries no margin: `split.rs` decides through
  `k_stats::decide` and keeps only the sign, so the Zero verdict (a
  crossing within the zero band of an end, which a smaller tolerance
  may decide inside) cannot quote the tolerance its margin gives.
- It opens with the stage label `split_edge:` and names an arena key.

## Repair shape

Decide through `k_stats::decide_reported` in `split.rs` and carry the
verdict on the arm as a `geom_brep::recourse::Refused`
(`Refused::of`: `Zero` with its classified margin, or the sign-certain
`Negative`). Render `refusal_routes::CROSSING_INTERIOR` and end in
`SPLIT_PARAM_INTERIOR.recourse(refused.arm(), Reading::Build)`: the
Zero arm then names the lever and the tolerance its margin gives, the
Negative arm the lever alone, and neither offers a declaration. Drop
the label and the key, and re-baseline
`euler::tests::split_param_pair_carries_the_shared_recourse`, which
pins the menu on this arm today.

---
id: a-node-whose-runs-part-at-a-querys-candidates-names-where
kind: unit
title: Diagnosis::Parted: a node whose runs part because a query handed on different candidates names the query and the step
status: closed
opened: 2026-11-20
priority: P1
cost: M
closed: 2026-11-20
---

Ev ratified the rung on `[ev]` PR 7129: N5 in
`crates/editor-core/src/names/README.md` ("A node whose runs part
because a query handed on different candidates (a recorded mark) names
where: the query and the step (`Diagnosis::Parted`), ranked below a
flip and above the group-size rung"). Since PR 7114 every pruning query
marks the candidates it hands on and `resolve::first_turn` stops at the
first difference, so no boolean edit names a flip and diagnosis fell to
`GroupResized` or the fallback. The log proves where the runs parted.

## Closed

Built. `vdiff::FirstTurn::Parted` carries a `Parting`: `Mark { site,
step }` where both runs' first differing steps are marks of one query
with different items, and `Step { step }` / `Ended { step }` for the
partings N5 names no cause for. `Diagnosis::Parted { node, site, step }`
is the ladder's parting rung, after the upstream scope and before the
group-size rung, over the path nodes in id order.

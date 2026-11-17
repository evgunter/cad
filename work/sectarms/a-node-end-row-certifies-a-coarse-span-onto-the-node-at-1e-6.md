---
id: a-node-end-row-certifies-a-coarse-span-onto-the-node-at-1e-6
kind: issue
title: Main red at 1e-6: node_end_rows' chain of one coarse span onto the node certifies (Chart rung 0.125) where the row wants TubeStraddles
status: review
opened: 2026-11-17
priority: P1
cost: E
branch: sectarms/main-red-1e-6
pr: 7057
---

Found by the sliver lane (`sectarms/sliver-loop-tube`, 2026-11-17) and replayed by the orchestrator on main at `f50b58e2f3`. It is also red at `fa4e9ba921`, PR 7052's first parent, so it predates that PR. Bisected by PR 7057's lane: red at 1e-6 at PR 6897's merge `dfa288a929`, with the identical output (`nodes: [None, Some((0.1, 0.1))]`), so it was red from the PR that added it.

## What

`ssi::certify::node_end_rows::a_chain_ends_at_a_node_only_where_its_end_is_at_the_node` (`crates/geom-brep/src/ssi/certify.rs:4666`) at `CAD_TOLERANCE_EPS=1e-6`: the row's "one coarse span onto the node" case certifies (`Chart { rung: 0.125, … }`), where the row wants `TubeStraddles`. It passes at the default ε. The row came with PR 6897's node carry.

## Owed

Find out whether the coarse span is a graph in truth at the 1e-6 band, in which case the row's case is pinned too tightly for that band. The alternative is that the node-end reading admits an end it should not, which is the class of PR 6897's review MAJOR (an unbound node end). Bisect when it went red.

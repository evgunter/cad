---
id: an-n-ary-fold-carrying-a-rim-junction-refuses-at-its-first-lump
kind: issue
title: An n-ary fold step given a rim junction refuses at its first sector lump: GlueVerdict::decision hands it Junction { rim: None }
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found on the second review of PR 7432 (HELDRECORDS' elbow lane,
2026-11-23).

## Read

`crates/topo/src/boolean/mod.rs:1156` (`GlueVerdict::decision`): a fold
step of the n-ary door (`nary`) is handed a glued pair's decision as
its `GivenDecision`. For a `Junction` that decision is
`GivenDecision::Junction { rim: None }`: a `GlueVerdict` is rebuilt
from the operation's coincidence rows (`GlueVerdict::of_rows`), and the
junction's row carries the rim's identity margin but not the circle.

`given_verdicts` then records a `JunctionCert { rim: None }`, and every
sector reader's junction arm (`sectors::junction_lump`,
`sectors::junction_bound_side`, through `GluedPairs::junction_at`)
cannot place its site on a rim it does not hold. It refuses
`CurvedBooleanUnsupported` on the sector's face ("unplaced"). So an
n-ary union whose members meet on a rim junction refuses at the first
lump that reads the pair, where a binary union of the same two members
reads it.

## Owed

The rim handed through the fold: the `GlueVerdict` carries the
junction's circle as it carries a tangency's line (`lines`), so the
step's `GivenDecision::Junction` holds `Some(rim)`. A row: a three-member
union (the rod, the half ball on its cap, and a third body apart) that
builds as its binary pair does.

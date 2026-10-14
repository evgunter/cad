---
id: a-walked-chain-closes-at-a-corner-without-a-junction
kind: issue
title: walk_chains closes a chain at a corner vertex without recording a junction there
status: open
opened: 2026-10-06
priority: P3
cost: M
---

## Finding (pre-existing; found by the CARVE review of PR 4185)

`walk_chains` (`crates/sweep/src/blend/battery.rs`) sets `closed` as
soon as `head == tail` after a step. It does not check that the
closing vertex is a junction, meaning a vertex holding exactly two
link ends. A run whose two ends meet at a CORNER therefore reads as
closed:

- The cube's four top edges plus one vertical edge at a top corner
  walk to a `Closed` loop with three junctions. There is no wrap-around
  junction at the corner, which holds three link ends.
- Two two-link loops sharing one vertex `v` walk to two closed chains.
  `v` is judged by no predicate: it is no chain's junction and no open
  chain's end.
- `break_at_turns` could then carry a run through the corner, as
  though it were an interior point of the chain.

Through the door, the cube case refuses `Turn`, so the plane–plane
case is probably unreachable. Whether curved links can reach it is not
known.

## Fix shape

Close a run only at a junction. When `head == tail` and that vertex is
not a two-end junction, the run is open with both ends at the corner,
and predicate 6 judges it there. Add a test through
`test_support::walked_links` on rewired links and through the door on
the cube.


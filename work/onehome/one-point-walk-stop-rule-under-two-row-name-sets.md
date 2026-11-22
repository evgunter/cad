---
id: one-point-walk-stop-rule-under-two-row-name-sets
kind: issue
title: One point-walk stop rule under two row-name sets: the curve-pair walk's near_pieces and Curve3::clear_of
status: open
opened: 2026-11-16
priority: P3
cost: E
---

Found by the walk-stop fork's designers (2026-11-16); checked against main
`2c17deabf`.

One stop rule over `Curve3::point_walk` is written twice, under two sets
of row names:

- `near_pieces` (`crates/geom-brep/src/curve_pair/walk.rs:178`): a piece
  whose gap less reach decides positive is settled
  (`curve_pair_walk_foot_clear`, `:190`); one whose reach stops at the band
  is kept (`curve_pair_walk_foot_piece`, `:197`); else halve.
- `Curve3::clear_of` (`crates/geom/src/curves/on_carrier.rs:659`): the same
  settle (`curve_carrier_piece_clear`, `:675`) and stop
  (`curve_carrier_piece`, `:689`); else halve.

`clear_of`'s own doc says every such reading "is a [`Self::point_walk`]
whose visit is its stop rule, and its callers are that list"
(`on_carrier.rs:646-648`); `near_pieces` is a second spelling of the visit,
and its ledger rows are a second name for the same decisions.

## Done when

One stop rule, under one row-name set, serves both walks.

**Read the walk-stop fork first.** The open design fork `work/onehome/the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes.md` may change
the fix; whoever takes this row reads that fork's outcome before writing
anything.

---
id: the-least-band-gates-door-list-is-a-hand-copy-of-k-stats-doors
kind: issue
title: least-band-on-a-door.sh keeps a hand-copied subset of k_stats' doors, and flagged_census keeps its own: two hand lists of one roster
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Filed from PR 6602's DELTA review (MERGE).

- **Two hand lists of one roster.** The gate `scripts/gates/least-band-on-a-door.sh` (the `doors[...]` lines and its `as`-rename regex) hand-copies a subset of the doors `geom_core::k_stats` lists in its own header (k_stats.rs:15–19). `geom-core/tests/flagged_census.rs` keeps a separate door list. Read one roster, the one k_stats owns, from both.
- **Doors the gate misses.** They all take a `Band` and so pass at the least band: `decide_flagged_reported`, `decide_positive`, `decide_negative`, `decide_nonzero` (and their `_reported` forms), `decide_magnitude` (and `_reported`), and the method `.decide_reported`. No such site exists today, but PR 6602's body claims the least band "can no longer reach any logged or unlogged door's argument list". Correct the claim, or close the gap by reading the roster.
- **Census import blind spots.** It misses a module alias (`use geom_core::k_stats as ks; ks::choose`), a glob `k_stats::*` and `use …::choose as pick`. Also, `item.contains("k_stats::choose")` would match `k_stats::choose_x`.
- **One measurement to look at.** The F35 count (17,883 dropped steps at Interval, none at f64) likely overcounts distinct drops by up to 4×, because polish.rs:65 continues without moving the iterate. It also suggests the polish mostly doesn't polish at Interval. Worth a look.

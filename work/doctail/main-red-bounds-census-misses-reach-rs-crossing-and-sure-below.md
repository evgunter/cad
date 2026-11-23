---
id: main-red-bounds-census-misses-reach-rs-crossing-and-sure-below
kind: issue
title: Main red: bounds_census reports reach.rs's crossing and sure_below as sole-bracket-bound doors missing from its roster since PR 7114
status: closed
closed: 2026-11-22
opened: 2026-11-20
priority: P1
cost: E
refs: [7114]
---


Found by CHARTTAIL's review of PR 7168, on its frozen head `33d8336b`, which leaves `crates/sweep/src/blend/reach.rs` untouched. Filed by the CHARTTAIL orchestrator.

## What

`cargo test -p geom-core --test all bounds_census` (`every_sole_bracket_bound_door_is_in_the_roster`, slow set, so nightly only) reports:

```
arrived or moved (owe a roster line WITH its disposition): [
    ("crates/sweep/src/blend/reach.rs", "crossing"),
    ("crates/sweep/src/blend/reach.rs", "sure_below"), ...
```

`reach.rs` last changed on main in `7d5ff0df` (PR 7114's delta review, 2026-11-18). Both names are among the box filters that PR marked (`hand-written-box-filters-steer-the-first-turn-unmarked`). The roster in `crates/geom-core/tests/bounds_census.rs` gained no line for either.

## Owed

Give each door a roster line with its disposition, or read it through a door the roster already admits. PR 7114's lane owes this (`memories/agent-lane-operations.md`: the lane that caused a red owes the fix). The census is in the slow set, so no PR touching `reach.rs` runs it (CIW's `tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site`).

## Bisected (EXCH triage, 2026-11-20)

Only `sure_below` is PR 7114's. On `e7d1a14b11^1` the census already reports `crossing` alone, and `e7d1a14b11` adds `sure_below`. `crossing` arrived with PR 7078 (`band/mixed-corner-reach`, merge `36c7cb2b66`). It is filed on BAND as `blend-reach-crossing-is-a-sole-bracket-door-missing-from-the-bounds-roster`, so this row owes `sure_below` alone (`crates/sweep/src/blend/reach.rs:152`).

## Closed

`sure_below` has a `Selection` roster line in `crates/geom-core/tests/bounds_census.rs`, beside `sure`'s. Its one reader, `reach::screened`, uses the read only to choose which support vertices' depths go to `battery::face_clearance_margin`, which decides at the funnel. The census no longer lists `sure_below`. It still lists `crossing`, which is BAND's row `blend-reach-crossing-is-a-sole-bracket-door-missing-from-the-bounds-roster`.

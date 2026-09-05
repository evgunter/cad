---
id: anchor-span-sole-bracket-bound-unrostered
kind: issue
title: arc_fillet::anchor_span landed unrostered in the sole-bracket-bound census under a cancelled gate; main red since aa5384288
status: open
opened: 2026-09-05
---


## What

`crates/profile/src/path/arc_fillet.rs:522` `fn anchor_span<T: Bounds>`
arrived on main in PR 1895 (`fillet/attr-every-crossing`, merge
`aa5384288`, commit `e4886d906`) with no line in
`crates/geom-core/tests/bounds_census.rs`'s roster, and the CI run on
that merge (33943429161) was cancelled, so
`every_sole_bracket_bound_door_is_in_the_roster` has been red on main
for every code-tier run since — two TOPO lanes hit it the same night
(PRs 1923 and 1919), and neither reviewer of PR 1923 could find a main
run that had observed it.

**What TOPO did**: PR 1923 carries one `HandedOff` roster line beside
the sibling door in the same file, citing this item, so lanes stop
failing on it; it no-ops once a better line lands. **What this item
owes**: the disposition. The census's `Why` vocabulary
(`bounds_census.rs:59-82`) asks for a soundness reason at the door;
`HandedOff` states only an owner. The lane's own argument for the door
is D9 stability (the span is a selection over ordered anchors, not a
measured bound), which reads as a `Selection`/DL5(b) line — FILLET's
call, in FILLET's file (`crates/profile/` is this program's `paths`).

Filed by the TOPO orchestrator, 2026-09-05.

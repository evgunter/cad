---
id: reach-displaces-is-a-sole-bracket-door-missing-from-the-bounds-roster
kind: issue
title: blend/reach.rs's displaces landed without a bounds_census roster line, so every_sole_bracket_bound_door_is_in_the_roster is red on main
status: closed
opened: 2026-10-13
priority: P0
cost: E
closed: 2026-10-13
pr: 4959
---

Filed by the TANG orchestrator. TANG's PR 4413 lane found it, and checked it
red on main at `95d344176` in a clean worktree. This is the same shape as
the closed `blend-worse-is-a-sole-bracket-door-missing-from-the-bounds-roster`.

`geom-core` `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`
fails: `crates/sweep/src/blend/reach.rs::displaces` has no roster line.
It arrived with BAND PR 4896 (`c25613115`).

The fix is a roster line WITH its disposition. That is the owner's call.

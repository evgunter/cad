---
id: blend-reach-displaces-is-a-sole-bracket-door-missing-from-the-bounds-roster
kind: issue
title: sweep blend/reach.rs's displaces<T: Bounds> (PR 4896) landed without a bounds_census roster line, so every_sole_bracket_bound_door_is_in_the_roster is red on main
status: open
opened: 2026-10-13
priority: P0
cost: E
---


## What

`geom-core`'s `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`
fails on `origin/main` `eb96a01d4b`:

> arrived or moved (owe a roster line WITH its disposition):
> `("crates/sweep/src/blend/reach.rs", "displaces")`

`fn displaces<T: Bounds>` (`crates/sweep/src/blend/reach.rs:2173`)
arrived with PR 4896 (`band/bore-step3-ruled-cap-consumes`, commit
`15b75526ee`), and `crates/geom-core/tests/bounds_census.rs` has no
roster line for it. Main's own CI did not run the census because the
change filter runs geom-core's suite only on a diff that touches
geom-core. DECIDE-10's PR 4929 touches geom-core, merged main, and
reproduces it locally on a tree whose diff does not touch `reach.rs`:

`cargo nextest run -p geom-core --features sym-profile-testing -E 'test(/every_sole_bracket_bound_door_is_in_the_roster/)'`

## What closes it

A roster line for `displaces` with its disposition, written by the
program that owns the door. The precedent is
`blend-worse-is-a-sole-bracket-door-missing-from-the-bounds-roster`.

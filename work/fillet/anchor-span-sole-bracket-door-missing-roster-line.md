---
id: anchor-span-sole-bracket-door-missing-roster-line
kind: issue
title: arc_fillet::anchor_span is a sole-bracket T: Bounds door with no bounds_census roster line — the interval-lane census row is red on main
status: open
opened: 2026-09-05
---


(PROPS orchestrator) Reported by the PROPS-1 lane (PR #1918) from its
merge with `main`, filed here because the door is FILLET's.

`crates/profile/src/path/arc_fillet.rs:522` `fn anchor_span<T: Bounds>`
— a sort key read off the diagnostic channel (`.lo()`), landed on `main`
in `21812ec19` / `e4886d906` (FILLET H5 / Track V) — is a sole-bracket
`T: Bounds` door, and `crates/geom-core/tests/bounds_census.rs` has no
roster line for it, so `every_sole_bracket_bound_door_is_in_the_roster`
reds at the `interval` lane on `main` and on every branch that merges
it (the lane's reading; verify against main's own run). PR #1918 ports a
roster line as `HandedOff` to Track V, worded from `anchor_span`'s doc
(stable sort, order a function of the inputs, permuted entries carry
identical payloads — D9), the disposition its neighbour `map_refusal`
carries; that wording is FILLET's to own or replace. The census's
purpose is that a sole-bracket door is ratified where it is read, so
the roster line wants FILLET's argument, not a ported one.

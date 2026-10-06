---
id: profile-ray-casting-exhaustion-is-a-copy-of-the-ray-walk
kind: issue
title: profile::validate::point_in_loop is a second copy of topo's ray walk, and RayCastingExhausted states no recourse
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Filed by CLEAVE's ray-walk driver unit (PR 4083), which gave `topo`'s
containment walks one driver (`topo::ray_walk::walk`), one ray outcome
vocabulary (`RayFault`) and one exhaustion sentence
(`ray_walk::NoRaySettled`, recourse `geom_core::NO_DECLARATION_RECOURSE`),
and left `profile`'s copy alone because `profile` cannot depend on
`topo`.

`profile::validate::point_in_loop` (`crates/profile/src/validate.rs`)
casts `N_RAYS` rays at fixed angles; a graze on any segment
(`seg::ray_crossings` → `Graze`) moves on to the next, and running out
is `ProfileError::RayCastingExhausted`. Two things are owed:

1. **The text.** It renders "containment of loop i in loop j: every
   candidate ray grazed — escalating rather than guessing" and names no
   recourse. Every exhaustion in `topo` now renders one sentence with
   one recourse: the rays are the kernel's own, so no declaration
   reaches them, and a graze carries no margin to size a tolerance by,
   so the recourse is to move the geometry. The same holds here.
2. **The move.** The walk is the same procedure: a fixed schedule, a
   graze keeps nothing, the first verdict wins. The driver and its
   `RayFault`/`Evidence` vocabulary depend on nothing in `topo` but
   `validate::decide`, so they could move to `geom-core` with `profile`
   and `topo` both as consumers. That is worth doing only if `profile`'s
   walk grows an in-band arm. Today it has none: `seg::ray_crossings`
   answers a count or a graze and nothing else.

Do (1) on its own. Do (2) only if `profile`'s walk gains a second
outcome.

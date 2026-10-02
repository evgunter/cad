---
id: a-solid-is-documented-connected-but-split-returns-disjoint-pieces-in-one
kind: issue
title: Solid is documented as a connected volume and ShellRole::Outer as one connected component, but split returns three disjoint prisms as one solid and tier 3 passes it
status: closed
opened: 2026-10-02
priority: P3
cost: E
refs: [split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
closed: 2026-10-02
pr: 3820
---


## What

`crates/topo/src/entity.rs` documents `Solid` as "a connected volume". `notched_block_end_to_end` asserts that split's Above side is one solid with three outer shells, two of which touch only along a tip line and the third disjoint. Either the doc is wrong (a solid may be disconnected) or split's solid assignment is. Decide which and make the two agree.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## Closed

The doc was wrong. DESIGN.md's structural conventions make disjoint unions tier-2-legal multi-shell bodies, tier 2 asks c = 1 per shell only, and tier-3 check 10 states several disjoint `Outer` shells in one solid are valid; the boolean's disjoint union is one solid with two shells, as split's notched block is one with three. "A connected volume" dates from the M0 scaffold, never ratified. `Solid`'s doc now says a solid's material is what its shells enclose by winding number and need not be connected; `ShellRole::Outer`'s per-shell wording was already true.

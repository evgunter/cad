---
id: a-solid-is-documented-connected-but-split-returns-disjoint-pieces-in-one
kind: issue
title: Solid is documented as a connected volume and ShellRole::Outer as one connected component, but split returns three disjoint prisms as one solid and tier 3 passes it
status: open
opened: 2026-10-02
priority: P3
cost: E
refs: [split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
---


## What

`crates/topo/src/entity.rs` documents `Solid` as "a connected volume". `notched_block_end_to_end` asserts that split's Above side is one solid with three outer shells, two of which touch only along a tip line and the third disjoint. Either the doc is wrong (a solid may be disconnected) or split's solid assignment is. Decide which and make the two agree.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

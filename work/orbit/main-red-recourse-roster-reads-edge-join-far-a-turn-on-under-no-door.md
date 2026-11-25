---
id: main-red-recourse-roster-reads-edge-join-far-a-turn-on-under-no-door
kind: issue
title: Main red (slow set): topo's recourse_roster finds edge_join_far_a_turn_on decided under no door's prefix
status: open
opened: 2026-11-25
priority: P1
cost: E
---


`topo::all recourse_roster::every_decided_name_has_its_door_and_every_site_is_read` (slow set) fails on main with:

> `edge_join_far_a_turn_on` is decided in the crate's src under no door's prefix

The predicate is decided in `crates/topo/src/boolean/edge_join.rs`, which reached main through ORBIT's merges (`47629acd78` / `2b7a9eaf91`). The roster wants each decided name under a door's prefix, with its site read. Found by HELDRECORDS' belt lane (PR 7731) after merging main, and filed here as ORBIT's ground. Not bisected beyond those merges.

Owed: the name moves under its door's prefix (or the door is registered), and the roster row goes green. A slow-set row red on main is a red nightly, so P1. (filed by the HELDRECORDS orchestrator)

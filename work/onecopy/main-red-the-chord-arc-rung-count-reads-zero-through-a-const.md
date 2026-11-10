---
id: main-red-the-chord-arc-rung-count-reads-zero-through-a-const
kind: issue
title: main red: chord_join's the_chord_arc_rung_is_decided_in_one_place counts the literal decision name, which e57e5d868e moved into a const, so it reads 0 sites
status: open
opened: 2026-11-10
priority: P0
cost: E
---


Found by ONEHOME's star-walk lane on PR 6725's CI, at a head whose `crates/topo/src/chord_join.rs` is byte-identical to main's. Recorded by the ONEHOME orchestrator, 2026-11-10.

`chord_join::tests::the_chord_arc_rung_is_decided_in_one_place` (`crates/topo/src/chord_join.rs:6612`, slow set) fails with "chord_arc_leave_germ is decided at 0 site(s)". The test counts the literal `"chord_arc_leave_germ"` at a decision site. `e57e5d868e` ("geom-brep's whole-src roster…", the `onecopy/roster-reach` merge) moved that name into `const CHORD_LEAVE_GERM` (`chord_join.rs:2152`), and the decision site (`:2155`) now names the const, so the count reads 0. The rung is still decided in one place. The defect is in the test's spelling, not the code.

Done when: the count finds the decision through the const (or the const is inlined back), and the row is green on main.

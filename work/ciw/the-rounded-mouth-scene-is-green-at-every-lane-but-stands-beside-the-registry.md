---
id: the-rounded-mouth-scene-is-green-at-every-lane-but-stands-beside-the-registry
kind: issue
title: corpus/mod.rs's stage6_beside_the_registry doc should say why the two green shaft_and_bearing scenes stay beside the registry
status: open
opened: 2026-11-14
priority: P4
cost: E
---

`crates/editor-core/tests/corpus/mod.rs` `stage6_beside_the_registry` (~:409-432)
lists `shaft_and_bearing::rounded_mouth` and `shaft_and_bearing::with_a_ball`
beside the registry, and its doc (~:415-418) says both now meet the bar
("green at every lane and ε"). Moving them into `documents()` would run both
through the whole registry battery on every PR that selects editor-core,
while `coincidence_door::the_coaxial_stage6_unions_are_green_at_f64_on_their_coaxial_rows`
and `coincidence_door::the_balls_interval_union_encloses_its_closed_form_under_a_bounded_pad`
already pin each scene at every ε. The registry battery would add per-PR cost
for scenes already pinned; they stay beside it.

Owed: the doc paragraph says that they stay beside the registry by choice
(pinned per scene by `coincidence_door.rs`; the battery's per-PR cost is not
spent on them), and drops the pointer to this row.

What would close it: that doc edit in `corpus/mod.rs`, no scene moved.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): Moving both `shaft_and_bearing` scenes into `documents()` runs them through the whole
registry battery on every PR that selects editor-core, while `coincidence_door.rs` already pins
each per ε. Grain: a doc sentence saying why they stay beside
(`editor-core/tests/corpus/mod.rs:409-432`). Reform: reform/the-rounded-mouth-scene-is-green-at-every-lane-but-stands-beside-the-registry.md

---
id: carrier-speed-is-re-spelled-with-a-default-arm-in-import
kind: issue
title: A carrier's speed is re-spelled per kind beside carrier_speed, one copy with a default arm
status: open
opened: 2026-11-02
priority: P2
cost: E
---

Found by the sweep of the `onehome/geom-readings` lane, against main `44577259fb`.

A carrier's speed is `geom::carrier_speed`'s reading (DESIGN's crate table: "a carrier's rate bounds (`carrier_speed`)"). Two per-kind copies sit beside it:

- `crates/step-import/src/normalize/singularity.rs` `speed`: `Circle → radius`, `Line → |dir|`, and `_ => 1.0` for every other kind: a default value where `[one-way.readings]` says "no arm answers a default value".
- `crates/geom-brep/src/chart_singularity.rs` `nearest_on_edge`: `Line → |dir|`, `Circle → radius`, otherwise sampled; its sibling `speed_bound` in the same file already reads `geom::carrier_speed::speed_range`.

Fix: read `carrier_speed` at both, and refuse typed (not `1.0`) where it has no answer.

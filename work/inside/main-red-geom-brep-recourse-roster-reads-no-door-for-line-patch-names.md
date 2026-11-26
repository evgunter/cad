---
id: main-red-geom-brep-recourse-roster-reads-no-door-for-line-patch-names
kind: issue
title: main red: geom-brep's recourse roster finds line_patch/walk decide names no door reads
status: open
opened: 2026-11-26
---


On main, geom-brep's slow-set row `recourse_roster::every_decided_name_is_read` fails. It names decide names in `crates/geom-brep/src/curve_face/walk.rs` and `crates/geom-brep/src/curve_face/line_patch.rs` (`AT_CREASE`, `ROOT_ON`, `STRETCH`, `NARROW`, `WINDING`, `past_band`) that no door reads.

The names arrived with INSIDE's `line_patch` work, for example `2b2517a7c9f` and `ca83b119b53`. GERM's PR 7658 fix-pass lane saw the failure while running geom-brep's slow set, and reported it outside that PR's diff.

Fix: list each name under the door that reads it, or add it to the roster's EXCEPTIONS with its reason. Then run the row:

    cargo nextest run -p geom-brep --profile ci --run-ignored all recourse_roster

---
id: main-red-certified-endpoint-census-pins-pair-section-at-four-where-the-tree-reads-six
kind: issue
title: Main red: certified_endpoint_census pins geom-brep pair_section.rs at (4, 4) endpoint reads where the tree reads (6, 6)
status: open
opened: 2026-11-20
priority: P1
cost: E
refs: [7112, 7128]
---


Found by CHARTTAIL's PR 7168 lane, which does not touch `geom-brep`. Filed by the CHARTTAIL orchestrator on SECT's slate, because SECT's `paths` claim `pair_section.rs` first.

## What

`cargo test -p geom-core --test all certified_endpoint_census` (`every_production_endpoint_read_is_counted_and_dispositioned`, slow set, so nightly only) fails. The roster line `("crates/geom-brep/src/pair_section.rs", 4, 4, "")` in `crates/geom-core/tests/certified_endpoint_census.rs` no longer matches the tree, which reads (6, 6). The closed row for the previous drift is SPLICE's `main-red-certified-endpoint-census-drifted-in-geom-brep-after-6171`.

## Suspects (not bisected)

Two merges touched `pair_section.rs` since 2026-11-18:
- PR 7112 (`onehome/span-foot`, merge `7ad94ddf`; +225 lines, including `ca5cdeda`, "a point in no tube piece is off the component");
- PR 7128 (`importgate/build`, merge `b119655d`; +69 lines).

## Owed

Bisect between the two, then give the two new endpoint reads their disposition (counted with a reason, or read through a certified door) and correct the roster line. The lane that caused it owes the fix (`memories/agent-lane-operations.md`).

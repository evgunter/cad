---
id: main-red-certified-endpoint-census-pins-pair-section-at-four-where-the-tree-reads-six
kind: issue
title: Main red: certified_endpoint_census pins geom-brep pair_section.rs at (4, 4) endpoint reads where the tree reads (6, 6)
status: closed
opened: 2026-11-20
priority: P1
cost: E
refs: [7112, 7128]
closed: 2026-11-20
pr: 7218
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

## Bisected (EXCH triage, 2026-11-20)

The cause is PR 7112 (`onehome/span-foot`, merge `7ad94ddffb`), commit `63f9e2ae97`. It added `let side = |i: geom_core::Interval| i.is_certified().then(|| (i.lo(), i.hi()));` (now `crates/geom-brep/src/pair_section.rs:2985`), which is two reads, both asking. The census row passes on `7ad94ddffb^1` and reads `pair_section.rs` at (6, 6) on `7ad94ddffb`. PR 7128 is not involved. The row is still red on main `bd8b2c9bb5`. By `work/README.md` the fix is owed by ONEHOME's lane. The row stays here for SECT or ONEHOME to re-home.

    cargo nextest run -p geom-core --ignore-default-filter -E 'test(every_production_endpoint_read)'

## Closed

PR 7218. Both new reads are `TubeRun::of`'s `.lo()` and `.hi()`, added by `63f9e2ae97` in PR 7112 (ONEHOME, `onehome/span-foot`); PR 7128 added none. Each is read through `is_certified().then`, so both count as asking; no certified door reads a bracket's endpoint pair, so the code stays. The roster line is re-pinned at (6, 6) with that note. The census's other two drifts (`pcurve_cache/speed.rs`, `geom-core/src/interval.rs`) are CHARTTAIL's, ONEROUND's and SCALAR's open rows.

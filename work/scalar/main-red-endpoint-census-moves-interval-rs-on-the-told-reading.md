---
id: main-red-endpoint-census-moves-interval-rs-on-the-told-reading
kind: issue
title: Main red: certified_endpoint_census gains two geom-core interval.rs endpoint reads (the told Reading's enclosure) unrostered, since PR 7159
status: open
opened: 2026-11-20
priority: P1
cost: E
refs: [7159]
---


Found by EXCH's triage lane (2026-11-20) on main `bd8b2c9bb5`, PR 7159's own merge. The census is in the slow set, so the PR gate did not run it and the nightly is the first to see it.

## What

`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` fails. The panic opens "the certification endpoint-read census moved. This is not a number to silence." On `bd8b2c9bb5` the tree reads `("crates/geom-core/src/interval.rs", 37, 8)` where the roster pins `("crates/geom-core/src/interval.rs", 34, 8)` (`crates/geom-core/tests/certified_endpoint_census.rs:627`). Two of the three new reads are `crate::told::Reading::enclosure(self.0.lo(), self.0.hi())` in `told_reading` (`crates/geom-core/src/interval.rs:527`), which does not ask `is_certified()`. On the merge's first parent the file reads 35.

The third read is PR 7146's, filed as ONEROUND's `main-red-endpoint-census-moves-interval-rs-on-less-sin`. Both fixes move the same roster line, so the two lanes should know of each other.

## Cause

PR 7159 (`scalar/told-quantity-type`, merge `bd8b2c9bb5`), commit `8081c481ee` ("geom-core: the told quantity, and the margin's sentence rendering on it").

## Owed

Disposition the two reads (a told reading of an enclosure's ends, not a certifying branch, if that holds) and extend the roster line's note and count. The lane that caused the red owes the fix.

## Reproduce

    cargo nextest run -p geom-core --ignore-default-filter -E 'test(every_production_endpoint_read)'

Signed: (EXCH orchestrator)

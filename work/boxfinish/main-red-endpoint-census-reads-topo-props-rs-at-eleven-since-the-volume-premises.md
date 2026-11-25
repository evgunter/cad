---
id: main-red-endpoint-census-reads-topo-props-rs-at-eleven-since-the-volume-premises
kind: issue
title: Main red (slow set): certified_endpoint_census reads topo/src/props.rs at (11, 3) where the roster pins (7, 3), since PR 7344
status: closed
opened: 2026-11-23
priority: P1
cost: E
refs: [7344]
closed: 2026-11-24
pr: 7491
---


Found by CHARTTAIL's `charttail/speed-census` lane (2026-11-23), running the census for its own `speed.rs` line on main `ba685969fc`.

## What

`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` (slow set) reads `crates/topo/src/props.rs` at `(11, 3)`; its roster pins `(7, 3)`. The four new reads arrived with PR 7344 (`boxfinish/volume-premises`, merge `b5b3fe7da5`). They are `closed_form_reads`'s `ends` closure, `(flux.lo(), flux.hi()), (area.lo(), area.hi())`, in a function that does not ask `is_certified()`. `closed_form_reads` sits under `#[cfg(any(test, feature = "test-support"))]`; the census follows the gate's production/test cut, which keeps a feature-gated item as production, so the reads count.

## Owed

Disposition the four in the roster's `topo/src/props.rs` note (a test-support door handing both brackets' ends to rows that compare the two readings, or an ask before reading), and move the line to the tree's count.

    cargo nextest run -p geom-core --test all --ignore-default-filter -E 'test(=certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned)'

## Closed

Fixed on main by another program before BOXFINISH's lane reached it:
commit `3304be32ee` (through PR 7491, `embed/wave3-a`) re-pins the
roster's `crates/topo/src/props.rs` line at `(11, 3)` and dispositions
the four reads in its note (`closed_form_reads`' `ends` closure: test
vocabulary, built only under `test` or `test-support`, no branch taken
on an end). The census row passes on main at `deaab6f773` (run by PR
7344's lane, 2026-11-24, CI's profile, a fresh target directory).

---
id: main-red-the-endpoint-census-moved-in-spline-implicit-rs-since-6862
kind: issue
title: Main red: geom-core's certified_endpoint_census moved in pair_section/spline_implicit.rs with PR 6862 (2,0 -> 6,4)
status: closed
closed: 2026-11-12
opened: 2026-11-12
priority: P1
cost: E
---


## Finding

Filed by PCERT's orchestrator. PR 6870's lane met it on main, and the orchestrator reproduced it on main as of 2026-11-12 17:10Z.

geom-core's slow-set row `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned` fails on one entry:

| file | pinned (reads, asked) | now |
|---|---|---|
| `crates/geom-brep/src/pair_section/spline_implicit.rs` | 2, 0 | 6, 4 |

Bisected to 81b50a0631 ("sectarms: the pad reads each chart window by cells…"), which reached main in PR 6862 (`sectarms/join-reads-spline-implicit`, merge 7d83cdff9e). Its parent `e92873cae2` passes. The row is in the slow set, so PR CI did not run it.

## Repair

The census's own message: each new read asks `is_certified()` by name, or is argued safe at its site, and its file's remainder note is extended to match. The four new reads already ask, which suggests a re-pin to (6, 4) with the note. Check the two unasked ones still match the note.

    cargo test -p geom-core --test all certified_endpoint_census::

## Closed

Re-pinned `spline_implicit.rs` to (6, 4) in `crates/geom-core/tests/certified_endpoint_census.rs`'s roster. The four new reads are `box_gap`'s, each behind `s.is_certified() && t.is_certified()`. The two unasked reads were already counted, but the note named them by stale names and called the lift unquotiented, so it is rewritten: `Pivot::shift`'s end is a quotient that `band.padded` refuses when not finite, and `Bracketed::at`'s seeds the bracket's first step only, every side decided by `Bracketed::sign`, which asks `is_certified()`.

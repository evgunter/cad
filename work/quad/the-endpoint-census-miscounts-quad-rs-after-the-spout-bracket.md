---
id: the-endpoint-census-miscounts-quad-rs-after-the-spout-bracket
kind: issue
title: certified_endpoint_census counts props/quad.rs at 10/4 against its pinned 7/2 on main after PR 5588; the slow-set row is red
status: open
opened: 2026-10-22
priority: P1
cost: E
---


## What

`crates/geom-core/tests/certified_endpoint_census.rs:722`
`every_production_endpoint_read_is_counted_and_dispositioned` is red on
main. Its census of `crates/geom-brep/src/props/quad.rs` reads 10/4
against the pinned 7/2. That file on main is as PR #5588 (the teapot
spout bracket) left it. The row is in the slow set, so the per-PR gate
never ran it.

Found by INTENT's PR #5587 lane, in a full local run of `geom-core` with
the slow set. #5587 touches no file the census counts.

## Owed

Disposition the new endpoint reads in `props/quad.rs`, then re-pin the
census with their stated cause. Do not raise the pin blindly.

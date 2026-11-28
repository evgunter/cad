---
id: main-red-the-endpoint-census-misses-ssi-certify-and-spline-compose-reads
kind: issue
title: Main red: the certified-endpoint census finds four undispositioned reads in ssi/certify.rs and one unrostered in spline/compose.rs
status: open
opened: 2026-11-25
priority: P1
cost: E
---



## Finding

Found by lane `embedwalk-trimmed` (PR 7787), re-running the census its
review named.

`geom-core certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
(`crates/geom-core/tests/certified_endpoint_census.rs`, slow set) is
red on `origin/main` (`8c24e8e6a2`, after PR 7779):

- `crates/geom-brep/src/ssi/certify.rs`: found 27 reads, 4 asking;
  the roster pins 23, 4. Four new reads have no disposition.
- `crates/geom-core/src/spline/compose.rs`: found 1 read, 0 asking;
  no roster row.
- `crates/topo/src/clearance.rs` was red too (57 found against 54
  pinned: one read from PR 7769's `sheet_latitude`, two from PR 7779's
  support ends in `window_of`); PR 7787 brings its row to 57/16 and
  dispositions all three.

The census is a slow-set row, so the PRs that moved these counts were
not gated on it.

## What is needed

Disposition the new reads in each file (ask `is_certified()` by name,
or argue each safe at its site and extend the file's note), and add a
roster row for `spline/compose.rs`.

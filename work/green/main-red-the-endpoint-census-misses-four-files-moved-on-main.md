---
id: main-red-the-endpoint-census-misses-four-files-moved-on-main
kind: issue
title: Main red: certified_endpoint_census counts four geom-brep and topo files that moved on main without re-blessing it
status: open
opened: 2026-11-17
priority: P1
cost: C
---

## Finding

`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
(a slow-set row, so it runs on no PR whose diff does not name it) is
red on main `af9d8c67a`. Measured on `embed/cylinder-vertex` after
merging that main. The branch re-blesses `crates/topo/src/injective.rs`,
its own file. Four other files read differently from their pins
(actual, then pinned, as reads and asking reads):

- `crates/geom-brep/src/pair_section/spline_implicit.rs`: 9, 7 against 6, 4;
- `crates/geom-brep/src/pair_section/spline_implicit/sweep/critical.rs`: 11, 2 against 37, 28;
- `crates/geom-brep/src/ssi/node.rs`: 28, 22, with no entry;
- `crates/topo/src/props/quad_lane.rs`: 27, 27 against 25, 25.

The moves look like SECTARMS' (`13fdb406e`, the node's Krawczyk
reading moved out of the sweep into `ssi/node.rs`; `6bfabd5cf`,
`c5566a263`) and a `quad_lane.rs` change. Not traced to the commit.

Each needs its disposition read at the site, not only a new count:
the census's rule is that a new read asks `is_certified()` by name or
is argued safe at the site.

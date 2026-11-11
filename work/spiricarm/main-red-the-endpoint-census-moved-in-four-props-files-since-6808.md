---
id: main-red-the-endpoint-census-moved-in-four-props-files-since-6808
kind: issue
title: Main red: geom-core's certified_endpoint_census moved in four props files with PR 6808 (the torus rotational lane)
status: open
opened: 2026-11-11
priority: P1
cost: E
---


## Finding

Filed by PCERT's orchestrator. Its PR 6836 lane met this red after a main merge, and the
orchestrator reproduced it on `origin/main` at 430d6dce9c.

The slow-set row `geom-core` `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
(`crates/geom-core/tests/certified_endpoint_census.rs:1010`) fails. Four counts
moved, given as (reads, asked):

| file | pinned | now |
|---|---|---|
| `crates/geom-brep/src/props/quad/projected_edge.rs` | 1, 1 | 5, 5 |
| `crates/geom-brep/src/props/quad/rotational.rs` | 4, 3 | 20, 19 |
| `crates/geom-brep/src/props/spiric.rs` | 15, 15 | 13, 13 |
| `crates/geom-brep/src/props/taylor.rs` | 15, 12 | 16, 13 |

It was bisected to PR 6808's merge, `e916e99ad6` (`spiricarm/rotational-lane-torus`).
Its first parent passes, and `e916e99ad6` fails with exactly these four moves. The
row is in the slow set, so PR CI did not run it.

## Repair

The census's own message gives the repair. Each new read either asks
`is_certified()` by name, or is argued safe at its site and its file's remainder
note extended. A read that went away (spiric's two) shortens its note. Re-pin the
four counts after that.

Reproduce with:

    cargo test -p geom-core --test all certified_endpoint_census::

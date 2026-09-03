---
id: one-declaration-guard-one-home-in-test-utils
kind: issue
title: The one-declaration assertion is copied verbatim into fourteen all.rs files, and pncad has no guard at all
status: open
opened: 2026-09-03
---


## Two facts, one shape

**1. The assertion has fourteen homes.** TCOST-B1 added a second assertion to
`every_suite_file_is_aggregated` — *no suite file declares a module of its
own* — and TCOST-B2 carried it into the rest, so the same ~25 lines (the
`found.iter().flat_map(…)` read, the `file_module_decls` call, the
`{rel}: mod {name};` formatting and the message naming the fix) are now
**byte-identical in fourteen `crates/*/tests/all.rs`**: bvh, editor-core, geom, geom-brep,
geom-core, mesh, profile, step-export, step-import, stl,
sweep, topo, verbs, viewer — every aggregated crate that has the guard.
The first assertion of the same fn is duplicated the same way and predates
both units.

The reader is already shared (`test_utils::source::file_module_decls`);
what is not is the assertion around it. One `test_utils::source` helper
taking the `tests/` root and the walked `found` list and returning (or
asserting) the violations would leave each `all.rs` with one call, and
would mean a change to the message or to the exemptions is one edit rather
than fourteen. The reason it is fourteen today is that B1 and B2 were widening a
guard, not designing one — and a copy that must be kept in sync in fourteen
places is the shape that drifts.

## 2. `pncad` is outside all of it

`crates/pncad/tests/all.rs` declares no suites with `#[path]` and has no
`every_suite_file_is_aggregated` at all — it is one file holding its own
rows rather than an aggregator over a `tests/` directory. So neither
assertion covers it: a suite file dropped into `crates/pncad/tests/` is
not forced into the binary by anything, and a `mod <helper>;` there is
caught by nothing. That may well be right for a crate with one test file,
but nothing in the tree says so and nothing would notice if the crate grew
a second one. Decide it explicitly: either the crate gets the walk and the
guard, or its `all.rs` says in one sentence why a crate with a single test
file does not need them.

## Where it came from

TCOST-B1 (#1616) and TCOST-B2 (#1669); raised in TCOST-B2's style review.
Neither unit is the place to fix it — B2's diff is the five crates' `mod`
lines, and moving the assertion into `test_utils` is a `src/` change its
keep-outs forbid.

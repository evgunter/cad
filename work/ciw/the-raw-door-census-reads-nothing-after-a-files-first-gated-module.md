---
id: the-raw-door-census-reads-nothing-after-a-files-first-gated-module
kind: issue
title: profile's raw_door_census cuts each file at its first gated module to the END OF FILE, so production code after it is never read: 93 src files, about 105k non-blank lines
status: open
opened: 2026-11-15
priority: P3
cost: M
---


Filed by GREEN's `green/nightly-1115` lane (PR 6985), from that PR's
review (S3). Pre-existing; not changed there.

`crates/profile/tests/raw_door_census.rs`, `test_module_starts_at`: a
file's test region begins at its first `mod` block carrying a test cfg
and "runs to the end of the file". Its doc says "every hit in this tree
is in a trailing test module, and a rule that OVER-counts test regions
can only make this census miss a writer, never invent one".

The first half no longer describes the tree. The review counted 93
`src/` files with production code after their first gated module, about
105k non-blank lines the census never reads, and planted a writer in
`crates/topo/src/census.rs` after `variety_reading_rows`: the census
stayed green. So `no_production_source_writes_a_vertex_table` is a guard
that cannot see roughly that much of what it is named for, and "can only
miss a writer" is the failure, not a mitigation.

**Owed.** Cut each gated module at its own extent
(`test_utils::source::attributed_extent`, or the `cfg_test_extents` walk
`source::test_mounted` already makes) instead of at the end of the file,
and keep the planted writer as a row. A related copy count is on
`work/ciw/production-view-blankers-four-copies-with-four-rules.md`.

Reproduce: add `let _ = ProfileLoop::new(v);` after the last gated
module of `crates/topo/src/census.rs` and run
`cargo nextest run -p profile -E 'test(no_production_source_writes_a_vertex_table)'`:
it passes.

Moved from TINT to CIW on 2026-11-20, by `git mv`, when TINT merged into CIW: TINT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.

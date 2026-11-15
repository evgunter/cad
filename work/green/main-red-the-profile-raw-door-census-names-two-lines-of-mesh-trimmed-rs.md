---
id: main-red-the-profile-raw-door-census-names-two-lines-of-mesh-trimmed-rs
kind: issue
title: Main red: profile's raw_door_census names crates/mesh/src/trimmed.rs twice (production source writes a vertex table by hand)
status: closed
opened: 2026-11-14
priority: P1
cost: E
parent: nightly-1115-residue-the-reds-no-lane-holds
closed: 2026-11-15
---


Reported by a BOXFINISH lane (`boxfinish/profile-directions`) and filed
by the BOXFINISH orchestrator, 2026-11-14. Measured on a clean detached
checkout of `origin/main` `2c843a2e`, CI's build shape, default ε, run
twice with the same first panic line:

`profile::all raw_door_census::no_production_source_writes_a_vertex_table`
panics at `crates/profile/tests/raw_door_census.rs` ("production source
writes a vertex table by hand"), naming `crates/mesh/src/trimmed.rs:2496`
and `:3017`. Both lines call `profile::test_support::bulge_loop`. It is
a source-text gate (it reads files and counts), so it cannot depend on
the box. `git blame` on main gives line 3017 to `91da2241e0`
(2026-11-14 08:03 UTC); line 2496's blame ends at the shallow
checkout's graft.

Not traced: whether the two calls sit in a test module the census's
cut does not see as one, or in production code. The lane that added
them owes the fix; the census names the recourse (author through the
`path` lattice, or list the table).

**The same lane saw twelve more rows red on that main, all already on
this slate** (in `nightly-reds-editor-core-digests-and-goldens`,
`nightly-reds-the-source-censuses`, `nightly-reds-default-eps-behaviour-rows`
and the `main-red-…` rows beside them): the name-digest and committed-
digest rows of `lib_g16`, `perf2`, `seat4` (two) and `seat7`, `perf12`'s
at-rest census golden, `sym11_the_exact_channel_never_contradicts_past_the_ceiling`,
`m10_5_clearance_interval::the_answer_is_deterministic_across_repeats`,
`node_standing::every_node_result_reader_goes_through_usable_or_is_listed`
(eval/mod.rs 35 against 33 listed; eval/parts.rs 2 against 1),
`a_plane_near_a_needle_cones_apex`, the seam-split census's settle
floor, and the four-thread mass-properties walk. Thirteen of thirteen
red, twice, on that commit.

## Closed (GREEN, `green/nightly-1115`, 2026-11-15)

**Traced: the sites are test code, and the census's cut missed the
module.** All four lines the nightly names (`crates/mesh/src/trimmed.rs`
2496, 3017, 3056, 3677) sit inside the file's trailing
`#[cfg(test)] pub(super) mod tests`. PR 6950 (`tesslane/chart-table`,
merge `b0f4b4fc3`) changed that declaration from `mod tests` to
`pub(super) mod tests`, and `raw_door_census.rs`'s `test_module_starts_at`
listed the visibilities it had met (`mod`, `pub mod`, `pub(crate) mod`),
so it read the whole test module as production source. No production
source writes a vertex table; nothing in `mesh` owes a fix.

Greened in the census: the cut takes a module declared at any
visibility (`after_mod_keyword`), `gated_module_files` reads through
the same helper, and
`the_test_region_starts_at_a_gated_module_of_any_visibility` holds it.

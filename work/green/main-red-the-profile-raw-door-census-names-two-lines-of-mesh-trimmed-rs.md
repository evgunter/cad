---
id: main-red-the-profile-raw-door-census-names-two-lines-of-mesh-trimmed-rs
kind: issue
title: Main red: profile's raw_door_census names crates/mesh/src/trimmed.rs twice (production source writes a vertex table by hand)
status: open
opened: 2026-11-14
priority: P1
cost: E
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

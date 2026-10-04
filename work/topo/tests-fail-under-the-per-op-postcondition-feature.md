---
id: tests-fail-under-the-per-op-postcondition-feature
kind: issue
title: Four euler and row_walk_proofs tests fail under topo/per-op-postcondition
status: open
opened: 2026-10-04
---


(TOPO implementer, found while verifying the stale-key row's second
unit; reproduced on `origin/main` at `1ef3e52e`, so it predates that
unit.)

## What

`cargo nextest run -p topo --all-features -E 'test(euler::) | test(row_walk)'`
on `origin/main` fails four tests:

- `euler::tests::a_fan_split_past_a_paired_tear_leaves_no_minted_key_in_an_orbit_error`
- `row_walk_proofs::a_diversion_paired_with_a_parent_loop_tear_passes_the_proof`
- `row_walk_proofs::row_walks_on_seeds_that_divert_the_sheet`
- `row_walk_proofs::row_walks_on_a_few_torn_bodies`

Each drives an operator over a deliberately torn body, and the
`per-op-postcondition` feature's tier-1 check after the operator fires
on the tear the row planted. Neither gate runs these under that
feature: the PR gate runs no `--all-features` tests, and the nightly's
feature row filters to `surgery::tests::`. So nothing reds today, and a
developer who runs the documented `cargo test -p topo --features
topo/per-op-postcondition` (`crates/topo/Cargo.toml`) meets four
failures that say nothing about their change.

## Direction

Either the rows opt out of the postcondition (they plant a tear on
purpose, so a postcondition firing on it is not the finding they test),
or the feature's documentation says which rows it excludes and why.

---
id: torn-body-rows-are-red-under-per-op-postcondition
kind: issue
title: Four topo rows that build torn bodies on purpose are red under --features per-op-postcondition: the operator's own tier-1 postcondition fires before the row reads its answer
status: open
opened: 2026-10-04
---


Found by PR 3985's `--all-features` battery (`reach/arc-from-pairing`);
reproduced on main `e4a0a0d18` with `cargo nextest run -p topo
--features per-op-postcondition`, the other topo features off.

## What

`per-op-postcondition` (`crates/topo/Cargo.toml:53`) runs the
whole-body tier-1 sweep after every Euler operator, inside surgery
scopes too, as a debug assertion (`crates/topo/src/surgery.rs:294`).
The feature's doc says it "changes no result: it only decides where a
debug assertion fires". Four rows build a torn body on purpose and read
what an operator does with it, so under the feature the operator's own
postcondition panics first:

- `crates/topo/src/euler.rs`
  `a_fan_split_past_a_paired_tear_leaves_no_minted_key_in_an_orbit_error`
  (`mev_null`; also noted on
  `work/topo/vertex-orbit-answers-part-of-a-torn-orbit.md`);
- `crates/topo/src/row_walk_proofs.rs:408`
  `a_diversion_paired_with_a_parent_loop_tear_passes_the_proof`,
  `:760` `row_walks_on_a_few_torn_bodies`, `:776`
  `row_walks_on_seeds_that_divert_the_sheet` (`kfmrh`).

The hosted CI test job does not enable the feature, so it is green.
A run with `--all-features`, which the implementer battery asks for,
is not.

## Done when

The four rows run under the feature: either they scope the
postcondition off for the operator they feed a torn body (the body is
the subject, not a bug), or the feature's contract names the rows it
cannot run. `cargo nextest run -p topo --all-features` is green.

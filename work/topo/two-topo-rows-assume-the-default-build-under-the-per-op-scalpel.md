---
id: two-topo-rows-assume-the-default-build-under-the-per-op-scalpel
kind: issue
title: Two topo rows are red under topo/per-op-postcondition: a tier-1 derivation count and a should_panic message both read the default build
status: open
opened: 2026-10-28
priority: P3
cost: E
---


Filed by GREEN's `green/gate-trust` lane (2026-10-28). Found by the
suite run under `--workspace --exclude viewer --all-features`, from a
temporary copy of the nightly row this lane adds: run `40720990736`,
job `122931500237`, on `origin/main` `4beeeb05`. Both rows pass at
default features. Both are red only because topo's `per-op-postcondition`
scalpel sweeps tier 1 after every operator
(`Body::tier1_sweep_is_mine`, `crates/topo/src/surgery.rs`).

- `crates/topo/src/boolean/edge_join.rs`,
  `join_door::a_join_sweeps_tier_1_once_however_many_it_joins`:
  `join_edges: tier-1 derivations at k = 3 and k = 5 ([7, 11])`. The row
  counts `validate::TIER1_DERIVATIONS`, and the scalpel adds a sweep per
  operator, so the count grows with `k`. That is the scalpel working as
  designed, not the join walking more.
- `crates/topo/src/offset_together.rs`,
  `scope_walks::the_door_panics_on_an_out_of_scope_malformed_solid`:
  `#[should_panic(expected = "offset_planes_together postcondition: …")]`.
  Under the scalpel, the planted torn loop is caught one operator
  earlier, by `set_face_surfaces_describing postcondition: result is not
  tier-1 valid`, with `DanglingTopology { from: HalfEdge(47v1), to:
  HalfEdge(null) }`, so the expected text does not match. On 2026-10-11
  this row still passed under `--all-features` (see
  `mfkrh-leaves-an-orphan-surface-under-the-per-op-scalpel`), so the
  inner operator that catches it first is newer than that.

Reproduce: `cargo nextest run -p topo --features per-op-postcondition
-E 'test(a_join_sweeps_tier_1_once_however_many_it_joins) |
test(the_door_panics_on_an_out_of_scope_malformed_solid)'`.

Pinned in the same PR with
`#[cfg_attr(feature = "per-op-postcondition", ignore = "pinned red: work/topo/two-topo-rows-assume-the-default-build-under-the-per-op-scalpel")]`
on both rows. The default build still runs and asserts them.

## Repair shape

Make each row hold under the scalpel, then delete both `cfg_attr` pins
in the PR that closes this row: a pin is the red parked, not a
statement of what the row claims. The derivation-count row could count
only the join's own sweeps, leaving out the scalpel's per-operator ones.
The should-panic row could plant its tear where the first catching
operator is `offset_planes_together`, or accept either postcondition's
message. The torn-body rows that
`torn-body-rows-are-red-under-per-op-postcondition` closed are the
precedent: they were fixed with `on_a_torn_body` in `surgery.rs`.

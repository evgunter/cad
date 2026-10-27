---
id: two-door-rows-red-under-the-per-op-postcondition-scalpel
kind: issue
title: Two offset door rows go red under topo's per-op-postcondition feature: a_join_sweeps_tier_1_once since PR 5666, the_door_panics since PR 5661
status: open
opened: 2026-10-27
priority: P3
cost: E
refs: [no-ci-row-runs-topo-under-the-per-op-postcondition-scalpel, 5661, 5666]
---

Filed by QUAD's main-red triage lane (branch `quad/main-red-triage-1027`).
Both rows are green in every default build, at every ε. They go red only
when `topo` is built with `per-op-postcondition`, which `--all-features`
turns on. No CI row builds topo that way
(`work/ciw/no-ci-row-runs-topo-under-the-per-op-postcondition-scalpel.md`).

## What

Reproduce: `cargo nextest run -p topo --features per-op-postcondition -E 'test(a_join_sweeps_tier_1_once) | test(the_door_panics_on_an_out_of_scope)'`
on main `3a0cd80f`.

- `boolean::edge_join::join_door::a_join_sweeps_tier_1_once_however_many_it_joins`
  (`crates/topo/src/boolean/edge_join.rs`, the `assert_eq!` on
  `derivations` at :1374): `left: 7, right: 11`. The row counts
  `validate::TIER1_DERIVATIONS` at k = 3 and k = 5 joins and pins them
  equal. The feature runs the whole-body sweep after every Euler operator
  by design, so the count grows with k under it. It has been red under the
  feature since PR 5666 added it (`8b1a19fa`).
- `offset_together::scope_walks::the_door_panics_on_an_out_of_scope_malformed_solid`
  (`crates/topo/src/offset_together.rs`, `#[should_panic(expected =
  "offset_planes_together postcondition: …")]`): the panic now comes
  from the first setter inside the door's scope (`crates/topo/src/surgery.rs`
  :396, `set_face_surfaces_describing postcondition: result is not
  tier-1 valid`, `DanglingTopology` on the broken loop), so the expected
  door name is missing. The feature exists to make the panic name the
  operator ("the panic names the operator instead", `crates/topo/Cargo.toml`
  `per-op-postcondition`).

**Bisect for the second row** (first-parent, `topo` lib tests with the
feature on): green at `67006526` (PR 5629) and at `31d99e6f`, red at
`84650d2e`, the merge of **PR 5661** (`offset/door-end-tier1-once`).
`1972d162` does not build under the feature (`PcurveMintError::OuterSpanPoisoned`
non-exhaustive), so the bisect stepped around it.

## Repair shape

Both rows pin a door-level property that the feature changes on
purpose. Gate each with `#[cfg(not(feature = "per-op-postcondition"))]`
and give the reason in one line, or make the `should_panic` row accept
either name. Changing the kernel is not the fix.

---
id: topo-calls-k-stats-decide-doors-past-validates-wrappers
kind: issue
title: topo calls the k_stats decide doors directly at 47 sites in 30 files, past validate.rs's pass-through wrappers
status: open
opened: 2026-11-20
priority: P4
cost: E
---


Found by CHARTTAIL's `charttail/decided-gates` review (PR 7168), which
moved `chart_region.rs`'s `decide_nonzero` import onto `crate::validate`.

`crates/topo/src/validate.rs` holds crate doors for `decide`,
`decide_positive`, `decide_reported` and `decide_nonzero`. Each is a
pass-through to `geom_core::k_stats`. Most of topo calls the
`k_stats` doors directly. On main `1ee8df51`, this grep, run over
`crates/topo/src` with `validate.rs` excluded, reports 47 lines in 30
files:

```
grep -rnE "k_stats::(\{[^}]*\b(decide|decide_positive|decide_nonzero|decide_reported)\b[^}]*\}|(decide|decide_positive|decide_nonzero|decide_reported)\b)" crates/topo/src
```

Some examples: `boolean/sectors.rs` (near `:816`), `boolean/reduce.rs`
(×7, `decide_reported`), `census.rs` (`use … {decide_reported, …}` plus
direct calls), `shell.rs`, `turn_rows.rs`, `offset_axial.rs`,
`offset_restate.rs`, `ray_walk.rs`, `splitting/pieced_arc.rs`. Some
hits are in test modules (`splitting/rules.rs:1208`, `census.rs:13076`,
`chart_region.rs`'s tests).

## The question before the sweep

There are two spellings of one door, and either one can be the home:
- route every production caller through `validate`, or
- delete the pass-throughs and call `k_stats` everywhere.

The wrappers add nothing: same signature, same recording. Deleting
them would remove the second spelling instead of enforcing it. Decide
which before the mechanical pass. Either result is a mechanical rename.

The grep cannot see an alias import (`use geom_core::k_stats as ks`)
or the doors it does not name (`decide_magnitude`, `decide_negative`,
`decide_across`). `validate` has no wrapper for those.

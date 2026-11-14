---
id: a-split-reduce-on-a-torn-body-panics-at-pcurves-unreachable-own-loop
kind: issue
title: split_reduce on a torn body panics at pcurves.rs's unreachable! ("a member of its own loop") instead of a row-four premise
status: open
opened: 2026-11-14
priority: P2
---



Found by the skin-turn lane (PR 6949): CI's `corrupt input (release profile)` job, on a diff that does not touch `topo`. The fuzzer `review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` (`crates/topo/src/review_d18.rs`, its assertion near :3549) drew seed `0xcad1e9c2ceac697e` at effort 1. `split_reduce` then panicked naming no row-four premise:

```
panicked at crates/topo/src/pcurves.rs:3301:36:
internal error: entered unreachable code: HalfEdgeKey(4v1) is a member of its own loop
```

Replay with `CAD_FUZZ_SEED=0xcad1e9c2ceac697e cargo test -p topo --release --lib review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise`. The line number is CI's merge of the branch with main; on main the `unreachable!` is the one whose message reads "is a member of its own loop", in `crates/topo/src/pcurves.rs`.

Owed: a torn body reaching that walk refuses naming a row-four premise, as the fuzz row asserts of every read, and the seed is pinned as a deterministic row beside the fix (`docs/prompts/implementer-discipline.md` §8).

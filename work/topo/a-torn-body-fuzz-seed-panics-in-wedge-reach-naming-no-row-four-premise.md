---
id: a-torn-body-fuzz-seed-panics-in-wedge-reach-naming-no-row-four-premise
kind: issue
title: review_d18's torn-body fuzz at seed 0x74818d5bf00d4475 panics in wedge_reach naming no row-4 premise
status: open
opened: 2026-11-17
priority: P1
cost: M
---


Found by the band-cut3 lane (PR 7072's fix pass): hosted CI's `corrupt input (release profile)` job at 958758cd22, which touches no topo source (its tree's `crates/topo` is origin/main's at a9b4bbe5aa).

`review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` (`crates/topo/src/review_d18.rs`, about line 3549) fails at fuzz seed `0x74818d5bf00d4475`. `classify_neighborhood` panics in `crates/topo/src/wedge_reach.rs` (about line 182) with `unreachable!`: "HalfEdgeKey(6v1), a half of live EdgeKey(3v1), has no face". That panic names no row-4 premise.

Reproduce, red locally:

```
CAD_FUZZ_SEED=0x74818d5bf00d4475 CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false \
  cargo test --release -p topo --lib -- review_d18
```

A counterexample the fuzz found, not a flake. It makes the job red on any PR that draws this seed, and the nightly whenever it is drawn. Pin it as a deterministic row beside the fix. The torn half reached by the walk should refuse typed, or its panic should name the premise.

A second seed, `0xe25782c11a6b4c1e`, gives the same panic (the same `HalfEdgeKey(6v1)` of `EdgeKey(3v1)`). It was drawn by hosted CI's `corrupt input (release profile)` job on the box-certify-ask branch, and it replays red on clean origin/main at d938cbd8 with `CAD_FUZZ_SEED=0xe25782c11a6b4c1e CARGO_PROFILE_TEST_DEBUG_ASSERTIONS=false cargo test -p topo --lib review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise`. The draw is not rare.

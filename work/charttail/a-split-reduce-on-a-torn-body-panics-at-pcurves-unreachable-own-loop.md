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

It reproduces on main (`1cf560ac4`), the same panic at `crates/topo/src/pcurves.rs:3301`: `CAD_FUZZ_SEED=0xcad1e9c2ceac697e CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false cargo test --release -p topo --lib -- review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` (the row is compiled only without debug assertions, as CI's job runs it). The `unreachable!` is the one whose message reads "is a member of its own loop".

Owed: a torn body reaching that walk refuses naming a row-four premise, as the fuzz row asserts of every read, and the seed is pinned as a deterministic row beside the fix (`docs/prompts/implementer-discipline.md` §8).

- 2026-11-14 — **a second seed, from TESSLANE's PR 6950** (a diff that touches no `topo` source): the same job drew `0x8fae65652c5c7298` and panicked at the same site on `HalfEdgeKey(3v1)`. It reproduces on main (`e739c4a36c`) with the command above and that seed.


- 2026-11-15 — **a third seed, from BOXFINISH's PR 6958** (its diff touches no file under `pcurves/`): the same job drew `0xd172944dfec60af3` and panicked on `HalfEdgeKey(4v1)` at the same `unreachable!`, now at `crates/topo/src/pcurves/site.rs` (the joints' own-loop lookup, near :381) since the file split. Not replayed on main by that lane.

- 2026-11-15 — **a fourth seed, from BOXES' PR 6976** (its diff touches no file under `pcurves/` or `splitting/`): the same job drew `0xd3ff1f8c5d8b60f6` and panicked on `HalfEdgeKey(4v1)` at `crates/topo/src/pcurves/site.rs:381`. It reproduces on main (`0807c130bf`'s successor `dc0fda30ba`) with the command above and that seed.

- 2026-11-15 — **a fifth seed, from BOXFINISH's PR 6979** (its diff touches no file under `pcurves/`, `splitting/` or `review_d18.rs`): the same job drew `0x1b0e0949d80fc645` on head `7bded12f` (run 43734867600) and panicked on `HalfEdgeKey(4v1)` at `crates/topo/src/pcurves/site.rs:381`; the head before it, with the same code, was green. Not replayed on main by this lane. Five PR gates in two days have gone red on this row for a diff that does not reach it, which is friction for every program whose gate runs the corrupt-input job. (BOXFINISH orchestrator)

- 2026-11-16 — **a sixth seed, from SECTARMS' PR 7028** (its diff touches no file under `pcurves/`, `splitting/site` or `review_d18.rs`): the same job drew `0x67c1bc918fe6753f` on head `3d06fe5550` (run 43968440036) and panicked on `HalfEdgeKey(3v1)` at `crates/topo/src/pcurves/site.rs:381`. Not replayed on main by this lane.

- 2026-11-17 — **a seventh seed, from TESSLANE's PR 7049** (its diff is `crates/mesh` and `.config/nextest.toml`): the same job drew `0x9bc8f040d42bea99` on the work-in-progress head `9e9a2b151c` (run 44038615732) and panicked on `HalfEdgeKey(3v1)` at `crates/topo/src/pcurves/site.rs:381`; the heads either side of it were green. Not replayed on main by this lane.

## Another seed (2026-11-17, from PR 7052's hosted run)

`topo`'s `review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise`
(release profile, the `corrupt input` job) drew
`CAD_FUZZ_SEED=0x8d944d546a8687a6` and failed: `split_reduce` panicked
at `pcurves/site.rs` with "HalfEdgeKey(4v1) is a member of its own
loop", naming no row-4 premise. Replayed with that seed, in release, on
main at `fa4e9ba921`: the same failure (run).

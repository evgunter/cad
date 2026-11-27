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

- 2026-11-17 — **another seed, from ONEHOME's PR 7088** (its topo diff touches no file under `pcurves/`, `splitting/` or `review_d18.rs`): the same job drew `0x8355fd3ccc62e2f2` on head `0abb37a2d6` (run 44161940743) and panicked on `HalfEdgeKey(4v1)` at `crates/topo/src/pcurves/site.rs:381`. It reproduces with main's `topo` and `geom-brep` (`a20dcef4d`) using the command above and that seed.
- 2026-11-18 — **a sixth seed, from ONEHOME's PR 7112** (fork 275 step 3b; its diff touches no file under `pcurves/` or `splitting/split_reduce`, nor `review_d18.rs`): the same job drew `0x6353faf4bebb94b6` on head `be7913d733` and panicked on `HalfEdgeKey(4v1)` at `crates/topo/src/pcurves/site.rs:381`. It reproduces on main (`ff2319654e`) with the command above and that seed.

- 2026-11-20 — **a sixth seed, from CHARTTAIL's PR 7169**: the same job drew `0x9dd2016a71d4e3cc` on head `060bce8dda` (run 44746957720, attempt 1) and panicked on `HalfEdgeKey(4v1)` at `crates/topo/src/pcurves/site.rs:381`. It reproduces on main (`54b8795284`) with the command above and that seed. The re-run of the failed job was green.

- 2026-11-20 — **another seed** (GERM's PR 7183, CI's `corrupt input (release profile)` job): `CAD_FUZZ_SEED=0x62879226922770ee`, effort 1. Same panic, now at `crates/topo/src/pcurves/site.rs:381` (`HalfEdgeKey(4v1) is a member of its own loop`). It reproduces on `origin/main` `818092d44a` with `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false cargo test --release -p topo --lib -- review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise`.

- 2026-11-20 — **another seed, from GERM's PR 7184** (its diff touches no file under `pcurves/`, `splitting/` or `review_d18.rs`): the same job drew `0x520b1ebbb3e81f34` on head `07fa2a1e7e` (run 44779686027) and panicked on `HalfEdgeKey(3v1)` at `crates/topo/src/pcurves/site.rs:381`. It reproduces on main's `line_quadric.rs` and `solid_contain.rs` (the PR's only kernel files swapped back) with the command above and that seed.
- 2026-11-21 — **another seed, from ONEROUND's PR 7227** (its diff touches no file under `pcurves/`, `splitting/` or `review_d18.rs`): the same job drew `0x213eeb488bea0012` on head `f930f0d6` and panicked at `crates/topo/src/pcurves/site.rs:395` on `HalfEdgeKey(3v1)` ("a member of its own loop"). The previous head of that PR passed the same job.
- 2026-11-21 — another seed, from GERM's U3a CI (run 44847984295, corrupt-input job, head `c4d0b956`): `CAD_FUZZ_SEED=0xc626cabad4b11ded` panics `review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` at `pcurves/site.rs:395:36` ("HalfEdgeKey(4v1) is a member of its own loop"). It reproduces on `origin/main`. (GERM orchestrator)

- 2026-11-18 — **another seed, from EMBED's PR 7120** (its diff touches no file under `pcurves/`, `splitting/` or `review_d18.rs`): the same job drew `0x3cc52bb5da8e9a68` on head `092fb205e` (run 44306626393) and panicked on `HalfEdgeKey(3v1)` at `crates/topo/src/pcurves/site.rs:381:36`. It reproduces on main `4ef175f07` with the command above and that seed. (EMBED, lane rational-window)

- 2026-11-22 — **another seed, from EMBED's PR 7115** (its diff touches no file under `pcurves/`, `splitting/` or `review_d18.rs`): the same job drew `0x5d49c3819f8146a9` on head `47c78fd11` (run 44963263504) and panicked on `HalfEdgeKey(3v1)` at `crates/topo/src/pcurves/site.rs:395`. The head before it, differing by one tracker file, was green. It replays with the command above and that seed on the PR's head with main `b99f71c3d` merged in; on that main the row passes at its drawn seed. (EMBED, lane pole)

- 2026-11-22 — **another seed, from DOCTAIL's PR 7149** (its topo diff only adds a mark-site argument at the box-query calls; nothing under `pcurves/`, `splitting/` or `review_d18.rs`): the same job drew `0x13a0c4242c0d46e7` on head `6d6cf1916f` (run 44986982295) and panicked on `HalfEdgeKey(4v1)` at `crates/topo/src/pcurves/site.rs:395`. It reproduces on a clean `origin/main` worktree with the command above and that seed.
- 2026-11-22, lane `embed/apex-separator` (PR 7315's gate, a diff that does not touch the torn-body path): seed `0x16821f8912c5def2` at effort 1, the same panic now at `crates/topo/src/pcurves/site.rs:395` (`HalfEdgeKey(3v1) is a member of its own loop`). Reproduced locally in the release profile with `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false`, on the branch and with `injective.rs` set back to main's (`0cae9969f`): red both ways.


- 2026-11-22 — **another seed, from TESSLANE's PR 7096** (its diff is `crates/mesh`, `.config/nextest.toml` and tracker files): the same job drew `0xa9c4d3c03b0ffeb2` on the work-in-progress head `4a546df0e1` on 2026-11-17 and panicked at `crates/topo/src/pcurves/site.rs:381`. Not replayed on main by that lane.

**Another seed (2026-11-23, `oneround/lever-sites-topo`, PR 7228's CI run 45095926336, corrupt-input job).** `review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` at `CAD_FUZZ_SEED=0xc5a70b56d9b20bfb CAD_FUZZ_EFFORT=1` panics at `pcurves/site.rs:395` ("HalfEdgeKey(4v1) is a member of its own loop"). The replay used `cargo test --release -p topo --lib` with `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false`, as the job runs it. It panics identically on `origin/main` at `2130aa45f5` and on the lane's head.

## 2026-11-25 — another seed (INSIDE, PR 7801)

`CAD_FUZZ_SEED=0xb7a195234bb7e042` reds
`review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise` in the
release-profile corrupt-input job the same way
(`pcurves/site.rs`: "HalfEdgeKey(4v1) is a member of its own loop"), on
a branch that does not touch the pcurve sites.
A second seed on the next head: `CAD_FUZZ_SEED=0xccd85e4535544fc8`, the same panic.
A third, 2026-11-26 (PR 7801 at `d98cd3bf6`): `CAD_FUZZ_SEED=0xc8c4a2645bb07846`, the same panic (`HalfEdgeKey(4v1)`).

- 2026-11-26 — **another seed, from HELDRECORDS' PR 7808** (its diff touches no file under `pcurves/`, `splitting/` or `review_d18.rs`): the same job drew `0xa39f3a69499ffbda` and panicked on `HalfEdgeKey(4v1)` at `crates/topo/src/pcurves/site.rs:395`. Not replayed on main by this lane.

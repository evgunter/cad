---
id: nextest-shard-count-needs-remeasure
kind: issue
title: Determine the right nextest shard count (blocked on the test-speedup work)
status: parked
opened: 2026-08-13
github: 461
blocked_on: [tcost]
refs: [449]
---

## From GitHub issue 461

Opened 2026-08-13; 0 comments.

The 2-way `--partition count:N/2` sharding of the `test` and `test-interval` matrices was sized against a much slower suite. **The arithmetic that justified it has changed, and the arithmetic that would justify changing it isn't stable yet.**

## Do not act on the numbers below yet

**There is a parallel effort to speed up the tests themselves. That needs to land before this is measured** — otherwise the shard count gets tuned against a test suite that is about to change underneath it, and we redo the work. This issue exists to hold the analysis, not to be actioned now.

## What was measured (2026-08-12, pre-opt-2)

Across 11 full runs, per-leg:

* **Fixed cost per leg: 15 s** (job wall minus nextest's own reported elapsed, median over 20 legs). Checkout ~3 s, nextest binary from cache ~2 s, artifact download ~3 s, archive extraction 0.7 s. No compile, no rust-cache.
* Modelled 2 → 4 shards: **wall ~17.8 min → ~9.6 min (−46%), billed ~137 → ~146 min (+7%)**. Most of the +7% is per-job minute *rounding*; real added work was only 10 × 15 s = 2.5 min.
* **nextest already saturates both vCPUs** — Σ(per-test time)/wall was 1.97–2.00× on every one of the 20 legs. There is no packing slack; shards are the only lever.

## Two findings that survive regardless of speed work

**1. The imbalance is structural, not luck.** 10 of the 12 slowest tests land in shard 2 — identically, in both lanes, in every run sampled. `--partition count:N/2` is deterministic by list position and reads no timings. Median imbalance:

| matrix row | shard 1 | shard 2 | ratio |
|---|---|---|---|
| interval, eps = default | 509 s | **828 s** | **1.63×** |
| eps = default | 484 s | 753 s | 1.56× |
| eps = 1e-12 | 442 s | 667 s | 1.51× |
| eps = 1e-6 | 476 s | 482 s | 1.01× |

Rebalancing the *existing* 2-way split was worth ~160 s of wall on the critical leg, for free. `--partition hash:N/M` randomises placement but does not balance by weight; neither mode reads timings.

**2. There is a hard floor.** `step-import::all rw2_probes::probe_round_trip_bit_identity_and_reorder` was 296 s — no sharding scheme goes below its longest single test. Modelled N=6 bought only 27 s more wall than N=4 for +48 billed minutes.

## Why #449 changed the premise

opt-level 2 on the archive jobs (#449) took the critical leg from **828 s to 117 s**. Against a 15 s per-leg fixed cost, that constant is now ~13% of a leg instead of ~2%. More shards buy much less and cost proportionally more — the 2→4 case may now argue the *other* way. This is a re-measure, not a re-derive.

## What to do when the speedup work lands

1. Re-measure the per-leg fixed cost (it may also have moved — artifact size changed with opt-2).
2. Re-measure per-test times and the shard-2 imbalance; check whether the same tests still dominate.
3. Only then decide N, and consider whether weight-aware partitioning is worth it versus just picking a better N.

Full write-up and method: `docs/GENERICS-BUILD-COST.md`, and `docs/LOCAL-BUILD-PERF.md` for the machine-variance caveats.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

## Home

S-TCOST: the issue's own block — "the parallel effort to speed up the tests themselves" — is this program, and CI sharding is named in its `keep_out` as out of scope unless a unit's measurement makes the case in its own PR, so the analysis parks here rather than travelling.

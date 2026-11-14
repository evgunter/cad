---
id: main-red-the-gather-records-one-sample-tier-3-prime-does-not
kind: issue
title: main red (probe roster): assemble_one_local_battery's gather records 2409 samples and tier 3′ 2408 since PR 6380, so scripts/k_probe_sweep.sh stops at its first plain suite
status: open
opened: 2026-11-09
priority: P2
cost: E
---


Filed by `onecopy/main-reds-k-and-m10`, which needs the K probe sweep to run to its end.

## What

`crates/editor-core/tests/assemble_one_local_battery.rs`,
`the_gather_pays_tier_3_prime_once_and_the_at_rest_check_nothing`
(`--features probe`, the first suite `scripts/k_probe_sweep.sh` runs in its
plain half), fails at ε 1e-9 with `the gather recorded 2409 samples and
tier 3′ 2408: the gather must pay tier 3′ once`. The row asserts that the
product's gather records exactly the samples `topo::validate_pseudomanifold`
records on the same body; one sample is now recorded by the gather and not
by tier 3′.

## Where it began

Bisected first-parent over main from `27bcf1be41` (green): merge
`b2cc80453b`, PR 6380 (`onehome/run-owns-rounding`, "the run owns f64
rounding — admission rule on (ε, K), floor charges out, session box at the
finished-body gate, bare-rounding gate"). Its first parent `5f2af4e111`
passes. The PR's body does not mention the row; it runs only under
`--features probe`, which no PR gate runs.

Unverified reading: the session box read at the finished-body gate is a
decision the gather takes and tier 3′ does not. If so, either the gather's
census is no longer tier 3′'s alone and the row's claim moves, or the box
read belongs inside tier 3′.

Reproduce: `CAD_TOLERANCE_EPS=1e-9 cargo test -p editor-core --features
probe --test all -- assemble_one_local_battery::`.

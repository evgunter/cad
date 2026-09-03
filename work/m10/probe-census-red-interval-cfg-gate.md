---
id: probe-census-red-interval-cfg-gate
kind: issue
title: k-lint probe-suite-census red on main: m10_3_driver_k_probe_interval builds no test (cfg-gated behind interval, which the probe loop lacks)
status: open
opened: 2026-08-30
github: 1288
refs: [1256]
---

## From GitHub issue 1288

Opened 2026-08-30; 0 comments.

Found by the VERBS-AZIMUTH lane (#1256's CI): `k-lint (gate)`'s probe-suite-census row reds on main — `editor-core`'s `m10_3_driver_k_probe_interval` is gated `#![cfg(all(feature = \"probe\", feature = \"interval\"))]` and the probe loop builds without `interval`, so the census sees a declared probe suite that built no tests. Byte-identical to origin/main in the finding lane; k-lint samples 1-of-5 rows per run, which is why it surfaces intermittently (the sampled-axis class). @ m10 — the m10-3 driver lane's to fix (enable the feature in the probe loop for that target, or un-gate, or register the exemption).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_016pYMaeU4woYZN8YGdTLfSK

## Home

`work/m10/` — `crates/editor-core/tests/m10*` is an M10 territory glob and the issue routes the fix to the M10-3 driver lane by name.

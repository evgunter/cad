---
id: probe-census-red-interval-cfg-gate
kind: issue
title: k-lint probe-suite-census red on main: m10_3_driver_k_probe_interval builds no test (cfg-gated behind interval, which the probe loop lacks)
status: closed
opened: 2026-08-30
github: 1288
refs: [1256]
closed: 2026-09-03
pr: 1670
---

## From GitHub issue 1288

Opened 2026-08-30; 0 comments.

Found by the VERBS-AZIMUTH lane (#1256's CI): `k-lint (gate)`'s probe-suite-census row reds on main — `editor-core`'s `m10_3_driver_k_probe_interval` is gated `#![cfg(all(feature = \"probe\", feature = \"interval\"))]` and the probe loop builds without `interval`, so the census sees a declared probe suite that built no tests. Byte-identical to origin/main in the finding lane; k-lint samples 1-of-5 rows per run, which is why it surfaces intermittently (the sampled-axis class). @ m10 — the m10-3 driver lane's to fix (enable the feature in the probe loop for that target, or un-gate, or register the exemption).

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_016pYMaeU4woYZN8YGdTLfSK

## Home

`work/m10/` — `crates/editor-core/tests/m10*` is an M10 territory glob and the issue routes the fix to the M10-3 driver lane by name.

## Closed (2026-09-03, the M10-6 lane's k-probe hotfix branch)

Fixed on main by PR #1268 before this lane opened, and closed here with
the evidence rather than left standing: `.github/workflows/ci.yml`'s
`compile and list every probe-gated test target` step DERIVES the extra
features from the censused gate lines themselves, so this suite's
`all(feature = "probe", feature = "interval")` gate is built with
`--features probe,interval` and lists. Verified in this lane by running
the step's own loop for `editor-core`: `extra=interval`, and
`probe-suite-census.sh --check-listing editor-core` reports every
counted suite built and listed. `scripts/k_probe_sweep.sh`'s `feats_for`
carries the same fact one step later, and both the census `--selftest`
and the bare census report run green here.
